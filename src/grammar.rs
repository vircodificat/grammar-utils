#[derive(Clone, Copy, Eq, PartialEq)]
struct SymbolIdx(usize);

#[derive(Clone, Copy, Eq, PartialEq)]
struct RuleIdx(usize);

pub struct GrammarBuilder {
    symbols: Vec<SymbolData>,
    rules: Vec<RuleData>,
}

pub struct Grammar {
    symbols: Vec<SymbolData>,
    rules: Vec<RuleData>,
}

struct SymbolData {
    name: String,
}


struct RuleData {
    lhs: SymbolIdx,
    rhs: Vec<SymbolIdx>,
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

#[derive(Clone, Copy)]
pub struct Symbol<'g> {
    grammar: &'g Grammar,
    index: SymbolIdx,
}

#[derive(Clone, Copy)]
pub struct Rule<'g> {
    grammar: &'g Grammar,
    index: RuleIdx,
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

impl<'g> std::hash::Hash for Rule<'g> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.0.hash(state)
    }
}

impl<'g> Symbol<'g> {
    fn data(&self) -> &SymbolData {
        &self.grammar.symbols[self.index.0]
    }

    pub fn name(&self) -> String {
        self.data().name.clone()
    }

    pub fn is_terminal(&self) -> bool {
        !self.is_nonterminal()
    }

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
    pub fn grammar(&self) -> &'g Grammar {
        self.grammar
    }

    fn data(&self) -> &RuleData {
        &self.grammar.rules[self.index.0]
    }

    pub fn lhs(&self) -> Symbol<'g> {
        Symbol {
            grammar: self.grammar,
            index: self.data().lhs,
        }
    }

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
    pub fn new() -> GrammarBuilder {
        GrammarBuilder {
            symbols: vec![],
            rules: vec![],
        }
    }

    pub fn symbols(&self) -> Vec<Symbol> {
        let mut result = vec![];
        for index in 0..self.symbols.len() {
            result.push(Symbol {
                grammar: self,
                index: SymbolIdx(index),
            });
        }
        result
    }

    pub fn rules(&self) -> Vec<Rule> {
        let mut result = vec![];
        for index in 0..self.rules.len() {
            result.push(Rule {
                grammar: self,
                index: RuleIdx(index),
            });
        }
        result
    }

    pub fn symbol<S: AsRef<str>>(&self, name: S) -> Option<Symbol> {
        for (index, symbol) in self.symbols.iter().enumerate() {
            if symbol.name == name.as_ref() {
                return Some(Symbol {
                    grammar: self,
                    index: SymbolIdx(index),
                });
            }
        }
        None
    }
}

impl GrammarBuilder {
    pub fn symbol<S: Into<String>>(mut self, name: S) -> Self {
        self.symbols.push(SymbolData {
            name: name.into(),
        });
        self
    }

    pub fn rule<S: AsRef<str>>(mut self, lhs: S, rhs: &[S]) -> Self {
        let rhs: Vec<SymbolIdx> = rhs.iter().map(|symbol_name| self.symbol_index(symbol_name.as_ref())).collect();

        self.rules.push(RuleData {
            lhs: self.symbol_index(lhs.as_ref()),
            rhs,
        });
        self
    }

    pub fn build(self) -> Grammar {
        Grammar {
            symbols: self.symbols,
            rules: self.rules,
        }
    }

    fn symbol_index(&self, symbol_name: &str) -> SymbolIdx {
        for (i, symbol) in self.symbols.iter().enumerate() {
            if symbol.name == symbol_name {
                return SymbolIdx(i);
            }
        }
        panic!("No such symbol: {symbol_name}")
    }
}
