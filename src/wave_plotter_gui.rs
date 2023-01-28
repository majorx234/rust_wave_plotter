use audio_lib::audiodata::Audiodata;
use eframe::egui;
use std::sync::Arc;
use std::sync::Mutex;

use egui::plot::{
    Arrows, Bar, BarChart, CoordinatesFormatter, Corner, GridInput, GridMark, HLine, Legend, Line,
    LineStyle, MarkerShape, Plot, PlotImage, Points, Polygon, Text, VLine, Values,
};

pub struct WavePlotterGui {
    pub audiodata: Arc<Mutex<Audiodata>>,
}

impl Default for WavePlotterGui {
    fn default() -> Self {
        Self {
            audiodata: Arc::new(Mutex::new(Audiodata::new(192000))),
        }
    }
}

impl eframe::App for WavePlotterGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("WavePlotterGui");
            let plot = Plot::new("audio-signal");
            plot.show(ui, |plot_ui| {
                plot_ui.points(Points::new(Values::from_ys_f32(
                    &self.audiodata.lock().unwrap().get_values(),
                )));
            });
        });
    }
}
