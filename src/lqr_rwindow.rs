#![allow(
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

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
/* __LQR_DEBUG__ */

pub unsafe extern "C" fn lqr_rwindow_fill_std(
    mut rwindow: *mut LqrReadingWindow,
    mut r: *mut LqrCarver,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> LqrRetVal {
    let mut _j: libc::c_int = 0;
    let mut read_float: LqrReadFunc = None;
    let mut buffer: *mut *mut libc::c_double = (*rwindow).buffer;
    match (*rwindow).read_t as libc::c_uint {
        0 => {
            read_float = Some(
                crate::lqr_energy::lqr_carver_read_brightness
                    as unsafe extern "C" fn(_: *mut LqrCarver, _: libc::c_int, _: libc::c_int) -> libc::c_double,
            )
        },
        1 => {
            read_float = Some(
                crate::lqr_energy::lqr_carver_read_luma
                    as unsafe extern "C" fn(_: *mut LqrCarver, _: libc::c_int, _: libc::c_int) -> libc::c_double,
            )
        },
        _ => {
            /* __LQR_DEBUG__ */
            return LQR_ERROR;
        },
    }
    let mut i: libc::c_int = -(*rwindow).radius;
    while i <= (*rwindow).radius {
        for j in -(*rwindow).radius..=(*rwindow).radius {
            if x + i < 0 as libc::c_int || x + i >= (*r).w || y + j < 0 as libc::c_int || y + j >= (*r).h {
                *(*buffer.offset(i as isize)).offset(j as isize) = 0 as libc::c_int as libc::c_double
            } else {
                *(*buffer.offset(i as isize)).offset(j as isize) =
                    read_float.expect("non-null function pointer")(r, x + i, y + j)
            }
        }
        i += 1
    }
    return LQR_OK;
}

pub unsafe extern "C" fn lqr_rwindow_fill_rgba(
    mut rwindow: *mut LqrReadingWindow,
    mut r: *mut LqrCarver,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> LqrRetVal {
    let mut _j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut buffer: *mut *mut libc::c_double = (*rwindow).buffer;
    if (lqr_rwindow_get_read_t(rwindow) as libc::c_uint == LQR_ER_RGBA as libc::c_int as libc::c_uint) as libc::c_int
        == 0 as libc::c_int
    {
        return LQR_ERROR;
    }
    let mut i: libc::c_int = -(*rwindow).radius;
    while i <= (*rwindow).radius {
        for j in -(*rwindow).radius..=(*rwindow).radius {
            if x + i < 0 as libc::c_int || x + i >= (*r).w || y + j < 0 as libc::c_int || y + j >= (*r).h {
                k = 0 as libc::c_int;
                while k < 4 as libc::c_int {
                    *(*buffer.offset(i as isize)).offset((4 as libc::c_int * j + k) as isize) =
                        0 as libc::c_int as libc::c_double;
                    k += 1
                }
            } else {
                k = 0 as libc::c_int;
                while k < 4 as libc::c_int {
                    *(*buffer.offset(i as isize)).offset((4 as libc::c_int * j + k) as isize) =
                        crate::lqr_energy::lqr_carver_read_rgba(r, x + i, y + j, k);
                    k += 1
                }
            }
        }
        i += 1
    }
    return LQR_OK;
}

pub unsafe extern "C" fn lqr_rwindow_fill_custom(
    mut rwindow: *mut LqrReadingWindow,
    mut r: *mut LqrCarver,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> LqrRetVal {
    let mut _j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut buffer: *mut *mut libc::c_double = (*rwindow).buffer;
    if (lqr_rwindow_get_read_t(rwindow) as libc::c_uint == LQR_ER_CUSTOM as libc::c_int as libc::c_uint) as libc::c_int
        == 0 as libc::c_int
    {
        return LQR_ERROR;
    }
    let mut i: libc::c_int = -(*rwindow).radius;
    while i <= (*rwindow).radius {
        for j in -(*rwindow).radius..=(*rwindow).radius {
            if x + i < 0 as libc::c_int || x + i >= (*r).w || y + j < 0 as libc::c_int || y + j >= (*r).h {
                k = 0 as libc::c_int;
                while k < (*r).channels {
                    *(*buffer.offset(i as isize)).offset(((*r).channels * j + k) as isize) =
                        0 as libc::c_int as libc::c_double;
                    k += 1
                }
            } else {
                k = 0 as libc::c_int;
                while k < (*r).channels {
                    *(*buffer.offset(i as isize)).offset(((*r).channels * j + k) as isize) =
                        crate::lqr_energy::lqr_carver_read_custom(r, x + i, y + j, k);
                    k += 1
                }
            }
        }
        i += 1
    }
    return LQR_OK;
}

pub unsafe extern "C" fn lqr_rwindow_fill(
    mut rwindow: *mut LqrReadingWindow,
    mut r: *mut LqrCarver,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> LqrRetVal {
    if ({
        let mut gaig_temp: libc::c_int = 0;
        if 0 as libc::c_int != 0 {
        } else {
        };
        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load_seqcst(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
    {
        return LQR_USRCANCEL;
    }
    (*rwindow).carver = r;
    (*rwindow).x = x;
    (*rwindow).y = y;
    if (*rwindow).use_rcache != 0 {
        return LQR_OK;
    }
    match (*rwindow).read_t as libc::c_uint {
        0 | 1 => {
            let mut ret_val: LqrRetVal = LQR_ERROR;
            ret_val = lqr_rwindow_fill_std(rwindow, r, x, y);
            if ret_val as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val;
            }
        },
        2 => {
            let mut ret_val_0: LqrRetVal = LQR_ERROR;
            ret_val_0 = lqr_rwindow_fill_rgba(rwindow, r, x, y);
            if ret_val_0 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val_0;
            }
        },
        3 => {
            let mut ret_val_1: LqrRetVal = LQR_ERROR;
            ret_val_1 = lqr_rwindow_fill_custom(rwindow, r, x, y);
            if ret_val_1 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val_1;
            }
        },
        _ => return LQR_ERROR,
    }
    return LQR_OK;
}

pub unsafe extern "C" fn lqr_rwindow_new_std(
    mut radius: libc::c_int,
    mut read_func_type: LqrEnergyReaderType,
    mut use_rcache: libc::c_int,
) -> *mut LqrReadingWindow {
    let mut out_rwindow: *mut LqrReadingWindow = ({
        let mut __n: libc::c_ulong = 1 as libc::c_int as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<LqrReadingWindow>() as libc::c_ulong;
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
    }) as *mut LqrReadingWindow;
    if out_rwindow.is_null() {
        return 0 as *mut LqrReadingWindow;
    }

    let mut buf_size1: libc::c_int = 2 as libc::c_int * radius + 1 as libc::c_int;
    let mut buf_size2: libc::c_int = buf_size1 * buf_size1;
    let mut out_buffer_aux: *mut libc::c_double = ({
        let mut __n: libc::c_ulong = buf_size2 as libc::c_ulong;
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
    }) as *mut libc::c_double;
    if out_buffer_aux.is_null() {
        return 0 as *mut LqrReadingWindow;
    }
    let mut out_buffer: *mut *mut libc::c_double = ({
        let mut __n: libc::c_ulong = buf_size1 as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong;
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
    }) as *mut *mut libc::c_double;
    if out_buffer.is_null() {
        return 0 as *mut LqrReadingWindow;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < buf_size1 {
        let ref mut fresh0 = *out_buffer.offset(i as isize);
        *fresh0 = out_buffer_aux.offset(radius as isize);
        out_buffer_aux = out_buffer_aux.offset(buf_size1 as isize);
        i += 1
    }
    out_buffer = out_buffer.offset(radius as isize);
    (*out_rwindow).buffer = out_buffer;
    (*out_rwindow).radius = radius;
    (*out_rwindow).read_t = read_func_type;
    (*out_rwindow).channels = 1 as libc::c_int;
    (*out_rwindow).use_rcache = use_rcache;
    (*out_rwindow).carver = 0 as *mut LqrCarver;
    (*out_rwindow).x = 0 as libc::c_int;
    (*out_rwindow).y = 0 as libc::c_int;
    return out_rwindow;
}

pub unsafe extern "C" fn lqr_rwindow_new_rgba(
    mut radius: libc::c_int,
    mut use_rcache: libc::c_int,
) -> *mut LqrReadingWindow {
    let mut out_rwindow: *mut LqrReadingWindow = ({
        let mut __n: libc::c_ulong = 1 as libc::c_int as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<LqrReadingWindow>() as libc::c_ulong;
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
    }) as *mut LqrReadingWindow;
    if out_rwindow.is_null() {
        return 0 as *mut LqrReadingWindow;
    }

    let mut buf_size1: libc::c_int = 2 as libc::c_int * radius + 1 as libc::c_int;
    let mut buf_size2: libc::c_int = buf_size1 * buf_size1 * 4 as libc::c_int;
    let mut out_buffer_aux: *mut libc::c_double = ({
        let mut __n: libc::c_ulong = buf_size2 as libc::c_ulong;
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
    }) as *mut libc::c_double;
    if out_buffer_aux.is_null() {
        return 0 as *mut LqrReadingWindow;
    }
    let mut out_buffer: *mut *mut libc::c_double = ({
        let mut __n: libc::c_ulong = buf_size1 as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong;
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
    }) as *mut *mut libc::c_double;
    if out_buffer.is_null() {
        return 0 as *mut LqrReadingWindow;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < buf_size1 {
        let ref mut fresh1 = *out_buffer.offset(i as isize);
        *fresh1 = out_buffer_aux.offset((radius * 4 as libc::c_int) as isize);
        out_buffer_aux = out_buffer_aux.offset((buf_size1 * 4 as libc::c_int) as isize);
        i += 1
    }
    out_buffer = out_buffer.offset(radius as isize);
    (*out_rwindow).buffer = out_buffer;
    (*out_rwindow).radius = radius;
    (*out_rwindow).read_t = LQR_ER_RGBA;
    (*out_rwindow).channels = 4 as libc::c_int;
    (*out_rwindow).use_rcache = use_rcache;
    (*out_rwindow).carver = 0 as *mut LqrCarver;
    (*out_rwindow).x = 0 as libc::c_int;
    (*out_rwindow).y = 0 as libc::c_int;
    return out_rwindow;
}

pub unsafe extern "C" fn lqr_rwindow_new_custom(
    mut radius: libc::c_int,
    mut use_rcache: libc::c_int,
    mut channels: libc::c_int,
) -> *mut LqrReadingWindow {
    let mut out_rwindow: *mut LqrReadingWindow = ({
        let mut __n: libc::c_ulong = 1 as libc::c_int as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<LqrReadingWindow>() as libc::c_ulong;
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
    }) as *mut LqrReadingWindow;
    if out_rwindow.is_null() {
        return 0 as *mut LqrReadingWindow;
    }

    let mut buf_size1: libc::c_int = 2 as libc::c_int * radius + 1 as libc::c_int;
    let mut buf_size2: libc::c_int = buf_size1 * buf_size1 * channels;
    let mut out_buffer_aux: *mut libc::c_double = ({
        let mut __n: libc::c_ulong = buf_size2 as libc::c_ulong;
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
    }) as *mut libc::c_double;
    if out_buffer_aux.is_null() {
        return 0 as *mut LqrReadingWindow;
    }
    let mut out_buffer: *mut *mut libc::c_double = ({
        let mut __n: libc::c_ulong = buf_size1 as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong;
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
    }) as *mut *mut libc::c_double;
    if out_buffer.is_null() {
        return 0 as *mut LqrReadingWindow;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < buf_size1 {
        let ref mut fresh2 = *out_buffer.offset(i as isize);
        *fresh2 = out_buffer_aux.offset((radius * channels) as isize);
        out_buffer_aux = out_buffer_aux.offset((buf_size1 * channels) as isize);
        i += 1
    }
    out_buffer = out_buffer.offset(radius as isize);
    (*out_rwindow).buffer = 0 as *mut *mut libc::c_double;
    (*out_rwindow).radius = radius;
    (*out_rwindow).read_t = LQR_ER_CUSTOM;
    (*out_rwindow).channels = channels;
    (*out_rwindow).use_rcache = use_rcache;
    (*out_rwindow).carver = 0 as *mut LqrCarver;
    (*out_rwindow).x = 0 as libc::c_int;
    (*out_rwindow).y = 0 as libc::c_int;
    return out_rwindow;
}

pub unsafe extern "C" fn lqr_rwindow_new(
    mut radius: libc::c_int,
    mut read_func_type: LqrEnergyReaderType,
    mut use_rcache: libc::c_int,
) -> *mut LqrReadingWindow {
    match read_func_type as libc::c_uint {
        0 | 1 => return lqr_rwindow_new_std(radius, read_func_type, use_rcache),
        2 => return lqr_rwindow_new_rgba(radius, use_rcache),
        3 | _ => {
            /* __LQR_DEBUG__ */
            return 0 as *mut LqrReadingWindow;
        },
    };
}

pub unsafe extern "C" fn lqr_rwindow_destroy(mut rwindow: *mut LqrReadingWindow) {
    if rwindow.is_null() {
        return;
    }
    if (*rwindow).buffer.is_null() {
        return;
    }
    let mut buffer: *mut *mut libc::c_double = (*rwindow).buffer;
    buffer = buffer.offset(-((*rwindow).radius as isize));
    let ref mut fresh3 = *buffer.offset(0 as libc::c_int as isize);
    *fresh3 = (*fresh3).offset(-(((*rwindow).radius * (*rwindow).channels) as isize));
    libc::free(*buffer.offset(0 as libc::c_int as isize) as *mut libc::c_void);
    libc::free(buffer as *mut libc::c_void);
    libc::free(rwindow as *mut libc::c_void);
}

pub unsafe extern "C" fn lqr_rwindow_read_bright(
    mut rwindow: *mut LqrReadingWindow,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_double {
    if (*rwindow).use_rcache != 0 {
        return crate::lqr_energy::lqr_carver_read_cached_std((*rwindow).carver, (*rwindow).x + x, (*rwindow).y + y);
    }
    return *(*(*rwindow).buffer.offset(x as isize)).offset(y as isize);
}

pub unsafe extern "C" fn lqr_rwindow_read_luma(
    mut rwindow: *mut LqrReadingWindow,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_double {
    if (*rwindow).use_rcache != 0 {
        return crate::lqr_energy::lqr_carver_read_cached_std((*rwindow).carver, (*rwindow).x + x, (*rwindow).y + y);
    }
    return *(*(*rwindow).buffer.offset(x as isize)).offset(y as isize);
}

pub unsafe extern "C" fn lqr_rwindow_get_read_t(mut rwindow: *mut LqrReadingWindow) -> LqrEnergyReaderType {
    return (*rwindow).read_t;
}
/* LQR_PUBLIC */
