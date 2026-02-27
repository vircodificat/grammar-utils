// Offset into Grammar::symbols
#[derive(Clone, Copy, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct SymbolIndex(usize);

// Offset into Grammar::rules
#[derive(Clone, Copy, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct RuleIndex(usize);

impl From<SymbolIndex> for usize {
    fn from(value: SymbolIndex) -> Self {
        value.0
    }
}

impl From<RuleIndex> for usize {
    fn from(value: RuleIndex) -> Self {
        value.0
    }
}

pub struct GrammarBuilder {
    symbols: Vec<SymbolData>,
    rules: Vec<RuleData>,
}

/// `Grammar` represents a context-free grammar.
///
/// It tracks a set of symbols and a set of production rules showing how those symbols interact.
pub struct Grammar {
    symbols: Vec<SymbolData>,
    rules: Vec<RuleData>,
}

// Symbols should have a name.
// Ideally, these should be a C-style identifier.
struct SymbolData {
    name: String,
}

// A production rule takes a nonterminal symbol on the LHS
// to a sequence of symbols (terminal and/or nonterminal) on the RHS
struct RuleData {
    lhs: SymbolIndex,
    rhs: Vec<SymbolIndex>,
}

impl std::fmt::Debug for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rule in &self.rules() {
            writeln!(f, "{rule:?}")?;
        }
        Ok(())
    }
}

impl<'g> std::fmt::Debug for Symbol<'g> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl<'g> std::fmt::Debug for Rule<'g> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lhs = format!("{:?}", self.lhs());
        let rhs = self.rhs().into_iter().map(|symbol| format!("{symbol:?}")).collect::<Vec<_>>().join(" ");
        write!(f, "{lhs} -> {rhs}")
    }
}

/// A `Symbol` is a handle to a symbol inside of a `Grammar`.
#[derive(Clone, Copy)]
pub struct Symbol<'g> {
    grammar: &'g Grammar,
    index: SymbolIndex,
}

/// A `Rule` is a handle to a rule inside of a `Grammar`.
#[derive(Clone, Copy)]
pub struct Rule<'g> {
    grammar: &'g Grammar,
    index: RuleIndex,
}

impl<'g> PartialEq for Symbol<'g> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.grammar, other.grammar) && self.index == other.index
    }
}

impl<'g> Eq for Symbol<'g> {}

impl<'g> PartialEq for Rule<'g> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.grammar, other.grammar) && self.index == other.index
    }
}

impl<'g> Eq for Rule<'g> {}

impl<'g> std::hash::Hash for Symbol<'g> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.0.hash(state)
    }
}

impl<'g> PartialOrd for Symbol<'g> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl<'g> Ord for Symbol<'g> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl<'g> std::hash::Hash for Rule<'g> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.0.hash(state)
    }
}

impl<'g> PartialOrd for Rule<'g> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl<'g> Ord for Rule<'g> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl<'g> Symbol<'g> {
    /// The `Grammar` backing this symbol.
    pub fn grammar(&self) -> &'g Grammar {
        self.grammar
    }

    fn data(&self) -> &SymbolData {
        &self.grammar.symbols[self.index.0]
    }

    /// The name of the symbol.
    pub fn name(&self) -> String {
        self.data().name.clone()
    }

    /// Is the symbol a terminal?
    pub fn is_terminal(&self) -> bool {
        !self.is_nonterminal()
    }

    /// Is the symbol a nonterminal?
    pub fn is_nonterminal(&self) -> bool {
        for rule in self.grammar.rules() {
            if rule.lhs() == *self {
                return true;
            }
        }
        false
    }
}

impl<'g> Rule<'g> {
    /// The `Grammar` backing this rule.
    pub fn grammar(&self) -> &'g Grammar {
        self.grammar
    }

    pub fn index(&self) -> RuleIndex {
        self.index
    }

    pub fn is_start_rule(&self) -> bool {
        usize::from(self.index) == 0
    }

    fn data(&self) -> &RuleData {
        &self.grammar.rules[self.index.0]
    }

    /// The symbol on the LHS.
    pub fn lhs(&self) -> Symbol<'g> {
        Symbol {
            grammar: self.grammar,
            index: self.data().lhs,
        }
    }

    /// The sequence of symbols on the RHS.
    pub fn rhs(&self) -> Vec<Symbol<'g>> {
        let mut result = vec![];
        for symbol in &self.data().rhs {
            result.push(Symbol {
                grammar: self.grammar,
                index: *symbol,
            });
        }
        result
    }
}

impl Grammar {
    /// A builder API for creating a Grammar object.
    pub fn new() -> GrammarBuilder {
        GrammarBuilder {
            symbols: vec![],
            rules: vec![],
        }
    }

    pub fn start_symbol(&self) -> Symbol<'_> {
        self.start_rule().lhs()
    }

    pub fn start_rule(&self) -> Rule<'_> {
        Rule {
            grammar: self,
            index: RuleIndex(0),
        }
    }

    /// The set of symbols.
    pub fn symbols(&self) -> Vec<Symbol<'_>> {
        let mut result = vec![];
        for index in 0..self.symbols.len() {
            result.push(Symbol {
                grammar: self,
                index: SymbolIndex(index),
            });
        }
        result
    }

    /// The set of terminal symbols.
    pub fn terminals(&self) -> Vec<Symbol<'_>> {
        let mut result = vec![];
        for symbol in self.symbols() {
            if symbol.is_terminal() {
                result.push(symbol);
            }
        }
        result
    }

    /// The set of nonterminal symbols.
    pub fn nonterminals(&self) -> Vec<Symbol<'_>> {
        let mut result = vec![];
        for symbol in self.symbols() {
            if symbol.is_nonterminal() {
                result.push(symbol);
            }
        }
        result
    }

    /// The set of rules.
    pub fn rules(&self) -> Vec<Rule<'_>> {
        let mut result = vec![];
        for index in 0..self.rules.len() {
            result.push(Rule {
                grammar: self,
                index: RuleIndex(index),
            });
        }
        result
    }

    /// Fetch the symbol with the given name if it exists.
    pub fn symbol<S: AsRef<str>>(&self, name: S) -> Option<Symbol<'_>> {
        for (index, symbol) in self.symbols.iter().enumerate() {
            if symbol.name == name.as_ref() {
                return Some(Symbol {
                    grammar: self,
                    index: SymbolIndex(index),
                });
            }
        }
        None
    }
}

impl GrammarBuilder {
    /// Declare a symbol.
    pub fn symbol<S: Into<String>>(mut self, name: S) -> Self {
        let name: String = name.into();
        assert!(!self.symbols.iter().any(|symbol_data| symbol_data.name == name), "Symbol declared twice: {name}");
        self.symbols.push(SymbolData {
            name,
        });
        self
    }

    /// Declare a rule for this grammar.
    pub fn rule<S: AsRef<str>>(mut self, lhs: S, rhs: &[S]) -> Self {
        let rhs: Vec<SymbolIndex> = rhs.iter().map(|symbol_name| self.symbol_index(symbol_name.as_ref())).collect();

        self.rules.push(RuleData {
            lhs: self.symbol_index(lhs.as_ref()),
            rhs,
        });
        self
    }

    /// Finish building this object and return the result as a `Grammar`.
    pub fn build(self) -> Grammar {
        let start_rule = &self.rules[0];
        assert_eq!(start_rule.rhs.len(), 1);

        Grammar {
            symbols: self.symbols,
            rules: self.rules,
        }
    }

    fn symbol_index(&self, symbol_name: &str) -> SymbolIndex {
        for (i, symbol) in self.symbols.iter().enumerate() {
            if symbol.name == symbol_name {
                return SymbolIndex(i);
            }
        }
        panic!("No such symbol: {symbol_name}")
    }
}
