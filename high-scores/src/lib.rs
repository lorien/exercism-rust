#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.iter().last() {
            Some(x) => Some(*x),
            None => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.scores.iter().max() {
            Some(x) => Some(*x),
            None => None,
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut ret = vec![];
        for val in self.scores.iter() {
            if let Some(idx) = ret.iter().position(|x| val >= x) {
                if ret.len() < 3 {
                    ret.insert(idx, *val);
                } else {
                    ret.pop();
                    ret.insert(idx, *val);
                }
            } else {
                if ret.len() < 3 {
                    ret.push(*val);
                }
            }
        }
        ret
    }
}
