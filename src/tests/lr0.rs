use crate::*;

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

    let table = ParseTable::new(&grammar, grammar.rules()[0]);
    dbg!(&table.states.len());
    dbg!(table.conflicts());
    for conflict in table.conflicts() {
        eprintln!("{conflict:?}");
        eprintln!("{:?}", conflict.state());
        eprintln!();
    }
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

    let table = ParseTable::new(&grammar, grammar.rules()[0]);
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
