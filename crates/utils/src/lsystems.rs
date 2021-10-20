use std::collections::HashMap;

pub struct LSystem<Alphabet: Clone> {
    axiom: Vec<Alphabet>,
    rule: Box<dyn FnMut(Alphabet) -> Vec<Alphabet>>,
    memo: HashMap<usize, Vec<Alphabet>>,
}

impl<Alphabet: Clone> LSystem<Alphabet> {
    pub fn new(axiom: Vec<Alphabet>, rule: Box<dyn FnMut(Alphabet) -> Vec<Alphabet>>) -> Self {
        Self {
            axiom,
            rule,
            memo: Default::default(),
        }
    }

    pub fn nth(&mut self, i: usize) -> Vec<Alphabet> {
        if i == 0 {
            return self.axiom.clone();
        }
        if let Some(a) = self.memo.get(&i) {
            return a.clone();
        }

        let last = self.nth(i - 1);

        let mut res = Vec::new();
        for letter in last {
            res.extend((self.rule)(letter).into_iter());
        }

        self.memo.insert(i, res.clone());

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lsystems() {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        enum Test {
            A,
            B,
        }
        use Test::*;

        let mut sys = LSystem::new(
            vec![A],
            Box::new(|i| match i {
                A => vec![A, B],
                B => vec![A],
            }),
        );

        assert_eq!(vec![A], sys.nth(0));
        assert_eq!(vec![A, B], sys.nth(1));
        assert_eq!(vec![A, B, A], sys.nth(2));
        assert_eq!(vec![A, B, A, A, B], sys.nth(3));
        assert_eq!(vec![A, B, A, A, B, A, B, A], sys.nth(4));
    }
}
