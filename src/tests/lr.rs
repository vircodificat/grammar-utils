use crate::table::ParseTable;
use crate::grammar;

#[test]
fn test_conflicts_lr() {
    let grammar = grammar! {
        start -> command;
        command -> write data to   file;
        command -> write file from data;
        command -> read  data from file;
        command -> read  file to   data;
        file -> identifier;
        data -> identifier;
    };

    let start_rule = grammar.rules()[0];
    let table = ParseTable::build(&grammar, start_rule);
    table.dump();
    assert_eq!(table.inadequate_states().len(), 1);
}

#[test]
fn test_conflicts_lr2() {
    let grammar = grammar! {
        start -> E;
        E  -> T;
        E  -> E Pm T;
        T  -> x;
        T  -> Lp E Rp;
        Pm -> p;
        Pm -> m;
    };

    let start_rule = grammar.rules()[0];
    let table = ParseTable::build(&grammar, start_rule);
    table.dump();
    assert_eq!(table.inadequate_states().len(), 1);
}

#[test]
fn test_conflicts_lr3() {
    let grammar = grammar! {
        start -> E;
        E -> a A d;
        E -> b A c;
        E -> a B c;
        E -> b B d;
        A -> e A;
        A -> e;
        B -> e B;
        B -> e;
    };

    let start_rule = grammar.rules()[0];
    let table = ParseTable::build(&grammar, start_rule);
    table.dump();
    assert_eq!(table.inadequate_states().len(), 1);
}

#[test]
fn test_conflicts_lr4() {
    let grammar = grammar! {
        start -> E;
        E -> A;
        E -> B;
        A -> x;
        B -> x;
    };

    let start_rule = grammar.rules()[0];
    let table = ParseTable::build(&grammar, start_rule);
    table.dump();
}
