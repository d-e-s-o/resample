use std::error::Error as StdError;
use std::ffi::CStr;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use libsamplerate_rs::src_strerror;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ErrorCode {
    Unknown = -1,
    NoError = 0,
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

impl ErrorCode {
    /// Create a new `ResampleType` enum from the corresponding integer.
    pub fn from_int(value: i32) -> Self {
        match value {
            0 => Self::NoError,
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
        }
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Error {
    code: ErrorCode,
}

impl Error {
    pub fn from_int(code: i32) -> Self {
        Self {
            code: ErrorCode::from_int(code),
        }
    }

    pub fn from_code(code: ErrorCode) -> Self {
        Self { code }
    }

    pub fn code(&self) -> ErrorCode {
        self.code
    }

    pub fn description(&self) -> &'static str {
        self.code.description()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        self.code.description()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_converter_type_from_int() {
        assert_eq!(ErrorCode::from_int(0), ErrorCode::NoError);
        assert_eq!(ErrorCode::from_int(1), ErrorCode::MallocFailed);
        assert_eq!(ErrorCode::from_int(2), ErrorCode::BadState);
        assert_eq!(ErrorCode::from_int(3), ErrorCode::BadData);
        assert_eq!(ErrorCode::from_int(4), ErrorCode::BadDataPtr);
        assert_eq!(ErrorCode::from_int(5), ErrorCode::NoPrivate);
        assert_eq!(ErrorCode::from_int(6), ErrorCode::BadSrcRatio);
        assert_eq!(ErrorCode::from_int(7), ErrorCode::BadProcPtr);
        assert_eq!(ErrorCode::from_int(8), ErrorCode::ShiftBits);
        assert_eq!(ErrorCode::from_int(9), ErrorCode::FilterLen);
        assert_eq!(ErrorCode::from_int(10), ErrorCode::BadConverter);
        assert_eq!(ErrorCode::from_int(11), ErrorCode::BadChannelCount);
        assert_eq!(ErrorCode::from_int(12), ErrorCode::SincBadBufferLen);
        assert_eq!(ErrorCode::from_int(13), ErrorCode::SizeIncompatibility);
        assert_eq!(ErrorCode::from_int(14), ErrorCode::BadPrivPtr);
        assert_eq!(ErrorCode::from_int(15), ErrorCode::BadSincState);
        assert_eq!(ErrorCode::from_int(16), ErrorCode::DataOverlap);
        assert_eq!(ErrorCode::from_int(17), ErrorCode::BadCallback);
        assert_eq!(ErrorCode::from_int(18), ErrorCode::BadMode);
        assert_eq!(ErrorCode::from_int(19), ErrorCode::NullCallback);
        assert_eq!(ErrorCode::from_int(20), ErrorCode::NoVariableRatio);
        assert_eq!(ErrorCode::from_int(21), ErrorCode::SincPrepareDataBadLen);
        assert_eq!(ErrorCode::from_int(22), ErrorCode::BadInternalState);
        assert_eq!(ErrorCode::from_int(23), ErrorCode::MaxError);
        assert_eq!(ErrorCode::from_int(24), ErrorCode::Unknown);
    }

    #[test]
    fn description() {
        assert_eq!(ErrorCode::NoError.description(), "No error.");
        assert_eq!(ErrorCode::MallocFailed.description(), "Malloc failed.");
        assert_eq!(
            ErrorCode::BadState.description(),
            "SRC_STATE pointer is NULL."
        );
        assert_eq!(
            ErrorCode::BadData.description(),
            "SRC_DATA pointer is NULL."
        );
        assert_eq!(
            ErrorCode::BadDataPtr.description(),
            "SRC_DATA->data_out or SRC_DATA->data_in is NULL."
        );
        assert_eq!(
            ErrorCode::NoPrivate.description(),
            "Internal error. No private data."
        );
        assert_eq!(
            ErrorCode::BadSrcRatio.description(),
            "SRC ratio outside [1/256, 256] range."
        );
        assert_eq!(
            ErrorCode::BadSincState.description(),
            "src_process() called without reset after end_of_input."
        );
        assert_eq!(
            ErrorCode::BadProcPtr.description(),
            "Internal error. No process pointer."
        );
        assert_eq!(
            ErrorCode::ShiftBits.description(),
            "Internal error. SHIFT_BITS too large."
        );
        assert_eq!(
            ErrorCode::FilterLen.description(),
            "Internal error. Filter length too large."
        );
        assert_eq!(
            ErrorCode::BadConverter.description(),
            "Bad converter number."
        );
        assert_eq!(
            ErrorCode::BadChannelCount.description(),
            "Channel count must be >= 1."
        );
        assert_eq!(
            ErrorCode::SincBadBufferLen.description(),
            "Internal error. Bad buffer length. Please report this."
        );
        assert_eq!(
            ErrorCode::SizeIncompatibility.description(),
            "Internal error. Input data / internal buffer size difference. Please report this."
        );
        assert_eq!(
            ErrorCode::BadPrivPtr.description(),
            "Internal error. Private pointer is NULL. Please report this."
        );
        assert_eq!(
            ErrorCode::DataOverlap.description(),
            "Input and output data arrays overlap."
        );
        assert_eq!(
            ErrorCode::BadCallback.description(),
            "Supplied callback function pointer is NULL."
        );
        assert_eq!(
            ErrorCode::BadMode.description(),
            "Calling mode differs from initialisation mode (ie process v callback)."
        );
        assert_eq!(
            ErrorCode::NullCallback.description(),
            "Callback function pointer is NULL in src_callback_read ()."
        );
        assert_eq!(
            ErrorCode::NoVariableRatio.description(),
            "This converter only allows constant conversion ratios."
        );
        assert_eq!(
            ErrorCode::SincPrepareDataBadLen.description(),
            "Internal error : Bad length in prepare_data ()."
        );
        assert_eq!(
            ErrorCode::BadInternalState.description(),
            "Error : Someone is trampling on my internal state."
        );
        assert_eq!(
            ErrorCode::MaxError.description(),
            "Placeholder. No error defined for this error number."
        );
        assert_eq!(ErrorCode::Unknown.description(), "Unkown error.");
    }

    #[test]
    fn error_from_code_and_int() {
        assert_eq!(Error::from_int(2), Error::from_code(ErrorCode::BadState));
    }

    #[test]
    fn error_description() {
        for i in -1..24 {
            assert_eq!(
                Error::from_int(i).description(),
                ErrorCode::from_int(i).description()
            );
        }
    }
}
