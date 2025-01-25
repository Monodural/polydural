use rodio::{OutputStream, Sink};
use rodio::buffer::SamplesBuffer;
use tokio::task;

pub async fn play_audio(samples: Vec<i16>) {
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to get default output stream");
    let sink = Sink::try_new(&stream_handle).expect("Failed to create audio sink");

    let samples_f32: Vec<f32> = samples.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
    let source = SamplesBuffer::new(1, 44100, samples_f32);

    sink.append(source);
    sink.sleep_until_end();
}