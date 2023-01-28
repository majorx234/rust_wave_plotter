use std::thread;
pub mod wave_plotter_gui;
use crate::wave_plotter_gui::WavePlotterGui;
pub mod jackprocess;
use crate::jackprocess::start_jack_thread;

fn main() {
    let mut wave_plotter_gui_app = WavePlotterGui::default();
    start_jack_thread(wave_plotter_gui_app.audiodata.clone());
    let mut options = eframe::NativeOptions::default();

    eframe::run_native(
        "WavePlotterGui",
        options,
        Box::new(|_cc| Box::new(wave_plotter_gui_app)),
    );
}
