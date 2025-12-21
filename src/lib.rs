//! A library for sample rate conversion of audio.

#![cfg_attr(feature = "nightly", feature(test))]

#[cfg(all(test, feature = "nightly"))]
extern crate test;

pub mod converter_type;
pub mod error;
pub mod samplerate;

#[cfg(test)]
mod sanity_test;

pub use libsamplerate_rs;

pub use crate::converter_type::ConverterType;
pub use crate::error::Error;
pub use crate::error::ErrorCode;
pub use crate::samplerate::Processed;
pub use crate::samplerate::Samplerate;


/// Perform a simple samplerate conversion of a large chunk of audio.
///
/// This function is not suitable for streamed audio. Use the
/// [`Samplerate`] type in such a context.
///
/// The length of `input` must be `input_frames * channels`.
///
/// # Example
///
/// ```
/// # use std::f32::consts::PI;
/// use resample::{convert, ConverterType};
///
/// // Generate a 880Hz sine wave for 1 second in 44100Hz with one channel.
/// let freq = PI * 880_f32 / 44100_f32;
/// let mut input = (0..44100).map(|i| (freq * i as f32).sin()).collect::<Vec<f32>>();
///
/// // Resample the input from 44100Hz to 48000Hz.
/// let converter = ConverterType::SincBestQuality;
/// let resampled = convert(converter, 1, 44100, 48000, &input).unwrap();
/// assert_eq!(resampled.len(), 48000);
/// ```
pub fn convert(
    converter_type: ConverterType,
    channels: u8,
    from_rate: u32,
    to_rate: u32,
    input: &[f32],
) -> Result<Vec<f32>, Error> {
    let input_len = input.len();
    let input_frames = input_len / usize::from(channels);
    let output_frames = (input_frames * to_rate as usize).div_ceil(from_rate as usize);
    let mut output = vec![0.0; output_frames * usize::from(channels)];
    let mut converter = Samplerate::new(converter_type, channels, from_rate, to_rate)?;

    let mut total = Processed::default();
    loop {
        let in_buf = &input[total.read..];
        let out_buf = &mut output[total.written..];

        let processed = converter.finalize(in_buf, out_buf)?;

        total.read += processed.read;
        total.written += processed.written;

        if total.read >= input.len() {
            break
        }

        // We haven't quite managed to process everything, due to output
        // buffer size constraints. Allocate a few more bytes and
        // continue.
        let () = output.resize(output.len() + 64 * usize::from(channels), 0.0);
    }

    debug_assert_eq!(total.read, input.len());

    let () = output.resize(total.written, 0.0);
    Ok(output)
}


#[cfg(test)]
#[cfg(feature = "nightly")]
mod tests {
    use super::*;

    use test::Bencher;


    fn bench_convert_impl(b: &mut Bencher, converter: ConverterType) {
        use std::f32::consts::PI;

        let freq = PI * 880f32 / 44100f32;
        let input = (0..44100)
            .map(|i| (freq * i as f32).sin())
            .collect::<Vec<_>>();

        let () = b.iter(|| {
            let resampled = convert(converter, 1, 44100, 48000, &input).unwrap();
            assert!((48000..=48001).contains(&resampled.len()));
        });
    }

    /// Benchmark sample rate conversion with the `SincBestQuality`
    /// type.
    #[bench]
    fn bench_convert_sinc_best(b: &mut Bencher) {
        let converter = ConverterType::SincBestQuality;
        let () = bench_convert_impl(b, converter);
    }

    /// Benchmark sample rate conversion with the `SincMediumQuality`
    /// type.
    #[bench]
    fn bench_convert_sinc_medium(b: &mut Bencher) {
        let converter = ConverterType::SincMediumQuality;
        let () = bench_convert_impl(b, converter);
    }

    /// Benchmark sample rate conversion with the `SincFastest` type.
    #[bench]
    fn bench_convert_sinc_fast(b: &mut Bencher) {
        let converter = ConverterType::SincFastest;
        let () = bench_convert_impl(b, converter);
    }

    /// Benchmark sample rate conversion with the `ZeroOrderHold` type.
    #[bench]
    fn bench_convert_zero_order_hold(b: &mut Bencher) {
        let converter = ConverterType::ZeroOrderHold;
        let () = bench_convert_impl(b, converter);
    }

    /// Benchmark sample rate conversion with the `Linear` type.
    #[bench]
    fn bench_convert_linear(b: &mut Bencher) {
        let converter = ConverterType::Linear;
        let () = bench_convert_impl(b, converter);
    }
}
