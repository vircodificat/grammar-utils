#[macro_export]
macro_rules! grammar {
    (
        $(
            $lhs:ident -> $( $rhs:ident )* ;
        )*
    ) => {{
        let mut symbols = ::std::collections::HashSet::<String>::new();
        let mut g = Grammar::new();

        $(
            symbols.insert(stringify!($lhs).to_string());
            $(
                symbols.insert(stringify!($rhs).to_string());
            )*
        )*

        for symbol in &symbols {
            g = g.symbol(symbol.as_str())
        }

        $(
            g = g.rule(stringify!($lhs), &[
                $(stringify!($rhs),)*
            ]);
        )*
        g.build()
    }};
}
