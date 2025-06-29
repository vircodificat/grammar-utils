use crate::*;
use super::*;

#[derive(Debug)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct StateIndex(pub(crate) usize);

#[derive(PartialEq, Eq)]
pub struct State<'g> {
    itemset: ItemSet<'g>,
}

impl<'g> State<'g> {
    pub(crate) fn new(itemset: ItemSet<'g>) -> Self {
        State {
            itemset,
        }
    }

    pub fn itemset(&self) -> &ItemSet<'g> {
        &self.itemset
    }
}

impl From<StateIndex> for usize {
    fn from(value: StateIndex) -> Self {
        value.0
    }
}
