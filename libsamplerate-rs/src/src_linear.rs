#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn fabs(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn lrint(__x: core::ffi::c_double) -> core::ffi::c_long;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SRC_STATE_tag {
    pub vt: *mut SRC_STATE_VT,
    pub last_ratio: core::ffi::c_double,
    pub last_position: core::ffi::c_double,
    pub error: SRC_ERROR,
    pub channels: core::ffi::c_int,
    pub mode: SRC_MODE,
    pub callback_func: src_callback_t,
    pub user_callback_data: *mut core::ffi::c_void,
    pub saved_frames: core::ffi::c_long,
    pub saved_data: *const core::ffi::c_float,
    pub private_data: *mut core::ffi::c_void,
}
pub type src_callback_t = Option<
    unsafe extern "C" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_float,
    ) -> core::ffi::c_long,
>;
pub type SRC_MODE = core::ffi::c_uint;
pub const SRC_MODE_CALLBACK: SRC_MODE = 1;
pub const SRC_MODE_PROCESS: SRC_MODE = 0;
pub type SRC_ERROR = core::ffi::c_uint;
pub const SRC_ERR_MAX_ERROR: SRC_ERROR = 23;
pub const SRC_ERR_BAD_INTERNAL_STATE: SRC_ERROR = 22;
pub const SRC_ERR_SINC_PREPARE_DATA_BAD_LEN: SRC_ERROR = 21;
pub const SRC_ERR_NO_VARIABLE_RATIO: SRC_ERROR = 20;
pub const SRC_ERR_NULL_CALLBACK: SRC_ERROR = 19;
pub const SRC_ERR_BAD_MODE: SRC_ERROR = 18;
pub const SRC_ERR_BAD_CALLBACK: SRC_ERROR = 17;
pub const SRC_ERR_DATA_OVERLAP: SRC_ERROR = 16;
pub const SRC_ERR_BAD_SINC_STATE: SRC_ERROR = 15;
pub const SRC_ERR_BAD_PRIV_PTR: SRC_ERROR = 14;
pub const SRC_ERR_SIZE_INCOMPATIBILITY: SRC_ERROR = 13;
pub const SRC_ERR_SINC_BAD_BUFFER_LEN: SRC_ERROR = 12;
pub const SRC_ERR_BAD_CHANNEL_COUNT: SRC_ERROR = 11;
pub const SRC_ERR_BAD_CONVERTER: SRC_ERROR = 10;
pub const SRC_ERR_FILTER_LEN: SRC_ERROR = 9;
pub const SRC_ERR_SHIFT_BITS: SRC_ERROR = 8;
pub const SRC_ERR_BAD_PROC_PTR: SRC_ERROR = 7;
pub const SRC_ERR_BAD_SRC_RATIO: SRC_ERROR = 6;
pub const SRC_ERR_NO_PRIVATE: SRC_ERROR = 5;
pub const SRC_ERR_BAD_DATA_PTR: SRC_ERROR = 4;
pub const SRC_ERR_BAD_DATA: SRC_ERROR = 3;
pub const SRC_ERR_BAD_STATE: SRC_ERROR = 2;
pub const SRC_ERR_MALLOC_FAILED: SRC_ERROR = 1;
pub const SRC_ERR_NO_ERROR: SRC_ERROR = 0;
pub type SRC_STATE_VT = SRC_STATE_VT_tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SRC_STATE_VT_tag {
    pub vari_process: Option<
        unsafe extern "C" fn(*mut SRC_STATE, *mut SRC_DATA) -> SRC_ERROR,
    >,
    pub const_process: Option<
        unsafe extern "C" fn(*mut SRC_STATE, *mut SRC_DATA) -> SRC_ERROR,
    >,
    pub reset: Option<unsafe extern "C" fn(*mut SRC_STATE) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut SRC_STATE) -> *mut SRC_STATE>,
    pub close: Option<unsafe extern "C" fn(*mut SRC_STATE) -> ()>,
}
pub type SRC_STATE = SRC_STATE_tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SRC_DATA {
    pub data_in: *const core::ffi::c_float,
    pub data_out: *mut core::ffi::c_float,
    pub input_frames: core::ffi::c_long,
    pub output_frames: core::ffi::c_long,
    pub input_frames_used: core::ffi::c_long,
    pub output_frames_gen: core::ffi::c_long,
    pub end_of_input: core::ffi::c_int,
    pub src_ratio: core::ffi::c_double,
}
pub type C2RustUnnamed = core::ffi::c_uint;
pub const SRC_LINEAR: C2RustUnnamed = 4;
pub const SRC_ZERO_ORDER_HOLD: C2RustUnnamed = 3;
pub const SRC_SINC_FASTEST: C2RustUnnamed = 2;
pub const SRC_SINC_MEDIUM_QUALITY: C2RustUnnamed = 1;
pub const SRC_SINC_BEST_QUALITY: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LINEAR_DATA {
    pub linear_magic_marker: core::ffi::c_int,
    pub dirty: bool,
    pub in_count: core::ffi::c_long,
    pub in_used: core::ffi::c_long,
    pub out_count: core::ffi::c_long,
    pub out_gen: core::ffi::c_long,
    pub last_value: *mut core::ffi::c_float,
}
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SRC_MAX_RATIO: core::ffi::c_int = 256 as core::ffi::c_int;
pub const SRC_MIN_RATIO_DIFF: core::ffi::c_double = 1e-20f64;
#[inline]
unsafe extern "C" fn fmod_one(mut x: core::ffi::c_double) -> core::ffi::c_double {
    let mut res: core::ffi::c_double = 0.;
    res = x - lrint(x) as core::ffi::c_double;
    if res < 0.0f64 {
        return res + 1.0f64;
    }
    return res;
}
#[inline]
unsafe extern "C" fn is_bad_src_ratio(
    mut ratio: core::ffi::c_double,
) -> core::ffi::c_int {
    return (ratio < 1.0f64 / SRC_MAX_RATIO as core::ffi::c_double
        || ratio > 1.0f64 * SRC_MAX_RATIO as core::ffi::c_double) as core::ffi::c_int;
}
pub const LINEAR_MAGIC_MARKER: core::ffi::c_int = 'l' as i32
    + (('i' as i32) << 4 as core::ffi::c_int) + (('n' as i32) << 8 as core::ffi::c_int)
    + (('e' as i32) << 12 as core::ffi::c_int) + (('a' as i32) << 16 as core::ffi::c_int)
    + (('r' as i32) << 20 as core::ffi::c_int);
static mut linear_state_vt: SRC_STATE_VT = unsafe {
    {
        let mut init = SRC_STATE_VT_tag {
            vari_process: Some(
                linear_vari_process
                    as unsafe extern "C" fn(*mut SRC_STATE, *mut SRC_DATA) -> SRC_ERROR,
            ),
            const_process: Some(
                linear_vari_process
                    as unsafe extern "C" fn(*mut SRC_STATE, *mut SRC_DATA) -> SRC_ERROR,
            ),
            reset: Some(linear_reset as unsafe extern "C" fn(*mut SRC_STATE) -> ()),
            copy: Some(
                linear_copy as unsafe extern "C" fn(*mut SRC_STATE) -> *mut SRC_STATE,
            ),
            close: Some(linear_close as unsafe extern "C" fn(*mut SRC_STATE) -> ()),
        };
        init
    }
};
unsafe extern "C" fn linear_vari_process(
    mut state: *mut SRC_STATE,
    mut data: *mut SRC_DATA,
) -> SRC_ERROR {
    let mut priv_0: *mut LINEAR_DATA = 0 as *mut LINEAR_DATA;
    let mut src_ratio: core::ffi::c_double = 0.;
    let mut input_index: core::ffi::c_double = 0.;
    let mut rem: core::ffi::c_double = 0.;
    let mut ch: core::ffi::c_int = 0;
    if (*data).input_frames <= 0 as core::ffi::c_long {
        return SRC_ERR_NO_ERROR;
    }
    if ((*state).private_data).is_null() {
        return SRC_ERR_NO_PRIVATE;
    }
    priv_0 = (*state).private_data as *mut LINEAR_DATA;
    if !(*priv_0).dirty {
        ch = 0 as core::ffi::c_int;
        while ch < (*state).channels {
            *((*priv_0).last_value).offset(ch as isize) = *((*data).data_in)
                .offset(ch as isize);
            ch += 1;
        }
        (*priv_0).dirty = true_0 != 0;
    }
    (*priv_0).in_count = (*data).input_frames * (*state).channels as core::ffi::c_long;
    (*priv_0).out_count = (*data).output_frames * (*state).channels as core::ffi::c_long;
    (*priv_0).out_gen = 0 as core::ffi::c_long;
    (*priv_0).in_used = (*priv_0).out_gen;
    src_ratio = (*state).last_ratio;
    if is_bad_src_ratio(src_ratio) != 0 {
        return SRC_ERR_BAD_INTERNAL_STATE;
    }
    input_index = (*state).last_position;
    while input_index < 1.0f64 && (*priv_0).out_gen < (*priv_0).out_count {
        if (*priv_0).in_used as core::ffi::c_double
            + (*state).channels as core::ffi::c_double * (1.0f64 + input_index)
            >= (*priv_0).in_count as core::ffi::c_double
        {
            break;
        }
        if (*priv_0).out_count > 0 as core::ffi::c_long
            && fabs((*state).last_ratio - (*data).src_ratio) > SRC_MIN_RATIO_DIFF
        {
            src_ratio = (*state).last_ratio
                + (*priv_0).out_gen as core::ffi::c_double
                    * ((*data).src_ratio - (*state).last_ratio)
                    / (*priv_0).out_count as core::ffi::c_double;
        }
        ch = 0 as core::ffi::c_int;
        while ch < (*state).channels {
            *((*data).data_out).offset((*priv_0).out_gen as isize) = (*((*priv_0)
                .last_value)
                .offset(ch as isize) as core::ffi::c_double
                + input_index
                    * (*((*data).data_in).offset(ch as isize) as core::ffi::c_double
                        - *((*priv_0).last_value).offset(ch as isize)
                            as core::ffi::c_double)) as core::ffi::c_float;
            (*priv_0).out_gen += 1;
            ch += 1;
        }
        input_index += 1.0f64 / src_ratio;
    }
    rem = fmod_one(input_index);
    (*priv_0).in_used
        += (*state).channels as core::ffi::c_long * lrint(input_index - rem);
    input_index = rem;
    while (*priv_0).out_gen < (*priv_0).out_count
        && (*priv_0).in_used as core::ffi::c_double
            + (*state).channels as core::ffi::c_double * input_index
            < (*priv_0).in_count as core::ffi::c_double
    {
        if (*priv_0).out_count > 0 as core::ffi::c_long
            && fabs((*state).last_ratio - (*data).src_ratio) > SRC_MIN_RATIO_DIFF
        {
            src_ratio = (*state).last_ratio
                + (*priv_0).out_gen as core::ffi::c_double
                    * ((*data).src_ratio - (*state).last_ratio)
                    / (*priv_0).out_count as core::ffi::c_double;
        }
        ch = 0 as core::ffi::c_int;
        while ch < (*state).channels {
            *((*data).data_out).offset((*priv_0).out_gen as isize) = (*((*data).data_in)
                .offset(
                    ((*priv_0).in_used - (*state).channels as core::ffi::c_long
                        + ch as core::ffi::c_long) as isize,
                ) as core::ffi::c_double
                + input_index
                    * (*((*data).data_in)
                        .offset(((*priv_0).in_used + ch as core::ffi::c_long) as isize)
                        as core::ffi::c_double
                        - *((*data).data_in)
                            .offset(
                                ((*priv_0).in_used - (*state).channels as core::ffi::c_long
                                    + ch as core::ffi::c_long) as isize,
                            ) as core::ffi::c_double)) as core::ffi::c_float;
            (*priv_0).out_gen += 1;
            ch += 1;
        }
        input_index += 1.0f64 / src_ratio;
        rem = fmod_one(input_index);
        (*priv_0).in_used
            += (*state).channels as core::ffi::c_long * lrint(input_index - rem);
        input_index = rem;
    }
    if (*priv_0).in_used > (*priv_0).in_count {
        input_index
            += (((*priv_0).in_used - (*priv_0).in_count)
                / (*state).channels as core::ffi::c_long) as core::ffi::c_double;
        (*priv_0).in_used = (*priv_0).in_count;
    }
    (*state).last_position = input_index;
    if (*priv_0).in_used > 0 as core::ffi::c_long {
        ch = 0 as core::ffi::c_int;
        while ch < (*state).channels {
            *((*priv_0).last_value).offset(ch as isize) = *((*data).data_in)
                .offset(
                    ((*priv_0).in_used - (*state).channels as core::ffi::c_long
                        + ch as core::ffi::c_long) as isize,
                );
            ch += 1;
        }
    }
    (*state).last_ratio = src_ratio;
    (*data).input_frames_used = (*priv_0).in_used
        / (*state).channels as core::ffi::c_long;
    (*data).output_frames_gen = (*priv_0).out_gen
        / (*state).channels as core::ffi::c_long;
    return SRC_ERR_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn linear_get_name(
    mut src_enum: core::ffi::c_int,
) -> *const core::ffi::c_char {
    if src_enum == SRC_LINEAR as core::ffi::c_int {
        return b"Linear Interpolator\0" as *const u8 as *const core::ffi::c_char;
    }
    return 0 as *const core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn linear_get_description(
    mut src_enum: core::ffi::c_int,
) -> *const core::ffi::c_char {
    if src_enum == SRC_LINEAR as core::ffi::c_int {
        return b"Linear interpolator, very fast, poor quality.\0" as *const u8
            as *const core::ffi::c_char;
    }
    return 0 as *const core::ffi::c_char;
}
unsafe extern "C" fn linear_data_new(
    mut channels: core::ffi::c_int,
) -> *mut LINEAR_DATA {
    let mut priv_0: *mut LINEAR_DATA = calloc(
        1 as size_t,
        ::core::mem::size_of::<LINEAR_DATA>() as size_t,
    ) as *mut LINEAR_DATA;
    if !priv_0.is_null() {
        (*priv_0).linear_magic_marker = LINEAR_MAGIC_MARKER;
        (*priv_0).last_value = calloc(
            channels as size_t,
            ::core::mem::size_of::<core::ffi::c_float>() as size_t,
        ) as *mut core::ffi::c_float;
        if ((*priv_0).last_value).is_null() {
            free(priv_0 as *mut core::ffi::c_void);
            priv_0 = 0 as *mut LINEAR_DATA;
        }
    }
    return priv_0;
}
#[no_mangle]
pub unsafe extern "C" fn linear_state_new(
    mut channels: core::ffi::c_int,
    mut error: *mut SRC_ERROR,
) -> *mut SRC_STATE {
    let mut state: *mut SRC_STATE = calloc(
        1 as size_t,
        ::core::mem::size_of::<SRC_STATE>() as size_t,
    ) as *mut SRC_STATE;
    if state.is_null() {
        *error = SRC_ERR_MALLOC_FAILED;
        return 0 as *mut SRC_STATE;
    }
    (*state).channels = channels;
    (*state).mode = SRC_MODE_PROCESS;
    (*state).private_data = linear_data_new((*state).channels) as *mut core::ffi::c_void;
    if ((*state).private_data).is_null() {
        free(state as *mut core::ffi::c_void);
        *error = SRC_ERR_MALLOC_FAILED;
        return 0 as *mut SRC_STATE;
    }
    (*state).vt = &mut linear_state_vt;
    linear_reset(state);
    *error = SRC_ERR_NO_ERROR;
    return state;
}
unsafe extern "C" fn linear_reset(mut state: *mut SRC_STATE) {
    let mut priv_0: *mut LINEAR_DATA = 0 as *mut LINEAR_DATA;
    priv_0 = (*state).private_data as *mut LINEAR_DATA;
    if priv_0.is_null() {
        return;
    }
    (*priv_0).dirty = false_0 != 0;
    memset(
        (*priv_0).last_value as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<core::ffi::c_float>() as size_t)
            .wrapping_mul((*state).channels as size_t),
    );
}
unsafe extern "C" fn linear_copy(mut state: *mut SRC_STATE) -> *mut SRC_STATE {
    if ((*state).private_data).is_null() {
        return 0 as *mut SRC_STATE;
    }
    let mut to: *mut SRC_STATE = calloc(
        1 as size_t,
        ::core::mem::size_of::<SRC_STATE>() as size_t,
    ) as *mut SRC_STATE;
    if to.is_null() {
        return 0 as *mut SRC_STATE;
    }
    memcpy(
        to as *mut core::ffi::c_void,
        state as *const core::ffi::c_void,
        ::core::mem::size_of::<SRC_STATE>() as size_t,
    );
    let mut from_priv: *mut LINEAR_DATA = (*state).private_data as *mut LINEAR_DATA;
    let mut to_priv: *mut LINEAR_DATA = calloc(
        1 as size_t,
        ::core::mem::size_of::<LINEAR_DATA>() as size_t,
    ) as *mut LINEAR_DATA;
    if to_priv.is_null() {
        free(to as *mut core::ffi::c_void);
        return 0 as *mut SRC_STATE;
    }
    memcpy(
        to_priv as *mut core::ffi::c_void,
        from_priv as *const core::ffi::c_void,
        ::core::mem::size_of::<LINEAR_DATA>() as size_t,
    );
    (*to_priv).last_value = malloc(
        (::core::mem::size_of::<core::ffi::c_float>() as size_t)
            .wrapping_mul((*state).channels as size_t),
    ) as *mut core::ffi::c_float;
    if ((*to_priv).last_value).is_null() {
        free(to as *mut core::ffi::c_void);
        free(to_priv as *mut core::ffi::c_void);
        return 0 as *mut SRC_STATE;
    }
    memcpy(
        (*to_priv).last_value as *mut core::ffi::c_void,
        (*from_priv).last_value as *const core::ffi::c_void,
        (::core::mem::size_of::<core::ffi::c_float>() as size_t)
            .wrapping_mul((*state).channels as size_t),
    );
    (*to).private_data = to_priv as *mut core::ffi::c_void;
    return to;
}
unsafe extern "C" fn linear_close(mut state: *mut SRC_STATE) {
    if !state.is_null() {
        let mut linear: *mut LINEAR_DATA = (*state).private_data as *mut LINEAR_DATA;
        if !linear.is_null() {
            if !((*linear).last_value).is_null() {
                free((*linear).last_value as *mut core::ffi::c_void);
                (*linear).last_value = 0 as *mut core::ffi::c_float;
            }
            free(linear as *mut core::ffi::c_void);
            linear = 0 as *mut LINEAR_DATA;
        }
        free(state as *mut core::ffi::c_void);
        state = 0 as *mut SRC_STATE;
    }
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
