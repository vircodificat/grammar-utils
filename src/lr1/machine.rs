use crate::*;
use super::*;

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
            for item in state.itemset().items() {
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

            {
                eprintln!("ACTION:  {action:?}");
                eprintln!();
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
