use crate::{ll1::ParseTable, *};

//#[test]
//fn debug_for_items2() {
//    let grammar = Grammar::new()
//        .symbol("S'")
//        .symbol("S")
//        .symbol("C")
//        .symbol("c")
//        .symbol("d")
//        .rule("S'", &["S"])
//        .rule("S", &["C", "C"])
//        .rule("C", &["c", "C"])
//        .rule("C", &["d"])
//        .build();
//
//    let rule0 = grammar.rules()[0];
//
//    let table = ParseTable::build(&grammar, rule0);
//    assert_eq!(table.conflicts().len(), 0);
//    assert_eq!(table.states.len(), 14);
//}
