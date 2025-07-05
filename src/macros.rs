#[macro_export]
macro_rules! grammar {
    (
        $start:ident -> $start_rhs:ident ;
        $(
            $lhs:ident -> $( $rhs:ident )* ;
        )*
    ) => {{
        let mut symbols = ::std::collections::BTreeSet::<String>::new();
        let mut g = $crate::Grammar::new();

        $(
            symbols.insert(stringify!($lhs).to_string());
            $(
                symbols.insert(stringify!($rhs).to_string());
            )*
        )*

        g = g.symbol(stringify!($start));
        for symbol in &symbols {
            g = g.symbol(symbol.as_str())
        }

        g = g.rule(stringify!($start), &[stringify!($start_rhs)]);
        $(
            g = g.rule(stringify!($lhs), &[
                $(stringify!($rhs),)*
            ]);
        )*
        g.build()
    }};
}

#[macro_export]
macro_rules! rule {
    (
        $grammar:ident,
        $lhs:ident -> $( $rhs:ident )*
    ) => {{
        'result: {
            let lhs = $grammar.symbol(stringify!($lhs)).unwrap();
            let mut rhs = vec![];
            $(
                rhs.push($grammar.symbol(stringify!($rhs)).unwrap());
            )*
            for rule in $grammar.rules() {
                if rule.lhs() == lhs && rule.rhs() == rhs {
                    break 'result rule;
                }
            }
            panic!("No such rule: {lhs:?} -> {rhs:?}")
        }
    }};
}
