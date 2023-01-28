pub mod wave_plotter_gui;
use crate::wave_plotter_gui::WavePlotterGui;

fn main() {
    let mut wave_plotter_gui_app = WavePlotterGui::default();
    let mut options = eframe::NativeOptions::default();

    eframe::run_native(
        "WavePlotterGui",
        options,
        Box::new(|_cc| Box::new(wave_plotter_gui_app)),
    );
}
