//! A library for sample rate conversion of audio.

pub mod converter_type;
pub mod error;
pub mod samplerate;

#[cfg(test)]
mod sanity_test;

pub use libsamplerate_rs;

use libsamplerate_rs::src_simple;
use libsamplerate_rs::SRC_DATA;

pub use crate::converter_type::ConverterType;
pub use crate::error::Error;
pub use crate::error::ErrorCode;
pub use crate::samplerate::Samplerate;

/// Perform a simple samplerate conversion of a large chunk of audio.
/// This calls `src_simple` of libsamplerate which is not suitable for streamed audio. Use the
/// `Samplerate` struct instead for this.
///
/// The length of `input` must be `input_frame_count * channels`.
/// The length of result `Vec<f32>` should be `input_frames * to_rate + (from_rate - 1)) / from_rate`
///
/// # Example
///
/// ```
/// use resample::{convert, ConverterType};
///
/// // Generate a 880Hz sine wave for 1 second in 44100Hz with one channel.
/// let freq = std::f32::consts::PI * 880f32 / 44100f32;
/// let mut input: Vec<f32> = (0..44100).map(|i| (freq * i as f32).sin()).collect();
///
/// // Resample the input from 44100Hz to 48000Hz.
/// let resampled = convert(44100, 48000, 1, ConverterType::SincBestQuality, &input).unwrap();
/// assert_eq!(resampled.len(), 48000);
/// ```
pub fn convert(
    from_rate: u32,
    to_rate: u32,
    channels: usize,
    converter_type: ConverterType,
    input: &[f32],
) -> Result<Vec<f32>, Error> {
    let input_len = input.len();
    assert_eq!(input_len % channels, 0);
    let input_frames = input_len / channels;
    let ratio = to_rate as f64 / from_rate as f64;
    let output_frames = (input_frames * to_rate as usize).div_ceil(from_rate as usize);
    let mut output = vec![0f32; output_frames * channels];
    let mut src = SRC_DATA {
        data_in: input.as_ptr(),
        data_out: output.as_mut_ptr(),
        input_frames: input_frames.try_into().unwrap(),
        output_frames: output_frames.try_into().unwrap(),
        src_ratio: ratio,
        end_of_input: 0,
        input_frames_used: 0,
        output_frames_gen: 0,
    };
    let error_int = unsafe {
        src_simple(
            &mut src as *mut SRC_DATA,
            converter_type as i32,
            channels as i32,
        )
    };
    let error_code = ErrorCode::from_int(error_int);
    match error_code {
        ErrorCode::NoError => Ok(output),
        _ => Err(Error::from_code(error_code)),
    }
}
