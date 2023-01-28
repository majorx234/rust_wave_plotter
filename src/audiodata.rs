use std::{collections::VecDeque, default};

pub struct Audiodata {
    pub values: VecDeque<f32>,
    pub window_size: usize,
}

impl Audiodata {
    fn new(window_size: usize) -> Self {
        Audiodata {
            values: VecDeque::new(),
            window_size: window_size,
        }
    }

    fn append(&mut self, sample: f32) {
        self.values.push_back(sample);
        while self.values.len() > self.window_size {
            self.values.pop_front();
        }
    }

    fn get_values(&self) -> Vec<f32> {
        self.values.iter().copied().collect()
    }
}
