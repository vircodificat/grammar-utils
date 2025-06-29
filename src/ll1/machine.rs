use std::iter::Peekable;

use crate::*;
use super::*;

pub struct Machine<'g, I>
    where I: Iterator<Item=Symbol<'g>> {
    table: ParseTable<'g>,
    stack: Vec<Symbol<'g>>,
    input: Peekable<I>,
}

impl<'g, I> Machine<'g, I> where I: Iterator<Item=Symbol<'g>> {
    pub fn new(table: ParseTable<'g>, start_symbol: Symbol<'g>, input: I) -> Machine<'g, I> {
        Machine {
            table,
            input: input.peekable(),
            stack: vec![start_symbol],
        }
    }

    pub fn step(&mut self) -> bool {
        match (self.stack.pop(), self.input.peek()) {
            (None, Some(_symbol)) => {
                return true;
            }
            (Some(state), token) => {
                if Some(state) == token.copied() {
                    self.input.next();
                } else {
                    let rules = self.table.get(state, token.copied());
                    match rules.as_slice() {
                        [] => panic!("Parse error"),
                        [rule] => {
                            for symbol in rule.rhs().into_iter().rev() {
                                self.stack.push(symbol);
                            }
                        }
                        _ => panic!("Conflict"),
                    }
                }
            }
            (None, None) => return true,
        }
        false
    }

    pub fn run(&mut self) {
        self.dump();
        loop {
            let halt = self.step();
            self.dump();
            if halt {
                break;
            }
        }
    }

    fn dump(&mut self) {
        eprintln!("Input: {:?}", self.input.peek());
        eprintln!("Stack: {}", self.stack.iter().map(|symbol| format!("{symbol:?}")).collect::<Vec<_>>().join(" "));
        eprintln!();
    }
}
