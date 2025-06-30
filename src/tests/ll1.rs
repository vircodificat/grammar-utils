use crate::*;

#[test]
fn debug_for_items2() {
    let grammar = Grammar::new()
        .symbol("S'")
        .symbol("S")
        .symbol("C")
        .symbol("c")
        .symbol("d")
        .rule("S'", &["S"])
        .rule("S", &["C", "C"])
        .rule("C", &["c", "C"])
        .rule("C", &["d"])
        .build();

    let rule0 = grammar.rules()[0];
    /*
    assert_eq!(&format!("{:?}", rule0.item(0)), "A -> . x y z");
    assert_eq!(&format!("{:?}", rule0.item(1)), "A -> x . y z");
    assert_eq!(&format!("{:?}", rule0.item(2)), "A -> x y . z");
    assert_eq!(&format!("{:?}", rule0.item(3)), "A -> x y z .");
    */

    let table = crate::lr1::ParseTable::build(&grammar, rule0);
    assert_eq!(table.conflicts().len(), 0);
    assert_eq!(table.states.len(), 9);
}

#[test]
fn debug_for_items() {
    let grammar = Grammar::new()
        .symbol("A")
        .symbol("x")
        .symbol("y")
        .symbol("z")
        .rule("A", &["x", "y", "z"])
        .build();

    let rule0 = grammar.rules()[0];
    /*
    assert_eq!(&format!("{:?}", rule0.item(0)), "A -> . x y z");
    assert_eq!(&format!("{:?}", rule0.item(1)), "A -> x . y z");
    assert_eq!(&format!("{:?}", rule0.item(2)), "A -> x y . z");
    assert_eq!(&format!("{:?}", rule0.item(3)), "A -> x y z .");
    */

    let table = crate::lr1::ParseTable::build(&grammar, rule0);
    dbg!(&table);
}

/*
#[test]
fn step_item() {
    let grammar = Grammar::new()
        .symbol("A")
        .symbol("x")
        .symbol("y")
        .symbol("z")
        .rule("A", &["x", "y", "z"])
        .build();

    let mut item = Item::new(grammar.rules()[0].item(0);
    assert_eq!(&format!("{item:?}"), "A -> . x y z");

    item = item.step().unwrap();
    assert_eq!(&format!("{item:?}"), "A -> x . y z");

    item = item.step().unwrap();
    assert_eq!(&format!("{item:?}"), "A -> x y . z");

    item = item.step().unwrap();
    assert_eq!(&format!("{item:?}"), "A -> x y z .");

    assert!(item.is_finished());
    assert!(item.step().is_none());
}
*/

