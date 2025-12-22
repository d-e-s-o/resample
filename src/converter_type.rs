use std::ffi::CStr;

use libsamplerate_rs::src_get_description;
use libsamplerate_rs::src_get_name;
use libsamplerate_rs::SRC_LINEAR;
use libsamplerate_rs::SRC_SINC_BEST_QUALITY;
use libsamplerate_rs::SRC_SINC_FASTEST;
use libsamplerate_rs::SRC_SINC_MEDIUM_QUALITY;
use libsamplerate_rs::SRC_ZERO_ORDER_HOLD;


/// A converter type used to distinguish the interpolation function used by libsamplerate.
/// Has a great impact on quality and performance.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ConverterType {
    SincBestQuality = SRC_SINC_BEST_QUALITY as isize,
    SincMediumQuality = SRC_SINC_MEDIUM_QUALITY as isize,
    SincFastest = SRC_SINC_FASTEST as isize,
    ZeroOrderHold = SRC_ZERO_ORDER_HOLD as isize,
    Linear = SRC_LINEAR as isize,
}

impl ConverterType {
    /// Return a human-readable name for this type of converter.
    pub fn name(&self) -> &'static str {
        unsafe { CStr::from_ptr(src_get_name(*self as i32)) }
            .to_str()
            .unwrap()
    }

    /// Return the human-readable description for this type of converter.
    pub fn description(&self) -> &'static str {
        unsafe { CStr::from_ptr(src_get_description(*self as i32)) }
            .to_str()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn name() {
        assert_eq!(
            ConverterType::SincBestQuality.name(),
            "Best Sinc Interpolator"
        );
        assert_eq!(
            ConverterType::SincMediumQuality.name(),
            "Medium Sinc Interpolator"
        );
        assert_eq!(
            ConverterType::SincFastest.name(),
            "Fastest Sinc Interpolator"
        );
        assert_eq!(ConverterType::ZeroOrderHold.name(), "ZOH Interpolator");
        assert_eq!(ConverterType::Linear.name(), "Linear Interpolator");
    }

    #[test]
    fn description() {
        assert_eq!(
            ConverterType::SincBestQuality.description(),
            "Band limited sinc interpolation, best quality, 144dB SNR, 96% BW."
        );
        assert_eq!(
            ConverterType::SincMediumQuality.description(),
            "Band limited sinc interpolation, medium quality, 121dB SNR, 90% BW."
        );
        assert_eq!(
            ConverterType::SincFastest.description(),
            "Band limited sinc interpolation, fastest, 97dB SNR, 80% BW."
        );
        assert_eq!(
            ConverterType::ZeroOrderHold.description(),
            "Zero order hold interpolator, very fast, poor quality."
        );
        assert_eq!(
            ConverterType::Linear.description(),
            "Linear interpolator, very fast, poor quality."
        );
    }
}
