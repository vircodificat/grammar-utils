use std::collections::HashSet;

use super::*;

#[derive(Clone, Copy)]
pub struct Item<'g> {
    rule: Rule<'g>,
    pos: usize,
}

impl<'g> Rule<'g> {
    pub fn item(&self, pos: usize) -> Item<'g> {
        assert!(pos <= self.rhs().len());

        Item {
            rule: *self,
            pos,
        }
    }
}

impl<'g> Item<'g> {
    pub fn rule(&self) -> Rule<'g> {
        self.rule
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn grammar(&self) -> &'g Grammar {
        self.rule.grammar()
    }

    pub fn lhs(&'g self) -> Symbol<'g> {
        self.rule.lhs()
    }

    pub fn rhs(&'g self) -> Vec<Symbol<'g>> {
        self.rule.rhs()
    }

    pub fn next_symbol(&self) -> Option<Symbol<'g>> {
        if self.pos() < self.rhs().len() {
            Some(self.rule.rhs()[self.pos])
        } else {
            None
        }
    }

    pub fn step(&self) -> Option<Item<'g>> {
        if self.pos() < self.rhs().len() {
            Some(self.rule.item(self.pos + 1))
        } else {
            None
        }
    }

    pub fn is_finished(&self) -> bool {
        self.pos() == self.rhs().len()
    }
}

impl<'g> std::fmt::Debug for Item<'g> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lhs = self.rule.lhs();
        let rhs = self.rule.rhs();
        write!(f, "{lhs:?} ->")?;
        for i in 0..self.pos {
            write!(f, " {:?}", &rhs[i])?;
        }

        write!(f, " .")?;

        for i in self.pos..rhs.len() {
            write!(f,  " {:?}", &rhs[i])?;
        }
        Ok(())
    }
}

impl<'g> PartialEq for Item<'g> {
    fn eq(&self, other: &Self) -> bool {
        self.rule() == other.rule() && self.pos == other.pos
    }
}

impl<'g> Eq for Item<'g> {}

#[test]
fn debug_for_items() {
    let grammar = Grammar::new()
        .symbol("A")
        .symbol("x")
        .symbol("y")
        .symbol("z")
        .rule("A", &["x", "y", "z"])
        .build();

    let rule0 = &grammar.rules()[0];
    assert_eq!(&format!("{:?}", rule0.item(0)), "A -> . x y z");
    assert_eq!(&format!("{:?}", rule0.item(1)), "A -> x . y z");
    assert_eq!(&format!("{:?}", rule0.item(2)), "A -> x y . z");
    assert_eq!(&format!("{:?}", rule0.item(3)), "A -> x y z .");
}

#[test]
fn step_item() {
    let grammar = Grammar::new()
        .symbol("A")
        .symbol("x")
        .symbol("y")
        .symbol("z")
        .rule("A", &["x", "y", "z"])
        .build();

    let mut item = grammar.rules()[0].item(0);
    assert_eq!(&format!("{item:?}"), "A -> . x y z");

    item = item.step().unwrap();
    assert_eq!(&format!("{item:?}"), "A -> x . y z");

    item = item.step().unwrap();
    assert_eq!(&format!("{item:?}"), "A -> x y . z");

    item = item.step().unwrap();
    assert_eq!(&format!("{item:?}"), "A -> x y z .");

    assert!(item.is_finished());
    assert!(item.step().is_none());
}

pub struct ItemSet<'g> {
    grammar: &'g Grammar,
    items: Vec<Item<'g>>,
}

impl<'g> ItemSet<'g> {
    pub fn grammar(&self) -> &'g Grammar {
        self.grammar
    }

    pub fn empty(grammar: &'g Grammar) -> Self {
        ItemSet {
            grammar,
            items: vec![],
        }
    }

    pub fn singleton(item: Item<'g>) -> Self {
        let grammar: &'g Grammar = item.grammar();
        ItemSet {
            grammar,
            items: vec![item],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn items(&self) -> Vec<Item<'g>> {
        self.items.clone()
    }

    fn insert(&mut self, item: Item<'g>) -> bool {
        for search_item in &self.items {
            if search_item == &item {
                return false;
            }
        }

        self.items.push(item);
        true
    }

    pub fn closure(&self) -> ItemSet<'g> {
        let mut nonterms_added = HashSet::new();
        let mut items = self.items.clone();

        loop {
            let mut dirty = false;
            let mut new_items = vec![];

            for item in &items {
                if let Some(symbol) = item.next_symbol() {
                    if symbol.is_nonterminal() {
                        if !nonterms_added.contains(&symbol) {
                            nonterms_added.insert(symbol);

                            let symbol_rules = self.grammar.rules().into_iter().filter(|rule| rule.lhs() == symbol);
                            for rule in symbol_rules {
                                let item = rule.item(0);
                                new_items.push(item);
                                dirty = true;
                            }
                        }
                    }
                }

            }

            for item in new_items {
                if !items.contains(&item) {
                    items.push(item);
                }
            }

            if !dirty {
                break;
            }
        }

        ItemSet {
            grammar: self.grammar,
            items,
        }
    }

    pub fn follow(&self, symbol: Symbol<'g>) -> ItemSet<'g> {
        let mut items = vec![];
        for item in &self.items {
            if let Some(next_symbol) = item.next_symbol() {
                if next_symbol == symbol {
                    items.push(item.step().unwrap());
                }
            }
        }

        let itemset = ItemSet {
            grammar: self.grammar,
            items,
        };

        itemset.closure()
    }
}
