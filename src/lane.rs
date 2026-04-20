use std::collections::{BTreeMap, BTreeSet};

use crate::{Grammar, GrammarAnalysis, Symbol, SymbolIndex, Item};
use crate::state::{State, StateIndex};
use crate::item::ItemSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Configuration<'g>(Item<'g>, StateIndex);

#[derive(Clone, Debug)]
pub struct Lane<'g>(Vec<Configuration<'g>>);

impl<'g> Lane<'g> {
    pub fn configurations(&self) -> &[Configuration<'g>] {
        self.0.as_slice()
    }

    pub fn lookaheads(&self, analysis: &GrammarAnalysis<'g>) -> BTreeSet<Symbol<'g>> {
        let configurations: Vec<&Configuration> = self.configurations()
            .iter()
            .rev()
            .collect();
        let last_configuration = configurations.first().unwrap();

        let mut lookahead = analysis.first_seq(&last_configuration.remaining_symbols());
        if !lookahead.is_empty() {
            return lookahead
        }

        for configuration in configurations.into_iter().skip(1) {
            // it we're looking at a configuration of the form:
            // A -> s1 ... * si si+1 ... sn
            // where si is a nonterminal
            // then we are about to "enter" the non-terminal
            // and this configuration generates a lookahead of
            // FIRST(si+1 ... sn)
            // (or if that's empty, we keep looking)
            if let Some(next_symbol) = configuration.item().next_symbol() &&
                next_symbol.is_nonterminal() {
                let remaining_symbols_skip1: Vec<_> = configuration
                    .item()
                    .remaining_symbols()
                    .into_iter()
                    .skip(1)
                    .collect();
                lookahead = analysis.first_seq(&remaining_symbols_skip1);
            }
        }

        lookahead
    }

    // Given a lane Z1, Z2, ..., Zn
    // calculates all possible configurations Z0 that would make a valid lane:
    // Z0, Z1, Z2, ..., Zn
    // and returns those lanes as a Vec.
    pub fn extensions(
        &self,
        states: &[State<'g>],
        transitions: &BTreeMap<(StateIndex, SymbolIndex), StateIndex>,
    ) -> Vec<Lane<'g>> {
        if self.configurations().is_empty() {
            return Vec::new();
        }

        let first_config = &self.configurations()[0];
        let target_state = first_config.state();
        let target_item = first_config.item();
        let grammar = target_item.grammar();

        let mut result = Vec::new();

        for (src_state_index, src_state) in states.iter().enumerate() {
            let src_state_index = StateIndex(src_state_index);
            let predecessors = self.find_predecessor_configs(
                src_state,
                src_state_index,
                target_state,
                target_item,
                grammar,
                transitions
            );

            for predecessor_config in predecessors {
                let extended_lane = self.create_extended_lane(predecessor_config);
                result.push(extended_lane);
            }
        }

        result
    }

    /// Find all predecessor configurations in a given source state that could lead
    /// to the target configuration
    fn find_predecessor_configs(
        &self,
        src_state: &State<'g>,
        src_state_index: StateIndex,
        target_state: StateIndex,
        target_item: &Item<'g>,
        grammar: &'g Grammar,
        transitions: &BTreeMap<(StateIndex, SymbolIndex), StateIndex>,
    ) -> Vec<Configuration<'g>> {
        let mut predecessors = Vec::new();

        for symbol in grammar.symbols() {
            if self.can_transition_to_target(src_state_index, symbol, target_state, transitions) {
                let matching_items = self.find_items_that_step_to_target(
                    src_state,
                    symbol,
                    target_item
                );

                for src_item in matching_items {
                    let predecessor_config = Configuration(src_item, src_state_index);
                    predecessors.push(predecessor_config);
                }
            }
        }

        predecessors
    }

    /// Check if a state can transition to the target state with the given symbol
    fn can_transition_to_target(
        &self,
        src_state_index: StateIndex,
        symbol: Symbol<'g>,
        target_state: StateIndex,
        transitions: &BTreeMap<(StateIndex, SymbolIndex), StateIndex>,
    ) -> bool {
        let symbol_index = symbol.index();
        if let Some(&dst_state_index) = transitions.get(&(src_state_index, symbol_index)) {
            dst_state_index == target_state
        } else {
            false
        }
    }

    /// Find all items in the source state that, when stepped with the given symbol,
    /// would produce the target item
    fn find_items_that_step_to_target(
        &self,
        src_state: &State<'g>,
        symbol: Symbol<'g>,
        target_item: &Item<'g>,
    ) -> Vec<Item<'g>> {
        let mut matching_items = Vec::new();

        for src_item in src_state.itemset().items() {
            if self.item_steps_to_target(&src_item, symbol, target_item) {
                matching_items.push(src_item);
            }
        }

        matching_items
    }

    /// Check if stepping an item with a symbol produces the target item
    fn item_steps_to_target(
        &self,
        src_item: &Item<'g>,
        symbol: Symbol<'g>,
        target_item: &Item<'g>,
    ) -> bool {
        if let Some(next_symbol) = src_item.next_symbol() {
            if next_symbol == symbol {
                if let Some(stepped_item) = src_item.step() {
                    return stepped_item.rule() == target_item.rule() &&
                           stepped_item.pos() == target_item.pos();
                }
            }
        }
        false
    }

    /// Create a new lane by prepending the predecessor configuration to this lane
    fn create_extended_lane(&self, predecessor_config: Configuration<'g>) -> Lane<'g> {
        let mut new_lane_configs = vec![predecessor_config];
        new_lane_configs.extend(self.configurations().iter().cloned());
        Lane(new_lane_configs)
    }
}

impl<'g> Configuration<'g> {
    pub fn item(&self) -> &Item<'g> {
        &self.0
    }

    pub fn state(&self) -> StateIndex {
        self.1
    }

    pub fn remaining_symbols(&self) -> Vec<Symbol<'g>> {
        self.item().remaining_symbols()
    }

    pub fn to_lane(&self) -> Lane<'g> {
        Lane(vec![self.clone()])
    }
}
