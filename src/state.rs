use crate::item::ItemSet;

// TODO contents should be private
#[derive(Debug)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct StateIndex(pub usize);

#[derive(PartialEq, Eq, Clone)]
pub struct State<'g> {
    pub(crate) index: StateIndex,
    pub(crate) itemset: ItemSet<'g>,
}

impl<'g> State<'g> {
    pub fn index(&self) -> StateIndex {
        self.index
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
