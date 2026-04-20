use std::collections::{BTreeMap, BTreeSet};

use crate::{Grammar, Rule, Symbol, SymbolIndex, Item};
use crate::state::{State, StateIndex};
use crate::item::ItemSet;

#[derive(Debug)]
pub struct ParseTable<'g> {
    grammar: &'g Grammar,
    pub(crate) states: Vec<State<'g>>,
    pub(crate) actions: Vec<Vec<Action<'g>>>,
}

#[derive(Debug)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reduction<'g>(Rule<'g>, ReductionCondition);

#[derive(Debug)]
#[derive(Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum ReductionCondition {
    Always,
    If(BTreeSet<SymbolIndex>),
}

#[derive(Debug)]
#[derive(Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Action<'g> {
    Shift(SymbolIndex, StateIndex),
    Reduce(Reduction<'g>),
    Halt,
}

//#[derive(Clone)]
//pub struct Conflict<'g, 't> {
//    table: &'t ParseTable<'g>,
//    state: StateIndex,
//    symbol: Option<Symbol<'g>>,
//    actions: Vec<Action<'g>>,
//}

impl<'g> std::fmt::Debug for State<'g> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.itemset().items())
    }
}

impl<'g> ParseTable<'g> {
    pub fn build(grammar: &'g Grammar, start_rule: Rule<'g>) -> ParseTable<'g> {
        let states = Self::build_states(&grammar, start_rule);
        let actions = Self::build_actions(grammar, &states);

        let table = ParseTable {
            grammar,
            states,
            actions,
        };

        table
    }

    pub fn grammar(&self) -> &'g Grammar {
        self.grammar
    }

    fn build_states(grammar: &'g Grammar, start_rule: Rule<'g>) -> Vec<State<'g>> {
        let mut states = vec![];
        let start_state = State {
            index: StateIndex(0),
            itemset: ItemSet::singleton(Item::new(start_rule, 0)),
        };
        let mut states_remaining = vec![start_state];

        while let Some(state) = states_remaining.pop() {
            for symbol in grammar.symbols() {
                let next_state = State {
                    index: StateIndex(0),
                    itemset: state.itemset().follow(symbol),
                };

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

        for (index, state) in states.iter_mut().enumerate() {
            state.index = StateIndex(index);
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

    fn build_actions(
        grammar: &'g Grammar,
        states: &[State<'g>],
    ) -> Vec<Vec<Action<'g>>> {
        let mut actions = vec![vec![]; states.len()];

        for ((src_state_index, symbol_index), dst_state_index) in Self::build_transitions(grammar, states) {
            let state_actions = &mut actions[src_state_index.0];
            state_actions.push(Action::Shift(symbol_index, dst_state_index));
        }

        for (state_index, reductions) in Self::build_reductions(states) {
            for reduction in reductions {
                let Reduction(_rule, cond) = &reduction;
                match cond {
                    ReductionCondition::If(_) => unreachable!(),
                    ReductionCondition::Always => {
                        let state_actions = &mut actions[state_index.0];
                        state_actions.push(Action::Reduce(reduction));
                    }
                }
            }
        }

        actions
    }

    fn transition(&self, state_index: StateIndex, symbol_index: SymbolIndex) -> Option<StateIndex> {
        let state_actions = &self.actions[state_index.0];
        for action in state_actions {
            match action {
                Action::Shift(symbol_index_, state_index) => {
                    if *symbol_index_ == symbol_index {
                        return Some(*state_index);
                    }
                }
                _ => (),
            }
        }
        None
    }

    pub fn inadequate_states(&self) -> Vec<State<'_>> {
        let mut inadequate_states = vec![];
        for state in &self.states {
            let state_actions = &self.actions[state.index.0];
            let has_reductions = state_actions.iter().any(|actions| matches!(actions, Action::Reduce(_)));
            if has_reductions && state_actions.len() > 1 {
                inadequate_states.push(state.clone());
            }
        }
        inadequate_states
    }

//    pub fn conflicts_on(&self, state_index: StateIndex) -> Vec<Conflict<'g, '_>> {
//        let mut conflicts = vec![];
//
//        let state = &self.states[state_index.0];
//
//        for symbol in self.grammar.symbols() {
//            let actions = self.transition(state.index(), symbol.index());
//            let next_state = self.transition(state_index, symbol.index());
//            if next_state.is_some() && actions.len() > 1 {
//                conflicts.push(Conflict {
//                    table: self,
//                    state: state_index,
//                    symbol: Some(symbol),
//                    actions: actions.clone(),
//                });
//            }
//        }
//
//        let symbol = None::<Symbol<'g>>;
//        let actions = self.actions_for(state.clone(), symbol);
//        if actions.len() > 1 {
//            conflicts.push(Conflict {
//                table: self,
//                state: state_index,
//                symbol,
//                actions: actions.clone(),
//            });
//        }
//        conflicts
//    }
//
//    pub fn conflicts(&self) -> Vec<Conflict<'_, '_>> {
//        let mut conflicts = vec![];
//        for state_index in 0..self.states.len() {
//            let state_index = StateIndex(state_index);
//            conflicts.extend(self.conflicts_on(state_index));
//        }
//        conflicts
//    }

    pub fn dump(&self) {
        for (state_index, state) in self.states.iter().enumerate() {
            let state_index = StateIndex(state_index);
            eprintln!("{state_index:?}");
            eprintln!("{state:?}");

            for symbol in self.grammar.symbols() {
                eprintln!("    {symbol:?} => {:?}", self.transition(state_index, symbol.index()));
            }
            //eprintln!("    None => {:?}", self.get(state_index, None));
            eprintln!();
        }
    }
}

//impl<'g, 't> std::fmt::Debug for Conflict<'g, 't> {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        let state_id = self.state;
//        let symbol = self.symbol;
//        let actions = &self.actions;
//        write!(f, "Conflict(state={state_id:?}, symbol={symbol:?}, actions={actions:?})")?;
//        Ok(())
//    }
//}
//
//impl<'g, 't> Conflict<'g, 't> {
//    pub fn table(&self) -> &'t ParseTable<'g> {
//        self.table
//    }
//
//    pub fn state(&self) -> &'t State<'_> {
//        &self.table.states[usize::from(self.state)]
//    }
//
//    pub fn symbol(&self) -> Option<Symbol<'g>> {
//        self.symbol
//    }
//
//    pub fn actions(&self) -> &[Action<'g>] {
//        &self.actions
//    }
//}
