use hound::WavWriter;
use std::f32::consts::PI;

const SAMPLE_RATE: u32 = 44100;
const TONE: f32 = 440.0; // 440Hz = A
const SAMPLE_REPETITION: f32 = 3.0; // 3 seconds

fn main() {
    // set format of WAV file
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    // create a WavWriter instance
    let mut fw = WavWriter::create("simple-sinewave.wav", spec).unwrap();

    // generate a sine wave
    let samples = SAMPLE_RATE as f32 * SAMPLE_REPETITION;
    for t in 0..samples as u32 {
        let x = (t as f32 / SAMPLE_RATE as f32) * TONE * 2.0 * PI;
        let y = x.sin();
        let sample = (y * i16::MAX as f32) as i16;
        fw.write_sample(sample).unwrap();
    }
}
