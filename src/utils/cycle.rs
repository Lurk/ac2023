use std::usize;

pub struct CycleDetector {
    stack: Vec<usize>,
}

impl CycleDetector {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, value: usize) -> Option<usize> {
        self.stack.push(value);
        self.has_cycle()
    }

    fn has_cycle(&self) -> Option<usize> {
        (self.stack.len() / 2..self.stack.len() - 1)
            .rev()
            .find(|&i| self.has_reflection_at_index(i))
    }

    fn has_reflection_at_index(&self, index: usize) -> bool {
        let (head, tail) = self.stack.split_at(index);
        head.ends_with(tail)
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn get_slice(&self, start: usize, end: usize) -> &[usize] {
        &self.stack[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle() {
        let detector = CycleDetector {
            stack: vec![1, 2, 3, 4, 2, 3, 4],
        };
        assert_eq!(detector.has_cycle(), Some(4));
    }
}
