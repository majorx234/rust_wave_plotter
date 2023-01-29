use audio_lib::audiodata::Audiodata;
use eframe::egui;
use std::sync::Arc;
use std::sync::Mutex;

use egui::plot::{
    Arrows, Bar, BarChart, CoordinatesFormatter, Corner, GridInput, GridMark, HLine, Legend, Line,
    LineStyle, MarkerShape, Plot, PlotImage, Points, Polygon, Text, VLine, Value, Values,
};

pub struct WavePlotterGui {
    pub audiodata: Arc<Mutex<Audiodata>>,
    pub window_size: usize,
}

impl WavePlotterGui {
    pub fn new(window_size: usize, chunk_size: usize) -> Self {
        Self {
            audiodata: Arc::new(Mutex::new(Audiodata::new(window_size, chunk_size))),
            window_size,
        }
    }
}

impl Default for WavePlotterGui {
    fn default() -> Self {
        Self {
            audiodata: Arc::new(Mutex::new(Audiodata::new(192000, 512))),
            window_size: 192000,
        }
    }
}

impl eframe::App for WavePlotterGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("WavePlotterGui");
            let plot = Plot::new("audio-signal")
                .include_y(-1.0)
                .include_y(1.0)
                .include_x(0.0)
                .include_x(self.window_size as f64);
            plot.show(ui, |plot_ui| {
                for (x, (min, max)) in self
                    .audiodata
                    .lock()
                    .unwrap()
                    .get_values()
                    .into_iter()
                    .enumerate()
                {
                    let p1 = Value::new(x as f64, min);
                    let p2 = Value::new(x as f64, max);
                    plot_ui.line(Line::new(Values::from_values(vec![p1, p2])));
                }
            });
        });
    }
}
