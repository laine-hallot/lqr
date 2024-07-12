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
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_progress_new() -> *mut LqrProgress {
    let mut progress: *mut LqrProgress = ({
        let mut __n: libc::c_ulong = 1 as libc::c_int as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<LqrProgress>() as libc::c_ulong;
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
    }) as *mut LqrProgress;
    if progress.is_null() {
        return 0 as *mut LqrProgress;
    }
    lqr_progress_set_init_width_message(progress, b"Resizing width...\x00" as *const u8 as *const libc::c_char);
    lqr_progress_set_init_height_message(progress, b"Resizing height...\x00" as *const u8 as *const libc::c_char);
    lqr_progress_set_end_width_message(progress, b"done\x00" as *const u8 as *const libc::c_char);
    lqr_progress_set_end_height_message(progress, b"done\x00" as *const u8 as *const libc::c_char);
    lqr_progress_set_update_step(progress, 0.02f64 as libc::c_float);
    return progress;
}

pub unsafe extern "C" fn lqr_progress_init(mut p: *mut LqrProgress, mut message: *const libc::c_char) -> LqrRetVal {
    if (p != 0 as *mut libc::c_void as *mut LqrProgress) as libc::c_int == 0 as libc::c_int {
        return LQR_ERROR;
    }
    if (*p).init.is_some() {
        return (*p).init.expect("non-null function pointer")(message);
    } else {
        return LQR_OK;
    };
}

pub unsafe extern "C" fn lqr_progress_update(mut p: *mut LqrProgress, mut percentage: libc::c_double) -> LqrRetVal {
    if (p != 0 as *mut libc::c_void as *mut LqrProgress) as libc::c_int == 0 as libc::c_int {
        return LQR_ERROR;
    }
    if (*p).update.is_some() {
        return (*p).update.expect("non-null function pointer")(percentage);
    } else {
        return LQR_OK;
    };
}

pub unsafe extern "C" fn lqr_progress_end(mut p: *mut LqrProgress, mut message: *const libc::c_char) -> LqrRetVal {
    if (p != 0 as *mut libc::c_void as *mut LqrProgress) as libc::c_int == 0 as libc::c_int {
        return LQR_ERROR;
    }
    if (*p).end.is_some() {
        return (*p).end.expect("non-null function pointer")(message);
    } else {
        return LQR_OK;
    };
}
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_progress_set_update_step(
    mut p: *mut LqrProgress,
    mut update_step: libc::c_float,
) -> LqrRetVal {
    (*p).update_step = update_step;
    return LQR_OK;
}
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_progress_set_init_width_message(
    mut p: *mut LqrProgress,
    mut message: *const libc::c_char,
) -> LqrRetVal {
    if p.is_null() {
        return LQR_ERROR;
    }
    strlcpy(
        (*p).init_width_message.as_mut_ptr(),
        message,
        1024 as libc::c_int as libc::c_ulong,
    );
    return LQR_OK;
}
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_progress_set_init_height_message(
    mut p: *mut LqrProgress,
    mut message: *const libc::c_char,
) -> LqrRetVal {
    if (p != 0 as *mut libc::c_void as *mut LqrProgress) as libc::c_int == 0 as libc::c_int {
        return LQR_ERROR;
    }
    strlcpy(
        (*p).init_height_message.as_mut_ptr(),
        message,
        1024 as libc::c_int as libc::c_ulong,
    );
    return LQR_OK;
}
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_progress_set_end_width_message(
    mut p: *mut LqrProgress,
    mut message: *const libc::c_char,
) -> LqrRetVal {
    if (p != 0 as *mut libc::c_void as *mut LqrProgress) as libc::c_int == 0 as libc::c_int {
        return LQR_ERROR;
    }
    strlcpy(
        (*p).end_width_message.as_mut_ptr(),
        message,
        1024 as libc::c_int as libc::c_ulong,
    );
    return LQR_OK;
}
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_progress_set_end_height_message(
    mut p: *mut LqrProgress,
    mut message: *const libc::c_char,
) -> LqrRetVal {
    if (p != 0 as *mut libc::c_void as *mut LqrProgress) as libc::c_int == 0 as libc::c_int {
        return LQR_ERROR;
    }
    strlcpy(
        (*p).end_height_message.as_mut_ptr(),
        message,
        1024 as libc::c_int as libc::c_ulong,
    );
    return LQR_OK;
}
