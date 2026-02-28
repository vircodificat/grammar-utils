#[derive(Debug)]
pub struct Dfa {
    transitions: Vec<Vec<StateIdx>>,
    state: StateIdx,
}

pub type StateIdx = usize;

pub type SymbolIdx = usize;


impl Dfa {
    pub fn new(transitions: Vec<Vec<StateIdx>>) -> Dfa {
        let num_states = transitions.len();
        let num_symbols = transitions[0].len();
        for row in &transitions {
            assert_eq!(num_symbols, row.len());
            for entry in row {
                assert!(usize::from(*entry) < num_states);
            }
        }

        Dfa {
            transitions,
            state: 0,
        }
    }

    pub fn state(&self) -> StateIdx {
        self.state
    }

    pub fn step(&mut self, symbol: SymbolIdx) -> StateIdx {
        let state_transitions = &self.transitions[self.state];
        let next_state = state_transitions[symbol];
        self.state = next_state;
        self.state
    }
}

#[test]
fn test1() {
    let mut dfa = Dfa::new(vec![
        vec![1, 4],
        vec![2, 3],
        vec![4, 4],
        vec![2, 3],
        vec![4, 4],
    ]);

    assert_eq!(dfa.state(), 0);
    dfa.step(0);
    assert_eq!(dfa.state(), 1);
    dfa.step(1);
    assert_eq!(dfa.state(), 3);
    dfa.step(1);
    assert_eq!(dfa.state(), 3);
    dfa.step(1);
    assert_eq!(dfa.state(), 3);
    dfa.step(0);
    assert_eq!(dfa.state(), 2);
    dfa.step(0);
    assert_eq!(dfa.state(), 4);
}
