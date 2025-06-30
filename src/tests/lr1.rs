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
        grammar.symbol("(").unwrap(),
        grammar.symbol("id").unwrap(),
        grammar.symbol("+").unwrap(),
        grammar.symbol("id").unwrap(),
        grammar.symbol(")").unwrap(),
    ].into_iter();
    machine.run(&mut input);
}

#[test]
fn test_conflicts2() {
    let grammar = Grammar::new()
        .symbol("start")
        .symbol("command")
        .symbol("data")
        .symbol("file")
        .symbol("write")
        .symbol("read")
        .symbol("to")
        .symbol("from")
        .symbol("identifier")
        .rule("start", &["command"])
        .rule("command", &["write", "data", "to",   "file"])
        .rule("command", &["write", "file", "from", "data"])
        .rule("command", &["read",  "data", "from", "file"])
        .rule("command", &["read",  "file", "to",   "data"])
        .rule("file", &["identifier"])
        .rule("data", &["identifier"])
        .build();

    let table = ParseTable::build(&grammar, grammar.rules()[0]);
    table.dump();
    assert_eq!(table.conflicts().len(), 0);

    let table2 = crate::lr0::ParseTable::build(&grammar, grammar.rules()[0]);
    assert!(table2.conflicts().len() > 0);
}
