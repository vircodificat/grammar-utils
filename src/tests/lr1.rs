use crate::*;
use crate::lr1::*;

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
    assert_eq!(table.conflicts().len(), 0);
}

#[test]
fn test_machine() {
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
    table.dump();
    let mut machine = Machine::new(&table);
    let mut input = [
        grammar.symbol("id").unwrap(),
        grammar.symbol("+").unwrap(),
        grammar.symbol("id").unwrap(),
        grammar.symbol("*").unwrap(),
        grammar.symbol("id").unwrap(),
    ].into_iter();
    machine.run(&mut input);
}
