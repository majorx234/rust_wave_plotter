use audio_lib::audiodata::Audiodata;
use jack;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::sync::Mutex;
use std::{thread, time::Duration};

pub fn start_jack_thread(audiodata: Arc<Mutex<Audiodata>>) -> std::thread::JoinHandle<()> {
    let chunk_size = audiodata.lock().unwrap().get_chunk_size();
    std::thread::spawn(move || {
        let mut run: bool = true;
        let (client, _status) =
            jack::Client::new("wave_plotter_gui", jack::ClientOptions::NO_START_SERVER)
                .expect("No Jack server running\n");

        let sample_rate = client.sample_rate();
        // register ports
        let in_a = client
            .register_port("wave_plotter_gui_l", jack::AudioIn::default())
            .unwrap();
        let in_b = client
            .register_port("wave_plotter_gui_r", jack::AudioIn::default())
            .unwrap();
        let frame_size = client.buffer_size() as usize;
        let process_callback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            let in_a_p = in_a.as_slice(ps);
            let in_b_p = in_b.as_slice(ps);

            // ToDo: use ringbuffer instead of VecDeque with Mutex
            for some_samples in in_a_p.chunks(chunk_size) {
                let (mut max, mut min) = (some_samples[0], some_samples[0]);
                for sample in some_samples {
                    if *sample < min {
                        min = *sample;
                    }
                    if *sample > max {
                        max = *sample;
                    }
                    audiodata.lock().unwrap().append((min, max));
                }
            }

            // in_b_p.into_iter().map(|x| audiodata.lock().append(*x));
            jack::Control::Continue
        };
        let process = jack::ClosureProcessHandler::new(process_callback);
        let active_client = client.activate_async((), process).unwrap();

        while run {
            thread::sleep(Duration::from_millis(100));
            /*
            match rx_close.recv() {
                Ok(running) => run = running,
                Err(_) => run = false,
            }*/
        }
        match active_client.deactivate() {
            Ok(_) => println!("exit audio thread\n"),
            Err(_) => println!("exit audio thread,client deactivation err\n"),
        }
    })
}
