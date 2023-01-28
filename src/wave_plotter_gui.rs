use audio_lib::audiodata::Audiodata;
use eframe::egui;

use egui::plot::{
    Arrows, Bar, BarChart, CoordinatesFormatter, Corner, GridInput, GridMark, HLine, Legend, Line,
    LineStyle, MarkerShape, Plot, PlotImage, Points, Polygon, Text, VLine,
};

pub struct WavePlotterGui {
    audiodata: Audiodata,
}

impl Default for WavePlotterGui {
    fn default() -> Self {
        Self {
            audiodata: Audiodata::new(192000),
        }
    }
}

impl eframe::App for WavePlotterGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("WavePlotterGui");
        });
    }
}
