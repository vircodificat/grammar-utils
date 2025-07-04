use std::collections::BTreeSet;

use crate::*;

#[derive(Clone)]
pub struct Item<'g> {
    rule: Rule<'g>,
    pos: usize,
    lookahead: BTreeSet<Option<Symbol<'g>>>,
}

impl<'g> Item<'g> {
    pub fn new(rule: Rule<'g>, pos: usize, lookahead: BTreeSet<Option<Symbol<'g>>>) -> Item<'g> {
        assert!(pos <= rule.rhs().len());

        Item {
            rule,
            pos,
            lookahead,
        }
    }

    pub fn rule(&self) -> Rule<'g> {
        self.rule
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn lookahead(&self) -> &BTreeSet<Option<Symbol<'g>>> {
        &self.lookahead
    }

    pub fn grammar(&self) -> &'g Grammar {
        self.rule.grammar()
    }

    pub fn lhs(&'g self) -> Symbol<'g> {
        self.rule.lhs()
    }

    pub fn rhs(&self) -> Vec<Symbol<'g>> {
        self.rule.rhs()
    }

    pub fn next_symbol(&self) -> Option<Symbol<'g>> {
        if self.pos() < self.rhs().len() {
            Some(self.rule.rhs()[self.pos])
        } else {
            None
        }
    }

    pub fn next_next_symbol(&self) -> Option<Symbol<'g>> {
        if self.pos() + 1 < self.rhs().len() {
            Some(self.rule.rhs()[self.pos + 1])
        } else {
            None
        }
    }

    pub fn step(&self) -> Option<Item<'g>> {
        if self.pos() < self.rhs().len() {
            Some(Item::new(self.rule, self.pos + 1, self.lookahead.clone()))
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

        write!(f, " {{ ")?;
        for symbol in &self.lookahead {
            write!(f, "{symbol:?} ")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<'g> PartialEq for Item<'g> {
    fn eq(&self, other: &Self) -> bool {
        self.rule() == other.rule() && self.pos == other.pos && self.lookahead == other.lookahead
    }
}

impl<'g> Eq for Item<'g> {}
