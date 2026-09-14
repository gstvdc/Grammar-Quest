use crate::symbol::Symbol;

#[derive(Debug, Default)]
pub struct Stack {
    items: Vec<Symbol>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Pushes `rhs` so that its leftmost symbol ends up on top of the
    /// stack — matching the professor's algorithm ("o símbolo mais a
    /// esquerda da produção deve estar no topo da pilha").
    pub fn push_production(&mut self, rhs: &[Symbol]) {
        for symbol in rhs.iter().rev() {
            self.items.push(symbol.clone());
        }
    }

    pub fn pop(&mut self) -> Option<Symbol> {
        self.items.pop()
    }

    pub fn snapshot_top_first(&self) -> Vec<Symbol> {
        self.items.iter().rev().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol::Symbol;

    #[test]
    fn new_stack_is_empty() {
        assert!(Stack::new().is_empty());
    }

    #[test]
    fn push_production_puts_leftmost_symbol_on_top() {
        let mut stack = Stack::new();
        stack.push_production(&[Symbol::Terminal('a'), Symbol::NonTerminal("S".to_string())]);
        assert_eq!(stack.pop(), Some(Symbol::Terminal('a')));
        assert_eq!(stack.pop(), Some(Symbol::NonTerminal("S".to_string())));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn snapshot_lists_top_of_stack_first() {
        let mut stack = Stack::new();
        stack.push_production(&[Symbol::Terminal('a'), Symbol::NonTerminal("S".to_string())]);
        assert_eq!(
            stack.snapshot_top_first(),
            vec![Symbol::Terminal('a'), Symbol::NonTerminal("S".to_string())]
        );
    }

    #[test]
    fn empty_production_pushes_nothing() {
        let mut stack = Stack::new();
        stack.push_production(&[]);
        assert!(stack.is_empty());
    }
}
