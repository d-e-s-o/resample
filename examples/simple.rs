use std::f32::consts::PI;

use hound::SampleFormat;
use hound::WavSpec;
use hound::WavWriter;

use resample::convert;
use resample::ResampleType;


fn main() {
    // Generate a 880Hz sine wave for 1 second in 44100Hz with one channel.
    let freq = PI * 880f32 / 44100f32;
    let input: Vec<f32> = (0..44100 * 5).map(|i| (freq * i as f32).sin()).collect();

    // Resample the input from 44100Hz to 48000Hz.
    let type_ = ResampleType::SincBestQuality;
    let resampled = convert(type_, 1, 44100, 48000, &input).unwrap();

    // Write the resampled pcm data to disk.
    let mut writer = WavWriter::create(
        "resampled.wav",
        WavSpec {
            channels: 1,
            sample_rate: 48000,
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
        },
    )
    .unwrap();
    resampled
        .iter()
        .for_each(|i| writer.write_sample(*i).unwrap());
}
