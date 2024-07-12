pub type _LqrRetVal = libc::c_uint;
pub const LQR_USRCANCEL: _LqrRetVal = 3;
pub const LQR_NOMEM: _LqrRetVal = 2;
pub const LQR_OK: _LqrRetVal = 1;
pub const LQR_ERROR: _LqrRetVal = 0;
pub type LqrRetVal = _LqrRetVal;
pub type _LqrColDepth = libc::c_uint;
pub const LQR_COLDEPTH_8I: _LqrColDepth = 0;
pub type LqrColDepth = _LqrColDepth;
pub type _LqrResizeOrder = libc::c_uint;
pub const LQR_RES_ORDER_HOR: _LqrResizeOrder = 0;
pub type LqrResizeOrder = _LqrResizeOrder;
pub type _LqrImageType = libc::c_uint;
pub const LQR_CUSTOM_IMAGE: _LqrImageType = 7;
pub const LQR_CMYKA_IMAGE: _LqrImageType = 6;
pub const LQR_GREYA_IMAGE: _LqrImageType = 3;
pub const LQR_GREY_IMAGE: _LqrImageType = 2;
pub const LQR_RGBA_IMAGE: _LqrImageType = 1;
pub const LQR_RGB_IMAGE: _LqrImageType = 0;
pub type LqrImageType = _LqrImageType;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _LqrCarver {
    pub w_start: libc::c_int,
    pub h_start: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub w0: libc::c_int,
    pub h0: libc::c_int,
    pub level: libc::c_int,
    pub max_level: libc::c_int,
    pub image_type: LqrImageType,
    pub channels: libc::c_int,
    pub alpha_channel: libc::c_int,
    pub black_channel: libc::c_int,
    pub col_depth: LqrColDepth,
    pub transposed: libc::c_int,
    pub active: libc::c_int,
    pub nrg_active: libc::c_int,
    pub root: *mut LqrCarver,
    pub resize_aux_layers: libc::c_int,
    pub dump_vmaps: libc::c_int,
    pub resize_order: LqrResizeOrder,
    pub attached_list: *mut LqrCarverList,
    pub rigidity: libc::c_float,
    pub rigidity_map: *mut libc::c_float,
    pub rigidity_mask: *mut libc::c_float,
    pub delta_x: libc::c_int,
    pub rgb: *mut libc::c_void,
    pub vs: *mut libc::c_int,
    pub en: *mut libc::c_float,
    pub bias: *mut libc::c_float,
    pub m: *mut libc::c_float,
    pub least: *mut libc::c_int,
    pub _raw: *mut libc::c_int,
    pub raw: *mut *mut libc::c_int,
    pub c: *mut LqrCursor,
    pub rgb_ro_buffer: *mut libc::c_void,
    pub vpath: *mut libc::c_int,
    pub vpath_x: *mut libc::c_int,
    pub leftright: libc::c_int,
    pub lr_switch_frequency: libc::c_int,
    pub enl_step: libc::c_float,
    pub progress: *mut LqrProgress,
    pub session_update_step: libc::c_int,
    pub session_rescale_total: libc::c_int,
    pub session_rescale_current: libc::c_int,
    pub nrg: LqrEnergyFunc,
    pub nrg_radius: libc::c_int,
    pub nrg_read_t: LqrEnergyReaderType,
    pub nrg_extra_data: *mut libc::c_void,
    pub rwindow: *mut LqrReadingWindow,
    pub nrg_xmin: *mut libc::c_int,
    pub nrg_xmax: *mut libc::c_int,
    pub nrg_uptodate: libc::c_int,
    pub rcache: *mut libc::c_double,
    pub use_rcache: libc::c_int,
    pub flushed_vs: *mut LqrVMapList,
    pub preserve_in_buffer: libc::c_int,
    pub state: libc::c_int,
    pub state_lock: libc::c_int,
    pub state_lock_queue: libc::c_int,
}
pub type LqrVMapList = _LqrVMapList;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _LqrVMapList {
    pub current: *mut LqrVMap,
    pub next: *mut LqrVMapList,
}
pub type LqrVMap = _LqrVMap;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _LqrVMap {
    pub buffer: *mut libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub depth: libc::c_int,
    pub orientation: libc::c_int,
}
pub type LqrReadingWindow = _LqrReadingWindow;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _LqrReadingWindow {
    pub buffer: *mut *mut libc::c_double,
    pub radius: libc::c_int,
    pub read_t: LqrEnergyReaderType,
    pub channels: libc::c_int,
    pub use_rcache: libc::c_int,
    pub carver: *mut LqrCarver,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
pub type LqrCarver = _LqrCarver;
pub type LqrEnergyReaderType = _LqrEnergyReaderType;
pub type _LqrEnergyReaderType = libc::c_uint;
pub const LQR_ER_CUSTOM: _LqrEnergyReaderType = 3;
pub const LQR_ER_RGBA: _LqrEnergyReaderType = 2;
pub const LQR_ER_LUMA: _LqrEnergyReaderType = 1;
pub const LQR_ER_BRIGHTNESS: _LqrEnergyReaderType = 0;
pub type LqrEnergyFunc = Option<
    unsafe extern "C" fn(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut LqrReadingWindow,
        _: *mut libc::c_void,
    ) -> libc::c_float,
>;
pub type LqrProgress = _LqrProgress;
pub type LqrCursor = _LqrCursor;
pub type LqrCarverList = _LqrCarverList;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _LqrCarverList {
    pub current: *mut LqrCarver,
    pub next: *mut LqrCarverList,
}
pub type LqrGradFunc = Option<unsafe extern "C" fn(_: libc::c_double, _: libc::c_double) -> libc::c_float>;
pub type _LqrEnergyFuncBuiltinType = libc::c_uint;
pub const LQR_EF_GRAD_XABS: _LqrEnergyFuncBuiltinType = 2;
pub type LqrEnergyFuncBuiltinType = _LqrEnergyFuncBuiltinType;
pub const LQR_CARVER_STATE_CANCELLED: _LqrCarverState = 5;
pub type _LqrCarverState = libc::c_uint;
pub const LQR_CARVER_STATE_FLATTENING: _LqrCarverState = 4;
pub const LQR_CARVER_STATE_TRANSPOSING: _LqrCarverState = 3;
pub const LQR_CARVER_STATE_INFLATING: _LqrCarverState = 2;
pub const LQR_CARVER_STATE_RESIZING: _LqrCarverState = 1;
pub const LQR_CARVER_STATE_STD: _LqrCarverState = 0;

pub type LqrProgressFuncEnd = Option<unsafe extern "C" fn(_: *const libc::c_char) -> LqrRetVal>;
pub type LqrProgressFuncUpdate = Option<unsafe extern "C" fn(_: libc::c_double) -> LqrRetVal>;
pub type LqrProgressFuncInit = Option<unsafe extern "C" fn(_: *const libc::c_char) -> LqrRetVal>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _LqrCursor {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub now: libc::c_int,
    pub o: *mut LqrCarver,
    pub eoc: libc::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union _LqrDataTok {
    pub carver: *mut LqrCarver,
    pub integer: libc::c_int,
    pub data: *mut libc::c_void,
}
pub type LqrDataTok = _LqrDataTok;
pub type LqrCarverFunc = Option<unsafe extern "C" fn(_: *mut LqrCarver, _: LqrDataTok) -> LqrRetVal>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _LqrProgress {
    pub update_step: libc::c_float,
    pub init: LqrProgressFuncInit,
    pub update: LqrProgressFuncUpdate,
    pub end: LqrProgressFuncEnd,
    pub init_width_message: [libc::c_char; 1024],
    pub end_width_message: [libc::c_char; 1024],
    pub init_height_message: [libc::c_char; 1024],
    pub end_height_message: [libc::c_char; 1024],
}
pub type LqrReadFunc =
    Option<unsafe extern "C" fn(_: *mut LqrCarver, _: libc::c_int, _: libc::c_int) -> libc::c_double>;
pub type LqrCarverState = _LqrCarverState;
pub type _LqrGradFuncType = libc::c_uint;

pub unsafe extern "C" fn strlcpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    siz: libc::c_ulong,
) -> libc::c_ulong {
    let mut d: *mut libc::c_char = dst;
    let mut s: *const libc::c_char = src;
    let mut n: libc::c_ulong = siz;
    /* Copy as many bytes as will fit */
    if n != 0 as libc::c_int as libc::c_ulong {
        loop {
            n = n.wrapping_sub(1);
            if !(n != 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let fresh0 = s;
            s = s.offset(1);
            let fresh1 = d;
            d = d.offset(1);
            *fresh1 = *fresh0;
            if *fresh1 as libc::c_int == '\u{0}' as i32 {
                break;
            }
        }
    }
    /* Not enough room in dst, add NUL and traverse rest of src */
    if n == 0 as libc::c_int as libc::c_ulong {
        if siz != 0 as libc::c_int as libc::c_ulong {
            *d = '\u{0}' as i32 as libc::c_char
        } /* NUL-terminate dst */
        loop {
            let fresh2 = s;
            s = s.offset(1);
            if !(*fresh2 != 0) {
                break;
            }
        }
    }
    return (s.offset_from(src) as libc::c_long - 1 as libc::c_int as libc::c_long) as libc::c_ulong;
    /* count does not include NUL */
}

#[inline(always)]
pub unsafe fn malloc0(size: u64) -> *mut libc::c_void {
    let p: *mut libc::c_void = libc::malloc(size as usize);
    if p.is_null() {
        return 0 as *mut libc::c_void;
    }
    libc::memset(p, 0 as libc::c_int, size as usize);
    return p;
}

#[inline(always)]
pub unsafe fn malloc(size: u64) -> *mut libc::c_void {
    let p: *mut libc::c_void = libc::malloc(size as usize);
    if p.is_null() {
        return 0 as *mut libc::c_void;
    }
    return p;
}

#[inline(always)]
pub unsafe fn calloc(nmemb: u64, size: u64) -> *mut libc::c_void {
    let p: *mut libc::c_void = libc::calloc(nmemb as usize, size as usize);
    if p.is_null() {
        return 0 as *mut libc::c_void;
    }
    return p;
}
