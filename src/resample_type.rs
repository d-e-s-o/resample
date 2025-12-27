use std::ffi::CStr;

use libsamplerate_rs::src_get_description;
use libsamplerate_rs::src_get_name;
use libsamplerate_rs::SRC_LINEAR;
use libsamplerate_rs::SRC_SINC_BEST_QUALITY;
use libsamplerate_rs::SRC_SINC_FASTEST;
use libsamplerate_rs::SRC_SINC_MEDIUM_QUALITY;
use libsamplerate_rs::SRC_ZERO_ORDER_HOLD;


/// The resampler type used to distinguish the interpolation function
/// used by `libsamplerate`.
///
/// Has a great impact on quality and performance.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ResampleType {
    SincBestQuality = SRC_SINC_BEST_QUALITY as isize,
    SincMediumQuality = SRC_SINC_MEDIUM_QUALITY as isize,
    SincFastest = SRC_SINC_FASTEST as isize,
    ZeroOrderHold = SRC_ZERO_ORDER_HOLD as isize,
    Linear = SRC_LINEAR as isize,
}

impl ResampleType {
    /// Return a human-readable name for this type of resampler.
    pub fn name(&self) -> &'static str {
        // SAFETY: `src_get_name` is always safe to call.
        let ptr = unsafe { src_get_name(*self as i32) };
        // SANITY: `src_get_name` always returns a valid pointer for a
        //         valid convert and we only have known convert types.
        assert!(!ptr.is_null());

        // SAFETY: `ptr` is not NULL and guaranteed to be valid.
        unsafe { CStr::from_ptr(ptr) }.to_str().unwrap()
    }

    /// Return the human-readable description for this type of resampler.
    pub fn description(&self) -> &'static str {
        // SAFETY: `src_get_description` is always safe to call.
        let ptr = unsafe { src_get_description(*self as i32) };
        // SANITY: `src_get_description` always returns a valid pointer
        //         for a valid convert and we only have known convert
        //         types.
        assert!(!ptr.is_null());

        // SAFETY: `ptr` is not NULL and guaranteed to be valid.
        unsafe { CStr::from_ptr(ptr) }.to_str().unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn name() {
        assert_eq!(
            ResampleType::SincBestQuality.name(),
            "Best Sinc Interpolator"
        );
        assert_eq!(
            ResampleType::SincMediumQuality.name(),
            "Medium Sinc Interpolator"
        );
        assert_eq!(
            ResampleType::SincFastest.name(),
            "Fastest Sinc Interpolator"
        );
        assert_eq!(ResampleType::ZeroOrderHold.name(), "ZOH Interpolator");
        assert_eq!(ResampleType::Linear.name(), "Linear Interpolator");
    }

    #[test]
    fn description() {
        assert_eq!(
            ResampleType::SincBestQuality.description(),
            "Band limited sinc interpolation, best quality, 144dB SNR, 96% BW."
        );
        assert_eq!(
            ResampleType::SincMediumQuality.description(),
            "Band limited sinc interpolation, medium quality, 121dB SNR, 90% BW."
        );
        assert_eq!(
            ResampleType::SincFastest.description(),
            "Band limited sinc interpolation, fastest, 97dB SNR, 80% BW."
        );
        assert_eq!(
            ResampleType::ZeroOrderHold.description(),
            "Zero order hold interpolator, very fast, poor quality."
        );
        assert_eq!(
            ResampleType::Linear.description(),
            "Linear interpolator, very fast, poor quality."
        );
    }
}
