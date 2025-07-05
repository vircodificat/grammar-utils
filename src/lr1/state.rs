use std::collections::{BTreeSet, BTreeMap};

use crate::*;
use super::*;

/// An LR(1) state.
/// Consists of an LR(1) itemset.
#[derive(Clone)]
pub struct State<'g> {
    grammar: &'g Grammar,
    items: Vec<Item<'g>>,
}

// TODO contents should be private
/// The index of a given state.
#[derive(Debug)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct StateIndex(pub usize);

impl From<StateIndex> for usize {
    fn from(value: StateIndex) -> Self {
        value.0
    }
}

impl<'g> std::fmt::Debug for State<'g> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in self.items() {
            writeln!(f, "{item:?}")?;
        }
        Ok(())
    }
}

impl<'g> PartialEq for State<'g> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.grammar, other.grammar) && self.items == other.items
    }
}

impl<'g> Eq for State<'g> {}

impl<'g> State<'g> {
    /// Get the underlying `Grammar` for this state.
    pub fn grammar(&self) -> &'g Grammar {
        self.grammar
    }

    /// Get the itemset for this state.
    pub fn items(&self) -> &[Item<'g>] {
        self.items.as_slice()
    }

    /// Generates the state representing the closure of a single item.
    pub(crate) fn singleton(item: Item<'g>, analysis: &GrammarAnalysis<'g>) -> Self {
        let grammar: &'g Grammar = item.grammar();
        let itemset = State {
            grammar,
            items: vec![item],
        };
        itemset.closure(analysis)
    }

    /// Calculate the ε-closure of the items in this state.
    ///
    /// This captures the fact that when the item is ready to accept a nonterminal,
    /// it is equivalently ready to begin parsing that nonterminal
    /// using of the rules in the grammar.
    pub(crate) fn closure(&self, analysis: &GrammarAnalysis<'g>) -> State<'g> {
        let mut itemset = self.items.clone();

        // Iterate repeatedly until no new items are found.
        // (That is, until `dirty` stays `false`.
        loop {
            let mut dirty = false;
            // We're about to iterate over itemset, so we create a buffer
            // to avoid iterator invalidation.
            let mut new_items = vec![];

            for item in &itemset {
                // If we're not at the end of the rule...
                if let Some(next_symbol) = item.next_symbol() {
                    // Calculate the look ahead.
                    //
                    // This step is done for LR(1) but not for LR(0).
                    //
                    // The lookahead is based on the symbol *following* the symbol at the cursor.
                    // If it's a nonterminal, we take its FIRST set as the lookahead.
                    // If it's a terminal, that becomes the (only) lookahead symbol.
                    //
                    // In the case the cursor points to the last symbol in the rule,
                    // we keep the same lookahead.
                    let lookahead = if let Some(next_next_symbol) = item.next_next_symbol() {
                        if next_next_symbol.is_nonterminal() {
                            analysis.first(next_next_symbol).into_iter().map(|symbol| Some(symbol)).collect()
                        } else {
                            [Some(next_next_symbol)].into_iter().collect()
                        }
                    } else {
                        item.lookahead().clone()
                    };

                    // If the cursor points at a nonterminal,
                    // find all of the rules for that nonterminal and add them
                    // (if they aren't already in the itemset).
                    //
                    // Adding this rule may enable even more items in the next iteration.
                    // Set `dirty` to `true` to indicate we need to iterate again.
                    if next_symbol.is_nonterminal() {
                        let symbol_rules = self.grammar
                            .rules()
                            .into_iter()
                            .filter(|rule| {
                                rule.lhs() == next_symbol
                            });

                        for rule in symbol_rules {
                            let item = Item::new(rule, 0, lookahead.clone());
                            if !itemset.contains(&item) {
                                new_items.push(item);
                                dirty = true;
                            }
                        }
                    }
                }

            }

            // Now that we're done iterating, we can safely copy the items into the itemset.
            for item in new_items {
                if !itemset.contains(&item) {
                    itemset.push(item);
                }
            }

            // And if we iterated without changing anything,
            // then we're done.
            if !dirty {
                break;
            }
        }

        State {
            grammar: self.grammar,
            items: self.squash(itemset),
        }
    }

    fn squash(&self, itemset: Vec<Item<'g>>) -> Vec<Item<'g>> {
        let mut lookaheads: BTreeMap<(Rule<'g>, usize), BTreeSet<Option<Symbol<'g>>>> = BTreeMap::new();
        for item in itemset {
            let key = (item.rule(), item.pos());
            if !lookaheads.contains_key(&key) {
                lookaheads.insert(key, BTreeSet::new());
            }
            lookaheads.get_mut(&key).unwrap().extend(item.lookahead());
        }
        lookaheads
            .into_iter()
            .map(|((rule, pos), lookaheads)| {
                Item::new(rule, pos, lookaheads)
            })
            .collect()
    }

    // Take the current state and calculate which state is reached
    // when it shifts `symbol` onto the stack.
    //
    // This has the effect of going through each item and advancing the cursor
    // for any item where the symbol at the cursor is `symbol`.
    // For any item where the symbol at the cursor is something else,
    // or in the case that the cursor is at the end,
    // the item is instead discarded.
    //
    // The result is then the ε-closure of the resulting itemset.
    pub fn follow(&self, analysis: &GrammarAnalysis<'g>, symbol: Symbol<'g>) -> State<'g> {
        let mut items = vec![];
        for item in &self.items {
            if let Some(next_symbol) = item.next_symbol() {
                if next_symbol == symbol {
                    items.push(item.step().unwrap());
                }
            }
        }

        let itemset = State {
            grammar: self.grammar,
            items,
        };

        itemset.closure(analysis)
    }
}
