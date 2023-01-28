use std::{collections::VecDeque, default};

pub struct Audiodata {
    pub values: VecDeque<f32>,
    pub window_size: usize,
}

impl Audiodata {
    pub fn new(window_size: usize) -> Self {
        Audiodata {
            values: VecDeque::new(),
            window_size,
        }
    }

    pub fn append(&mut self, sample: f32) {
        self.values.push_back(sample);
        while self.values.len() > self.window_size {
            self.values.pop_front();
        }
    }

    pub fn get_values(&self) -> Vec<f32> {
        self.values.iter().copied().collect()
    }
}
