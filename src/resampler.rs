use libsamplerate_rs::src_delete;
use libsamplerate_rs::src_is_valid_ratio;
use libsamplerate_rs::src_new;
use libsamplerate_rs::src_process;
use libsamplerate_rs::SRC_DATA;
use libsamplerate_rs::SRC_STATE;

use crate::error::Error;
use crate::error::ErrorCode;
use crate::resample_type::ResampleType;


/// A type representing the result of a samplerate conversion.
#[derive(Debug, Default)]
pub struct Processed {
    /// The number of input samples read.
    pub read: usize,
    /// The number of output samples written.
    pub written: usize,
}


/// A samplerate converter.
///
/// This is a wrapper around `libsamplerate`'s `SRC_STATE`.
///
/// # Example
///
/// ```
/// # use std::f32::consts::PI;
/// use resample::{Resampler, ResampleType};
///
/// // Generate a 880Hz sine wave for 1 second in 44100Hz with one channel.
/// let freq = PI * 880_f32 / 44100_f32;
/// let mut input = (0..44100).map(|i| (freq * i as f32).sin()).collect::<Vec<f32>>();
/// let mut output = vec![0.0; 48000];
///
/// // Instantiate a new resampler.
/// let mut resampler = Resampler::new(ResampleType::SincBestQuality, 1, 44100, 48000).unwrap();
///
/// // Resample the input from 44100Hz to 48000Hz.
/// let processed = resampler.finalize(&input, &mut output).unwrap();
/// assert_eq!(processed.read, 44100);
/// assert_eq!(processed.written, 48000);
/// ```
#[derive(Debug)]
pub struct Resampler {
    state: *mut SRC_STATE,
    channels: u8,
    ratio: f64,
}

impl Resampler {
    /// Create a new samplerate converter assuming the given channel
    /// count and sample rates.
    pub fn new(
        converter_type: ResampleType,
        channels: u8,
        from_rate: u32,
        to_rate: u32,
    ) -> Result<Self, Error> {
        // Make sure that the provided ratio is supported by `libsamplerate`.
        let ratio = to_rate as f64 / from_rate as f64;
        // SAFETY: `src_is_valid_ratio` is always safe to call.
        if unsafe { src_is_valid_ratio(ratio) } == 0 {
            return Err(Error::from_code(ErrorCode::BadSrcRatio));
        }
        // Construct the `SRC_STATE` struct and check if that worked.
        let mut error = 0i32;
        // SAFETY: `error` is a valid pointer coming from a reference.
        let state = unsafe { src_new(converter_type as i32, i32::from(channels), &raw mut error) };
        let error = ErrorCode::from_int(error);

        match error {
            ErrorCode::NoError => {
                let slf = Self {
                    state,
                    ratio,
                    channels,
                };
                Ok(slf)
            },
            error => Err(Error::from_code(error)),
        }
    }

    fn process_impl(
        &mut self,
        input: &[f32],
        output: &mut [f32],
        end_of_input: bool,
    ) -> Result<Processed, Error> {
        let channels = usize::from(self.channels);
        debug_assert_eq!(input.len() % channels, 0);

        let mut src = SRC_DATA {
            data_in: input.as_ptr(),
            data_out: output.as_mut_ptr(),
            input_frames: (input.len() / channels).try_into().unwrap(),
            output_frames: (output.len() / channels).try_into().unwrap(),
            src_ratio: self.ratio,
            end_of_input: if end_of_input { 1 } else { 0 },
            input_frames_used: 0,
            output_frames_gen: 0,
        };

        // SAFETY: `state` is valid and guaranteed to be coming from a
        //          previous `src_new` call and `src` is a pointer
        //          originating from a reference.
        let error = unsafe { src_process(self.state, &raw mut src) };
        match ErrorCode::from_int(error) {
            ErrorCode::NoError => {
                let processed = Processed {
                    read: usize::try_from(src.input_frames_used).unwrap() * channels,
                    written: usize::try_from(src.output_frames_gen).unwrap() * channels,
                };
                Ok(processed)
            },
            _ => Err(Error::from_int(error)),
        }
    }

    /// Perform a samplerate conversion on a block of data.
    ///
    /// If the number of channels used was not `1` (Mono), the samples
    /// are expected to be stored interleaved.
    ///
    /// # Notes
    /// Even if all input samples are cleanly processed with this
    /// method, you will still need to [`finalize`][Self::finalize] the
    /// conversion.
    pub fn process(&mut self, input: &[f32], output: &mut [f32]) -> Result<Processed, Error> {
        self.process_impl(input, output, false)
    }

    /// Perform a samplerate conversion on last block of given input
    /// data (which may be empty).
    ///
    /// If the number of channels used was not `1` (Mono), the samples
    /// are expected to be stored interleaved.
    pub fn finalize(&mut self, input: &[f32], output: &mut [f32]) -> Result<Processed, Error> {
        let mut total = Processed::default();

        loop {
            let in_buf = &input[total.read..];
            let out_buf = &mut output[total.written..];

            let processed = self.process_impl(in_buf, out_buf, true)?;

            total.read += processed.read;
            total.written += processed.written;

            if processed.written == 0 {
                break Ok(total)
            }
        }
    }
}

impl Drop for Resampler {
    fn drop(&mut self) {
        // SAFETY: `state` is valid and guaranteed to be coming from a
        //          previous `src_new` call.
        unsafe { src_delete(self.state) };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::f32::consts::PI;


    #[test]
    fn samplerate_new_channels_correct() {
        let resampler = Resampler::new(ResampleType::Linear, 4, 44100, 48000).unwrap();
        assert_eq!(resampler.channels, 4);
    }

    #[test]
    fn samplerate_conversion() {
        // Generate a 880Hz sine wave for 1 second in 44100Hz with one channel.
        let freq = PI * 880f32 / 44100f32;
        let input = (0..44100)
            .map(|i| (freq * i as f32).sin())
            .collect::<Vec<f32>>();

        let mut resampler = Resampler::new(ResampleType::SincBestQuality, 1, 44100, 48000).unwrap();

        // Resample the audio in chunks.
        let mut resampled = vec![0f32; 0];
        let in_chunk_size = 4410; // 100ms
        let mut out_chunk_buf = vec![0.0; 4800];

        let mut in_chunks = input.chunks_exact(in_chunk_size);
        for in_chunk in in_chunks.by_ref() {
            let processed = resampler.process(in_chunk, &mut out_chunk_buf).unwrap();
            assert_eq!(processed.read, in_chunk.len());
            let () = resampled.extend(&out_chunk_buf[..processed.written]);
        }

        let rest = in_chunks.remainder();
        // NB: Even if `rest` is empty should finalization be performed.
        let processed = resampler.finalize(rest, &mut out_chunk_buf).unwrap();
        assert_eq!(processed.read, rest.len());
        let () = resampled.extend(&out_chunk_buf[..processed.written]);

        assert_eq!(resampled.len(), 48000);

        // Resample the audio back.
        let mut resampler = Resampler::new(ResampleType::SincBestQuality, 1, 48000, 44100).unwrap();
        let mut output = vec![0f32; 0];
        let in_chunk_size = 4800; // 100ms
        let mut out_chunk_buf = vec![0.0; 4410];

        let mut in_chunks = resampled.chunks_exact(in_chunk_size);
        for in_chunk in in_chunks.by_ref() {
            let processed = resampler.process(in_chunk, &mut out_chunk_buf).unwrap();
            assert_eq!(processed.read, in_chunk.len());
            let () = output.extend(&out_chunk_buf[..processed.written]);
        }

        let rest = in_chunks.remainder();
        let processed = resampler.finalize(rest, &mut out_chunk_buf).unwrap();
        assert_eq!(processed.read, rest.len());
        let () = output.extend(&out_chunk_buf[..processed.written]);

        assert_eq!(output.len(), 44100);

        // Expect the difference between all input frames and all output frames to be less than
        // an epsilon.
        let error = input
            .iter()
            .zip(output)
            .fold(0f32, |max, (input, output)| max.max((input - output).abs()));
        assert!(error < 0.002);
    }
}
