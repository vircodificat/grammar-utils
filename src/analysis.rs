use std::collections::{HashMap, HashSet};

use super::*;

/// A structure for calculating the set of nullable nonterminals
/// as well as the FIRST and FOLLOW sets for each nonterminal.
pub struct GrammarAnalysis<'g> {
    nullables: HashSet<Symbol<'g>>,
    first_follows: FirstFollows<'g>,
}

impl<'g> GrammarAnalysis<'g> {
    /// The constructor for `GrammarAnalysis`.
    /// This builds the nullable set and the FIRST and FOLLOW sets for `grammar`.
    pub fn build(grammar: &'g Grammar) -> GrammarAnalysis<'g> {
        let nullables = Self::calc_nullables(grammar);
        let first_follows = Self::calc_first_follows(grammar, &nullables);

        GrammarAnalysis {
            nullables,
            first_follows,
        }
    }

    /// Returns the set of nullable symbols.
    ///
    /// The result is a set of nonterminals.
    /// A `Symbol` is nullable if it can expand into an empty string of terminals
    /// through the application of some sequence of production rules.
    pub fn nullables(&self) -> HashSet<Symbol<'g>> {
        self.nullables.clone()
    }

    /// Returns whether the given symbol is nullable.
    pub fn is_nullable(&self, symbol: Symbol<'g>) -> bool {
        self.nullables.contains(&symbol)
    }

    pub fn first_seq(&self, seq: &[Symbol<'g>]) -> HashSet<Symbol<'g>> {
        let mut result = HashSet::new();

        for symbol in seq {
            if symbol.is_terminal() {
                result.insert(*symbol);
                return result;
            } else {
                result.extend(self.first(*symbol));
                if !self.is_nullable(*symbol) {
                    return result;
                }
            }
        }
        result
    }

    pub fn is_nullable_seq(&self, seq: &[Symbol<'g>]) -> bool {
        seq.iter().copied().all(|symbol| self.is_nullable(symbol))
    }

    /// Returns the FIRST set for a nonterminal `Symbol`.
    ///
    /// The result is a set of terminals.
    /// A terminal is in the FIRST set of a nonterminal if some sequence of production rules
    /// starting with that nonterminal expands to a string of terminals starting with that terminal.
    pub fn first(&self, symbol: Symbol<'g>) -> HashSet<Symbol<'g>> {
        self.first_follows.terminals_from(FFNode::First(symbol))
    }

    /// Returns the FOLLOW set for a nonterminal `Symbol`.
    ///
    /// The result is a set of terminals.
    /// A terminal is in the FOLLOW set of a nonterminal that terminal could legally follow the
    /// nonterminal during parsing.
    pub fn follow(&self, symbol: Symbol<'g>) -> HashSet<Symbol<'g>> {
        self.first_follows.terminals_from(FFNode::Follow(symbol))
    }

    fn calc_nullables(grammar: &'g Grammar) -> HashSet<Symbol<'g>> {
        let mut nullables = HashSet::new();

        // Repeat until an iteration adds nothing new.
        loop {
            let mut dirty = false;

            for rule in grammar.rules() {
                // Skip rules whose LHS is already known to be nullable.
                if !nullables.contains(&rule.lhs()) {
                    // If we know (based on what we've seen so far) that all RHS symbols are nullable,
                    // then the LHS is also nullable.
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

    // Calculate the FirstFollows graph of the `Grammar`
    fn calc_first_follows(grammar: &'g Grammar, nullables: &HashSet<Symbol<'g>>) -> FirstFollows<'g> {
        let mut first_follows = FirstFollows::new();

        for rule in grammar.rules() {
            // Firsts are Firsts
            // Note: We iterate the rule's RHS until we hit a non-nullable symbol
            for symbol in rule.rhs() {
                if symbol.is_terminal() {
                    first_follows.link(FFNode::First(rule.lhs()), FFNode::Terminal(symbol));
                } else {
                    first_follows.link(FFNode::First(rule.lhs()), FFNode::First(symbol));
                }

                // if we'ved reached the last nullable nonterminal, break early
                if !nullables.contains(&symbol) {
                    break;
                }
            }

            // Follows are Firsts, too
            for (i, symbol) in rule.rhs().iter().copied().enumerate() {
                // When `symbol`...
                for j in i+1 .. rule.rhs().len() {
                    // ... is followed by `follow`...
                    let follow = rule.rhs()[j];

                    if follow.is_terminal() {
                        first_follows.link(FFNode::Follow(symbol), FFNode::Terminal(follow));
                    } else {
                        first_follows.link(FFNode::Follow(symbol), FFNode::First(follow));
                    }

                    // if we'ved reached the last nullable nonterminal, break early
                    if !nullables.contains(&follow) {
                        break;
                    }
                }
            }

            // Follows can also be follows, though
            // Note: We iterate the rule's RHS in reverse until we hit a non-nullable symbol
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

// A graph data structure which tracks containment information
// for FIRST sets, FOLLOW sets, and temrinals
struct FirstFollows<'g> {
    // An adjacency list mapping `from_node` to `to_node`.
    // This indicates that the set represented by `from_node` contains the set represented by `to_node`
    edges: HashMap<FFNode<'g>, HashSet<FFNode<'g>>>,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum FFNode<'g> {
    // The FIRST set of the symbol
    First(Symbol<'g>),
    // The FOLLOW set of the symbol
    Follow(Symbol<'g>),
    // The singleton set containing just the given symbol
    Terminal(Symbol<'g>),
}

impl<'g> FirstFollows<'g> {
    fn new() -> FirstFollows<'g> {
        FirstFollows {
            edges: HashMap::new(),
        }
    }

    // Declare that the set represented by `from_node` contains the set represented by `to_node`.
    // Note: If the `from_node` is not present in the graph already, it allocates a new adjacency list for it.
    fn link(&mut self, from_node: FFNode<'g>, to_node: FFNode<'g>) {
        if !self.edges.contains_key(&from_node) {
            self.edges.insert(from_node, HashSet::new());
        }

        self.edges.get_mut(&from_node).unwrap().insert(to_node);
    }

    // Perform a breadth-first search of the graph starting from `from_node`
    // and return the set of terminals reachable from it.
    // These are precisely the set represented by the node.
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
