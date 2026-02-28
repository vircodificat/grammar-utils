use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub struct Nfa {
    num_states: u32,
    num_symbols: u32,
    transitions: BTreeMap<(StateIdx, Option<SymbolIdx>), BTreeSet<StateIdx>>,
    states: BTreeSet<StateIdx>,
}

pub type StateIdx = usize;
pub type SymbolIdx = usize;

impl Nfa {
    pub fn new(num_states: u32, num_symbols: u32) -> Nfa {
        Nfa {
            transitions: BTreeMap::new(),
            states: [0].into_iter().collect(),
            num_states,
            num_symbols,
        }
    }

    pub fn add_transition(&mut self, from: StateIdx, to: StateIdx, through: SymbolIdx) {
        debug_assert!(from < self.num_states as usize);
        debug_assert!(to < self.num_states as usize);
        debug_assert!(through < self.num_symbols as usize);

        let key = (from, Some(through));
        if !self.transitions.contains_key(&key) {
            self.transitions.insert(key, BTreeSet::new());
        }
        let targets = self.transitions.get_mut(&(from, Some(through))).unwrap();
        targets.insert(to);
    }

    pub fn add_free_transition(&mut self, from: StateIdx, to: StateIdx) {
        debug_assert!(from < self.num_states as usize);
        debug_assert!(to < self.num_states as usize);

        let key = (from, None);
        if !self.transitions.contains_key(&key) {
            self.transitions.insert(key, BTreeSet::new());
        }
        let targets = self.transitions.get_mut(&key).unwrap();
        targets.insert(to);
    }

    pub fn states(&self) -> Vec<StateIdx> {
        let mut state: Vec<StateIdx> = self.states.clone().into_iter().collect();
        state.sort();
        state
    }

    pub fn step(&mut self, symbol: SymbolIdx) -> &BTreeSet<StateIdx> {
        let mut next_states = BTreeSet::new();
        for from_state in &self.states {
            let key = (*from_state, Some(symbol));
            if self.transitions.contains_key(&key) {
                let to_states = &self.transitions[&key];
                next_states.extend(to_states);
            }
        }
        self.states = next_states.clone();
        self.step_frees();
        &self.states
    }

    fn step_frees(&mut self) {
        let mut visited = self.states.clone();
        let mut queue: Vec<StateIdx> = self.states.clone().into_iter().collect();

        while let Some(from_state) = queue.pop() {
            let key = (from_state, None);
            if let Some(to_states) = self.transitions.get(&key) {
                for to_state in to_states {
                    if !(visited.contains(to_state)) {
                        visited.insert(*to_state);
                        queue.push(*to_state);
                    }
                }
            }
        }

        self.states = visited;
    }
}

#[test]
fn test2() {
    let mut nfa = Nfa::new(3, 2);

    nfa.add_transition(0 as StateIdx, 1 as StateIdx, 0 as SymbolIdx);
    nfa.add_transition(1 as StateIdx, 1 as StateIdx, 1 as SymbolIdx);
    nfa.add_transition(1 as StateIdx, 2 as StateIdx, 0 as SymbolIdx);
    nfa.add_transition(1 as StateIdx, 2 as StateIdx, 1 as SymbolIdx);
    nfa.add_free_transition(1 as StateIdx, 2 as StateIdx);

    assert_eq!(nfa.states(), vec![0]);
    nfa.step(0);
    assert_eq!(nfa.states(), vec![1, 2]);
    nfa.step(1);
    assert_eq!(nfa.states(), vec![1, 2]);
    nfa.step(1);
    assert_eq!(nfa.states(), vec![1, 2]);
    nfa.step(0);
    assert_eq!(nfa.states(), vec![2]);
}

#[test]
fn test3() {
    let mut nfa = Nfa::new(6, 2);

    nfa.add_transition(0 as StateIdx, 1 as StateIdx, 0 as SymbolIdx);
    nfa.add_transition(4 as StateIdx, 5 as StateIdx, 1 as SymbolIdx);
    nfa.add_free_transition(1 as StateIdx, 2 as StateIdx);
    nfa.add_free_transition(1 as StateIdx, 3 as StateIdx);
    nfa.add_free_transition(3 as StateIdx, 4 as StateIdx);
    nfa.add_free_transition(1 as StateIdx, 0 as StateIdx);

    assert_eq!(nfa.states(), vec![0]);
    nfa.step(0);
    assert_eq!(nfa.states(), vec![0, 1, 2, 3, 4]);
    nfa.step(0);
    assert_eq!(nfa.states(), vec![0, 1, 2, 3, 4]);
    nfa.step(1);
    assert_eq!(nfa.states(), vec![5]);
    nfa.step(0);
    assert_eq!(nfa.states(), vec![]);
}
