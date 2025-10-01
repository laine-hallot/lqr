#![allow(
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::intrinsics::AtomicOrdering;

use crate::lqr_globals::*;

/* LiquidRescaling Library
* Copyright (C) 2007-2009 Carlo Baldassi (the "Author") <carlobaldassi@gmail.com>.
* All Rights Reserved.
*
* This library implements the algorithm described in the paper
* "Seam Carving for Content-Aware Image Resizing"
* by Shai Avidan and Ariel Shamir
* which can be found at http://www.faculty.idc.ac.il/arik/imret.pdf
*
* This program is free software; you can redistribute it and/or modify
* it under the terms of the GNU Lesser General Public License as published by
* the Free Software Foundation; version 3 dated June, 2007.

* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Lesser General Public License for more details.

* You should have received a copy of the GNU Lesser General Public License
* along with this program; if not, see <http://www.gnu.org/licenses/>
*/
/* __LQR_VERBOSE__ */
/* __LQR_DEBUG__ */
/* *** LQR_CARVER CLASS FUNCTIONS *** */
/* ** constructor & destructor ** */
/* constructors */

pub unsafe extern "C" fn lqr_carver_new_common(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut channels: libc::c_int,
) -> *mut LqrCarver {
    let mut r: *mut LqrCarver = ({
        let mut __n: libc::c_ulong = 1 as libc::c_int as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<LqrCarver>() as libc::c_ulong;
        let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = malloc(__n)
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = malloc(__n.wrapping_mul(__s))
        } else {
            __p = calloc(__n, __s)
        }
        __p
    }) as *mut LqrCarver;
    if r.is_null() {
        return 0 as *mut LqrCarver;
    }
    let mut gais_temp: libc::c_int = LQR_CARVER_STATE_STD as libc::c_int;

    ::std::intrinsics::atomic_store::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int, *&mut gais_temp);
    let mut gais_temp_0: libc::c_int = 0 as libc::c_int;

    ::std::intrinsics::atomic_store::<_, {AtomicOrdering::SeqCst}>(
        &mut (*r).state_lock as *mut libc::c_int as *mut libc::c_int,
        *&mut gais_temp_0,
    );
    let mut gais_temp_1: libc::c_int = 0 as libc::c_int;

    ::std::intrinsics::atomic_store::<_, {AtomicOrdering::SeqCst}>(
        &mut (*r).state_lock_queue as *mut libc::c_int as *mut libc::c_int,
        *&mut gais_temp_1,
    );
    (*r).level = 1 as libc::c_int;
    (*r).max_level = 1 as libc::c_int;
    (*r).transposed = 0 as libc::c_int;
    (*r).active = 0 as libc::c_int;
    (*r).nrg_active = 0 as libc::c_int;
    (*r).root = 0 as *mut LqrCarver;
    (*r).rigidity = 0 as libc::c_int as libc::c_float;
    (*r).resize_aux_layers = 0 as libc::c_int;
    (*r).dump_vmaps = 0 as libc::c_int;
    (*r).resize_order = LQR_RES_ORDER_HOR;
    (*r).attached_list = 0 as *mut LqrCarverList;
    (*r).flushed_vs = 0 as *mut LqrVMapList;
    (*r).preserve_in_buffer = 0 as libc::c_int;
    (*r).progress = crate::lqr_progress::lqr_progress_new();
    if (*r).progress.is_null() {
        return 0 as *mut LqrCarver;
    }
    (*r).session_update_step = 1 as libc::c_int;
    (*r).session_rescale_total = 0 as libc::c_int;
    (*r).session_rescale_current = 0 as libc::c_int;
    (*r).en = 0 as *mut libc::c_float;
    (*r).bias = 0 as *mut libc::c_float;
    (*r).m = 0 as *mut libc::c_float;
    (*r).least = 0 as *mut libc::c_int;
    (*r)._raw = 0 as *mut libc::c_int;
    (*r).raw = 0 as *mut *mut libc::c_int;
    (*r).vpath = 0 as *mut libc::c_int;
    (*r).vpath_x = 0 as *mut libc::c_int;
    (*r).rigidity_map = 0 as *mut libc::c_float;
    (*r).rigidity_mask = 0 as *mut libc::c_float;
    (*r).delta_x = 1 as libc::c_int;
    (*r).h = height;
    (*r).w = width;
    (*r).channels = channels;
    (*r).w0 = (*r).w;
    (*r).h0 = (*r).h;
    (*r).w_start = (*r).w;
    (*r).h_start = (*r).h;
    (*r).rcache = 0 as *mut libc::c_double;
    (*r).use_rcache = (0 as libc::c_int == 0) as libc::c_int;
    (*r).rwindow = 0 as *mut LqrReadingWindow;
    crate::lqr_energy::lqr_carver_set_energy_function_builtin(r, LQR_EF_GRAD_XABS);
    (*r).nrg_xmin = 0 as *mut libc::c_int;
    (*r).nrg_xmax = 0 as *mut libc::c_int;
    (*r).nrg_uptodate = 0 as libc::c_int;
    (*r).leftright = 0 as libc::c_int;
    (*r).lr_switch_frequency = 0 as libc::c_int;
    (*r).enl_step = 2.0f64 as libc::c_float;
    (*r).vs = ({
        let mut __n: libc::c_ulong = ((*r).w * (*r).h) as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
        let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = malloc0(__n)
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = malloc0(__n.wrapping_mul(__s))
        } else {
            __p = calloc(__n, __s)
        }
        __p
    }) as *mut libc::c_int;
    if (*r).vs.is_null() {
        return 0 as *mut LqrCarver;
    }
    /* initialize cursor */
    (*r).c = crate::lqr_cursor::lqr_cursor_create(r);
    if (*r).c.is_null() {
        return 0 as *mut LqrCarver;
    }
    match channels {
        1 => {
            lqr_carver_set_image_type(r, LQR_GREY_IMAGE);
        },
        2 => {
            lqr_carver_set_image_type(r, LQR_GREYA_IMAGE);
        },
        3 => {
            lqr_carver_set_image_type(r, LQR_RGB_IMAGE);
        },
        4 => {
            lqr_carver_set_image_type(r, LQR_RGBA_IMAGE);
        },
        5 => {
            lqr_carver_set_image_type(r, LQR_CMYKA_IMAGE);
        },
        _ => {
            lqr_carver_set_image_type(r, LQR_CUSTOM_IMAGE);
        },
    }
    return r;
}
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_carver_new(
    mut buffer: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut channels: libc::c_int,
) -> *mut LqrCarver {
    return lqr_carver_new_ext(buffer as *mut libc::c_void, width, height, channels, LQR_COLDEPTH_8I);
}
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_carver_new_ext(
    mut buffer: *mut libc::c_void,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut channels: libc::c_int,
    mut colour_depth: LqrColDepth,
) -> *mut LqrCarver {
    let mut r: *mut LqrCarver = lqr_carver_new_common(width, height, channels);
    if r.is_null() {
        return 0 as *mut LqrCarver;
    }
    (*r).rgb = buffer;
    match colour_depth as libc::c_uint {
        0 => {
            (*r).rgb_ro_buffer = ({
                let mut __n: libc::c_ulong = ((*r).channels * (*r).w) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_uchar as *mut libc::c_void;
            if (*r).rgb_ro_buffer.is_null() {
                return 0 as *mut LqrCarver;
            }
        },
        1 => {
            (*r).rgb_ro_buffer = ({
                let mut __n: libc::c_ulong = ((*r).channels * (*r).w) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_ushort as *mut libc::c_void;
            if (*r).rgb_ro_buffer.is_null() {
                return 0 as *mut LqrCarver;
            }
        },
        2 => {
            (*r).rgb_ro_buffer = ({
                let mut __n: libc::c_ulong = ((*r).channels * (*r).w) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_float as *mut libc::c_void;
            if (*r).rgb_ro_buffer.is_null() {
                return 0 as *mut LqrCarver;
            }
        },
        3 => {
            (*r).rgb_ro_buffer = ({
                let mut __n: libc::c_ulong = ((*r).channels * (*r).w) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_double>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_double as *mut libc::c_void;
            if (*r).rgb_ro_buffer.is_null() {
                return 0 as *mut LqrCarver;
            }
        },
        _ => {},
    }
    (*r).col_depth = colour_depth;
    return r;
}
/* destructor */
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_carver_destroy(mut r: *mut LqrCarver) {
    if (*r).preserve_in_buffer == 0 {
        libc::free((*r).rgb);
    }
    if (*r).root.is_null() {
        libc::free((*r).vs as *mut libc::c_void);
    }
    libc::free((*r).rgb_ro_buffer);
    libc::free((*r).en as *mut libc::c_void);
    libc::free((*r).bias as *mut libc::c_void);
    libc::free((*r).m as *mut libc::c_void);
    libc::free((*r).rcache as *mut libc::c_void);
    libc::free((*r).least as *mut libc::c_void);
    crate::lqr_cursor::lqr_cursor_destroy((*r).c);
    libc::free((*r).vpath as *mut libc::c_void);
    libc::free((*r).vpath_x as *mut libc::c_void);
    if !(*r).rigidity_map.is_null() {
        (*r).rigidity_map = (*r).rigidity_map.offset(-((*r).delta_x as isize));
        libc::free((*r).rigidity_map as *mut libc::c_void);
    }
    libc::free((*r).rigidity_mask as *mut libc::c_void);
    crate::lqr_rwindow::lqr_rwindow_destroy((*r).rwindow);
    libc::free((*r).nrg_xmin as *mut libc::c_void);
    libc::free((*r).nrg_xmax as *mut libc::c_void);
    crate::lqr_vmap_list::lqr_vmap_list_destroy((*r).flushed_vs);
    crate::lqr_carver_list::lqr_carver_list_destroy((*r).attached_list);
    libc::free((*r).progress as *mut libc::c_void);
    libc::free((*r)._raw as *mut libc::c_void);
    libc::free((*r).raw as *mut libc::c_void);
    libc::free(r as *mut libc::c_void);
}
/* ** initialization ** */

pub unsafe extern "C" fn lqr_carver_init_energy_related(mut r: *mut LqrCarver) -> LqrRetVal {
    let mut _x: libc::c_int = 0;
    if ((*r).active == 0 as libc::c_int) as libc::c_int == 0 as libc::c_int {
        return LQR_ERROR;
    }
    if ((*r).nrg_active == 0 as libc::c_int) as libc::c_int == 0 as libc::c_int {
        return LQR_ERROR;
    }
    (*r).en = ({
        let mut __n: libc::c_ulong = ((*r).w * (*r).h) as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
        let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = malloc(__n)
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = malloc(__n.wrapping_mul(__s))
        } else {
            __p = calloc(__n, __s)
        }
        __p
    }) as *mut libc::c_float;
    if (*r).en.is_null() {
        return LQR_NOMEM;
    }
    (*r)._raw = ({
        let mut __n: libc::c_ulong = ((*r).h_start * (*r).w_start) as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
        let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = malloc(__n)
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = malloc(__n.wrapping_mul(__s))
        } else {
            __p = calloc(__n, __s)
        }
        __p
    }) as *mut libc::c_int;
    if (*r)._raw.is_null() {
        return LQR_NOMEM;
    }
    (*r).raw = ({
        let mut __n: libc::c_ulong = (*r).h_start as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong;
        let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = malloc(__n)
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = malloc(__n.wrapping_mul(__s))
        } else {
            __p = calloc(__n, __s)
        }
        __p
    }) as *mut *mut libc::c_int;
    if (*r).raw.is_null() {
        return LQR_NOMEM;
    }
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*r).h {
        let ref mut fresh0 = *(*r).raw.offset(y as isize);
        *fresh0 = (*r)._raw.offset((y * (*r).w_start) as isize);

        for x in 0 as libc::c_int..(*r).w_start {
            *(*(*r).raw.offset(y as isize)).offset(x as isize) = y * (*r).w_start + x;
        }
        y += 1
    }
    (*r).nrg_active = (0 as libc::c_int == 0) as libc::c_int;
    return LQR_OK;
}
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_carver_init(
    mut r: *mut LqrCarver,
    mut delta_x: libc::c_int,
    mut rigidity: libc::c_float,
) -> LqrRetVal {
    if ({
        let mut gaig_temp: libc::c_int = 0;

        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
    {
        return LQR_USRCANCEL;
    }
    if ((*r).active == 0 as libc::c_int) as libc::c_int == 0 as libc::c_int {
        return LQR_ERROR;
    }
    if (*r).nrg_active == 0 as libc::c_int {
        let mut ret_val: LqrRetVal = LQR_ERROR;
        ret_val = lqr_carver_init_energy_related(r);
        if ret_val as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val;
        }
    }
    /* LQR_CATCH_MEM (r->bias = g_try_new0 (libc::c_float, r->w * r->h)); */
    (*r).m = ({
        let mut __n: libc::c_ulong = ((*r).w * (*r).h) as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
        let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = malloc(__n)
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = malloc(__n.wrapping_mul(__s))
        } else {
            __p = calloc(__n, __s)
        }
        __p
    }) as *mut libc::c_float;
    if (*r).m.is_null() {
        return LQR_NOMEM;
    }
    (*r).least = ({
        let mut __n: libc::c_ulong = ((*r).w * (*r).h) as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
        let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = malloc(__n)
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = malloc(__n.wrapping_mul(__s))
        } else {
            __p = calloc(__n, __s)
        }
        __p
    }) as *mut libc::c_int;
    if (*r).least.is_null() {
        return LQR_NOMEM;
    }
    (*r).vpath = ({
        let mut __n: libc::c_ulong = (*r).h as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
        let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = malloc(__n)
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = malloc(__n.wrapping_mul(__s))
        } else {
            __p = calloc(__n, __s)
        }
        __p
    }) as *mut libc::c_int;
    if (*r).vpath.is_null() {
        return LQR_NOMEM;
    }
    (*r).vpath_x = ({
        let mut __n: libc::c_ulong = (*r).h as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
        let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = malloc(__n)
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = malloc(__n.wrapping_mul(__s))
        } else {
            __p = calloc(__n, __s)
        }
        __p
    }) as *mut libc::c_int;
    if (*r).vpath_x.is_null() {
        return LQR_NOMEM;
    }
    (*r).nrg_xmin = ({
        let mut __n: libc::c_ulong = (*r).h as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
        let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = malloc(__n)
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = malloc(__n.wrapping_mul(__s))
        } else {
            __p = calloc(__n, __s)
        }
        __p
    }) as *mut libc::c_int;
    if (*r).nrg_xmin.is_null() {
        return LQR_NOMEM;
    }
    (*r).nrg_xmax = ({
        let mut __n: libc::c_ulong = (*r).h as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
        let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = malloc(__n)
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = malloc(__n.wrapping_mul(__s))
        } else {
            __p = calloc(__n, __s)
        }
        __p
    }) as *mut libc::c_int;
    if (*r).nrg_xmax.is_null() {
        return LQR_NOMEM;
    }
    /* set rigidity map */
    (*r).delta_x = delta_x;
    (*r).rigidity = rigidity;
    (*r).rigidity_map = ({
        let mut __n: libc::c_ulong = (2 as libc::c_int * (*r).delta_x + 1 as libc::c_int) as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
        let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
        if __s == 1 as libc::c_int as libc::c_ulong {
            __p = malloc0(__n)
        } else if 0 != 0
            && (__s == 0 as libc::c_int as libc::c_ulong
                || __n
                    <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_ulong)
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_div(__s))
        {
            __p = malloc0(__n.wrapping_mul(__s))
        } else {
            __p = calloc(__n, __s)
        }
        __p
    }) as *mut libc::c_float;
    (*r).rigidity_map = (*r).rigidity_map.offset((*r).delta_x as isize);
    let mut x: libc::c_int = -(*r).delta_x;
    while x <= (*r).delta_x {
        *(*r).rigidity_map.offset(x as isize) = (*r).rigidity
            * libm::powf(libm::fabsf(x as libc::c_float), 1.5f64 as libc::c_float)
            / (*r).h as libc::c_float;
        x += 1
    }
    (*r).active = (0 as libc::c_int == 0) as libc::c_int;
    return LQR_OK;
}
/* ** set attributes ** */
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_carver_set_image_type(mut r: *mut LqrCarver, mut image_type: LqrImageType) -> LqrRetVal {
    if ({
        let mut gaig_temp: libc::c_int = 0;

        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
    {
        return LQR_USRCANCEL;
    }
    match image_type as libc::c_uint {
        2 => {
            if (*r).channels != 1 as libc::c_int {
                return LQR_ERROR;
            }
            (*r).alpha_channel = -(1 as libc::c_int);
            (*r).black_channel = -(1 as libc::c_int)
        },
        3 => {
            if (*r).channels != 2 as libc::c_int {
                return LQR_ERROR;
            }
            (*r).alpha_channel = 1 as libc::c_int;
            (*r).black_channel = -(1 as libc::c_int)
        },
        4 | 0 => {
            if (*r).channels != 3 as libc::c_int {
                return LQR_ERROR;
            }
            (*r).alpha_channel = -(1 as libc::c_int);
            (*r).black_channel = -(1 as libc::c_int)
        },
        5 => {
            if (*r).channels != 4 as libc::c_int {
                return LQR_ERROR;
            }
            (*r).alpha_channel = -(1 as libc::c_int);
            (*r).black_channel = 3 as libc::c_int
        },
        1 => {
            if (*r).channels != 4 as libc::c_int {
                return LQR_ERROR;
            }
            (*r).alpha_channel = 3 as libc::c_int;
            (*r).black_channel = -(1 as libc::c_int)
        },
        6 => {
            if (*r).channels != 5 as libc::c_int {
                return LQR_ERROR;
            }
            (*r).alpha_channel = 4 as libc::c_int;
            (*r).black_channel = 3 as libc::c_int
        },
        7 => {
            (*r).alpha_channel = -(1 as libc::c_int);
            (*r).black_channel = -(1 as libc::c_int)
        },
        _ => return LQR_ERROR,
    }
    (*r).image_type = image_type;
    libc::free((*r).rcache as *mut libc::c_void);
    (*r).rcache = 0 as *mut libc::c_double;
    (*r).nrg_uptodate = 0 as libc::c_int;
    return LQR_OK;
}
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_carver_set_preserve_input_image(mut r: *mut LqrCarver) {
    (*r).preserve_in_buffer = (0 as libc::c_int == 0) as libc::c_int;
}
/* ** compute maps (energy, minpath & visibility) ** */
/* build multisize image up to given depth
 * it is progressive (can be called multilple times) */

pub unsafe extern "C" fn lqr_carver_build_maps(mut r: *mut LqrCarver, mut depth: libc::c_int) -> LqrRetVal {
    /* __LQR_DEBUG__ */
    if ({
        let mut gaig_temp: libc::c_int = 0;

        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
    {
        return LQR_USRCANCEL;
    }
    /* only go deeper if needed */
    if depth > (*r).max_level {
        if (*r).active == 0 as libc::c_int {
            return LQR_ERROR;
        }
        if ((*r).root == 0 as *mut libc::c_void as *mut LqrCarver) as libc::c_int == 0 as libc::c_int {
            return LQR_ERROR;
        }
        /* set to minimum width reached so far */
        lqr_carver_set_width(r, (*r).w_start - (*r).max_level + 1 as libc::c_int);
        /* compute energy & minpath maps */
        let mut ret_val: LqrRetVal = LQR_ERROR;
        ret_val = lqr_carver_build_emap(r);
        if ret_val as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val;
        }
        let mut ret_val_0: LqrRetVal = LQR_ERROR;
        ret_val_0 = lqr_carver_build_mmap(r);
        if ret_val_0 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val_0;
        }
        /* compute visibility map */
        let mut ret_val_1: LqrRetVal = LQR_ERROR;
        ret_val_1 = lqr_carver_build_vsmap(r, depth);
        if ret_val_1 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val_1;
        }
    }
    return LQR_OK;
}
/* compute energy map */

pub unsafe extern "C" fn lqr_carver_build_emap(mut r: *mut LqrCarver) -> LqrRetVal {
    let mut _x: libc::c_int = 0;

    if ({
        let mut gaig_temp: libc::c_int = 0;

        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
    {
        return LQR_USRCANCEL;
    }
    if (*r).nrg_uptodate != 0 {
        return LQR_OK;
    }
    if (*r).use_rcache != 0 && (*r).rcache.is_null() {
        (*r).rcache = crate::lqr_energy::lqr_carver_generate_rcache(r);
        if (*r).rcache.is_null() {
            return LQR_NOMEM;
        }
    }
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*r).h {
        if ({
            let mut gaig_temp: libc::c_int = 0;

            *(&mut gaig_temp as *mut libc::c_int) =
                ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
            gaig_temp
        }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
        {
            return LQR_USRCANCEL;
        }

        for x in 0 as libc::c_int..(*r).w {
            let mut ret_val: LqrRetVal = LQR_ERROR;

            ret_val = lqr_carver_compute_e(r, x, y);

            if ret_val as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val;
            }
        }
        y += 1
    }
    (*r).nrg_uptodate = (0 as libc::c_int == 0) as libc::c_int;
    return LQR_OK;
}

pub unsafe extern "C" fn lqr_carver_compute_e(
    mut r: *mut LqrCarver,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> LqrRetVal {
    let mut b_add: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut data: libc::c_int = *(*(*r).raw.offset(y as isize)).offset(x as isize);
    let mut ret_val: LqrRetVal = LQR_ERROR;
    ret_val = crate::lqr_rwindow::lqr_rwindow_fill((*r).rwindow, r, x, y);
    if ret_val as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
        return ret_val;
    }
    if !(*r).bias.is_null() {
        b_add = *(*r).bias.offset(data as isize) / (*r).w_start as libc::c_float
    }
    *(*r).en.offset(data as isize) =
        (*r).nrg.expect("non-null function pointer")(x, y, (*r).w, (*r).h, (*r).rwindow, (*r).nrg_extra_data) + b_add;
    return LQR_OK;
}
/* compute auxiliary minpath map
 * defined as
 *   y = 1 : m(x,y) = e(x,y)
 *   y > 1 : m(x,y) = min_{x'=-dx,..,dx} ( m(x-x',y-1) + rig(x') ) + e(x,y)
 * where
 *   e(x,y)  is the energy at point (x,y)
 *   dx      is the max seam step delta_x
 *   rig(x') is the rigidity for step x'
 */

pub unsafe extern "C" fn lqr_carver_build_mmap(mut r: *mut LqrCarver) -> LqrRetVal {
    let mut data: libc::c_int = 0;
    let mut data_down: libc::c_int = 0;
    let mut x1_min: libc::c_int = 0;
    let mut x1_max: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut m: libc::c_float = 0.;
    let mut m1: libc::c_float = 0.;
    let mut r_fact: libc::c_float = 0.;
    if ({
        let mut gaig_temp: libc::c_int = 0;

        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
    {
        return LQR_USRCANCEL;
    }
    let mut x: libc::c_int = 0 as libc::c_int;
    while x < (*r).w {
        data = *(*(*r).raw.offset(0 as libc::c_int as isize)).offset(x as isize);
        /* __LQR_DEBUG__ */
        *(*r).m.offset(data as isize) = *(*r).en.offset(data as isize);
        x += 1
    }
    let mut y: libc::c_int = 1 as libc::c_int;
    while y < (*r).h {
        x = 0 as libc::c_int;
        while x < (*r).w {
            if ({
                let mut gaig_temp: libc::c_int = 0;

                *(&mut gaig_temp as *mut libc::c_int) =
                    ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
                gaig_temp
            }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
            {
                return LQR_USRCANCEL;
            }
            data = *(*(*r).raw.offset(y as isize)).offset(x as isize);
            /* __LQR_DEBUG__ */
            /* watch for boundaries */
            x1_min = if -x > -(*r).delta_x { -x } else { -(*r).delta_x };
            x1_max = if (*r).w - 1 as libc::c_int - x < (*r).delta_x {
                ((*r).w - 1 as libc::c_int) - x
            } else {
                (*r).delta_x
            };
            if !(*r).rigidity_mask.is_null() {
                r_fact = *(*r).rigidity_mask.offset(data as isize)
            } else {
                r_fact = 1 as libc::c_int as libc::c_float
            }
            /* we use the data_down pointer to be able to
             * track the seams later (needed for rigidity) */
            data_down = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset((x + x1_min) as isize);
            *(*r).least.offset(data as isize) = data_down;
            if (*r).rigidity != 0. {
                m = *(*r).m.offset(data_down as isize) + r_fact * *(*r).rigidity_map.offset(x1_min as isize);
                x1 = x1_min + 1 as libc::c_int;
                while x1 <= x1_max {
                    data_down = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset((x + x1) as isize);
                    /* m = MIN(m, r->m[data_down] + r->rigidity_map[x1]); */
                    m1 = *(*r).m.offset(data_down as isize) + r_fact * *(*r).rigidity_map.offset(x1 as isize);
                    if m1 < m || m1 == m && (*r).leftright == 1 as libc::c_int {
                        m = m1;
                        *(*r).least.offset(data as isize) = data_down
                    }
                    x1 += 1
                }
            } else {
                m = *(*r).m.offset(data_down as isize);
                x1 = x1_min + 1 as libc::c_int;
                while x1 <= x1_max {
                    data_down = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset((x + x1) as isize);
                    /* find the min among the neighbors
                     * in the last row */
                    /* find the min among the neighbors
                     * in the last row */
                    m1 = *(*r).m.offset(data_down as isize);
                    if m1 < m || m1 == m && (*r).leftright == 1 as libc::c_int {
                        m = m1;
                        *(*r).least.offset(data as isize) = data_down
                    }
                    m = if m < *(*r).m.offset(data_down as isize) {
                        m
                    } else {
                        *(*r).m.offset(data_down as isize)
                    };
                    x1 += 1
                }
            }
            /* set current m */
            *(*r).m.offset(data as isize) = *(*r).en.offset(data as isize) + m;
            x += 1
        }
        y += 1
    }
    return LQR_OK;
}
/* compute (vertical) visibility map up to given depth
 * (it also calls inflate() to add image enlargment information) */

pub unsafe extern "C" fn lqr_carver_build_vsmap(mut r: *mut LqrCarver, mut depth: libc::c_int) -> LqrRetVal {
    let mut lr_switch_interval: libc::c_int = 0 as libc::c_int;
    let mut data_tok: LqrDataTok = _LqrDataTok {
        carver: 0 as *mut LqrCarver,
    };
    /* __LQR_VERBOSE__ */
    /* __LQR_DEBUG__ */
    /* default behaviour : compute all possible levels
     * (complete map) */
    if depth == 0 as libc::c_int {
        depth = (*r).w_start + 1 as libc::c_int
    }
    /* here we assume that
     * lqr_carver_set_width(w_start - max_level + 1);
     * has been given */
    /* left-right switch interval */
    if (*r).lr_switch_frequency != 0 {
        lr_switch_interval = (depth - (*r).max_level - 1 as libc::c_int) / (*r).lr_switch_frequency + 1 as libc::c_int
    }
    let mut l: libc::c_int = (*r).max_level;
    while l < depth {
        if ({
            let mut gaig_temp: libc::c_int = 0;

            *(&mut gaig_temp as *mut libc::c_int) =
                ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
            gaig_temp
        }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
        {
            return LQR_USRCANCEL;
        }
        if (l - (*r).max_level + (*r).session_rescale_current) % (*r).session_update_step == 0 as libc::c_int {
            crate::lqr_progress::lqr_progress_update(
                (*r).progress,
                (l - (*r).max_level + (*r).session_rescale_current) as libc::c_double
                    / (*r).session_rescale_total as libc::c_double,
            );
        }
        /* __LQR_DEBUG__ */
        /* compute vertical seam */
        lqr_carver_build_vpath(r);
        /* update visibility map
         * (assign level to the seam) */
        lqr_carver_update_vsmap(r, l + (*r).max_level - 1 as libc::c_int);
        /* increase (in)visibility level
         * (make the last seam invisible) */
        (*r).level += 1;
        (*r).w -= 1;
        /* update raw data */
        lqr_carver_carve(r);
        if (*r).w > 1 as libc::c_int {
            /* update the energy */
            /* LQR_CATCH (lqr_carver_build_emap (r)); */
            let mut ret_val: LqrRetVal = LQR_ERROR;
            ret_val = lqr_carver_update_emap(r);
            if ret_val as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val;
            }
            /* recalculate the minpath map */
            if (*r).lr_switch_frequency != 0
                && (l - (*r).max_level + lr_switch_interval / 2 as libc::c_int) % lr_switch_interval == 0 as libc::c_int
            {
                (*r).leftright ^= 1 as libc::c_int;
                let mut ret_val_0: LqrRetVal = LQR_ERROR;
                ret_val_0 = lqr_carver_build_mmap(r);
                if ret_val_0 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                    return ret_val_0;
                }
            } else {
                /* lqr_carver_build_mmap (r); */
                let mut ret_val_1: LqrRetVal = LQR_ERROR;
                ret_val_1 = lqr_carver_update_mmap(r);
                if ret_val_1 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                    return ret_val_1;
                }
            }
        } else {
            /* complete the map (last seam) */
            lqr_carver_finish_vsmap(r);
        }
        l += 1
    }
    /* insert seams for image enlargement */
    let mut ret_val_2: LqrRetVal = LQR_ERROR;
    ret_val_2 = lqr_carver_inflate(r, depth - 1 as libc::c_int);
    if ret_val_2 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
        return ret_val_2;
    }
    /* reset image size */
    lqr_carver_set_width(r, (*r).w_start);
    /* repeat for auxiliary layers */
    data_tok.integer = (*r).w_start;
    let mut ret_val_3: LqrRetVal = LQR_ERROR;
    ret_val_3 = crate::lqr_carver_list::lqr_carver_list_foreach_recursive(
        (*r).attached_list,
        Some(lqr_carver_set_width_attached as unsafe extern "C" fn(_: *mut LqrCarver, _: LqrDataTok) -> LqrRetVal),
        data_tok,
    );
    if ret_val_3 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
        return ret_val_3;
    }
    /* __LQR_VERBOSE__ */
    return LQR_OK;
}
/* enlarge the image by seam insertion
 * visibility map is updated and the resulting multisize image
 * is complete in both directions */

pub unsafe extern "C" fn lqr_carver_inflate(mut r: *mut LqrCarver, mut l: libc::c_int) -> LqrRetVal {
    let mut vs: libc::c_int = 0;
    let mut k: libc::c_int = 0;

    let mut c_left: libc::c_int = 0;
    let mut new_rgb: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut new_vs: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp_rgb: libc::c_double = 0.;
    let mut new_bias: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut new_rigmask: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut data_tok: LqrDataTok = _LqrDataTok {
        carver: 0 as *mut LqrCarver,
    };
    let mut prev_state: LqrCarverState = LQR_CARVER_STATE_STD;
    /* __LQR_VERBOSE__ */
    /* __LQR_DEBUG__ */
    if ({
        let mut gaig_temp: libc::c_int = 0;

        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
    {
        return LQR_USRCANCEL;
    }
    if (*r).root.is_null() {
        prev_state = ({
            let mut gaig_temp: libc::c_int = 0;

            *&mut gaig_temp =
                ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
            gaig_temp
        }) as LqrCarverState;
        let mut ret_val: LqrRetVal = LQR_ERROR;
        ret_val = lqr_carver_set_state(r, LQR_CARVER_STATE_INFLATING, (0 as libc::c_int == 0) as libc::c_int);
        if ret_val as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val;
        }
    }
    /* first iterate on attached carvers */
    data_tok.integer = l;
    let mut ret_val_0: LqrRetVal = LQR_ERROR;
    ret_val_0 = crate::lqr_carver_list::lqr_carver_list_foreach(
        (*r).attached_list,
        Some(lqr_carver_inflate_attached as unsafe extern "C" fn(_: *mut LqrCarver, _: LqrDataTok) -> LqrRetVal),
        data_tok,
    );
    if ret_val_0 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
        return ret_val_0;
    }
    /* scale to current maximum size
     * (this is the original size the first time) */
    lqr_carver_set_width(r, (*r).w0);
    let mut w1: libc::c_int = (*r).w0 + l - (*r).max_level + 1 as libc::c_int;
    /* allocate room for new maps */
    match (*r).col_depth as libc::c_uint {
        0 => {
            new_rgb = ({
                let mut __n: libc::c_ulong = (w1 * (*r).h0 * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_uchar as *mut libc::c_void;
            if new_rgb.is_null() {
                return LQR_NOMEM;
            }
        },
        1 => {
            new_rgb = ({
                let mut __n: libc::c_ulong = (w1 * (*r).h0 * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_ushort as *mut libc::c_void;
            if new_rgb.is_null() {
                return LQR_NOMEM;
            }
        },
        2 => {
            new_rgb = ({
                let mut __n: libc::c_ulong = (w1 * (*r).h0 * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_float as *mut libc::c_void;
            if new_rgb.is_null() {
                return LQR_NOMEM;
            }
        },
        3 => {
            new_rgb = ({
                let mut __n: libc::c_ulong = (w1 * (*r).h0 * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_double>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_double as *mut libc::c_void;
            if new_rgb.is_null() {
                return LQR_NOMEM;
            }
        },
        _ => {},
    }
    if (*r).root.is_null() {
        new_vs = ({
            let mut __n: libc::c_ulong = (w1 * (*r).h0) as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc0(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc0(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_int;
        if new_vs.is_null() {
            return LQR_NOMEM;
        }
    }
    if (*r).active != 0 {
        if !(*r).bias.is_null() {
            new_bias = ({
                let mut __n: libc::c_ulong = (w1 * (*r).h0) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_float;
            if new_bias.is_null() {
                return LQR_NOMEM;
            }
        }
        if !(*r).rigidity_mask.is_null() {
            new_rigmask = ({
                let mut __n: libc::c_ulong = (w1 * (*r).h0) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_float;
            if new_rigmask.is_null() {
                return LQR_NOMEM;
            }
        }
    }
    /* span the image with a cursor
     * and build the new image */
    crate::lqr_cursor::lqr_cursor_reset((*r).c);

    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut z0: libc::c_int = 0 as libc::c_int;
    while z0 < w1 * (*r).h0 {
        if ({
            let mut gaig_temp: libc::c_int = 0;

            *(&mut gaig_temp as *mut libc::c_int) =
                ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
            gaig_temp
        }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
        {
            return LQR_USRCANCEL;
        }
        /* read visibility */
        vs = *(*r).vs.offset((*(*r).c).now as isize);
        if vs != 0 as libc::c_int
            && vs <= l + (*r).max_level - 1 as libc::c_int
            && vs >= 2 as libc::c_int * (*r).max_level - 1 as libc::c_int
        {
            /* the point belongs to a previously computed seam
             * and was not inserted during a previous
             * inflate() call : insert another seam */
            /* the new pixel value is equal to the average of its
             * left and right neighbors */
            if (*(*r).c).x > 0 as libc::c_int {
                c_left = crate::lqr_cursor::lqr_cursor_left((*r).c)
            } else {
                c_left = (*(*r).c).now
            }
            k = 0 as libc::c_int;
            while k < (*r).channels {
                match (*r).col_depth as libc::c_uint {
                    0 => {
                        tmp_rgb = ((*((*r).rgb as *mut libc::c_uchar).offset((c_left * (*r).channels + k) as isize)
                            as libc::c_int
                            + *((*r).rgb as *mut libc::c_uchar).offset(((*(*r).c).now * (*r).channels + k) as isize)
                                as libc::c_int)
                            / 2 as libc::c_int) as libc::c_double;
                        *(new_rgb as *mut libc::c_uchar).offset((z0 * (*r).channels + k) as isize) =
                            (tmp_rgb + 0.499999f64) as libc::c_uchar
                    },
                    1 => {
                        tmp_rgb = ((*((*r).rgb as *mut libc::c_ushort).offset((c_left * (*r).channels + k) as isize)
                            as libc::c_int
                            + *((*r).rgb as *mut libc::c_ushort).offset(((*(*r).c).now * (*r).channels + k) as isize)
                                as libc::c_int)
                            / 2 as libc::c_int) as libc::c_double;
                        *(new_rgb as *mut libc::c_ushort).offset((z0 * (*r).channels + k) as isize) =
                            (tmp_rgb + 0.499999f64) as libc::c_ushort
                    },
                    2 => {
                        tmp_rgb = ((*((*r).rgb as *mut libc::c_float).offset((c_left * (*r).channels + k) as isize)
                            + *((*r).rgb as *mut libc::c_float).offset(((*(*r).c).now * (*r).channels + k) as isize))
                            / 2 as libc::c_int as libc::c_float) as libc::c_double;
                        *(new_rgb as *mut libc::c_float).offset((z0 * (*r).channels + k) as isize) =
                            tmp_rgb as libc::c_float
                    },
                    3 => {
                        tmp_rgb = (*((*r).rgb as *mut libc::c_double).offset((c_left * (*r).channels + k) as isize)
                            + *((*r).rgb as *mut libc::c_double).offset(((*(*r).c).now * (*r).channels + k) as isize))
                            / 2 as libc::c_int as libc::c_double;
                        *(new_rgb as *mut libc::c_double).offset((z0 * (*r).channels + k) as isize) = tmp_rgb
                    },
                    _ => {},
                }
                k += 1
            }
            if (*r).active != 0 {
                if !(*r).bias.is_null() {
                    *new_bias.offset(z0 as isize) = (*(*r).bias.offset(c_left as isize)
                        + *(*r).bias.offset((*(*r).c).now as isize))
                        / 2 as libc::c_int as libc::c_float
                }
                if !(*r).rigidity_mask.is_null() {
                    *new_rigmask.offset(z0 as isize) = (*(*r).rigidity_mask.offset(c_left as isize)
                        + *(*r).rigidity_mask.offset((*(*r).c).now as isize))
                        / 2 as libc::c_int as libc::c_float
                }
            }
            /* the first time inflate() is called
             * the new visibility should be -vs + 1 but we shift it
             * so that the final minimum visibiliy will be 1 again
             * and so that vs=0 still means "uninitialized".
             * Subsequent inflations account for that */
            if (*r).root.is_null() {
                *new_vs.offset(z0 as isize) = l - vs + (*r).max_level
            }
            z0 += 1
        }
        k = 0 as libc::c_int;
        while k < (*r).channels {
            match (*r).col_depth as libc::c_uint {
                0 => {
                    *(new_rgb as *mut libc::c_uchar).offset((z0 * (*r).channels + k) as isize) =
                        *((*r).rgb as *mut libc::c_uchar).offset(((*(*r).c).now * (*r).channels + k) as isize)
                },
                1 => {
                    *(new_rgb as *mut libc::c_ushort).offset((z0 * (*r).channels + k) as isize) =
                        *((*r).rgb as *mut libc::c_ushort).offset(((*(*r).c).now * (*r).channels + k) as isize)
                },
                2 => {
                    *(new_rgb as *mut libc::c_float).offset((z0 * (*r).channels + k) as isize) =
                        *((*r).rgb as *mut libc::c_float).offset(((*(*r).c).now * (*r).channels + k) as isize)
                },
                3 => {
                    *(new_rgb as *mut libc::c_double).offset((z0 * (*r).channels + k) as isize) =
                        *((*r).rgb as *mut libc::c_double).offset(((*(*r).c).now * (*r).channels + k) as isize)
                },
                _ => {},
            }
            k += 1
        }
        if (*r).active != 0 {
            if !(*r).bias.is_null() {
                *new_bias.offset(z0 as isize) = *(*r).bias.offset((*(*r).c).now as isize)
            }
            if !(*r).rigidity_mask.is_null() {
                *new_rigmask.offset(z0 as isize) = *(*r).rigidity_mask.offset((*(*r).c).now as isize)
            }
        }
        if vs != 0 as libc::c_int {
            /* visibility has to be shifted up */
            if (*r).root.is_null() {
                *new_vs.offset(z0 as isize) = vs + l - (*r).max_level + 1 as libc::c_int
            }
        } else if !(*r).raw.is_null() {
            /* __LQR_DEBUG__ */
            *(*(*r).raw.offset(y as isize)).offset(x as isize) = z0;
            x += 1;
            if x >= (*r).w_start - l {
                x = 0 as libc::c_int;
                y += 1
            }
        }
        z0 += 1;
        crate::lqr_cursor::lqr_cursor_next((*r).c);
    }
    /* __LQR_DEBUG__ */
    /* substitute maps */
    if (*r).preserve_in_buffer == 0 {
        libc::free((*r).rgb);
    }
    /* libc::free (r->vs); */
    libc::free((*r).en as *mut libc::c_void);
    libc::free((*r).m as *mut libc::c_void);
    libc::free((*r).rcache as *mut libc::c_void);
    libc::free((*r).least as *mut libc::c_void);
    libc::free((*r).bias as *mut libc::c_void);
    libc::free((*r).rigidity_mask as *mut libc::c_void);
    (*r).bias = 0 as *mut libc::c_float;
    (*r).rcache = 0 as *mut libc::c_double;
    (*r).nrg_uptodate = 0 as libc::c_int;
    (*r).rgb = new_rgb;
    (*r).preserve_in_buffer = 0 as libc::c_int;
    if (*r).root.is_null() {
        libc::free((*r).vs as *mut libc::c_void);
        (*r).vs = new_vs;
        let mut ret_val_1: LqrRetVal = LQR_ERROR;
        ret_val_1 = lqr_carver_propagate_vsmap(r);
        if ret_val_1 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val_1;
        }
    }
    if (*r).nrg_active != 0 {
        (*r).en = ({
            let mut __n: libc::c_ulong = (w1 * (*r).h0) as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc0(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc0(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_float;
        if (*r).en.is_null() {
            return LQR_NOMEM;
        }
    }
    if (*r).active != 0 {
        (*r).bias = new_bias;
        (*r).rigidity_mask = new_rigmask;
        (*r).m = ({
            let mut __n: libc::c_ulong = (w1 * (*r).h0) as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc0(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc0(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_float;
        if (*r).m.is_null() {
            return LQR_NOMEM;
        }
        (*r).least = ({
            let mut __n: libc::c_ulong = (w1 * (*r).h0) as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc0(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc0(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_int;
        if (*r).least.is_null() {
            return LQR_NOMEM;
        }
    }
    /* set new widths & levels (w_start is kept for reference) */
    (*r).level = l + 1 as libc::c_int;
    (*r).max_level = l + 1 as libc::c_int;
    (*r).w0 = w1;
    (*r).w = (*r).w_start;
    /* reset readout buffer */
    libc::free((*r).rgb_ro_buffer);
    match (*r).col_depth as libc::c_uint {
        0 => {
            (*r).rgb_ro_buffer = ({
                let mut __n: libc::c_ulong = ((*r).w0 * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_uchar as *mut libc::c_void;
            if (*r).rgb_ro_buffer.is_null() {
                return LQR_NOMEM;
            }
        },
        1 => {
            (*r).rgb_ro_buffer = ({
                let mut __n: libc::c_ulong = ((*r).w0 * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_ushort as *mut libc::c_void;
            if (*r).rgb_ro_buffer.is_null() {
                return LQR_NOMEM;
            }
        },
        2 => {
            (*r).rgb_ro_buffer = ({
                let mut __n: libc::c_ulong = ((*r).w0 * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_float as *mut libc::c_void;
            if (*r).rgb_ro_buffer.is_null() {
                return LQR_NOMEM;
            }
        },
        3 => {
            (*r).rgb_ro_buffer = ({
                let mut __n: libc::c_ulong = ((*r).w0 * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_double>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_double as *mut libc::c_void;
            if (*r).rgb_ro_buffer.is_null() {
                return LQR_NOMEM;
            }
        },
        _ => {},
    }
    /* __LQR_VERBOSE__ */
    if (*r).root.is_null() {
        let mut ret_val_2: LqrRetVal = LQR_ERROR;
        ret_val_2 = lqr_carver_set_state(r, prev_state, (0 as libc::c_int == 0) as libc::c_int);
        if ret_val_2 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val_2;
        }
    }
    return LQR_OK;
}

pub unsafe extern "C" fn lqr_carver_inflate_attached(mut r: *mut LqrCarver, mut data: LqrDataTok) -> LqrRetVal {
    return lqr_carver_inflate(r, data.integer);
}
/* ** internal functions for maps computations ** */
/* do the carving
 * this actually carves the raw array,
 * which holds the indices to be used
 * in all the other maps */

pub unsafe extern "C" fn lqr_carver_carve(mut r: *mut LqrCarver) {
    let mut _x: libc::c_int = 0;

    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*r).h_start {
        for x in *(*r).vpath_x.offset(y as isize)..(*r).w {
            *(*(*r).raw.offset(y as isize)).offset(x as isize) =
                *(*(*r).raw.offset(y as isize)).offset((x + 1 as libc::c_int) as isize);
        }
        y += 1
    }
    (*r).nrg_uptodate = 0 as libc::c_int;
}
/* update energy map after seam removal */

pub unsafe extern "C" fn lqr_carver_update_emap(mut r: *mut LqrCarver) -> LqrRetVal {
    let mut x: libc::c_int = 0;

    let mut _y1: libc::c_int = 0;
    let mut y1_min: libc::c_int = 0;
    let mut y1_max: libc::c_int = 0;
    if ({
        let mut gaig_temp: libc::c_int = 0;

        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
    {
        return LQR_USRCANCEL;
    }
    if (*r).nrg_uptodate != 0 {
        return LQR_OK;
    }
    if (*r).use_rcache != 0 {
        if ((*r).rcache != 0 as *mut libc::c_void as *mut libc::c_double) as libc::c_int == 0 as libc::c_int {
            return LQR_ERROR;
        }
    }
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*r).h {
        /* note: here the vpath has already
         * been carved */
        x = *(*r).vpath_x.offset(y as isize);
        *(*r).nrg_xmin.offset(y as isize) = x;
        *(*r).nrg_xmax.offset(y as isize) = x - 1 as libc::c_int;
        y += 1
    }
    y = 0 as libc::c_int;
    while y < (*r).h {
        x = *(*r).vpath_x.offset(y as isize);
        y1_min = if y - (*r).nrg_radius > 0 as libc::c_int {
            (y) - (*r).nrg_radius
        } else {
            0 as libc::c_int
        };
        y1_max = if y + (*r).nrg_radius < (*r).h - 1 as libc::c_int {
            (y) + (*r).nrg_radius
        } else {
            ((*r).h) - 1 as libc::c_int
        };

        for y1 in y1_min..=y1_max {
            *(*r).nrg_xmin.offset(y1 as isize) = if *(*r).nrg_xmin.offset(y1 as isize) < x - (*r).nrg_radius {
                *(*r).nrg_xmin.offset(y1 as isize)
            } else {
                (x) - (*r).nrg_radius
            };

            *(*r).nrg_xmin.offset(y1 as isize) = if 0 as libc::c_int > *(*r).nrg_xmin.offset(y1 as isize) {
                0 as libc::c_int
            } else {
                *(*r).nrg_xmin.offset(y1 as isize)
            };
            /* note: the -1 below is because of the previous carving */
            *(*r).nrg_xmax.offset(y1 as isize) =
                if *(*r).nrg_xmax.offset(y1 as isize) > x + (*r).nrg_radius - 1 as libc::c_int {
                    *(*r).nrg_xmax.offset(y1 as isize)
                } else {
                    (x + (*r).nrg_radius) - 1 as libc::c_int
                };

            *(*r).nrg_xmax.offset(y1 as isize) = if ((*r).w - 1 as libc::c_int) < *(*r).nrg_xmax.offset(y1 as isize) {
                ((*r).w) - 1 as libc::c_int
            } else {
                *(*r).nrg_xmax.offset(y1 as isize)
            };
        }
        y += 1
    }
    y = 0 as libc::c_int;
    while y < (*r).h {
        if ({
            let mut gaig_temp: libc::c_int = 0;

            *(&mut gaig_temp as *mut libc::c_int) =
                ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
            gaig_temp
        }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
        {
            return LQR_USRCANCEL;
        }
        x = *(*r).nrg_xmin.offset(y as isize);
        while x <= *(*r).nrg_xmax.offset(y as isize) {
            let mut ret_val: LqrRetVal = LQR_ERROR;
            ret_val = lqr_carver_compute_e(r, x, y);
            if ret_val as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val;
            }
            x += 1
        }
        y += 1
    }
    (*r).nrg_uptodate = (0 as libc::c_int == 0) as libc::c_int;
    return LQR_OK;
}
/* update the auxiliary minpath map
 * this only updates the affected pixels,
 * which start form the beginning of the changed
 * energy region around the seam and expand
 * at most by delta_x (in both directions)
 * at each row */

pub unsafe extern "C" fn lqr_carver_update_mmap(mut r: *mut LqrCarver) -> LqrRetVal {
    let mut x1: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut x1_min: libc::c_int = 0;
    let mut x1_max: libc::c_int = 0;
    let mut data: libc::c_int = 0;
    let mut data_down: libc::c_int = 0;
    let mut least: libc::c_int = 0;
    let mut m: libc::c_float = 0.;
    let mut m1: libc::c_float = 0.;
    let mut r_fact: libc::c_float = 0.;
    let mut new_m: libc::c_float = 0.;
    let mut mc: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut stop: libc::c_int = 0;
    let mut x_stop: libc::c_int = 0;
    if ({
        let mut gaig_temp: libc::c_int = 0;

        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
    {
        return LQR_USRCANCEL;
    }
    if (*r).nrg_uptodate == 0 as libc::c_int {
        return LQR_ERROR;
    }
    if (*r).rigidity != 0. {
        mc = ({
            let mut __n: libc::c_ulong = (2 as libc::c_int * (*r).delta_x + 1 as libc::c_int) as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_float;
        if mc.is_null() {
            return LQR_NOMEM;
        }
        mc = mc.offset((*r).delta_x as isize)
    }

    let mut x_min: libc::c_int = if *(*r).nrg_xmin.offset(0 as libc::c_int as isize) > 0 as libc::c_int {
        *(*r).nrg_xmin.offset(0 as libc::c_int as isize)
    } else {
        0 as libc::c_int
    };
    let mut x_max: libc::c_int = if *(*r).nrg_xmax.offset(0 as libc::c_int as isize) < (*r).w - 1 as libc::c_int {
        *(*r).nrg_xmax.offset(0 as libc::c_int as isize)
    } else {
        ((*r).w) - 1 as libc::c_int
    };
    let mut x: libc::c_int = x_min;
    while x <= x_max {
        data = *(*(*r).raw.offset(0 as libc::c_int as isize)).offset(x as isize);
        *(*r).m.offset(data as isize) = *(*r).en.offset(data as isize);
        x += 1
    }
    let mut y: libc::c_int = 1 as libc::c_int;
    while y < (*r).h {
        if ({
            let mut gaig_temp: libc::c_int = 0;

            *(&mut gaig_temp as *mut libc::c_int) =
                ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
            gaig_temp
        }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
        {
            return LQR_USRCANCEL;
        }
        /* make sure to include the changed energy region */
        x_min = if x_min < *(*r).nrg_xmin.offset(y as isize) {
            x_min
        } else {
            *(*r).nrg_xmin.offset(y as isize)
        };
        x_max = if x_max > *(*r).nrg_xmax.offset(y as isize) {
            x_max
        } else {
            *(*r).nrg_xmax.offset(y as isize)
        };
        /* expand the affected region by delta_x */
        x_min = if x_min - (*r).delta_x > 0 as libc::c_int {
            (x_min) - (*r).delta_x
        } else {
            0 as libc::c_int
        };
        x_max = if x_max + (*r).delta_x < (*r).w - 1 as libc::c_int {
            (x_max) + (*r).delta_x
        } else {
            ((*r).w) - 1 as libc::c_int
        };
        /* span the affected region */
        stop = 0 as libc::c_int;
        x_stop = 0 as libc::c_int;
        x = x_min;
        while x <= x_max {
            data = *(*(*r).raw.offset(y as isize)).offset(x as isize);
            if !(*r).rigidity_mask.is_null() {
                r_fact = *(*r).rigidity_mask.offset(data as isize)
            } else {
                r_fact = 1 as libc::c_int as libc::c_float
            }
            /* find the minimum in the previous rows
             * as in build_mmap() */
            x1_min = if 0 as libc::c_int > x - (*r).delta_x {
                0 as libc::c_int
            } else {
                (x) - (*r).delta_x
            };
            x1_max = if ((*r).w - 1 as libc::c_int) < x + (*r).delta_x {
                ((*r).w) - 1 as libc::c_int
            } else {
                (x) + (*r).delta_x
            };
            if (*r).rigidity != 0. {
                dx = x1_min - x;
                match x1_max - x1_min + 1 as libc::c_int {
                    1 => {
                        *mc.offset(dx as isize) = *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                        ) + r_fact * *(*r).rigidity_map.offset(dx as isize);
                        m = if (*r).leftright != 0 {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize);
                            *mc.offset(dx as isize)
                        } else {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize);
                            *mc.offset(dx as isize)
                        }
                    },
                    2 => {
                        *mc.offset(dx as isize) = *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                        ) + r_fact * *(*r).rigidity_map.offset(dx as isize);
                        *mc.offset((dx + 1 as libc::c_int) as isize) = *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 1 as libc::c_int) as isize) as isize,
                        ) + r_fact
                            * *(*r).rigidity_map.offset((dx + 1 as libc::c_int) as isize);
                        m = if (*r).leftright != 0 {
                            if *mc.offset(dx as isize) < *mc.offset((dx + 1 as libc::c_int) as isize) {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize);
                                *mc.offset(dx as isize)
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 1 as libc::c_int) as isize);
                                *mc.offset((dx + 1 as libc::c_int) as isize)
                            }
                        } else if *mc.offset(dx as isize) <= *mc.offset((dx + 1 as libc::c_int) as isize) {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize);
                            *mc.offset(dx as isize)
                        } else {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 1 as libc::c_int) as isize);
                            *mc.offset((dx + 1 as libc::c_int) as isize)
                        }
                    },
                    3 => {
                        *mc.offset(dx as isize) = *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                        ) + r_fact * *(*r).rigidity_map.offset(dx as isize);
                        *mc.offset((dx + 1 as libc::c_int) as isize) = *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 1 as libc::c_int) as isize) as isize,
                        ) + r_fact
                            * *(*r).rigidity_map.offset((dx + 1 as libc::c_int) as isize);
                        *mc.offset((dx + 2 as libc::c_int) as isize) = *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 2 as libc::c_int) as isize) as isize,
                        ) + r_fact
                            * *(*r).rigidity_map.offset((dx + 2 as libc::c_int) as isize);
                        m = if (*r).leftright != 0 {
                            if *mc.offset(dx as isize) < *mc.offset((dx + 1 as libc::c_int) as isize) {
                                if *mc.offset(dx as isize) < *mc.offset((dx + 2 as libc::c_int) as isize) {
                                    least =
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize);
                                    *mc.offset(dx as isize)
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize);
                                    *mc.offset((dx + 2 as libc::c_int) as isize)
                                }
                            } else if *mc.offset((dx + 1 as libc::c_int) as isize)
                                < *mc.offset((dx + 2 as libc::c_int) as isize)
                            {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 1 as libc::c_int) as isize);
                                *mc.offset((dx + 1 as libc::c_int) as isize)
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize);
                                *mc.offset((dx + 2 as libc::c_int) as isize)
                            }
                        } else if *mc.offset(dx as isize) <= *mc.offset((dx + 1 as libc::c_int) as isize) {
                            if *mc.offset(dx as isize) <= *mc.offset((dx + 2 as libc::c_int) as isize) {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize);
                                *mc.offset(dx as isize)
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize);
                                *mc.offset((dx + 2 as libc::c_int) as isize)
                            }
                        } else if *mc.offset((dx + 1 as libc::c_int) as isize)
                            <= *mc.offset((dx + 2 as libc::c_int) as isize)
                        {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 1 as libc::c_int) as isize);
                            *mc.offset((dx + 1 as libc::c_int) as isize)
                        } else {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 2 as libc::c_int) as isize);
                            *mc.offset((dx + 2 as libc::c_int) as isize)
                        }
                    },
                    4 => {
                        *mc.offset(dx as isize) = *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                        ) + r_fact * *(*r).rigidity_map.offset(dx as isize);
                        *mc.offset((dx + 1 as libc::c_int) as isize) = *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 1 as libc::c_int) as isize) as isize,
                        ) + r_fact
                            * *(*r).rigidity_map.offset((dx + 1 as libc::c_int) as isize);
                        *mc.offset((dx + 2 as libc::c_int) as isize) = *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 2 as libc::c_int) as isize) as isize,
                        ) + r_fact
                            * *(*r).rigidity_map.offset((dx + 2 as libc::c_int) as isize);
                        *mc.offset((dx + 3 as libc::c_int) as isize) = *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 3 as libc::c_int) as isize) as isize,
                        ) + r_fact
                            * *(*r).rigidity_map.offset((dx + 3 as libc::c_int) as isize);
                        m = if (*r).leftright != 0 {
                            if *mc.offset(dx as isize) < *mc.offset((dx + 1 as libc::c_int) as isize) {
                                if *mc.offset(dx as isize) < *mc.offset((dx + 2 as libc::c_int) as isize) {
                                    if *mc.offset(dx as isize) < *mc.offset((dx + 3 as libc::c_int) as isize) {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset(x1_min as isize);
                                        *mc.offset(dx as isize)
                                    } else {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 3 as libc::c_int) as isize);
                                        *mc.offset((dx + 3 as libc::c_int) as isize)
                                    }
                                } else if *mc.offset((dx + 2 as libc::c_int) as isize)
                                    < *mc.offset((dx + 3 as libc::c_int) as isize)
                                {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize);
                                    *mc.offset((dx + 2 as libc::c_int) as isize)
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize);
                                    *mc.offset((dx + 3 as libc::c_int) as isize)
                                }
                            } else if *mc.offset((dx + 1 as libc::c_int) as isize)
                                < *mc.offset((dx + 2 as libc::c_int) as isize)
                            {
                                if *mc.offset((dx + 1 as libc::c_int) as isize)
                                    < *mc.offset((dx + 3 as libc::c_int) as isize)
                                {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 1 as libc::c_int) as isize);
                                    *mc.offset((dx + 1 as libc::c_int) as isize)
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize);
                                    *mc.offset((dx + 3 as libc::c_int) as isize)
                                }
                            } else if *mc.offset((dx + 2 as libc::c_int) as isize)
                                < *mc.offset((dx + 3 as libc::c_int) as isize)
                            {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize);
                                *mc.offset((dx + 2 as libc::c_int) as isize)
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize);
                                *mc.offset((dx + 3 as libc::c_int) as isize)
                            }
                        } else if *mc.offset(dx as isize) <= *mc.offset((dx + 1 as libc::c_int) as isize) {
                            if *mc.offset(dx as isize) <= *mc.offset((dx + 2 as libc::c_int) as isize) {
                                if *mc.offset(dx as isize) <= *mc.offset((dx + 3 as libc::c_int) as isize) {
                                    least =
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize);
                                    *mc.offset(dx as isize)
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize);
                                    *mc.offset((dx + 3 as libc::c_int) as isize)
                                }
                            } else if *mc.offset((dx + 2 as libc::c_int) as isize)
                                <= *mc.offset((dx + 3 as libc::c_int) as isize)
                            {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize);
                                *mc.offset((dx + 2 as libc::c_int) as isize)
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize);
                                *mc.offset((dx + 3 as libc::c_int) as isize)
                            }
                        } else if *mc.offset((dx + 1 as libc::c_int) as isize)
                            <= *mc.offset((dx + 2 as libc::c_int) as isize)
                        {
                            if *mc.offset((dx + 1 as libc::c_int) as isize)
                                <= *mc.offset((dx + 3 as libc::c_int) as isize)
                            {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 1 as libc::c_int) as isize);
                                *mc.offset((dx + 1 as libc::c_int) as isize)
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize);
                                *mc.offset((dx + 3 as libc::c_int) as isize)
                            }
                        } else if *mc.offset((dx + 2 as libc::c_int) as isize)
                            <= *mc.offset((dx + 3 as libc::c_int) as isize)
                        {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 2 as libc::c_int) as isize);
                            *mc.offset((dx + 2 as libc::c_int) as isize)
                        } else {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 3 as libc::c_int) as isize);
                            *mc.offset((dx + 3 as libc::c_int) as isize)
                        }
                    },
                    5 => {
                        *mc.offset(dx as isize) = *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                        ) + r_fact * *(*r).rigidity_map.offset(dx as isize);
                        *mc.offset((dx + 1 as libc::c_int) as isize) = *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 1 as libc::c_int) as isize) as isize,
                        ) + r_fact
                            * *(*r).rigidity_map.offset((dx + 1 as libc::c_int) as isize);
                        *mc.offset((dx + 2 as libc::c_int) as isize) = *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 2 as libc::c_int) as isize) as isize,
                        ) + r_fact
                            * *(*r).rigidity_map.offset((dx + 2 as libc::c_int) as isize);
                        *mc.offset((dx + 3 as libc::c_int) as isize) = *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 3 as libc::c_int) as isize) as isize,
                        ) + r_fact
                            * *(*r).rigidity_map.offset((dx + 3 as libc::c_int) as isize);
                        *mc.offset((dx + 4 as libc::c_int) as isize) = *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 4 as libc::c_int) as isize) as isize,
                        ) + r_fact
                            * *(*r).rigidity_map.offset((dx + 4 as libc::c_int) as isize);
                        m = if (*r).leftright != 0 {
                            if *mc.offset(dx as isize) < *mc.offset((dx + 1 as libc::c_int) as isize) {
                                if *mc.offset(dx as isize) < *mc.offset((dx + 2 as libc::c_int) as isize) {
                                    if *mc.offset(dx as isize) < *mc.offset((dx + 3 as libc::c_int) as isize) {
                                        if *mc.offset(dx as isize) < *mc.offset((dx + 4 as libc::c_int) as isize) {
                                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                                .offset(x1_min as isize);
                                            *mc.offset(dx as isize)
                                        } else {
                                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                                .offset((x1_min + 4 as libc::c_int) as isize);
                                            *mc.offset((dx + 4 as libc::c_int) as isize)
                                        }
                                    } else if *mc.offset((dx + 3 as libc::c_int) as isize)
                                        < *mc.offset((dx + 4 as libc::c_int) as isize)
                                    {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 3 as libc::c_int) as isize);
                                        *mc.offset((dx + 3 as libc::c_int) as isize)
                                    } else {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize);
                                        *mc.offset((dx + 4 as libc::c_int) as isize)
                                    }
                                } else if *mc.offset((dx + 2 as libc::c_int) as isize)
                                    < *mc.offset((dx + 3 as libc::c_int) as isize)
                                {
                                    if *mc.offset((dx + 2 as libc::c_int) as isize)
                                        < *mc.offset((dx + 4 as libc::c_int) as isize)
                                    {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 2 as libc::c_int) as isize);
                                        *mc.offset((dx + 2 as libc::c_int) as isize)
                                    } else {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize);
                                        *mc.offset((dx + 4 as libc::c_int) as isize)
                                    }
                                } else if *mc.offset((dx + 3 as libc::c_int) as isize)
                                    < *mc.offset((dx + 4 as libc::c_int) as isize)
                                {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize);
                                    *mc.offset((dx + 3 as libc::c_int) as isize)
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize);
                                    *mc.offset((dx + 4 as libc::c_int) as isize)
                                }
                            } else if *mc.offset((dx + 1 as libc::c_int) as isize)
                                < *mc.offset((dx + 2 as libc::c_int) as isize)
                            {
                                if *mc.offset((dx + 1 as libc::c_int) as isize)
                                    < *mc.offset((dx + 3 as libc::c_int) as isize)
                                {
                                    if *mc.offset((dx + 1 as libc::c_int) as isize)
                                        < *mc.offset((dx + 4 as libc::c_int) as isize)
                                    {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 1 as libc::c_int) as isize);
                                        *mc.offset((dx + 1 as libc::c_int) as isize)
                                    } else {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize);
                                        *mc.offset((dx + 4 as libc::c_int) as isize)
                                    }
                                } else if *mc.offset((dx + 3 as libc::c_int) as isize)
                                    < *mc.offset((dx + 4 as libc::c_int) as isize)
                                {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize);
                                    *mc.offset((dx + 3 as libc::c_int) as isize)
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize);
                                    *mc.offset((dx + 4 as libc::c_int) as isize)
                                }
                            } else if *mc.offset((dx + 2 as libc::c_int) as isize)
                                < *mc.offset((dx + 3 as libc::c_int) as isize)
                            {
                                if *mc.offset((dx + 2 as libc::c_int) as isize)
                                    < *mc.offset((dx + 4 as libc::c_int) as isize)
                                {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize);
                                    *mc.offset((dx + 2 as libc::c_int) as isize)
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize);
                                    *mc.offset((dx + 4 as libc::c_int) as isize)
                                }
                            } else if *mc.offset((dx + 3 as libc::c_int) as isize)
                                < *mc.offset((dx + 4 as libc::c_int) as isize)
                            {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize);
                                *mc.offset((dx + 3 as libc::c_int) as isize)
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 4 as libc::c_int) as isize);
                                *mc.offset((dx + 4 as libc::c_int) as isize)
                            }
                        } else if *mc.offset(dx as isize) <= *mc.offset((dx + 1 as libc::c_int) as isize) {
                            if *mc.offset(dx as isize) <= *mc.offset((dx + 2 as libc::c_int) as isize) {
                                if *mc.offset(dx as isize) <= *mc.offset((dx + 3 as libc::c_int) as isize) {
                                    if *mc.offset(dx as isize) <= *mc.offset((dx + 4 as libc::c_int) as isize) {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset(x1_min as isize);
                                        *mc.offset(dx as isize)
                                    } else {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize);
                                        *mc.offset((dx + 4 as libc::c_int) as isize)
                                    }
                                } else if *mc.offset((dx + 3 as libc::c_int) as isize)
                                    <= *mc.offset((dx + 4 as libc::c_int) as isize)
                                {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize);
                                    *mc.offset((dx + 3 as libc::c_int) as isize)
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize);
                                    *mc.offset((dx + 4 as libc::c_int) as isize)
                                }
                            } else if *mc.offset((dx + 2 as libc::c_int) as isize)
                                <= *mc.offset((dx + 3 as libc::c_int) as isize)
                            {
                                if *mc.offset((dx + 2 as libc::c_int) as isize)
                                    <= *mc.offset((dx + 4 as libc::c_int) as isize)
                                {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize);
                                    *mc.offset((dx + 2 as libc::c_int) as isize)
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize);
                                    *mc.offset((dx + 4 as libc::c_int) as isize)
                                }
                            } else if *mc.offset((dx + 3 as libc::c_int) as isize)
                                <= *mc.offset((dx + 4 as libc::c_int) as isize)
                            {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize);
                                *mc.offset((dx + 3 as libc::c_int) as isize)
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 4 as libc::c_int) as isize);
                                *mc.offset((dx + 4 as libc::c_int) as isize)
                            }
                        } else if *mc.offset((dx + 1 as libc::c_int) as isize)
                            <= *mc.offset((dx + 2 as libc::c_int) as isize)
                        {
                            if *mc.offset((dx + 1 as libc::c_int) as isize)
                                <= *mc.offset((dx + 3 as libc::c_int) as isize)
                            {
                                if *mc.offset((dx + 1 as libc::c_int) as isize)
                                    <= *mc.offset((dx + 4 as libc::c_int) as isize)
                                {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 1 as libc::c_int) as isize);
                                    *mc.offset((dx + 1 as libc::c_int) as isize)
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize);
                                    *mc.offset((dx + 4 as libc::c_int) as isize)
                                }
                            } else if *mc.offset((dx + 3 as libc::c_int) as isize)
                                <= *mc.offset((dx + 4 as libc::c_int) as isize)
                            {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize);
                                *mc.offset((dx + 3 as libc::c_int) as isize)
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 4 as libc::c_int) as isize);
                                *mc.offset((dx + 4 as libc::c_int) as isize)
                            }
                        } else if *mc.offset((dx + 2 as libc::c_int) as isize)
                            <= *mc.offset((dx + 3 as libc::c_int) as isize)
                        {
                            if *mc.offset((dx + 2 as libc::c_int) as isize)
                                <= *mc.offset((dx + 4 as libc::c_int) as isize)
                            {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize);
                                *mc.offset((dx + 2 as libc::c_int) as isize)
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 4 as libc::c_int) as isize);
                                *mc.offset((dx + 4 as libc::c_int) as isize)
                            }
                        } else if *mc.offset((dx + 3 as libc::c_int) as isize)
                            <= *mc.offset((dx + 4 as libc::c_int) as isize)
                        {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 3 as libc::c_int) as isize);
                            *mc.offset((dx + 3 as libc::c_int) as isize)
                        } else {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 4 as libc::c_int) as isize);
                            *mc.offset((dx + 4 as libc::c_int) as isize)
                        }
                    },
                    _ => {
                        data_down = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize);
                        least = data_down;
                        let fresh1 = dx;
                        dx = dx + 1;
                        m = *(*r).m.offset(data_down as isize) + r_fact * *(*r).rigidity_map.offset(fresh1 as isize);
                        /* fprintf(stderr, "y,x=%i,%i x1_min,max=%i,%i least=%i m=%g\n", y, x, x1_min, x1_max, least,
                         * m); fflush(stderr); */
                        /* fprintf(stderr, "y,x=%i,%i x1=%i dx=%i mr=%g MR=%g m=%g M=%g\n", y, x, x1_min, dx, m,
                         * MRDOWN(y, x1_min, dx), r->m[data_down], MDOWN(y, x1_min)); fflush(stderr); */
                        x1 = x1_min + 1 as libc::c_int;
                        while x1 <= x1_max {
                            data_down = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1 as isize);
                            m1 = *(*r).m.offset(data_down as isize) + r_fact * *(*r).rigidity_map.offset(dx as isize);
                            /* fprintf(stderr, "y,x=%i,%i x1=%i dx=%i mr=%g MR=%g m=%g M=%g\n", y, x, x1, dx, m1,
                             * MRDOWN(y, x1, dx), r->m[data_down], MDOWN(y, x1)); fflush(stderr); */
                            if m1 < m || m1 == m && (*r).leftright == 1 as libc::c_int {
                                m = m1;
                                least = data_down
                            }
                            x1 += 1;
                            dx += 1
                        }
                    },
                }
            } else {
                match x1_max - x1_min + 1 as libc::c_int {
                    1 => {
                        m = if (*r).leftright != 0 {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize);
                            *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                            )
                        } else {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize);
                            *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                            )
                        }
                    },
                    2 => {
                        m = if (*r).leftright != 0 {
                            if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                            ) < *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 1 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize)
                                        as isize,
                                )
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 1 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 1 as libc::c_int) as isize)
                                        as isize,
                                )
                            }
                        } else if *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                        ) <= *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 1 as libc::c_int) as isize) as isize,
                        ) {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize);
                            *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                            )
                        } else {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 1 as libc::c_int) as isize);
                            *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 1 as libc::c_int) as isize)
                                    as isize,
                            )
                        }
                    },
                    3 => {
                        m = if (*r).leftright != 0 {
                            if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                            ) < *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 1 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                if *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize)
                                        as isize,
                                ) < *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize)
                                        as isize,
                                ) {
                                    least =
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize)
                                            as isize,
                                    )
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 2 as libc::c_int) as isize)
                                            as isize,
                                    )
                                }
                            } else if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 1 as libc::c_int) as isize)
                                    as isize,
                            ) < *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 1 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 1 as libc::c_int) as isize)
                                        as isize,
                                )
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize)
                                        as isize,
                                )
                            }
                        } else if *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                        ) <= *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 1 as libc::c_int) as isize) as isize,
                        ) {
                            if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                            ) <= *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize)
                                        as isize,
                                )
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize)
                                        as isize,
                                )
                            }
                        } else if *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 1 as libc::c_int) as isize) as isize,
                        ) <= *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 2 as libc::c_int) as isize) as isize,
                        ) {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 1 as libc::c_int) as isize);
                            *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 1 as libc::c_int) as isize)
                                    as isize,
                            )
                        } else {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 2 as libc::c_int) as isize);
                            *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize)
                                    as isize,
                            )
                        }
                    },
                    4 => {
                        m = if (*r).leftright != 0 {
                            if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                            ) < *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 1 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                if *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize)
                                        as isize,
                                ) < *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize)
                                        as isize,
                                ) {
                                    if *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize)
                                            as isize,
                                    ) < *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 3 as libc::c_int) as isize)
                                            as isize,
                                    ) {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset(x1_min as isize);
                                        *(*r).m.offset(
                                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize)
                                                as isize,
                                        )
                                    } else {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 3 as libc::c_int) as isize);
                                        *(*r).m.offset(
                                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                                .offset((x1_min + 3 as libc::c_int) as isize)
                                                as isize,
                                        )
                                    }
                                } else if *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize)
                                        as isize,
                                ) < *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize)
                                        as isize,
                                ) {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 2 as libc::c_int) as isize)
                                            as isize,
                                    )
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 3 as libc::c_int) as isize)
                                            as isize,
                                    )
                                }
                            } else if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 1 as libc::c_int) as isize)
                                    as isize,
                            ) < *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                if *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 1 as libc::c_int) as isize)
                                        as isize,
                                ) < *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize)
                                        as isize,
                                ) {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 1 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 1 as libc::c_int) as isize)
                                            as isize,
                                    )
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 3 as libc::c_int) as isize)
                                            as isize,
                                    )
                                }
                            } else if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize)
                                    as isize,
                            ) < *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize)
                                        as isize,
                                )
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize)
                                        as isize,
                                )
                            }
                        } else if *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                        ) <= *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 1 as libc::c_int) as isize) as isize,
                        ) {
                            if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                            ) <= *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                if *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize)
                                        as isize,
                                ) <= *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize)
                                        as isize,
                                ) {
                                    least =
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize)
                                            as isize,
                                    )
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 3 as libc::c_int) as isize)
                                            as isize,
                                    )
                                }
                            } else if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize)
                                    as isize,
                            ) <= *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize)
                                        as isize,
                                )
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize)
                                        as isize,
                                )
                            }
                        } else if *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 1 as libc::c_int) as isize) as isize,
                        ) <= *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 2 as libc::c_int) as isize) as isize,
                        ) {
                            if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 1 as libc::c_int) as isize)
                                    as isize,
                            ) <= *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 1 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 1 as libc::c_int) as isize)
                                        as isize,
                                )
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize)
                                        as isize,
                                )
                            }
                        } else if *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 2 as libc::c_int) as isize) as isize,
                        ) <= *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 3 as libc::c_int) as isize) as isize,
                        ) {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 2 as libc::c_int) as isize);
                            *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize)
                                    as isize,
                            )
                        } else {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 3 as libc::c_int) as isize);
                            *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize)
                                    as isize,
                            )
                        }
                    },
                    5 => {
                        m = if (*r).leftright != 0 {
                            if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                            ) < *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 1 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                if *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize)
                                        as isize,
                                ) < *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize)
                                        as isize,
                                ) {
                                    if *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize)
                                            as isize,
                                    ) < *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 3 as libc::c_int) as isize)
                                            as isize,
                                    ) {
                                        if *(*r).m.offset(
                                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize)
                                                as isize,
                                        ) < *(*r).m.offset(
                                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                                .offset((x1_min + 4 as libc::c_int) as isize)
                                                as isize,
                                        ) {
                                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                                .offset(x1_min as isize);
                                            *(*r).m.offset(
                                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                                    .offset(x1_min as isize)
                                                    as isize,
                                            )
                                        } else {
                                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                                .offset((x1_min + 4 as libc::c_int) as isize);
                                            *(*r).m.offset(
                                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                                    .offset((x1_min + 4 as libc::c_int) as isize)
                                                    as isize,
                                            )
                                        }
                                    } else if *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 3 as libc::c_int) as isize)
                                            as isize,
                                    ) < *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize)
                                            as isize,
                                    ) {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 3 as libc::c_int) as isize);
                                        *(*r).m.offset(
                                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                                .offset((x1_min + 3 as libc::c_int) as isize)
                                                as isize,
                                        )
                                    } else {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize);
                                        *(*r).m.offset(
                                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                                .offset((x1_min + 4 as libc::c_int) as isize)
                                                as isize,
                                        )
                                    }
                                } else if *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize)
                                        as isize,
                                ) < *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize)
                                        as isize,
                                ) {
                                    if *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 2 as libc::c_int) as isize)
                                            as isize,
                                    ) < *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize)
                                            as isize,
                                    ) {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 2 as libc::c_int) as isize);
                                        *(*r).m.offset(
                                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                                .offset((x1_min + 2 as libc::c_int) as isize)
                                                as isize,
                                        )
                                    } else {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize);
                                        *(*r).m.offset(
                                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                                .offset((x1_min + 4 as libc::c_int) as isize)
                                                as isize,
                                        )
                                    }
                                } else if *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize)
                                        as isize,
                                ) < *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize)
                                        as isize,
                                ) {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 3 as libc::c_int) as isize)
                                            as isize,
                                    )
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize)
                                            as isize,
                                    )
                                }
                            } else if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 1 as libc::c_int) as isize)
                                    as isize,
                            ) < *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                if *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 1 as libc::c_int) as isize)
                                        as isize,
                                ) < *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize)
                                        as isize,
                                ) {
                                    if *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 1 as libc::c_int) as isize)
                                            as isize,
                                    ) < *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize)
                                            as isize,
                                    ) {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 1 as libc::c_int) as isize);
                                        *(*r).m.offset(
                                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                                .offset((x1_min + 1 as libc::c_int) as isize)
                                                as isize,
                                        )
                                    } else {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize);
                                        *(*r).m.offset(
                                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                                .offset((x1_min + 4 as libc::c_int) as isize)
                                                as isize,
                                        )
                                    }
                                } else if *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize)
                                        as isize,
                                ) < *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize)
                                        as isize,
                                ) {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 3 as libc::c_int) as isize)
                                            as isize,
                                    )
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize)
                                            as isize,
                                    )
                                }
                            } else if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize)
                                    as isize,
                            ) < *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                if *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize)
                                        as isize,
                                ) < *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize)
                                        as isize,
                                ) {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 2 as libc::c_int) as isize)
                                            as isize,
                                    )
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize)
                                            as isize,
                                    )
                                }
                            } else if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize)
                                    as isize,
                            ) < *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 4 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize)
                                        as isize,
                                )
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 4 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize)
                                        as isize,
                                )
                            }
                        } else if *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                        ) <= *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 1 as libc::c_int) as isize) as isize,
                        ) {
                            if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize) as isize,
                            ) <= *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                if *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize)
                                        as isize,
                                ) <= *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize)
                                        as isize,
                                ) {
                                    if *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize)
                                            as isize,
                                    ) <= *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize)
                                            as isize,
                                    ) {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset(x1_min as isize);
                                        *(*r).m.offset(
                                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize)
                                                as isize,
                                        )
                                    } else {
                                        least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize);
                                        *(*r).m.offset(
                                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                                .offset((x1_min + 4 as libc::c_int) as isize)
                                                as isize,
                                        )
                                    }
                                } else if *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize)
                                        as isize,
                                ) <= *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize)
                                        as isize,
                                ) {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 3 as libc::c_int) as isize)
                                            as isize,
                                    )
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize)
                                            as isize,
                                    )
                                }
                            } else if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize)
                                    as isize,
                            ) <= *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                if *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize)
                                        as isize,
                                ) <= *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize)
                                        as isize,
                                ) {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 2 as libc::c_int) as isize)
                                            as isize,
                                    )
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize)
                                            as isize,
                                    )
                                }
                            } else if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize)
                                    as isize,
                            ) <= *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 4 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize)
                                        as isize,
                                )
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 4 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize)
                                        as isize,
                                )
                            }
                        } else if *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 1 as libc::c_int) as isize) as isize,
                        ) <= *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 2 as libc::c_int) as isize) as isize,
                        ) {
                            if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 1 as libc::c_int) as isize)
                                    as isize,
                            ) <= *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                if *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 1 as libc::c_int) as isize)
                                        as isize,
                                ) <= *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize)
                                        as isize,
                                ) {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 1 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 1 as libc::c_int) as isize)
                                            as isize,
                                    )
                                } else {
                                    least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize);
                                    *(*r).m.offset(
                                        *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                            .offset((x1_min + 4 as libc::c_int) as isize)
                                            as isize,
                                    )
                                }
                            } else if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize)
                                    as isize,
                            ) <= *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 4 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 3 as libc::c_int) as isize)
                                        as isize,
                                )
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 4 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize)
                                        as isize,
                                )
                            }
                        } else if *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 2 as libc::c_int) as isize) as isize,
                        ) <= *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 3 as libc::c_int) as isize) as isize,
                        ) {
                            if *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize)
                                    as isize,
                            ) <= *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 4 as libc::c_int) as isize)
                                    as isize,
                            ) {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 2 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 2 as libc::c_int) as isize)
                                        as isize,
                                )
                            } else {
                                least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 4 as libc::c_int) as isize);
                                *(*r).m.offset(
                                    *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                        .offset((x1_min + 4 as libc::c_int) as isize)
                                        as isize,
                                )
                            }
                        } else if *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 3 as libc::c_int) as isize) as isize,
                        ) <= *(*r).m.offset(
                            *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 4 as libc::c_int) as isize) as isize,
                        ) {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 3 as libc::c_int) as isize);
                            *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 3 as libc::c_int) as isize)
                                    as isize,
                            )
                        } else {
                            least = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                .offset((x1_min + 4 as libc::c_int) as isize);
                            *(*r).m.offset(
                                *(*(*r).raw.offset((y - 1 as libc::c_int) as isize))
                                    .offset((x1_min + 4 as libc::c_int) as isize)
                                    as isize,
                            )
                        }
                    },
                    _ => {
                        data_down = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1_min as isize);
                        least = data_down;
                        m = *(*r).m.offset(data_down as isize);
                        x1 = x1_min + 1 as libc::c_int;
                        while x1 <= x1_max {
                            data_down = *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x1 as isize);
                            m1 = *(*r).m.offset(data_down as isize);
                            if m1 < m || m1 == m && (*r).leftright == 1 as libc::c_int {
                                m = m1;
                                least = data_down
                            }
                            x1 += 1
                        }
                    },
                }
                /* fprintf(stderr, "y,x=%i,%i x1_min,max=%i,%i least=%i m=%g\n", y, x, x1_min,
                 * x1_max, least, m); fflush(stderr); */
            }
            new_m = *(*r).en.offset(data as isize) + m;
            /* reduce the range if there's no (relevant) difference
             * with the previous map */
            if *(*r).least.offset(data as isize) == least {
                if (libm::fabsf(*(*r).m.offset(data as isize) - new_m) as libc::c_double) < 1e-5f64 {
                    if stop == 0 as libc::c_int {
                        x_stop = x
                    }
                    stop = 1 as libc::c_int;
                    new_m = *(*r).m.offset(data as isize)
                } else {
                    stop = 0 as libc::c_int;
                    *(*r).m.offset(data as isize) = new_m
                }
                if x == x_min && stop != 0 {
                    x_min += 1
                }
            } else {
                stop = 0 as libc::c_int;
                *(*r).m.offset(data as isize) = new_m
            }
            *(*r).least.offset(data as isize) = least;
            if x == x_max && stop != 0 {
                x_max = x_stop
            }
            x += 1
        }
        y += 1
    }
    if (*r).rigidity != 0. {
        mc = mc.offset(-((*r).delta_x as isize));
        libc::free(mc as *mut libc::c_void);
    }
    return LQR_OK;
}
/* compute seam path from minpath map */

pub unsafe extern "C" fn lqr_carver_build_vpath(mut r: *mut LqrCarver) {
    let mut m1: libc::c_float = 0.;
    let mut last: libc::c_int = -(1 as libc::c_int);
    let mut last_x: libc::c_int = 0 as libc::c_int;
    let mut x_min: libc::c_int = 0;
    let mut x_max: libc::c_int = 0;

    let mut y: libc::c_int = (*r).h - 1 as libc::c_int;
    let mut m: libc::c_float = ((1 as libc::c_int) << 29 as libc::c_int) as libc::c_float;
    let mut x: libc::c_int = 0 as libc::c_int;
    while x < (*r).w {
        /* __LQR_DEBUG__ */
        m1 = *(*r)
            .m
            .offset(*(*(*r).raw.offset(y as isize)).offset(x as isize) as isize);
        if m1 < m || m1 == m && (*r).leftright == 1 as libc::c_int {
            last = *(*(*r).raw.offset(y as isize)).offset(x as isize);
            last_x = x;
            m = m1
        }
        x += 1;
    }
    /* __LQR_DEBUG__ */
    /* follow the track for the other rows */
    y = (*r).h0 - 1 as libc::c_int;
    while y >= 0 as libc::c_int {
        /* __LQR_DEBUG__ */
        *(*r).vpath.offset(y as isize) = last;
        *(*r).vpath_x.offset(y as isize) = last_x;
        if y > 0 as libc::c_int {
            last = *(*r)
                .least
                .offset(*(*(*r).raw.offset(y as isize)).offset(last_x as isize) as isize);
            /* __LQR_DEBUG__ */
            x_min = if last_x - (*r).delta_x > 0 as libc::c_int {
                (last_x) - (*r).delta_x
            } else {
                0 as libc::c_int
            };
            x_max = if last_x + (*r).delta_x < (*r).w - 1 as libc::c_int {
                (last_x) + (*r).delta_x
            } else {
                ((*r).w) - 1 as libc::c_int
            };
            x = x_min;
            while x <= x_max {
                if *(*(*r).raw.offset((y - 1 as libc::c_int) as isize)).offset(x as isize) == last {
                    last_x = x;
                    break;
                } else {
                    x += 1
                }
            }
        }
        y -= 1
    }
}
/* we also need to retrieve the x coordinate */
/* update visibility map after seam computation */

pub unsafe extern "C" fn lqr_carver_update_vsmap(mut r: *mut LqrCarver, mut l: libc::c_int) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*r).h {
        /* __LQR_DEBUG__ */
        *(*r).vs.offset(*(*r).vpath.offset(y as isize) as isize) = l;
        y += 1
    }
}
/* complete visibility map (last seam) */
/* set the last column of pixels to vis. level w0 */

pub unsafe extern "C" fn lqr_carver_finish_vsmap(mut r: *mut LqrCarver) {
    /* __LQR_DEBUG__ */
    crate::lqr_cursor::lqr_cursor_reset((*r).c);
    let mut y: libc::c_int = 1 as libc::c_int;
    while y <= (*r).h {
        /* __LQR_DEBUG__ */
        *(*r).vs.offset((*(*r).c).now as isize) = (*r).w0;
        y += 1;
        crate::lqr_cursor::lqr_cursor_next((*r).c);
    }
    crate::lqr_cursor::lqr_cursor_reset((*r).c);
}
/* propagate the root carver's visibility map */

pub unsafe extern "C" fn lqr_carver_propagate_vsmap(mut r: *mut LqrCarver) -> LqrRetVal {
    let mut data_tok: LqrDataTok = _LqrDataTok {
        carver: 0 as *mut LqrCarver,
    };
    if ({
        let mut gaig_temp: libc::c_int = 0;

        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
    {
        return LQR_USRCANCEL;
    }
    data_tok.data = 0 as *mut libc::c_void;
    let mut ret_val: LqrRetVal = LQR_ERROR;
    ret_val = crate::lqr_carver_list::lqr_carver_list_foreach_recursive(
        (*r).attached_list,
        Some(
            lqr_carver_propagate_vsmap_attached as unsafe extern "C" fn(_: *mut LqrCarver, _: LqrDataTok) -> LqrRetVal,
        ),
        data_tok,
    );
    if ret_val as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
        return ret_val;
    }
    return LQR_OK;
}

pub unsafe extern "C" fn lqr_carver_propagate_vsmap_attached(
    mut r: *mut LqrCarver,
    mut _data: LqrDataTok,
) -> LqrRetVal {
    /* LqrDataTok data_tok;
    data_tok.data = NULL; */
    (*r).vs = (*(*r).root).vs;
    lqr_carver_scan_reset(r);
    /* LQR_CATCH (lqr_carver_list_foreach (r->attached_list,  lqr_carver_propagate_vsmap_attached,
     * data_tok)); */
    return LQR_OK;
}
/* ** image manipulations ** */
/* set width of the multisize image
 * (maps have to be computed already) */

pub unsafe extern "C" fn lqr_carver_set_width(mut r: *mut LqrCarver, mut w1: libc::c_int) {
    /* __LQR_DEBUG__ */
    (*r).w = w1;
    (*r).level = (*r).w0 - w1 + 1 as libc::c_int;
}

pub unsafe extern "C" fn lqr_carver_set_width_attached(mut r: *mut LqrCarver, mut data: LqrDataTok) -> LqrRetVal {
    lqr_carver_set_width(r, data.integer);
    return LQR_OK;
}
/* flatten the image to its current state
 * (all maps are reset, invisible points are lost) */
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_carver_flatten(mut r: *mut LqrCarver) -> LqrRetVal {
    let mut new_rgb: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut new_bias: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut new_rigmask: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut _x: libc::c_int = 0;

    let mut _k: libc::c_int = 0;
    let mut z0: libc::c_int = 0;
    let mut data_tok: LqrDataTok = _LqrDataTok {
        carver: 0 as *mut LqrCarver,
    };
    let mut prev_state: LqrCarverState = LQR_CARVER_STATE_STD;
    /* __LQR_VERBOSE__ */
    if ({
        let mut gaig_temp: libc::c_int = 0;

        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
    {
        return LQR_USRCANCEL;
    }
    if (*r).root.is_null() {
        prev_state = ({
            let mut gaig_temp: libc::c_int = 0;

            *&mut gaig_temp =
                ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
            gaig_temp
        }) as LqrCarverState;
        let mut ret_val: LqrRetVal = LQR_ERROR;
        ret_val = lqr_carver_set_state(r, LQR_CARVER_STATE_FLATTENING, (0 as libc::c_int == 0) as libc::c_int);
        if ret_val as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val;
        }
    }
    /* first iterate on attached carvers */
    data_tok.data = 0 as *mut libc::c_void;
    let mut ret_val_0: LqrRetVal = LQR_ERROR;
    ret_val_0 = crate::lqr_carver_list::lqr_carver_list_foreach(
        (*r).attached_list,
        Some(lqr_carver_flatten_attached as unsafe extern "C" fn(_: *mut LqrCarver, _: LqrDataTok) -> LqrRetVal),
        data_tok,
    );
    if ret_val_0 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
        return ret_val_0;
    }
    /* free non needed maps first */
    libc::free((*r).en as *mut libc::c_void);
    libc::free((*r).m as *mut libc::c_void);
    libc::free((*r).rcache as *mut libc::c_void);
    libc::free((*r).least as *mut libc::c_void);
    (*r).rcache = 0 as *mut libc::c_double;
    (*r).nrg_uptodate = 0 as libc::c_int;
    /* allocate room for new map */
    match (*r).col_depth as libc::c_uint {
        0 => {
            new_rgb = ({
                let mut __n: libc::c_ulong = ((*r).w * (*r).h * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_uchar as *mut libc::c_void;
            if new_rgb.is_null() {
                return LQR_NOMEM;
            }
        },
        1 => {
            new_rgb = ({
                let mut __n: libc::c_ulong = ((*r).w * (*r).h * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_ushort as *mut libc::c_void;
            if new_rgb.is_null() {
                return LQR_NOMEM;
            }
        },
        2 => {
            new_rgb = ({
                let mut __n: libc::c_ulong = ((*r).w * (*r).h * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_float as *mut libc::c_void;
            if new_rgb.is_null() {
                return LQR_NOMEM;
            }
        },
        3 => {
            new_rgb = ({
                let mut __n: libc::c_ulong = ((*r).w * (*r).h * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_double>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_double as *mut libc::c_void;
            if new_rgb.is_null() {
                return LQR_NOMEM;
            }
        },
        _ => {},
    }
    if (*r).active != 0 {
        if !(*r).rigidity_mask.is_null() {
            new_rigmask = ({
                let mut __n: libc::c_ulong = ((*r).w * (*r).h) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_float;
            if new_rigmask.is_null() {
                return LQR_NOMEM;
            }
        }
    }
    if (*r).nrg_active != 0 {
        if !(*r).bias.is_null() {
            new_bias = ({
                let mut __n: libc::c_ulong = ((*r).w * (*r).h) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_float;
            if new_bias.is_null() {
                return LQR_NOMEM;
            }
        }
        libc::free((*r)._raw as *mut libc::c_void);
        libc::free((*r).raw as *mut libc::c_void);
        (*r)._raw = ({
            let mut __n: libc::c_ulong = ((*r).w * (*r).h) as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_int;
        if (*r)._raw.is_null() {
            return LQR_NOMEM;
        }
        (*r).raw = ({
            let mut __n: libc::c_ulong = (*r).h as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut *mut libc::c_int;
        if (*r).raw.is_null() {
            return LQR_NOMEM;
        }
    }
    /* span the image with the cursor and copy
     * it in the new array */
    crate::lqr_cursor::lqr_cursor_reset((*r).c);
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*r).h {
        if ({
            let mut gaig_temp: libc::c_int = 0;

            *(&mut gaig_temp as *mut libc::c_int) =
                ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
            gaig_temp
        }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
        {
            return LQR_USRCANCEL;
        }
        if (*r).nrg_active != 0 {
            let ref mut fresh2 = *(*r).raw.offset(y as isize);
            *fresh2 = (*r)._raw.offset((y * (*r).w) as isize)
        }

        for x in 0 as libc::c_int..(*r).w {
            z0 = y * (*r).w + x;
            for k in 0 as libc::c_int..(*r).channels {
                match (*r).col_depth as libc::c_uint {
                    0 => {
                        *(new_rgb as *mut libc::c_uchar).offset((z0 * (*r).channels + k) as isize) =
                            *((*r).rgb as *mut libc::c_uchar).offset(((*(*r).c).now * (*r).channels + k) as isize)
                    },
                    1 => {
                        *(new_rgb as *mut libc::c_ushort).offset((z0 * (*r).channels + k) as isize) =
                            *((*r).rgb as *mut libc::c_ushort).offset(((*(*r).c).now * (*r).channels + k) as isize)
                    },
                    2 => {
                        *(new_rgb as *mut libc::c_float).offset((z0 * (*r).channels + k) as isize) =
                            *((*r).rgb as *mut libc::c_float).offset(((*(*r).c).now * (*r).channels + k) as isize)
                    },
                    3 => {
                        *(new_rgb as *mut libc::c_double).offset((z0 * (*r).channels + k) as isize) =
                            *((*r).rgb as *mut libc::c_double).offset(((*(*r).c).now * (*r).channels + k) as isize)
                    },
                    _ => {},
                }
            }

            if (*r).active != 0 {
                if !(*r).rigidity_mask.is_null() {
                    *new_rigmask.offset(z0 as isize) = *(*r).rigidity_mask.offset((*(*r).c).now as isize)
                }
            }

            if (*r).nrg_active != 0 {
                if !(*r).bias.is_null() {
                    *new_bias.offset(z0 as isize) = *(*r).bias.offset((*(*r).c).now as isize)
                }
                *(*(*r).raw.offset(y as isize)).offset(x as isize) = z0
            }

            crate::lqr_cursor::lqr_cursor_next((*r).c);
        }
        y += 1
    }
    /* substitute the old maps */
    if (*r).preserve_in_buffer == 0 {
        libc::free((*r).rgb);
    }
    (*r).rgb = new_rgb;
    (*r).preserve_in_buffer = 0 as libc::c_int;
    if (*r).nrg_active != 0 {
        libc::free((*r).bias as *mut libc::c_void);
        (*r).bias = new_bias
    }
    if (*r).active != 0 {
        libc::free((*r).rigidity_mask as *mut libc::c_void);
        (*r).rigidity_mask = new_rigmask
    }
    /* init the other maps */
    if (*r).root.is_null() {
        libc::free((*r).vs as *mut libc::c_void);
        (*r).vs = ({
            let mut __n: libc::c_ulong = ((*r).w * (*r).h) as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc0(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc0(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_int;
        if (*r).vs.is_null() {
            return LQR_NOMEM;
        }
        let mut ret_val_1: LqrRetVal = LQR_ERROR;
        ret_val_1 = lqr_carver_propagate_vsmap(r);
        if ret_val_1 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val_1;
        }
    }
    if (*r).nrg_active != 0 {
        (*r).en = ({
            let mut __n: libc::c_ulong = ((*r).w * (*r).h) as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc0(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc0(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_float;
        if (*r).en.is_null() {
            return LQR_NOMEM;
        }
    }
    if (*r).active != 0 {
        (*r).m = ({
            let mut __n: libc::c_ulong = ((*r).w * (*r).h) as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc0(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc0(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_float;
        if (*r).m.is_null() {
            return LQR_NOMEM;
        }
        (*r).least = ({
            let mut __n: libc::c_ulong = ((*r).w * (*r).h) as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_int;
        if (*r).least.is_null() {
            return LQR_NOMEM;
        }
    }
    /* reset widths, heights & levels */
    (*r).w0 = (*r).w;
    (*r).h0 = (*r).h;
    (*r).w_start = (*r).w;
    (*r).h_start = (*r).h;
    (*r).level = 1 as libc::c_int;
    (*r).max_level = 1 as libc::c_int;
    /* __LQR_VERBOSE__ */
    if (*r).root.is_null() {
        let mut ret_val_2: LqrRetVal = LQR_ERROR;
        ret_val_2 = lqr_carver_set_state(r, prev_state, (0 as libc::c_int == 0) as libc::c_int);
        if ret_val_2 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val_2;
        }
    }
    return LQR_OK;
}

pub unsafe extern "C" fn lqr_carver_flatten_attached(mut r: *mut LqrCarver, mut _data: LqrDataTok) -> LqrRetVal {
    return lqr_carver_flatten(r);
}
/* transpose the image, in its current state
 * (all maps and invisible points are lost) */

pub unsafe extern "C" fn lqr_carver_transpose(mut r: *mut LqrCarver) -> LqrRetVal {
    let mut _y: libc::c_int = 0;
    let mut _k: libc::c_int = 0;
    let mut z0: libc::c_int = 0;
    let mut z1: libc::c_int = 0;

    let mut new_rgb: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut new_bias: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut new_rigmask: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut data_tok: LqrDataTok = _LqrDataTok {
        carver: 0 as *mut LqrCarver,
    };
    let mut prev_state: LqrCarverState = LQR_CARVER_STATE_STD;
    /* __LQR_VERBOSE__ */
    if ({
        let mut gaig_temp: libc::c_int = 0;

        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
    {
        return LQR_USRCANCEL;
    }
    if (*r).root.is_null() {
        prev_state = ({
            let mut gaig_temp: libc::c_int = 0;

            *&mut gaig_temp =
                ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
            gaig_temp
        }) as LqrCarverState;
        let mut ret_val: LqrRetVal = LQR_ERROR;
        ret_val = lqr_carver_set_state(r, LQR_CARVER_STATE_TRANSPOSING, (0 as libc::c_int == 0) as libc::c_int);
        if ret_val as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val;
        }
    }
    if (*r).level > 1 as libc::c_int {
        let mut ret_val_0: LqrRetVal = LQR_ERROR;
        ret_val_0 = lqr_carver_flatten(r);
        if ret_val_0 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val_0;
        }
    }
    /* first iterate on attached carvers */
    data_tok.data = 0 as *mut libc::c_void;
    let mut ret_val_1: LqrRetVal = LQR_ERROR;
    ret_val_1 = crate::lqr_carver_list::lqr_carver_list_foreach(
        (*r).attached_list,
        Some(lqr_carver_transpose_attached as unsafe extern "C" fn(_: *mut LqrCarver, _: LqrDataTok) -> LqrRetVal),
        data_tok,
    );
    if ret_val_1 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
        return ret_val_1;
    }
    /* free non needed maps first */
    if (*r).root.is_null() {
        libc::free((*r).vs as *mut libc::c_void);
    }
    libc::free((*r).en as *mut libc::c_void);
    libc::free((*r).m as *mut libc::c_void);
    libc::free((*r).rcache as *mut libc::c_void);
    libc::free((*r).least as *mut libc::c_void);
    libc::free((*r).rgb_ro_buffer);
    (*r).rcache = 0 as *mut libc::c_double;
    (*r).nrg_uptodate = 0 as libc::c_int;
    /* allocate room for the new maps */
    match (*r).col_depth as libc::c_uint {
        0 => {
            new_rgb = ({
                let mut __n: libc::c_ulong = ((*r).w0 * (*r).h0 * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_uchar as *mut libc::c_void;
            if new_rgb.is_null() {
                return LQR_NOMEM;
            }
        },
        1 => {
            new_rgb = ({
                let mut __n: libc::c_ulong = ((*r).w0 * (*r).h0 * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_ushort as *mut libc::c_void;
            if new_rgb.is_null() {
                return LQR_NOMEM;
            }
        },
        2 => {
            new_rgb = ({
                let mut __n: libc::c_ulong = ((*r).w0 * (*r).h0 * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_float as *mut libc::c_void;
            if new_rgb.is_null() {
                return LQR_NOMEM;
            }
        },
        3 => {
            new_rgb = ({
                let mut __n: libc::c_ulong = ((*r).w0 * (*r).h0 * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_double>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_double as *mut libc::c_void;
            if new_rgb.is_null() {
                return LQR_NOMEM;
            }
        },
        _ => {},
    }
    if (*r).active != 0 {
        if !(*r).rigidity_mask.is_null() {
            new_rigmask = ({
                let mut __n: libc::c_ulong = ((*r).w0 * (*r).h0) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_float;
            if new_rigmask.is_null() {
                return LQR_NOMEM;
            }
        }
    }
    if (*r).nrg_active != 0 {
        if !(*r).bias.is_null() {
            new_bias = ({
                let mut __n: libc::c_ulong = ((*r).w0 * (*r).h0) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_float;
            if new_bias.is_null() {
                return LQR_NOMEM;
            }
        }
        libc::free((*r)._raw as *mut libc::c_void);
        libc::free((*r).raw as *mut libc::c_void);
        (*r)._raw = ({
            let mut __n: libc::c_ulong = ((*r).h0 * (*r).w0) as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc0(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc0(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_int;
        if (*r)._raw.is_null() {
            return LQR_NOMEM;
        }
        (*r).raw = ({
            let mut __n: libc::c_ulong = (*r).w0 as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc0(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc0(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut *mut libc::c_int;
        if (*r).raw.is_null() {
            return LQR_NOMEM;
        }
    }
    let mut x: libc::c_int = 0 as libc::c_int;
    while x < (*r).w {
        if (*r).nrg_active != 0 {
            let ref mut fresh3 = *(*r).raw.offset(x as isize);
            *fresh3 = (*r)._raw.offset((x * (*r).h0) as isize)
        }

        for y in 0 as libc::c_int..(*r).h {
            z0 = y * (*r).w0 + x;

            z1 = x * (*r).h0 + y;
            for k in 0 as libc::c_int..(*r).channels {
                match (*r).col_depth as libc::c_uint {
                    0 => {
                        *(new_rgb as *mut libc::c_uchar).offset((z1 * (*r).channels + k) as isize) =
                            *((*r).rgb as *mut libc::c_uchar).offset((z0 * (*r).channels + k) as isize)
                    },
                    1 => {
                        *(new_rgb as *mut libc::c_ushort).offset((z1 * (*r).channels + k) as isize) =
                            *((*r).rgb as *mut libc::c_ushort).offset((z0 * (*r).channels + k) as isize)
                    },
                    2 => {
                        *(new_rgb as *mut libc::c_float).offset((z1 * (*r).channels + k) as isize) =
                            *((*r).rgb as *mut libc::c_float).offset((z0 * (*r).channels + k) as isize)
                    },
                    3 => {
                        *(new_rgb as *mut libc::c_double).offset((z1 * (*r).channels + k) as isize) =
                            *((*r).rgb as *mut libc::c_double).offset((z0 * (*r).channels + k) as isize)
                    },
                    _ => {},
                }
            }

            if (*r).active != 0 {
                if !(*r).rigidity_mask.is_null() {
                    *new_rigmask.offset(z1 as isize) = *(*r).rigidity_mask.offset(z0 as isize)
                }
            }

            if (*r).nrg_active != 0 {
                if !(*r).bias.is_null() {
                    *new_bias.offset(z1 as isize) = *(*r).bias.offset(z0 as isize)
                }
                *(*(*r).raw.offset(x as isize)).offset(y as isize) = z1
            }
        }
        x += 1
    }
    /* substitute the map */
    if (*r).preserve_in_buffer == 0 {
        libc::free((*r).rgb);
    }
    (*r).rgb = new_rgb;
    (*r).preserve_in_buffer = 0 as libc::c_int;
    if (*r).nrg_active != 0 {
        libc::free((*r).bias as *mut libc::c_void);
        (*r).bias = new_bias
    }
    if (*r).active != 0 {
        libc::free((*r).rigidity_mask as *mut libc::c_void);
        (*r).rigidity_mask = new_rigmask
    }
    /* init the other maps */
    if (*r).root.is_null() {
        (*r).vs = ({
            let mut __n: libc::c_ulong = ((*r).w0 * (*r).h0) as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc0(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc0(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_int;
        if (*r).vs.is_null() {
            return LQR_NOMEM;
        }
        let mut ret_val_2: LqrRetVal = LQR_ERROR;
        ret_val_2 = lqr_carver_propagate_vsmap(r);
        if ret_val_2 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val_2;
        }
    }
    if (*r).nrg_active != 0 {
        (*r).en = ({
            let mut __n: libc::c_ulong = ((*r).w0 * (*r).h0) as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc0(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc0(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_float;
        if (*r).en.is_null() {
            return LQR_NOMEM;
        }
    }
    if (*r).active != 0 {
        (*r).m = ({
            let mut __n: libc::c_ulong = ((*r).w0 * (*r).h0) as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc0(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc0(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_float;
        if (*r).m.is_null() {
            return LQR_NOMEM;
        }
        (*r).least = ({
            let mut __n: libc::c_ulong = ((*r).w0 * (*r).h0) as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_int;
        if (*r).least.is_null() {
            return LQR_NOMEM;
        }
    }
    let mut d: libc::c_int = (*r).w0;
    (*r).w0 = (*r).h0;
    (*r).h0 = d;
    (*r).w = (*r).w0;
    (*r).h = (*r).h0;
    /* reset w_start, h_start & levels */
    (*r).w_start = (*r).w0;
    (*r).h_start = (*r).h0;
    (*r).level = 1 as libc::c_int;
    (*r).max_level = 1 as libc::c_int;
    /* reset seam path, cursor and readout buffer */
    if (*r).active != 0 {
        libc::free((*r).vpath as *mut libc::c_void);
        (*r).vpath = ({
            let mut __n: libc::c_ulong = (*r).h as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_int;
        if (*r).vpath.is_null() {
            return LQR_NOMEM;
        }
        libc::free((*r).vpath_x as *mut libc::c_void);
        (*r).vpath_x = ({
            let mut __n: libc::c_ulong = (*r).h as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_int;
        if (*r).vpath_x.is_null() {
            return LQR_NOMEM;
        }
        libc::free((*r).nrg_xmin as *mut libc::c_void);
        (*r).nrg_xmin = ({
            let mut __n: libc::c_ulong = (*r).h as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_int;
        if (*r).nrg_xmin.is_null() {
            return LQR_NOMEM;
        }
        libc::free((*r).nrg_xmax as *mut libc::c_void);
        (*r).nrg_xmax = ({
            let mut __n: libc::c_ulong = (*r).h as libc::c_ulong;
            let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong;
            let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
            if __s == 1 as libc::c_int as libc::c_ulong {
                __p = malloc(__n)
            } else if 0 != 0
                && (__s == 0 as libc::c_int as libc::c_ulong
                    || __n
                        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong)
                            .wrapping_div(__s))
            {
                __p = malloc(__n.wrapping_mul(__s))
            } else {
                __p = calloc(__n, __s)
            }
            __p
        }) as *mut libc::c_int;
        if (*r).nrg_xmax.is_null() {
            return LQR_NOMEM;
        }
    }
    match (*r).col_depth as libc::c_uint {
        0 => {
            (*r).rgb_ro_buffer = ({
                let mut __n: libc::c_ulong = ((*r).w0 * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_uchar as *mut libc::c_void;
            if (*r).rgb_ro_buffer.is_null() {
                return LQR_NOMEM;
            }
        },
        1 => {
            (*r).rgb_ro_buffer = ({
                let mut __n: libc::c_ulong = ((*r).w0 * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_ushort as *mut libc::c_void;
            if (*r).rgb_ro_buffer.is_null() {
                return LQR_NOMEM;
            }
        },
        2 => {
            (*r).rgb_ro_buffer = ({
                let mut __n: libc::c_ulong = ((*r).w0 * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_float>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_float as *mut libc::c_void;
            if (*r).rgb_ro_buffer.is_null() {
                return LQR_NOMEM;
            }
        },
        3 => {
            (*r).rgb_ro_buffer = ({
                let mut __n: libc::c_ulong = ((*r).w0 * (*r).channels) as libc::c_ulong;
                let mut __s: libc::c_ulong = ::std::mem::size_of::<libc::c_double>() as libc::c_ulong;
                let mut __p: *mut libc::c_void = 0 as *mut libc::c_void;
                if __s == 1 as libc::c_int as libc::c_ulong {
                    __p = malloc0(__n)
                } else if 0 != 0
                    && (__s == 0 as libc::c_int as libc::c_ulong
                        || __n
                            <= (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(__s))
                {
                    __p = malloc0(__n.wrapping_mul(__s))
                } else {
                    __p = calloc(__n, __s)
                }
                __p
            }) as *mut libc::c_double as *mut libc::c_void;
            if (*r).rgb_ro_buffer.is_null() {
                return LQR_NOMEM;
            }
        },
        _ => {},
    }
    /* rescale rigidity */
    if (*r).active != 0 {
        x = -(*r).delta_x;
        while x <= (*r).delta_x {
            *(*r).rigidity_map.offset(x as isize) =
                *(*r).rigidity_map.offset(x as isize) * (*r).w0 as libc::c_float / (*r).h0 as libc::c_float;
            x += 1
        }
    }
    /* set transposed flag */
    (*r).transposed = if (*r).transposed != 0 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    /* __LQR_VERBOSE__ */
    if (*r).root.is_null() {
        let mut ret_val_3: LqrRetVal = LQR_ERROR;
        ret_val_3 = lqr_carver_set_state(r, prev_state, (0 as libc::c_int == 0) as libc::c_int);
        if ret_val_3 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val_3;
        }
    }
    return LQR_OK;
}

pub unsafe extern "C" fn lqr_carver_transpose_attached(mut r: *mut LqrCarver, mut _data: LqrDataTok) -> LqrRetVal {
    return lqr_carver_transpose(r);
}
/* resize w + h: these are the liquid rescale methods.
 * They automatically determine the depth of the map
 * according to the desired size, can be called multiple
 * times, transpose the image as necessasry */

pub unsafe extern "C" fn lqr_carver_resize_width(mut r: *mut LqrCarver, mut w1: libc::c_int) -> LqrRetVal {
    let mut data_tok: LqrDataTok = _LqrDataTok {
        carver: 0 as *mut LqrCarver,
    };
    let mut delta: libc::c_int = 0;
    let mut gamma: libc::c_int = 0;
    let mut delta_max: libc::c_int = 0;
    /* delta is used to determine the required depth
     * gamma to decide if action is necessary */
    if (*r).transposed == 0 {
        delta = w1 - (*r).w_start;
        gamma = w1 - (*r).w;
        delta_max = (((*r).enl_step - 1 as libc::c_int as libc::c_float) * (*r).w_start as libc::c_float) as libc::c_int
            - 1 as libc::c_int
    } else {
        delta = w1 - (*r).h_start;
        gamma = w1 - (*r).h;
        delta_max = (((*r).enl_step - 1 as libc::c_int as libc::c_float) * (*r).h_start as libc::c_float) as libc::c_int
            - 1 as libc::c_int
    }
    if delta_max < 1 as libc::c_int {
        delta_max = 1 as libc::c_int
    }
    if delta < 0 as libc::c_int {
        delta = -delta;
        delta_max = delta
    }
    if ({
        let mut gaig_temp: libc::c_int = 0;

        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
    {
        return LQR_USRCANCEL;
    }
    if (({
        let mut gaig_temp: libc::c_int = 0;

        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) == LQR_CARVER_STATE_STD as libc::c_int) as libc::c_int
        == 0 as libc::c_int
    {
        return LQR_ERROR;
    }
    let mut ret_val: LqrRetVal = LQR_ERROR;
    ret_val = lqr_carver_set_state(r, LQR_CARVER_STATE_RESIZING, (0 as libc::c_int == 0) as libc::c_int);
    if ret_val as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
        return ret_val;
    }
    /* update step for progress reprt */
    (*r).session_rescale_total = if gamma > 0 as libc::c_int { gamma } else { -gamma };
    (*r).session_rescale_current = 0 as libc::c_int;
    (*r).session_update_step = if (*r).session_rescale_total as libc::c_float * (*(*r).progress).update_step
        > 1 as libc::c_int as libc::c_float
    {
        ((*r).session_rescale_total as libc::c_float) * (*(*r).progress).update_step
    } else {
        1 as libc::c_int as libc::c_float
    } as libc::c_int;
    if (*r).session_rescale_total != 0 {
        crate::lqr_progress::lqr_progress_init((*r).progress, (*(*r).progress).init_width_message.as_mut_ptr());
    }
    while gamma != 0 {
        let mut delta0: libc::c_int = if delta < delta_max { delta } else { delta_max };

        delta -= delta0;
        if (*r).transposed != 0 {
            let mut ret_val_0: LqrRetVal = LQR_ERROR;
            ret_val_0 = lqr_carver_transpose(r);
            if ret_val_0 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val_0;
            }
        }
        let mut new_w: libc::c_int = if w1 < (*r).w_start + delta_max {
            w1
        } else {
            ((*r).w_start) + delta_max
        };
        gamma = w1 - new_w;
        let mut ret_val_1: LqrRetVal = LQR_ERROR;
        ret_val_1 = lqr_carver_build_maps(r, delta0 + 1 as libc::c_int);
        if ret_val_1 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val_1;
        }
        lqr_carver_set_width(r, new_w);
        data_tok.integer = new_w;
        crate::lqr_carver_list::lqr_carver_list_foreach_recursive(
            (*r).attached_list,
            Some(lqr_carver_set_width_attached as unsafe extern "C" fn(_: *mut LqrCarver, _: LqrDataTok) -> LqrRetVal),
            data_tok,
        );
        (*r).session_rescale_current =
            (*r).session_rescale_total - (if gamma > 0 as libc::c_int { gamma } else { -gamma });
        if (*r).dump_vmaps != 0 {
            let mut ret_val_2: LqrRetVal = LQR_ERROR;
            ret_val_2 = crate::lqr_vmap::lqr_vmap_internal_dump(r);
            if ret_val_2 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val_2;
            }
        }
        if new_w < w1 {
            let mut ret_val_3: LqrRetVal = LQR_ERROR;
            ret_val_3 = lqr_carver_flatten(r);
            if ret_val_3 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val_3;
            }
            delta_max = (((*r).enl_step - 1 as libc::c_int as libc::c_float) * (*r).w_start as libc::c_float)
                as libc::c_int
                - 1 as libc::c_int;
            if delta_max < 1 as libc::c_int {
                delta_max = 1 as libc::c_int
            }
        }
    }
    if (*r).session_rescale_total != 0 {
        crate::lqr_progress::lqr_progress_end((*r).progress, (*(*r).progress).end_width_message.as_mut_ptr());
    }
    let mut ret_val_4: LqrRetVal = LQR_ERROR;
    ret_val_4 = lqr_carver_set_state(r, LQR_CARVER_STATE_STD, (0 as libc::c_int == 0) as libc::c_int);
    if ret_val_4 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
        return ret_val_4;
    }
    return LQR_OK;
}

pub unsafe extern "C" fn lqr_carver_resize_height(mut r: *mut LqrCarver, mut h1: libc::c_int) -> LqrRetVal {
    let mut data_tok: LqrDataTok = _LqrDataTok {
        carver: 0 as *mut LqrCarver,
    };
    let mut delta: libc::c_int = 0;
    let mut gamma: libc::c_int = 0;
    let mut delta_max: libc::c_int = 0;
    /* delta is used to determine the required depth
     * gamma to decide if action is necessary */
    if (*r).transposed == 0 {
        delta = h1 - (*r).h_start;
        gamma = h1 - (*r).h;
        delta_max = (((*r).enl_step - 1 as libc::c_int as libc::c_float) * (*r).h_start as libc::c_float) as libc::c_int
            - 1 as libc::c_int
    } else {
        delta = h1 - (*r).w_start;
        gamma = h1 - (*r).w;
        delta_max = (((*r).enl_step - 1 as libc::c_int as libc::c_float) * (*r).w_start as libc::c_float) as libc::c_int
            - 1 as libc::c_int
    }
    if delta_max < 1 as libc::c_int {
        delta_max = 1 as libc::c_int
    }
    if delta < 0 as libc::c_int {
        delta_max = -delta
    }
    delta = if delta > 0 as libc::c_int { delta } else { -delta };
    if ({
        let mut gaig_temp: libc::c_int = 0;

        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
    {
        return LQR_USRCANCEL;
    }
    if (({
        let mut gaig_temp: libc::c_int = 0;

        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) == LQR_CARVER_STATE_STD as libc::c_int) as libc::c_int
        == 0 as libc::c_int
    {
        return LQR_ERROR;
    }
    let mut ret_val: LqrRetVal = LQR_ERROR;
    ret_val = lqr_carver_set_state(r, LQR_CARVER_STATE_RESIZING, (0 as libc::c_int == 0) as libc::c_int);
    if ret_val as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
        return ret_val;
    }
    /* update step for progress reprt */
    (*r).session_rescale_total = if gamma > 0 as libc::c_int { gamma } else { -gamma };
    (*r).session_rescale_current = 0 as libc::c_int;
    (*r).session_update_step = if (*r).session_rescale_total as libc::c_float * (*(*r).progress).update_step
        > 1 as libc::c_int as libc::c_float
    {
        ((*r).session_rescale_total as libc::c_float) * (*(*r).progress).update_step
    } else {
        1 as libc::c_int as libc::c_float
    } as libc::c_int;
    if (*r).session_rescale_total != 0 {
        crate::lqr_progress::lqr_progress_init((*r).progress, (*(*r).progress).init_height_message.as_mut_ptr());
    }
    while gamma != 0 {
        let mut delta0: libc::c_int = if delta < delta_max { delta } else { delta_max };

        delta -= delta0;
        if (*r).transposed == 0 {
            let mut ret_val_0: LqrRetVal = LQR_ERROR;
            ret_val_0 = lqr_carver_transpose(r);
            if ret_val_0 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val_0;
            }
        }
        let mut new_w: libc::c_int = if h1 < (*r).w_start + delta_max {
            h1
        } else {
            ((*r).w_start) + delta_max
        };
        gamma = h1 - new_w;
        let mut ret_val_1: LqrRetVal = LQR_ERROR;
        ret_val_1 = lqr_carver_build_maps(r, delta0 + 1 as libc::c_int);
        if ret_val_1 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val_1;
        }
        lqr_carver_set_width(r, new_w);
        data_tok.integer = new_w;
        crate::lqr_carver_list::lqr_carver_list_foreach_recursive(
            (*r).attached_list,
            Some(lqr_carver_set_width_attached as unsafe extern "C" fn(_: *mut LqrCarver, _: LqrDataTok) -> LqrRetVal),
            data_tok,
        );
        (*r).session_rescale_current =
            (*r).session_rescale_total - (if gamma > 0 as libc::c_int { gamma } else { -gamma });
        if (*r).dump_vmaps != 0 {
            let mut ret_val_2: LqrRetVal = LQR_ERROR;
            ret_val_2 = crate::lqr_vmap::lqr_vmap_internal_dump(r);
            if ret_val_2 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val_2;
            }
        }
        if new_w < h1 {
            let mut ret_val_3: LqrRetVal = LQR_ERROR;
            ret_val_3 = lqr_carver_flatten(r);
            if ret_val_3 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val_3;
            }
            delta_max = (((*r).enl_step - 1 as libc::c_int as libc::c_float) * (*r).w_start as libc::c_float)
                as libc::c_int
                - 1 as libc::c_int;
            if delta_max < 1 as libc::c_int {
                delta_max = 1 as libc::c_int
            }
        }
    }
    if (*r).session_rescale_total != 0 {
        crate::lqr_progress::lqr_progress_end((*r).progress, (*(*r).progress).end_height_message.as_mut_ptr());
    }
    let mut ret_val_4: LqrRetVal = LQR_ERROR;
    ret_val_4 = lqr_carver_set_state(r, LQR_CARVER_STATE_STD, (0 as libc::c_int == 0) as libc::c_int);
    if ret_val_4 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
        return ret_val_4;
    }
    return LQR_OK;
}
/* liquid rescale public method */
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_carver_resize(
    mut r: *mut LqrCarver,
    mut w1: libc::c_int,
    mut h1: libc::c_int,
) -> LqrRetVal {
    /* __LQR_VERBOSE__ */
    if w1 <= 0 && h1 <= 0 {
        return LQR_ERROR;
    }

    if !(*r).root.is_null() {
        return LQR_ERROR;
    }

    if ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int) == LQR_CARVER_STATE_CANCELLED as _ {
        return LQR_USRCANCEL;
    }

    if ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int) != LQR_CARVER_STATE_STD as _ {
        return LQR_ERROR;
    }

    match (*r).resize_order as libc::c_uint {
        0 => {
            match lqr_carver_resize_width(r, w1) {
                LQR_OK => {},
                _ => return LQR_ERROR,
            };

            match lqr_carver_resize_height(r, h1) {
                LQR_OK => {},
                _ => return LQR_ERROR,
            };
            /* __LQR_DEBUG__ */
        },
        1 => {
            match lqr_carver_resize_height(r, h1) {
                LQR_OK => {},
                _ => return LQR_ERROR,
            };

            match lqr_carver_resize_width(r, w1) {
                LQR_OK => {},
                _ => return LQR_ERROR,
            };
        },
        _ => {},
    }
    lqr_carver_scan_reset_all(r);
    /* __LQR_VERBOSE__ */
    return LQR_OK;
}

pub unsafe extern "C" fn lqr_carver_set_state(
    mut r: *mut LqrCarver,
    mut state: LqrCarverState,
    mut skip_canceled: libc::c_int,
) -> LqrRetVal {
    let mut data_tok: LqrDataTok = _LqrDataTok {
        carver: 0 as *mut LqrCarver,
    };

    if ((*r).root == 0 as *mut libc::c_void as *mut LqrCarver) as libc::c_int == 0 as libc::c_int {
        return LQR_ERROR;
    }
    let mut lock_pos: libc::c_int =
        { ::std::intrinsics::atomic_xadd::<_, _, {AtomicOrdering::SeqCst}>(&mut (*r).state_lock_queue as *mut libc::c_int, 1 as libc::c_int) };
    /* GLIB_VERSION < 2.30 */
    while ({
        let mut gaig_temp: libc::c_int = 0;

        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state_lock as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) != lock_pos
    {
        libc::sleep(10000);
    }
    if skip_canceled != 0
        && ({
            let mut gaig_temp: libc::c_int = 0;

            *(&mut gaig_temp as *mut libc::c_int) =
                ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
            gaig_temp
        }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
    {
        ::std::intrinsics::atomic_xadd::<_, _, {AtomicOrdering::SeqCst}>(&mut (*r).state_lock, 1 as libc::c_int);
        return LQR_OK;
    }
    let mut gais_temp: libc::c_int = state as libc::c_int;

    ::std::intrinsics::atomic_store::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int, *&mut gais_temp);
    data_tok.integer = state as libc::c_int;
    let mut ret_val: LqrRetVal = LQR_ERROR;
    ret_val = crate::lqr_carver_list::lqr_carver_list_foreach_recursive(
        (*r).attached_list,
        Some(lqr_carver_set_state_attached as unsafe extern "C" fn(_: *mut LqrCarver, _: LqrDataTok) -> LqrRetVal),
        data_tok,
    );
    if ret_val as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
        return ret_val;
    }

    ::std::intrinsics::atomic_xadd::<_, _, {AtomicOrdering::SeqCst}>(&mut (*r).state_lock, 1 as libc::c_int);
    return LQR_OK;
}

pub unsafe extern "C" fn lqr_carver_set_state_attached(mut r: *mut LqrCarver, mut data: LqrDataTok) -> LqrRetVal {
    let mut gais_temp: libc::c_int = data.integer;

    ::std::intrinsics::atomic_store::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int, *&mut gais_temp);
    return LQR_OK;
}

pub unsafe extern "C" fn lqr_carver_get_width(mut r: *mut LqrCarver) -> libc::c_int {
    return if (*r).transposed != 0 { (*r).h } else { (*r).w };
}
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_carver_get_height(mut r: *mut LqrCarver) -> libc::c_int {
    return if (*r).transposed != 0 { (*r).w } else { (*r).h };
}
/* readout reset */
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_carver_scan_reset(mut r: *mut LqrCarver) {
    crate::lqr_cursor::lqr_cursor_reset((*r).c);
}

pub unsafe extern "C" fn lqr_carver_scan_reset_attached(mut r: *mut LqrCarver, mut data: LqrDataTok) -> LqrRetVal {
    lqr_carver_scan_reset(r);
    return crate::lqr_carver_list::lqr_carver_list_foreach(
        (*r).attached_list,
        Some(lqr_carver_scan_reset_attached as unsafe extern "C" fn(_: *mut LqrCarver, _: LqrDataTok) -> LqrRetVal),
        data,
    );
}

pub unsafe extern "C" fn lqr_carver_scan_reset_all(mut r: *mut LqrCarver) {
    let mut data: LqrDataTok = _LqrDataTok {
        carver: 0 as *mut LqrCarver,
    };
    data.data = 0 as *mut libc::c_void;
    lqr_carver_scan_reset(r);
    crate::lqr_carver_list::lqr_carver_list_foreach(
        (*r).attached_list,
        Some(lqr_carver_scan_reset_attached as unsafe extern "C" fn(_: *mut LqrCarver, _: LqrDataTok) -> LqrRetVal),
        data,
    );
}
/* readout all, pixel by bixel */
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_carver_scan(
    mut r: *mut LqrCarver,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut rgb: *mut *mut libc::c_uchar,
) -> libc::c_int {
    if (*r).col_depth as libc::c_uint != LQR_COLDEPTH_8I as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if (*(*r).c).eoc != 0 {
        lqr_carver_scan_reset(r);
        return 0 as libc::c_int;
    }
    *x = if (*r).transposed != 0 { (*(*r).c).y } else { (*(*r).c).x };
    *y = if (*r).transposed != 0 { (*(*r).c).x } else { (*(*r).c).y };
    let mut k: libc::c_int = 0 as libc::c_int;
    while k < (*r).channels {
        *((*r).rgb_ro_buffer as *mut libc::c_uchar).offset(k as isize) =
            *((*r).rgb as *mut libc::c_uchar).offset(((*(*r).c).now * (*r).channels + k) as isize);
        k += 1
    }
    *rgb = (*r).rgb_ro_buffer as *mut libc::c_uchar;
    crate::lqr_cursor::lqr_cursor_next((*r).c);
    return (0 as libc::c_int == 0) as libc::c_int;
}
/* LQR_PUBLIC */
