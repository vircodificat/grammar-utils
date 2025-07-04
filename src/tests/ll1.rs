/*
use crate::{ll1::ParseTable, *};

#[test]
fn debug_for_items2() {
    let grammar = grammar! {
        start -> S;
        S -> C C;
        C -> c C;
        C -> d;
    };

    let rule0 = grammar.rules()[0];

    let table = ParseTable::build(&grammar, rule0);
    assert_eq!(table.conflicts().len(), 0);
    assert_eq!(table.states.len(), 14);
}
*/
