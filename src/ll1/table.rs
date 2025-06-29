use std::collections::HashMap;

use crate::*;

pub struct ParseTable<'g> {
    grammar: &'g Grammar,
    start_symbol: Symbol<'g>,
    table: HashMap<(Symbol<'g>, Option<Symbol<'g>>), Vec<Rule<'g>>>,
}

impl<'g> ParseTable<'g> {
    pub fn build(grammar: &'g Grammar, start_symbol: Symbol<'g>) -> ParseTable<'g> {
        ParseTable {
            grammar,
            start_symbol,
            table: Self::build_table(grammar, start_symbol),
        }
    }

    pub fn grammar(&self) -> &'g Grammar {
        self.grammar
    }

    pub fn start_symbol(&self) -> Symbol<'g> {
        self.start_symbol
    }

    fn build_table(grammar: &'g Grammar, start_symbol: Symbol<'g>)
        -> HashMap<(Symbol<'g>, Option<Symbol<'g>>), Vec<Rule<'g>>> {

        let analysis = GrammarAnalysis::build(grammar);
        let mut map = HashMap::new();

        let rules = grammar.rules();

        for rule in rules.iter().copied() {
            let rhs = rule.rhs();
            if analysis.is_nullable_seq(&rhs) {
                for follow in analysis.follow(rule.lhs()) {
                    let key = (rule.lhs(), Some(follow));
                    if !map.contains_key(&key) {
                        map.insert(key, vec![]);
                    }
                    map.get_mut(&key).unwrap().push(rule);
                }

                if analysis.can_end_with(start_symbol, rule.lhs()) {
                    let key = (rule.lhs(), None);
                    if !map.contains_key(&key) {
                        map.insert(key, vec![]);
                    }
                    map.get_mut(&key).unwrap().push(rule);
                }
            } else {
                for first in analysis.first_seq(&rhs) {
                    let key = (rule.lhs(), Some(first));
                    if !map.contains_key(&key) {
                        map.insert(key, vec![]);
                    }
                    map.get_mut(&key).unwrap().push(rule);
                }
            }
        }

        map
    }

    pub fn get(&self, state: Symbol<'g>, input: Option<Symbol<'g>>) -> Vec<Rule<'g>> {
        self.table.get(&(state, input)).map(|v| v.as_slice()).unwrap_or_else(|| &[]).to_vec()
    }
}

impl<'g> std::fmt::Debug for ParseTable<'g> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for nonterminal in self.grammar.nonterminals() {
            writeln!(f, "{nonterminal:?}")?;
            for terminal in self.grammar.terminals() {
                let entry = &self.get(nonterminal, Some(terminal));
                if entry.len() > 0 {
                    writeln!(f, "    {terminal:?}\t{entry:?}")?;
                }
            }

            let entry = &self.get(nonterminal, None);
            if entry.len() > 0 {
                writeln!(f, "    EOF\t{entry:?}")?;
            }
        }
        Ok(())
    }
}
