use crate::*;
use crate::lr1::*;

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
    dbg!(&table.states().len());
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
    table.dump();
    let mut input = [
        grammar.symbol("id").unwrap(),
        grammar.symbol("plus").unwrap(),
        grammar.symbol("id").unwrap(),
        grammar.symbol("times").unwrap(),
        grammar.symbol("lparen").unwrap(),
        grammar.symbol("id").unwrap(),
        grammar.symbol("plus").unwrap(),
        grammar.symbol("id").unwrap(),
        grammar.symbol("rparen").unwrap(),
    ].into_iter();
    let mut machine = Machine::new(&table, &mut input);
    machine.run();
}

#[test]
fn test_conflicts2() {
    let grammar = grammar! {
        start -> command;
        command -> write data to   file;
        command -> write file from data;
        command -> read  data from file;
        command -> read  file to   data;
        file -> identifier;
        data -> identifier;
    };

    let table = ParseTable::build(&grammar, grammar.rules()[0]);
    table.dump();
    assert_eq!(table.conflicts().len(), 0);

    let table2 = crate::lr0::ParseTable::build(&grammar, grammar.rules()[0]);
    assert!(table2.conflicts().len() > 0);
}
