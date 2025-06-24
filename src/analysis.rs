use std::collections::{HashMap, HashSet};

use super::*;

pub struct GrammarAnalysis<'g> {
    nullables: HashSet<Symbol<'g>>,
    first_follows: FirstFollows<'g>,
}

impl<'g> GrammarAnalysis<'g> {
    pub fn build(grammar: &'g Grammar) -> GrammarAnalysis<'g> {
        let nullables = Self::calc_nullables(grammar);
        let first_follows = Self::calc_first_follows(grammar, &nullables);

        GrammarAnalysis {
            nullables,
            first_follows,
        }
    }

    pub fn nullables(&self) -> HashSet<Symbol<'g>> {
        self.nullables.clone()
    }

    pub fn first(&self, symbol: Symbol<'g>) -> HashSet<Symbol<'g>> {
        self.first_follows.terminals_from(FFNode::First(symbol))
    }

    pub fn follow(&self, symbol: Symbol<'g>) -> HashSet<Symbol<'g>> {
        self.first_follows.terminals_from(FFNode::Follow(symbol))
    }

    fn calc_nullables(grammar: &'g Grammar) -> HashSet<Symbol<'g>> {
        let mut nullables = HashSet::new();

        loop {
            let mut dirty = false;

            for rule in grammar.rules() {
                if !nullables.contains(&rule.lhs()) {
                    if rule.rhs().iter().all(|symbol| nullables.contains(symbol)) {
                        nullables.insert(rule.lhs());
                        dirty = true;
                    }
                }
            }

            if !dirty {
                break;
            }
        }

        nullables
    }

    fn calc_first_follows(grammar: &'g Grammar, nullables: &HashSet<Symbol<'g>>) -> FirstFollows<'g> {
        let mut first_follows = FirstFollows::new();

        for rule in grammar.rules() {
            // Firsts are Firsts
            for symbol in rule.rhs() {
                if symbol.is_terminal() {
                    first_follows.link(FFNode::First(rule.lhs()), FFNode::Terminal(symbol));
                } else {
                    first_follows.link(FFNode::First(rule.lhs()), FFNode::First(symbol));
                }

                if !nullables.contains(&symbol) {
                    break;
                }
            }

            // Follows are Firsts, too
            for (i, symbol) in rule.rhs().iter().copied().enumerate() {
                for j in i+1 .. rule.rhs().len() {
                    let follow = rule.rhs()[j];

                    if follow.is_terminal() {
                        first_follows.link(FFNode::Follow(symbol), FFNode::Terminal(follow));
                    } else {
                        first_follows.link(FFNode::Follow(symbol), FFNode::First(follow));
                    }
                }
            }

            // Follows can also be follows, though
            for symbol in rule.rhs().into_iter().rev() {
                if symbol.is_nonterminal() {
                    first_follows.link(FFNode::Follow(symbol), FFNode::Follow(rule.lhs()));
                }

                if !nullables.contains(&symbol) {
                    break;
                }
            }
        }

        first_follows
    }
}

struct FirstFollows<'g> {
    edges: HashMap<FFNode<'g>, HashSet<FFNode<'g>>>,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum FFNode<'g> {
    First(Symbol<'g>),
    Follow(Symbol<'g>),
    Terminal(Symbol<'g>),
}

impl<'g> FirstFollows<'g> {
    fn new() -> FirstFollows<'g> {
        FirstFollows {
            edges: HashMap::new(),
        }
    }

    fn link(&mut self, from_node: FFNode<'g>, to_node: FFNode<'g>) {
        if !self.edges.contains_key(&from_node) {
            self.edges.insert(from_node, HashSet::new());
        }

        self.edges.get_mut(&from_node).unwrap().insert(to_node);
    }

    fn terminals_from(&self, from_node: FFNode<'g>) -> HashSet<Symbol<'g>> {
        let mut visited = HashSet::new();
        let mut queue = vec![from_node];
        let mut terminals = HashSet::new();

        while let Some(node) = queue.pop() {
            visited.insert(node);
            if let FFNode::Terminal(symbol) = node {
                terminals.insert(symbol);
            } else if self.edges.contains_key(&node) {
                for next_node in &self.edges[&node] {
                    if !visited.contains(next_node) {
                        queue.push(*next_node);
                    }
                }
            }
        }

        terminals
    }
}
