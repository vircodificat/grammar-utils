use std::collections::BTreeSet;

use crate::*;
use crate::analysis::GrammarAnalysis;

#[test]
fn test_grammar() {
    let grammar = grammar! {
        S -> A;
        A -> B x;
        B -> y A;
        B -> y y;
    };

    let symbols_actual: BTreeSet<String> = grammar.symbols().into_iter().map(|symbol| symbol.name()).collect();
    let symbols_expected: BTreeSet<String> = vec!["x", "y", "A", "B", "S"].into_iter().map(|s| s.to_string()).collect();
    assert_eq!(symbols_expected, symbols_actual);
}

#[test]
fn test_nullables() {
    let grammar = grammar! {
        S -> A;
        A -> ;
        B -> ;
        A -> x;
        C -> y;
        D -> y;
        D -> B;
        E -> A B;
    };

    let analysis = GrammarAnalysis::build(&grammar);
    let nullables_actual: BTreeSet<String> = analysis.nullables().into_iter().map(|symbol| symbol.name()).collect();
    let nullables_expected: BTreeSet<String> = vec!["S", "A", "B", "D", "E"].into_iter().map(|s| s.to_string()).collect();
    assert_eq!(nullables_actual, nullables_expected);
}

#[test]
fn test_nullables_with_empty() {
    let grammar = grammar! {
        S -> A;
        A -> x;
        A -> ;
        B -> A;
        C -> x;
    };

    let s = grammar.symbol("S").unwrap();
    let a = grammar.symbol("A").unwrap();
    let b = grammar.symbol("B").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert_eq!(analysis.nullables(), [s, a, b].iter().copied().collect());
}

#[test]
fn test_follow_simple() {
    let grammar = grammar! {
        S -> A;
        A -> x;
        A -> ;
        B -> A x;
        B -> A y;
    };

    let a = grammar.symbol("A").unwrap();
    let b = grammar.symbol("B").unwrap();
    let x = grammar.symbol("x").unwrap();
    let y = grammar.symbol("y").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert_eq!(analysis.follow(a), [x, y].iter().copied().collect());
    assert_eq!(analysis.follow(b), [].iter().copied().collect());
}

/// Make sure FIRST(A) is defined in the "obvious" case.
#[test]
fn test_first() {
    let grammar = grammar! {
        S -> A;
        A -> x;
        A -> y;
    };

    let a = grammar.symbol("A").unwrap();
    let x = grammar.symbol("x").unwrap();
    let y = grammar.symbol("y").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert_eq!(analysis.first(a), [x, y].iter().copied().collect());
}

/// Make sure NULLABLE(A) is defined.
/// Ensure that it handles both the immediate case and the recursive case.
#[test]
fn test_empty() {
    let grammar = grammar! {
        S -> A;
        A -> x;
        A -> ;
        B -> A;
        C -> x;
    };

    let s = grammar.symbol("S").unwrap();
    let a = grammar.symbol("A").unwrap();
    let b = grammar.symbol("B").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert_eq!(analysis.nullables(), [s, a, b].iter().copied().collect());
}

/// Make sure FIRST(A) handles the case where the first symbol of the RHS of a rule is nullable.
#[test]
fn test_first_with_empty() {
    let grammar = grammar! {
        S -> A;
        A -> x;
        A -> ;
        B -> A x;
    };

    let a = grammar.symbol("A").unwrap();
    let b = grammar.symbol("B").unwrap();
    let x = grammar.symbol("x").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert_eq!(analysis.first(a), [x].iter().copied().collect());
    assert_eq!(analysis.first(b), [x].iter().copied().collect());
}

/// Test FIRST even when you have left recursion.
#[test]
fn test_first_left_recursion() {
    let grammar = grammar! {
        S -> A;
        A -> x;
        A -> A x;
    };

    let a = grammar.symbol("A").unwrap();
    let x = grammar.symbol("x").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert_eq!(analysis.first(a), [x].iter().copied().collect());
}

/// Test FIRST even when you have mutual recursion.
#[test]
fn test_first_mutual_recursion() {
    let grammar = grammar! {
        S -> A;
        A -> x;
        A -> B;
        B -> A;
    };

    let a = grammar.symbol("A").unwrap();
    let x = grammar.symbol("x").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert_eq!(analysis.first(a), [x].iter().copied().collect());
}

#[test]
fn test_follow_nullable() {
    let grammar = grammar! {
        S -> A;
        A -> x;
        B -> A y;
        B -> A C z;
        C -> ;
    };

    let a = grammar.symbol("A").unwrap();
    let y = grammar.symbol("y").unwrap();
    let z = grammar.symbol("z").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert_eq!(analysis.follow(a), [y, z].iter().copied().collect());
}

#[test]
fn test_follow_nullable2() {
    let grammar = grammar! {
        S -> A;
        A -> B C D;
        B -> x;
        B -> ;
        C -> y;
        C -> ;
        D -> ;
        E -> A z;
    };

    let b = grammar.symbol("B").unwrap();
    let y = grammar.symbol("y").unwrap();
    let z = grammar.symbol("z").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert_eq!(analysis.follow(b), [y, z].iter().copied().collect());
}

#[test]
fn test_follow_nullable3() {
    let grammar = grammar! {
        S -> A;
        A -> B C D;
        B -> x;
        C -> y;
        D -> z;
    };

    let b = grammar.symbol("B").unwrap();
    let y = grammar.symbol("y").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert_eq!(analysis.follow(b), [y].iter().copied().collect());
}

#[test]
fn test_first2() {
    let grammar = grammar! {
        S -> A;
        A -> B;
        B -> C;
        C -> D y;
        D -> x;
        D -> ;
    };

    let analysis = GrammarAnalysis::build(&grammar);

    let a = grammar.symbol("A").unwrap();
    let b = grammar.symbol("B").unwrap();
    let c = grammar.symbol("C").unwrap();
    let d = grammar.symbol("D").unwrap();
    let x = grammar.symbol("x").unwrap();
    let y = grammar.symbol("y").unwrap();

    assert_eq!(analysis.first(a), [x, y].iter().copied().collect());
    assert_eq!(analysis.first(b), [x, y].iter().copied().collect());
    assert_eq!(analysis.first(c), [x, y].iter().copied().collect());
    assert_eq!(analysis.first(d), [x].iter().copied().collect());
}

#[test]
fn test_first_nullable() {
    let grammar = grammar! {
        S -> A;
        A -> A x;
        A -> ;
    };

    let analysis = GrammarAnalysis::build(&grammar);

    let a = grammar.symbol("A").unwrap();
    let x = grammar.symbol("x").unwrap();

    assert_eq!(analysis.first(a), [x].iter().copied().collect());
}

#[test]
fn test_is_terminal() {
    let grammar = grammar! {
        S -> A;
        A -> A x;
        A -> ;
    };

    let a = grammar.symbol("A").unwrap();
    let x = grammar.symbol("x").unwrap();

    assert!(!a.is_terminal());
    assert!(x.is_terminal());
}

#[test]
fn test_is_nullable_seq() {
    let grammar = grammar! {
        S -> A;
        A -> A x;
        A -> ;
        B -> A A A;
        B -> x;
    };

    let a = grammar.symbol("A").unwrap();
    let b = grammar.symbol("B").unwrap();
    let x = grammar.symbol("x").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert!(analysis.is_nullable_seq(&[a, b]));
    assert!(analysis.is_nullable_seq(&[]));
    assert!(analysis.is_nullable_seq(&[a, a, a, a]));

    assert!(!analysis.is_nullable_seq(&[x, b]));
}

#[test]
fn test_first_seq() {
    let grammar = grammar! {
        S -> A;
        A -> A x;
        A -> ;
        B -> A A A;
        B -> y;
    };

    let a = grammar.symbol("A").unwrap();
    let b = grammar.symbol("B").unwrap();
    let x = grammar.symbol("x").unwrap();
    let y = grammar.symbol("y").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert_eq!(analysis.first_seq(&[]), vec![].into_iter().collect());
    assert_eq!(analysis.first_seq(&[a]), analysis.first(a));
    assert_eq!(analysis.first_seq(&[b]), analysis.first(b));

    assert_eq!(analysis.first_seq(&[a, b]), vec![x, y].into_iter().collect());
    assert_eq!(analysis.first_seq(&[a, y]), vec![x, y].into_iter().collect());
}

#[test]
fn test_can_end_with() {
    let grammar = grammar! {
        S -> A;
        A -> x;
        A -> ;
        A -> B;
        A -> C B;
        B -> y;
        C -> x;
    };

    let a = grammar.symbol("A").unwrap();
    let b = grammar.symbol("B").unwrap();
    let c = grammar.symbol("C").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert!(analysis.can_end_with(a, a));
    assert!(analysis.can_end_with(a, b));
    assert!(!analysis.can_end_with(a, c));

    assert!(!analysis.can_end_with(b, a));
    assert!(analysis.can_end_with(b, b));
    assert!(!analysis.can_end_with(b, c));

    assert!(!analysis.can_end_with(c, a));
    assert!(!analysis.can_end_with(c, b));
    assert!(analysis.can_end_with(c, c));
}

#[test]
fn ll1_example1() {
    let grammar = grammar! {
        S -> E;
        E -> T Emore;
        Emore -> plus T Emore;
        Emore -> ;
        T -> F Tmore ;
        Tmore -> times F Tmore ;
        Tmore -> ;
        F -> id;
        F -> lparen E rparen;
    };

    let table = ll1::ParseTable::build(&grammar, grammar.symbol("E").unwrap());
    eprintln!("{table:?}");

    let input = vec![
        grammar.symbol("id").unwrap(),
        grammar.symbol("plus").unwrap(),
        grammar.symbol("id").unwrap(),
        grammar.symbol("times").unwrap(),
        grammar.symbol("id").unwrap(),
    ].into_iter();
    let mut machine = ll1::Machine::new(table, grammar.symbol("E").unwrap(), input);
    machine.run();
}
