use std::collections::HashSet;

use super::*;
use super::analysis::GrammarAnalysis;

#[test]
fn test_grammar() {
    let grammar = Grammar::new()
        .symbol("x")
        .symbol("y")
        .symbol("A")
        .symbol("B")
        .rule("A", &["B", "x"])
        .rule("B", &["y", "A"])
        .rule("B", &["y", "y"])
        .build();

    let symbols_actual: Vec<String> = grammar.symbols().into_iter().map(|symbol| symbol.name()).collect();
    let symbols_expected: Vec<String> = vec!["x", "y", "A", "B"].into_iter().map(|s| s.to_string()).collect();
    assert_eq!(symbols_expected, symbols_actual);
}

#[test]
fn test_nullables() {
    let grammar = Grammar::new()
        .symbol("x")
        .symbol("y")
        .symbol("A")
        .symbol("B")
        .symbol("C")
        .symbol("D")
        .symbol("E")
        .rule("A", &[])
        .rule("B", &[])
        .rule("A", &["x"])
        .rule("C", &["y"])
        .rule("D", &["y"])
        .rule("D", &["B"])
        .rule("E", &["A", "B"])
        .build();

    let analysis = GrammarAnalysis::build(&grammar);
    let nullables_actual: HashSet<String> = analysis.nullables().into_iter().map(|symbol| symbol.name()).collect();
    let nullables_expected: HashSet<String> = vec!["A", "B", "D", "E"].into_iter().map(|s| s.to_string()).collect();
    assert_eq!(nullables_actual, nullables_expected);
}

#[test]
fn test_nullables_with_empty() {
    let grammar = Grammar::new()
        .symbol("x")
        .symbol("A")
        .symbol("B")
        .symbol("C")
        .rule("A", &["x"])
        .rule("A", &[])
        .rule("B", &["A"])
        .rule("C", &["x"])
        .build();

    let a = grammar.symbol("A").unwrap();
    let b = grammar.symbol("B").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert_eq!(analysis.nullables(), [a, b].iter().copied().collect());
}

#[test]
fn test_follow_simple() {
    let grammar = Grammar::new()
        .symbol("x")
        .symbol("y")
        .symbol("A")
        .symbol("B")
        .rule("A", &["x"])
        .rule("A", &[])
        .rule("B", &["A", "x"])
        .rule("B", &["A", "y"])
        .build();

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
    let grammar = Grammar::new()
        .symbol("A")
        .symbol("x")
        .symbol("y")
        .rule("A", &["x"])
        .rule("A", &["y"])
        .build();

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
    let grammar = Grammar::new()
        .symbol("A")
        .symbol("B")
        .symbol("C")
        .symbol("x")
        .rule("A", &["x"])
        .rule("A", &[])
        .rule("B", &["A"])
        .rule("C", &["x"])
        .build();

    let a = grammar.symbol("A").unwrap();
    let b = grammar.symbol("B").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert_eq!(analysis.nullables(), [a, b].iter().copied().collect());
}

/// Make sure FIRST(A) handles the case where the first symbol of the RHS of a rule is nullable.
#[test]
fn test_first_with_empty() {
    let grammar = Grammar::new()
        .symbol("A")
        .symbol("B")
        .symbol("x")
        .rule("A", &["x"])
        .rule("A", &[])
        .rule("B", &["A", "x"])
        .build();

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
    let grammar = Grammar::new()
        .symbol("A")
        .symbol("B")
        .symbol("x")
        .rule("A", &["x"])
        .rule("A", &["A", "x"])
        .build();

    let a = grammar.symbol("A").unwrap();
    let x = grammar.symbol("x").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert_eq!(analysis.first(a), [x].iter().copied().collect());
}

/// Test FIRST even when you have mutual recursion.
#[test]
fn test_first_mutual_recursion() {
    let grammar = Grammar::new()
        .symbol("A")
        .symbol("B")
        .symbol("x")
        .rule("A", &["x"])
        .rule("A", &["B"])
        .rule("B", &["A"])
        .build();

    let a = grammar.symbol("A").unwrap();
    let x = grammar.symbol("x").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert_eq!(analysis.first(a), [x].iter().copied().collect());
}

#[test]
fn test_follow_nullable() {
    let grammar = Grammar::new()
        .symbol("A")
        .symbol("B")
        .symbol("C")
        .symbol("x")
        .symbol("y")
        .symbol("z")
        .rule("A", &["x"])
        .rule("B", &["A", "y"])
        .rule("B", &["A", "C", "z"])
        .rule("C", &[])
        .build();

    let a = grammar.symbol("A").unwrap();
    let y = grammar.symbol("y").unwrap();
    let z = grammar.symbol("z").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert_eq!(analysis.follow(a), [y, z].iter().copied().collect());
}

#[test]
fn test_follow_nullable2() {
    let grammar = Grammar::new()
        .symbol("A")
        .symbol("B")
        .symbol("C")
        .symbol("D")
        .symbol("E")
        .symbol("x")
        .symbol("y")
        .symbol("z")
        .rule("A", &["B", "C", "D"])
        .rule("B", &["x"])
        .rule("B", &[])
        .rule("C", &["y"])
        .rule("C", &[])
        .rule("D", &[])
        .rule("E", &["A", "z"])
        .build();

    let b = grammar.symbol("B").unwrap();
    let y = grammar.symbol("y").unwrap();
    let z = grammar.symbol("z").unwrap();

    let analysis = GrammarAnalysis::build(&grammar);

    assert_eq!(analysis.follow(b), [y, z].iter().copied().collect());
}

#[test]
fn test_first2() {
    let grammar = Grammar::new()
        .symbol("A")
        .symbol("B")
        .symbol("C")
        .symbol("D")
        .symbol("x")
        .symbol("y")
        .rule("A", &["B"])
        .rule("B", &["C"])
        .rule("C", &["D", "y"])
        .rule("D", &["x"])
        .rule("D", &[])
        .build();

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
    let grammar = Grammar::new()
        .symbol("A")
        .symbol("x")
        .rule("A", &["A", "x"])
        .rule("A", &[])
        .build();

    let analysis = GrammarAnalysis::build(&grammar);

    let a = grammar.symbol("A").unwrap();
    let x = grammar.symbol("x").unwrap();

    assert_eq!(analysis.first(a), [x].iter().copied().collect());
}

#[test]
fn test_is_terminal() {
    let grammar = Grammar::new()
        .symbol("A")
        .symbol("x")
        .rule("A", &["A", "x"])
        .rule("A", &[])
        .build();

    let a = grammar.symbol("A").unwrap();
    let x = grammar.symbol("x").unwrap();

    assert!(!a.is_terminal());
    assert!(x.is_terminal());
}
