use crate::*;
use crate::lr0::*;

#[test]
fn test_conflicts() {
    let grammar = grammar! {
        S -> E;
        E -> T plus E;
        E -> T;
        T -> F times T;
        T -> F;
        F -> id;
        F -> lparen E rparen;
    };
    let table = ParseTable::build(&grammar, grammar.rules()[0]);

    dbg!(&table.states.len());
    dbg!(table.conflicts());
    for conflict in table.conflicts() {
        eprintln!("{conflict:?}");
        eprintln!("{:?}", conflict.state());
        eprintln!();
    }
    assert_eq!(table.conflicts().len(), 2);
}

#[test]
fn test_machine() {
    let grammar = grammar! {
        Sprime -> S;
        S -> a A;
        A -> b;
    };

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
    let grammar = grammar! {
        A -> x y z;
    };

    let rule0 = grammar.rules()[0];
    assert_eq!(&format!("{:?}", Item::new(rule0, 0)), "A -> . x y z");
    assert_eq!(&format!("{:?}", Item::new(rule0, 1)), "A -> x . y z");
    assert_eq!(&format!("{:?}", Item::new(rule0, 2)), "A -> x y . z");
    assert_eq!(&format!("{:?}", Item::new(rule0, 3)), "A -> x y z .");
}

#[test]
fn step_item() {
    let grammar = grammar! {
        A -> x y z;
    };

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
