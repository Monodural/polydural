use rodio::{OutputStream, Sink};
use std::sync::Arc;
use rodio::buffer::SamplesBuffer;
use std::sync::mpsc;
use std::thread;

pub fn start_audio_thread(audio_files: Vec<Vec<i16>>) -> mpsc::Sender<(usize, f32)> {
    let (tx, rx): (mpsc::Sender<(usize, f32)>, mpsc::Receiver<(usize, f32)>) = mpsc::channel();
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to initialize audio output");
        let stream_handle = Arc::new(stream_handle);

        let mut active_sinks = Vec::new();

        while let Ok(index) = rx.recv() {
            if index.0 >= audio_files.len() {
                eprintln!("Invalid audio index: {}", index.0);
                continue;
            }

            let samples: &Vec<i16> = &audio_files[index.0];
            let samples_f32: Vec<f32> = samples.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
            let source = SamplesBuffer::new(1, 44100, samples_f32);

            let sink = Sink::try_new(&stream_handle).expect("Failed to create audio sink");
            sink.set_volume(index.1);
            sink.append(source);

            active_sinks.push(sink);
            active_sinks.retain(|s| !s.empty());
        }
    });

    tx
}