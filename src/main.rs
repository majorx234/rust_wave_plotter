pub mod wave_plotter_gui;
use crate::wave_plotter_gui::WavePlotterGui;
pub mod jackprocess;
use crate::jackprocess::start_jack_thread;
use clap::{Arg, Command, Parser};

/// Simple program to plot a wave
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// window size - how many samples to display
    #[arg(short, long, default_value_t = 192000)]
    pub window_size: usize,

    /// how many samples to display in one line (max 1024)
    #[arg(short, long, default_value_t = 512)]
    pub chunk_size: usize,
}

fn main() {
    let args = Args::parse();

    let mut wave_plotter_gui_app = WavePlotterGui::new(args.window_size, args.chunk_size);
    start_jack_thread(wave_plotter_gui_app.audiodata.clone());
    let mut options = eframe::NativeOptions::default();

    eframe::run_native(
        "WavePlotterGui",
        options,
        Box::new(|_cc| Box::new(wave_plotter_gui_app)),
    );
}
