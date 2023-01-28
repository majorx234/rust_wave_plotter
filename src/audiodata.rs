use std::{collections::VecDeque, default};

pub struct Audiodata {
    pub values: VecDeque<(f32, f32)>,
    pub window_size: usize,
    pub chunk_size: usize,
}

impl Audiodata {
    pub fn new(window_size: usize, chunk_size: usize) -> Self {
        Audiodata {
            values: VecDeque::new(),
            window_size,
            chunk_size,
        }
    }

    pub fn append(&mut self, sample: (f32, f32)) {
        self.values.push_back(sample);

        while self.values.len() > self.window_size {
            self.values.pop_front();
        }
    }

    pub fn get_values(&self) -> Vec<(f32, f32)> {
        self.values.iter().copied().collect::<Vec<(f32, f32)>>()
    }

    pub fn get_chunk_size(&self) -> usize {
        self.chunk_size
    }
}
