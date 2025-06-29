use std::collections::HashMap;

use super::*;

#[derive(Debug)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action<'g> {
    Shift(StateIndex),
    Reduce(Rule<'g>),
    Halt,
}

#[derive(Debug)]
pub struct ParseTable<'g> {
    grammar: &'g Grammar,
    states: Vec<State<'g>>,
    actions: HashMap<(StateIndex, Option<Symbol<'g>>), Vec<Action<'g>>>,
}

#[derive(Debug)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct StateIndex(usize);

impl From<StateIndex> for usize {
    fn from(value: StateIndex) -> Self {
        value.0
    }
}

#[derive(PartialEq, Eq)]
pub struct State<'g> {
    itemset: ItemSet<'g>,
}

impl<'g> std::fmt::Debug for State<'g> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.itemset.items())
    }
}

impl<'g> ParseTable<'g> {
    pub fn new(grammar: &'g Grammar, start_rule: Rule<'g>) -> ParseTable<'g> {
        let states = Self::build_states(&grammar, start_rule);
        let actions = Self::build_actions(&grammar, &states, start_rule);

        ParseTable {
            grammar,
            states,
            actions,
        }
    }

    pub fn grammar(&self) -> &'g Grammar {
        self.grammar
    }

    fn build_states(grammar: &'g Grammar, start_rule: Rule<'g>) -> Vec<State<'g>> {
        let mut states = vec![];

        let start_state = State {
            itemset: ItemSet::singleton(start_rule.item(0)),
        };

        let mut states_remaining = vec![start_state];

        while let Some(state) = states_remaining.pop() {
            for symbol in grammar.symbols() {
                let next_state = State {
                    itemset: state.itemset.follow(symbol),
                };

                if next_state.itemset.is_empty() {
                    continue;
                }

                if !states.contains(&next_state) {
                    states_remaining.push(next_state);
                }
            }

            states.push(state);
        }

        states
    }

    fn build_actions(
        grammar: &'g Grammar,
        states: &[State<'g>],
        start_rule: Rule<'g>,
    ) -> HashMap<(StateIndex, Option<Symbol<'g>>), Vec<Action<'g>>> {

        let mut actions = HashMap::new();

        // Pre-allocate an empty list for all (state_i, maybe_symbol)-pairs
        for (src_state_index, _src_state) in states.iter().enumerate() {
            let src_state_index = StateIndex(src_state_index);
            for symbol in grammar.symbols() {
                let key = (src_state_index, Some(symbol));
                actions.insert(key, vec![]);
            }
            actions.insert((src_state_index, None), vec![]);
        }

        for (src_state_index, src_state) in states.iter().enumerate() {
            let src_state_index = StateIndex(src_state_index);
            for src_item in src_state.itemset.items() {
                match src_item.next_symbol() {
                    Some(symbol) => {
                        let dst_state = src_state.itemset.follow(symbol);
                        let dst_state_index = Self::state_index(&dst_state, states);
                        let key = (src_state_index, Some(symbol));
                        let actions_for = actions.get_mut(&key).unwrap();

                        let action = Action::Shift(StateIndex(dst_state_index));
                        if !actions_for.contains(&action) {
                            actions_for.push(action);
                        }

                    }
                    None => {
                        for symbol in grammar.symbols() {
                            let key = (src_state_index, Some(symbol));
                            let actions_for = actions.get_mut(&key).unwrap();
                            actions_for.push(Action::Reduce(src_item.rule()));
                        }

                        let key = (src_state_index, None);
                        let actions_for = actions.get_mut(&key).unwrap();
                        actions_for.push(Action::Reduce(src_item.rule()));
                    }
                }
            }
        }

        let key = (StateIndex(0), Some(start_rule.lhs()));
        actions.get_mut(&key).unwrap().insert(0, Action::Halt);

        actions
    }

    fn state_index(itemset: &ItemSet, itemsets: &[State]) -> usize {
        itemsets
            .iter()
            .enumerate()
            .find_map(|(j, st)| {
                if itemset == &st.itemset {
                    Some(j)
                } else {
                    None
                }
            })
            .unwrap()
    }

    pub fn conflicts(&self) -> Vec<Conflict> {
        let mut conflicts = vec![];
        for (state_index, _state) in self.states.iter().enumerate() {
            let state_index = StateIndex(state_index);
            for symbol in self.grammar.symbols() {
                let key = (state_index, Some(symbol));
                let actions = &self.actions[&key];
                if actions.len() > 1 {
                    conflicts.push(Conflict {
                        table: self,
                        state: state_index,
                        symbol: Some(symbol),
                        actions: actions.clone(),
                    });
                }
            }
        }
        conflicts
    }
}

#[derive(Clone)]
pub struct Conflict<'g, 't> {
    table: &'t ParseTable<'g>,
    state: StateIndex,
    symbol: Option<Symbol<'g>>,
    actions: Vec<Action<'g>>,
}

impl<'g, 't> std::fmt::Debug for Conflict<'g, 't> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_id = self.state;
        let symbol = self.symbol;
        let actions = &self.actions;
        write!(f, "Conflict(state={state_id:?}, symbol={symbol:?}, actions={actions:?})")?;
        Ok(())
    }
}

impl<'g, 't> Conflict<'g, 't> {
    pub fn table(&self) -> &'t ParseTable<'g> {
        self.table
    }

    pub fn state(&self) -> &'t State {
        &self.table.states[usize::from(self.state)]
    }

    pub fn symbol(&self) -> Option<Symbol<'g>> {
        self.symbol
    }

    pub fn actions(&self) -> &[Action<'g>] {
        &self.actions
    }
}

pub struct Machine<'g, 't> {
    parse_table: &'t ParseTable<'g>,
    head: Vec<Symbol<'g>>,
    stack: Vec<(StateIndex, Symbol<'g>)>,
    halted: bool,
    step: usize,
}

impl<'g, 't> Machine<'g, 't> {
    pub fn new(parse_table: &'t ParseTable<'g>) -> Machine<'g, 't> {
        Machine {
            parse_table,
            head: vec![],
            stack: vec![],
            halted: false,
            step: 0,
        }
    }

    fn state(&self) -> StateIndex {
        self.stack
            .last()
            .map(|(state_index, _symbol)| {
                *state_index
            })
            .unwrap_or(StateIndex(0))
    }

    fn step(&mut self, symbol: Option<Symbol<'g>>) {
        let state = self.state();

        {
            eprintln!("STEP:   {:?}", self.step);
            eprintln!("SYMBOL: {:?}", symbol);
            eprintln!("STACK:  {:?}", &self.stack);
            eprintln!("STATE:  {:?}", state);
            eprintln!();

            let state = &self.parse_table.states[usize::from(state)];
            for item in state.itemset.items() {
                eprintln!("    {item:?}");
            }
            eprintln!();
        }


        let key = (state, symbol);
        let actions = &self.parse_table.actions.get(&key);

        if let Some(actions) = actions {
            let action = if actions.len() == 0 {
                panic!("Machine halted unexpectedly")
            } else if actions.len() == 1 {
                actions[0]
            } else {
                panic!("Multiple actions: {actions:?}")
            };

            match action {
                Action::Shift(dst_state_index) => {
                                self.stack.push((dst_state_index, symbol.unwrap()));
                            }
                Action::Reduce(rule) => {
                                self.head.insert(0, rule.lhs());

                                if let Some(symbol) = symbol {
                                    self.head.insert(0, symbol);
                                }

                                let mut children = vec![];

                                for _ in 0..rule.rhs().len() {
                                    let Some((_state, sym)) = self.stack.pop() else { panic!() };
                                    children.insert(0, sym);
                                }
                            }
                Action::Halt => {
                    self.halted = true;
                }
            }
        }
    }

    pub fn run(&mut self, input: &mut impl Iterator<Item=Symbol<'g>>) {
        while !self.halted {
            if let Some(symbol) = self.head.pop() {
                self.step(Some(symbol));
            } else {
                let symbol = input.next();
                self.step(symbol);
            }

            self.step += 1;
        }
    }
}

#[test]
fn test_conflicts() {
    let grammar = Grammar::new()
        .symbol("*")
        .symbol("+")
        .symbol("id")
        .symbol("(")
        .symbol(")")
        .symbol("E")
        .symbol("E'")
        .symbol("T")
        .symbol("T'")
        .symbol("F")
        .symbol("S")
        .rule("S", &["E"])
        .rule("E", &["T", "+", "E"])
        .rule("E", &["T"])
        .rule("T", &["F", "*", "T"])
        .rule("T", &["F"])
        .rule("F", &["id"])
        .rule("F", &["(", "E", ")"])
        .build();

    let table = ParseTable::new(&grammar, grammar.rules()[0]);
    dbg!(&table.states.len());
    dbg!(table.conflicts());
    for conflict in table.conflicts() {
        eprintln!("{conflict:?}");
        eprintln!("{:?}", conflict.state());
        eprintln!();
    }
}

#[test]
fn test_machine() {
    let grammar = Grammar::new()
        .symbol("(")
        .symbol(")")
        .symbol("S'")
        .symbol("S")
        .symbol("A")
        .symbol("a")
        .symbol("b")
        .rule("S'", &["S"])
        .rule("S", &["a", "A"])
        .rule("A", &["b"])
        .build();

    let table = ParseTable::new(&grammar, grammar.rules()[0]);
    dbg!(&table.states.len());

    eprintln!("STATES:");
    eprintln!();
    for state in &table.states {
        eprintln!("{state:?}");
        eprintln!();
    }

    let mut machine = Machine::new(&table);

    let mut input = vec![
        grammar.symbol("a").unwrap(),
        grammar.symbol("b").unwrap(),
    ].into_iter();
    machine.run(&mut input);
}
