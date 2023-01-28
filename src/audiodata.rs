use std::{collections::VecDeque, default};

pub struct Audiodata {
    pub values: VecDeque<f32>,
    pub window_size: usize,
}

impl Audiodata {
    fn new() -> Self {
        Audiodata {
            values: VecDeque::new(),
            window_size: 192000,
        }
    }
}
