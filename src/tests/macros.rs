use std::collections::HashSet;

use crate::*;

#[test]
fn test_macro() {
    let grammar = grammar! {
        A -> x B;
        B -> y B;
        B -> ;
    };

    let symbols_actual: HashSet<_> =
        grammar
        .symbols()
        .into_iter()
        .map(|symbol| format!("{symbol:?}"))
        .collect();
    let symbols_expected: HashSet<_> =
        ["A", "B", "x", "y"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(symbols_expected, symbols_actual);
}
