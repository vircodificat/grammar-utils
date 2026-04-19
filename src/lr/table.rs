use std::collections::{BTreeMap, BTreeSet};

use crate::*;
use super::*;

#[derive(Debug)]
pub struct ParseTable<'g> {
    grammar: &'g Grammar,
    pub(crate) states: Vec<State<'g>>,
    pub(crate) transitions: BTreeMap<(StateIndex, SymbolIndex), StateIndex>,
    pub(crate) reductions: BTreeMap<StateIndex, Vec<Reduction<'g>>>,
}

#[derive(Debug)]
#[derive(Clone, PartialEq, Eq)]
pub struct Reduction<'g>(Rule<'g>, ReductionCondition);

#[derive(Debug)]
#[derive(Clone, PartialEq, Eq)]
pub enum ReductionCondition {
    Always,
    If(BTreeSet<SymbolIndex>),
}

#[derive(Debug)]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum Action<'g> {
    Shift(SymbolIndex, StateIndex),
    Reduce(Rule<'g>),
    Halt,
}

#[derive(Clone)]
pub struct Conflict<'g, 't> {
    table: &'t ParseTable<'g>,
    state: StateIndex,
    symbol: Option<Symbol<'g>>,
    actions: Vec<Action<'g>>,
}

impl<'g> std::fmt::Debug for State<'g> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.itemset().items())
    }
}

impl<'g> ParseTable<'g> {
    pub fn build(grammar: &'g Grammar, start_rule: Rule<'g>) -> ParseTable<'g> {
        let states = Self::build_states(&grammar, start_rule);
        let transitions = Self::build_transitions(&grammar, &states);
        let reductions = Self::build_reductions(&states);

        let table = ParseTable {
            grammar,
            states,
            transitions,
            reductions,
        };

        for conflict in table.conflicts() {
            eprintln!("{conflict:?}");
        }

        table
    }

    pub fn grammar(&self) -> &'g Grammar {
        self.grammar
    }

    fn build_states(grammar: &'g Grammar, start_rule: Rule<'g>) -> Vec<State<'g>> {
        let mut states = vec![];
        let start_state = State::new(ItemSet::singleton(Item::new(start_rule, 0)));
        let mut states_remaining = vec![start_state];

        while let Some(state) = states_remaining.pop() {
            for symbol in grammar.symbols() {
                let next_state = State::new(state.itemset().follow(symbol));

                if next_state.itemset().is_empty() {
                    continue;
                }

                if !states.contains(&next_state) {
                    states_remaining.push(next_state);
                }
            }

            if !states.contains(&state) {
                states.push(state);
            }
        }

        states
    }

    fn state_index(
        states: &[State<'g>],
        itemset: &ItemSet<'g>,
    ) -> Option<StateIndex> {
        for (i, state) in states.iter().enumerate() {
            if state.itemset() == itemset {
                return Some(StateIndex(i));
            }
        }
        None
    }

    fn build_transitions(
        grammar: &'g Grammar,
        states: &[State<'g>],
    ) -> BTreeMap<(StateIndex, SymbolIndex), StateIndex> {
        let mut transitions: BTreeMap<(StateIndex, SymbolIndex), StateIndex> = BTreeMap::new();

        for (src_state_index, src_state) in states.iter().enumerate() {
            let src_state_index = StateIndex(src_state_index);
            for symbol in grammar.symbols() {
                let symbol_index = symbol.index();
                let dst_state_itemset: ItemSet<'_> = src_state.itemset().follow(symbol);
                if let Some(dst_state_index) = Self::state_index(states, &dst_state_itemset) {
                    transitions.insert((src_state_index, symbol_index), dst_state_index);
                }
            }
        }

        transitions
    }

    fn build_reductions(
        states: &[State<'g>],
    ) -> BTreeMap<StateIndex, Vec<Reduction<'g>>> {
        let mut reduction_map = BTreeMap::new();
        for (state_index, state) in states.iter().enumerate() {
            let state_index = StateIndex(state_index);
            for item in state.itemset().items() {
                if item.is_finished() {
                    if !reduction_map.contains_key(&state_index) {
                        reduction_map.insert(state_index, vec![]);
                    }
                    let reductions = reduction_map.get_mut(&state_index).unwrap();
                    reductions.push(Reduction(item.rule(), ReductionCondition::Always));
                }
            }
        }
        reduction_map
    }

    pub fn actions(&self, state: State<'g>, symbol: Option<Symbol<'g>>) -> Vec<Action<'g>> {
        let state_index = Self::state_index(&self.states, state.itemset()).unwrap();
        self.get(state_index, symbol)
    }

    pub fn conflicts(&self) -> Vec<Conflict<'_, '_>> {
        let mut conflicts = vec![];
        for (state_index, state) in self.states.iter().enumerate() {
            let state_index = StateIndex(state_index);
            for symbol in self.grammar.symbols() {
                let actions = self.actions(state.clone(), Some(symbol));
                let next_state = self.transitions.get(&(state_index, symbol.index()));
                if next_state.is_some() && actions.len() > 1 {
                    conflicts.push(Conflict {
                        table: self,
                        state: state_index,
                        symbol: Some(symbol),
                        actions: actions.clone(),
                    });
                }
            }

            let symbol = None::<Symbol<'g>>;
            let actions = self.actions(state.clone(), symbol);
            if actions.len() > 1 {
                conflicts.push(Conflict {
                    table: self,
                    state: state_index,
                    symbol,
                    actions: actions.clone(),
                });
            }
        }
        conflicts
    }

    pub fn get(&self, state_index: StateIndex, symbol: Option<Symbol<'g>>) -> Vec<Action<'g>> {
        let mut actions = vec![];
        if let Some(symbol) = symbol {
            let key = (state_index, symbol.index());
            if let Some(shift_state_index) = self.transitions.get(&key) {
                actions.push(Action::Shift(symbol.index(), *shift_state_index));
            }
        }

        if let Some(reductions) = self.reductions.get(&state_index) {
            for Reduction(rule, cond) in reductions {
                match cond {
                    ReductionCondition::Always => {
                        actions.push(Action::Reduce(*rule));
                    }
                    ReductionCondition::If(nexts) => {
                        if let Some(symbol) = symbol {
                            if nexts.contains(&symbol.index()) {
                                actions.push(Action::Reduce(*rule));
                            }
                        }
                    }
                }
            }
        }
        actions
    }

    pub fn dump(&self) {
        for (state_index, state) in self.states.iter().enumerate() {
            let state_index = StateIndex(state_index);
            eprintln!("{state_index:?}");
            eprintln!("{state:?}");

            for symbol in self.grammar.symbols() {
                eprintln!("    {symbol:?} => {:?}", self.get(state_index, Some(symbol)));
            }
            eprintln!("    None => {:?}", self.get(state_index, None));
            eprintln!();
        }
    }
}

impl<'g, 't> std::fmt::Debug for Conflict<'g, 't> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_id = self.state;
        let symbol = self.symbol;
        let actions = &self.actions;
        write!(f, "Conflict(state={state_id:?}, symbol={symbol:?}, actions={actions:?})")?;
        Ok(())
    }
}

impl<'g, 't> Conflict<'g, 't> {
    pub fn table(&self) -> &'t ParseTable<'g> {
        self.table
    }

    pub fn state(&self) -> &'t State<'_> {
        &self.table.states[usize::from(self.state)]
    }

    pub fn symbol(&self) -> Option<Symbol<'g>> {
        self.symbol
    }

    pub fn actions(&self) -> &[Action<'g>] {
        &self.actions
    }
}
