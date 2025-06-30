use crate::*;
use crate::lr0::*;

#[test]
fn test_conflicts() {
    let grammar = Grammar::new()
        .symbol("*")
        .symbol("+")
        .symbol("id")
        .symbol("(")
        .symbol(")")
        .symbol("E")
        .symbol("E'")
        .symbol("T")
        .symbol("T'")
        .symbol("F")
        .symbol("S")
        .rule("S", &["E"])
        .rule("E", &["T", "+", "E"])
        .rule("E", &["T"])
        .rule("T", &["F", "*", "T"])
        .rule("T", &["F"])
        .rule("F", &["id"])
        .rule("F", &["(", "E", ")"])
        .build();

    let table = ParseTable::build(&grammar, grammar.rules()[0]);
    dbg!(&table.states.len());
    dbg!(table.conflicts());
    for conflict in table.conflicts() {
        eprintln!("{conflict:?}");
        eprintln!("{:?}", conflict.state());
        eprintln!();
    }
    assert_eq!(table.conflicts().len(), 3);
}

#[test]
fn test_machine() {
    let grammar = Grammar::new()
        .symbol("(")
        .symbol(")")
        .symbol("S'")
        .symbol("S")
        .symbol("A")
        .symbol("a")
        .symbol("b")
        .rule("S'", &["S"])
        .rule("S", &["a", "A"])
        .rule("A", &["b"])
        .build();

    let table = ParseTable::build(&grammar, grammar.rules()[0]);
    dbg!(&table.states.len());

    eprintln!("STATES:");
    eprintln!();
    for state in &table.states {
        eprintln!("{state:?}");
        eprintln!();
    }

    let mut machine = Machine::new(&table);

    let mut input = vec![
        grammar.symbol("a").unwrap(),
        grammar.symbol("b").unwrap(),
    ].into_iter();
    machine.run(&mut input);
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
    assert_eq!(&format!("{:?}", Item::new(rule0, 0)), "A -> . x y z");
    assert_eq!(&format!("{:?}", Item::new(rule0, 1)), "A -> x . y z");
    assert_eq!(&format!("{:?}", Item::new(rule0, 2)), "A -> x y . z");
    assert_eq!(&format!("{:?}", Item::new(rule0, 3)), "A -> x y z .");
}

#[test]
fn step_item() {
    let grammar = Grammar::new()
        .symbol("A")
        .symbol("x")
        .symbol("y")
        .symbol("z")
        .rule("A", &["x", "y", "z"])
        .build();

    let mut item = Item::new(grammar.rules()[0], 0);
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
