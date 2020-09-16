#[derive(Debug, Clone)]
pub struct Plugboard {
    pairs: Vec<(usize, usize)>,
}

impl Plugboard {
    pub fn new(a: &Vec<(usize, usize)>) -> Self {
        Self { pairs: a.clone() }
    }

    pub fn input(&self, i: usize) -> usize {
        if let Some(a) = self.pairs.iter().find(|x| x.0 == i) {
            a.1
        } else if let Some(a) = self.pairs.iter().find(|x| x.1 == i) {
            a.0
        } else {
            i
        }
    }
}

impl Default for Plugboard {
    fn default() -> Self {
        Self { pairs: vec![] }
    }
}
