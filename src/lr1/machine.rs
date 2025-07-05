use std::iter::Peekable;

use crate::*;
use super::*;

pub struct Machine<'g, 't, I>
where I: Iterator<Item=Symbol<'g>> {
    input: Peekable<I>,
    parse_table: &'t ParseTable<'g>,
    stack: Vec<(StateIndex, Symbol<'g>)>,
    halted: bool,
    step: usize,
}

impl<'g, 't, I> Machine<'g, 't, I>
where I: Iterator<Item=Symbol<'g>> {
    pub fn new(parse_table: &'t ParseTable<'g>, input: I) -> Machine<'g, 't, I> {
        Machine {
            input: input.peekable(),
            parse_table,
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

    fn step(&mut self) {
        let symbol = self.input.peek().copied();
        let state = self.state();

        {
            eprintln!("STEP:   {:?}", self.step);
            eprintln!("SYMBOL: {:?}", symbol);
            eprintln!("STACK:  {:?}", &self.stack);
            eprintln!("STATE:  {:?}", state);
            eprintln!();

            let state = &self.parse_table[state];
            for item in state.items() {
                eprintln!("    {item:?}");
            }
            eprintln!();
        }

        let actions = &self.parse_table.get(state, symbol);

        let action = if actions.len() == 0 {
            panic!("Machine halted unexpectedly")
        } else if actions.len() == 1 {
            actions[0]
        } else {
            panic!("Multiple actions: {actions:?}")
        };

        {
            eprintln!("ACTION:  {action:?}");
            eprintln!();
        }

        match action {
            Action::Shift(dst_state_index) => {
                self.input.next();
                self.stack.push((dst_state_index, symbol.unwrap()));
            }
            Action::Reduce(rule) => {
                let mut children = vec![];

                for _ in 0..rule.rhs().len() {
                    let Some((_state, sym)) = self.stack.pop() else { panic!() };
                    children.insert(0, sym);
                }

                eprintln!("REDUCE {rule:?} with children {children:?}");
                eprintln!();

                if rule == self.parse_table.grammar().start_rule() {
                    self.halted = true;
                    return;
                }

                let next_actions = self.parse_table.get(self.state(), Some(rule.lhs()));
                let next_action = if next_actions.len() != 1 {
                    panic!("Expected GOTO but found: {next_actions:?}")
                } else {
                    next_actions[0]
                };

                match next_action {
                    Action::Shift(dst_state_index) => {
                        self.stack.push((dst_state_index, rule.lhs()));
                    }
                    _ => {
                        panic!("Expected Shift after reduction but found {next_action:?}")
                    }
                };
            }
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
            self.step += 1;
        }
    }
}
