use std::collections::BTreeSet;

use crate::*;

#[test]
fn test_macro() {
    let grammar = grammar! {
        S -> A;
        A -> x B;
        B -> y B;
        B -> ;
    };

    let symbols_actual: BTreeSet<_> =
        grammar
        .symbols()
        .into_iter()
        .map(|symbol| format!("{symbol:?}"))
        .collect();
    let symbols_expected: BTreeSet<_> =
        ["S", "A", "B", "x", "y"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(symbols_expected, symbols_actual);
}


#[test]
fn test_rule() {
    let grammar = grammar! {
        S -> A;
        A -> x;
        A -> y;
    };

    assert_eq!(usize::from(rule!(grammar, S -> A).index()), 0);
    assert_eq!(usize::from(rule!(grammar, A -> x).index()), 1);
    assert_eq!(usize::from(rule!(grammar, A -> y).index()), 2);
}
