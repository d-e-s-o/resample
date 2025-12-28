use std::error::Error as StdError;
use std::ffi::CStr;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use libsamplerate_rs::src_strerror;


/// A type specifying a general category of sample rate conversion error.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ErrorKind {
    Unknown = -1,
    MallocFailed = 1,
    BadState = 2,
    BadData = 3,
    BadDataPtr = 4,
    NoPrivate = 5,
    BadSrcRatio = 6,
    BadProcPtr = 7,
    ShiftBits = 8,
    FilterLen = 9,
    BadConverter = 10,
    BadChannelCount = 11,
    SincBadBufferLen = 12,
    SizeIncompatibility = 13,
    BadPrivPtr = 14,
    BadSincState = 15,
    DataOverlap = 16,
    BadCallback = 17,
    BadMode = 18,
    NullCallback = 19,
    NoVariableRatio = 20,
    SincPrepareDataBadLen = 21,
    BadInternalState = 22,
    MaxError = 23,
}

impl ErrorKind {
    /// Create a new [`ErrorKind`] enum from the corresponding integer.
    pub(crate) fn from_int(value: i32) -> Option<Self> {
        let slf = match value {
            0 => return None,
            1 => Self::MallocFailed,
            2 => Self::BadState,
            3 => Self::BadData,
            4 => Self::BadDataPtr,
            5 => Self::NoPrivate,
            6 => Self::BadSrcRatio,
            7 => Self::BadProcPtr,
            8 => Self::ShiftBits,
            9 => Self::FilterLen,
            10 => Self::BadConverter,
            11 => Self::BadChannelCount,
            12 => Self::SincBadBufferLen,
            13 => Self::SizeIncompatibility,
            14 => Self::BadPrivPtr,
            15 => Self::BadSincState,
            16 => Self::DataOverlap,
            17 => Self::BadCallback,
            18 => Self::BadMode,
            19 => Self::NullCallback,
            20 => Self::NoVariableRatio,
            21 => Self::SincPrepareDataBadLen,
            22 => Self::BadInternalState,
            23 => Self::MaxError,
            _ => Self::Unknown,
        };
        Some(slf)
    }

    /// Return the human-readable description for this error.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Unknown => "Unkown error.",
            _ => {
                // SAFETY: `src_strerror` is always safe to call.
                let ptr = unsafe { src_strerror(*self as i32) };
                // SANITY: `src_strerror` always returns a valid pointer.
                assert!(!ptr.is_null());

                // SAFETY: `ptr` is not NULL and guaranteed to be valid.
                unsafe { CStr::from_ptr(ptr) }.to_str().unwrap()
            },
        }
    }
}


/// The error type used by the crate.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub(crate) fn check_int(error: i32) -> Result<(), Self> {
        match ErrorKind::from_int(error) {
            None => Ok(()),
            Some(kind) => Err(Self::from(kind)),
        }
    }

    #[inline]
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    #[inline]
    pub fn description(&self) -> &'static str {
        self.kind.description()
    }
}

impl From<ErrorKind> for Error {
    #[inline]
    fn from(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}

impl StdError for Error {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_converter_type_from_int() {
        assert_eq!(ErrorKind::from_int(0), None);
        assert_eq!(ErrorKind::from_int(1), Some(ErrorKind::MallocFailed));
        assert_eq!(ErrorKind::from_int(2), Some(ErrorKind::BadState));
        assert_eq!(ErrorKind::from_int(3), Some(ErrorKind::BadData));
        assert_eq!(ErrorKind::from_int(4), Some(ErrorKind::BadDataPtr));
        assert_eq!(ErrorKind::from_int(5), Some(ErrorKind::NoPrivate));
        assert_eq!(ErrorKind::from_int(6), Some(ErrorKind::BadSrcRatio));
        assert_eq!(ErrorKind::from_int(7), Some(ErrorKind::BadProcPtr));
        assert_eq!(ErrorKind::from_int(8), Some(ErrorKind::ShiftBits));
        assert_eq!(ErrorKind::from_int(9), Some(ErrorKind::FilterLen));
        assert_eq!(ErrorKind::from_int(10), Some(ErrorKind::BadConverter));
        assert_eq!(ErrorKind::from_int(11), Some(ErrorKind::BadChannelCount));
        assert_eq!(ErrorKind::from_int(12), Some(ErrorKind::SincBadBufferLen));
        assert_eq!(
            ErrorKind::from_int(13),
            Some(ErrorKind::SizeIncompatibility)
        );
        assert_eq!(ErrorKind::from_int(14), Some(ErrorKind::BadPrivPtr));
        assert_eq!(ErrorKind::from_int(15), Some(ErrorKind::BadSincState));
        assert_eq!(ErrorKind::from_int(16), Some(ErrorKind::DataOverlap));
        assert_eq!(ErrorKind::from_int(17), Some(ErrorKind::BadCallback));
        assert_eq!(ErrorKind::from_int(18), Some(ErrorKind::BadMode));
        assert_eq!(ErrorKind::from_int(19), Some(ErrorKind::NullCallback));
        assert_eq!(ErrorKind::from_int(20), Some(ErrorKind::NoVariableRatio));
        assert_eq!(
            ErrorKind::from_int(21),
            Some(ErrorKind::SincPrepareDataBadLen)
        );
        assert_eq!(ErrorKind::from_int(22), Some(ErrorKind::BadInternalState));
        assert_eq!(ErrorKind::from_int(23), Some(ErrorKind::MaxError));
        assert_eq!(ErrorKind::from_int(24), Some(ErrorKind::Unknown));
    }

    #[test]
    fn description() {
        assert_eq!(ErrorKind::MallocFailed.description(), "Malloc failed.");
        assert_eq!(
            ErrorKind::BadState.description(),
            "SRC_STATE pointer is NULL."
        );
        assert_eq!(
            ErrorKind::BadData.description(),
            "SRC_DATA pointer is NULL."
        );
        assert_eq!(
            ErrorKind::BadDataPtr.description(),
            "SRC_DATA->data_out or SRC_DATA->data_in is NULL."
        );
        assert_eq!(
            ErrorKind::NoPrivate.description(),
            "Internal error. No private data."
        );
        assert_eq!(
            ErrorKind::BadSrcRatio.description(),
            "SRC ratio outside [1/256, 256] range."
        );
        assert_eq!(
            ErrorKind::BadSincState.description(),
            "src_process() called without reset after end_of_input."
        );
        assert_eq!(
            ErrorKind::BadProcPtr.description(),
            "Internal error. No process pointer."
        );
        assert_eq!(
            ErrorKind::ShiftBits.description(),
            "Internal error. SHIFT_BITS too large."
        );
        assert_eq!(
            ErrorKind::FilterLen.description(),
            "Internal error. Filter length too large."
        );
        assert_eq!(
            ErrorKind::BadConverter.description(),
            "Bad converter number."
        );
        assert_eq!(
            ErrorKind::BadChannelCount.description(),
            "Channel count must be >= 1."
        );
        assert_eq!(
            ErrorKind::SincBadBufferLen.description(),
            "Internal error. Bad buffer length. Please report this."
        );
        assert_eq!(
            ErrorKind::SizeIncompatibility.description(),
            "Internal error. Input data / internal buffer size difference. Please report this."
        );
        assert_eq!(
            ErrorKind::BadPrivPtr.description(),
            "Internal error. Private pointer is NULL. Please report this."
        );
        assert_eq!(
            ErrorKind::DataOverlap.description(),
            "Input and output data arrays overlap."
        );
        assert_eq!(
            ErrorKind::BadCallback.description(),
            "Supplied callback function pointer is NULL."
        );
        assert_eq!(
            ErrorKind::BadMode.description(),
            "Calling mode differs from initialisation mode (ie process v callback)."
        );
        assert_eq!(
            ErrorKind::NullCallback.description(),
            "Callback function pointer is NULL in src_callback_read ()."
        );
        assert_eq!(
            ErrorKind::NoVariableRatio.description(),
            "This converter only allows constant conversion ratios."
        );
        assert_eq!(
            ErrorKind::SincPrepareDataBadLen.description(),
            "Internal error : Bad length in prepare_data ()."
        );
        assert_eq!(
            ErrorKind::BadInternalState.description(),
            "Error : Someone is trampling on my internal state."
        );
        assert_eq!(
            ErrorKind::MaxError.description(),
            "Placeholder. No error defined for this error number."
        );
        assert_eq!(ErrorKind::Unknown.description(), "Unkown error.");
    }
}
