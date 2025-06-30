use std::collections::HashMap;

use crate::*;
use super::*;

#[derive(Debug)]
pub struct ParseTable<'g> {
    grammar: &'g Grammar,
    pub(crate) states: Vec<State<'g>>,
    pub(crate) actions: HashMap<(StateIndex, Option<Symbol<'g>>), Vec<Action<'g>>>,
}

#[derive(Debug)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action<'g> {
    Shift(StateIndex),
    Reduce(Rule<'g>),
    Halt,
}

#[derive(Clone)]
pub struct Conflict<'g, 't> {
    table: &'t ParseTable<'g>,
    state: StateIndex,
    symbol: Option<Symbol<'g>>,
    actions: Vec<Action<'g>>,
}

impl<'g> ParseTable<'g> {
    pub fn build(grammar: &'g Grammar, start_rule: Rule<'g>) -> ParseTable<'g> {
        let analysis = GrammarAnalysis::build(grammar);
        let states = Self::build_states(&grammar, &analysis, start_rule);
        let actions = Self::build_actions(&grammar, &analysis, &states, start_rule);

        ParseTable {
            grammar,
            states,
            actions,
        }
    }

    pub fn grammar(&self) -> &'g Grammar {
        self.grammar
    }

    fn build_states(
        grammar: &'g Grammar,
        analysis: &GrammarAnalysis<'g>,
        start_rule: Rule<'g>,
    ) -> Vec<State<'g>> {
        let mut states = vec![];
        let start_state = State::new(ItemSet::singleton(Item::new(start_rule, 0, vec![None].into_iter().collect()), analysis));
        let mut states_remaining = vec![start_state];

        while let Some(state) = states_remaining.pop() {
            for symbol in grammar.symbols() {
                let next_state = State::new(state.itemset().follow(analysis, symbol));

                if next_state.itemset().is_empty() {
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
        analysis: &GrammarAnalysis<'g>,
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
            for src_item in src_state.itemset().items() {
                match src_item.next_symbol() {
                    Some(symbol) => {
                        let dst_state = src_state.itemset().follow(&analysis, symbol);
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
                if itemset == st.itemset() {
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
