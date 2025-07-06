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
/* __LQR_DEBUG__ */
/* *** SEAMS BUFFER FUNCTIONS *** */
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_vmap_new(
    mut buffer: *mut libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut depth: libc::c_int,
    mut orientation: libc::c_int,
) -> *mut LqrVMap {
    let mut vmap: *mut LqrVMap = ({
        let mut __n: libc::c_ulong = 1 as libc::c_int as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<LqrVMap>() as libc::c_ulong;
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
    }) as *mut LqrVMap;
    if vmap.is_null() {
        return 0 as *mut LqrVMap;
    }
    (*vmap).buffer = buffer;
    (*vmap).width = width;
    (*vmap).height = height;
    (*vmap).orientation = orientation;
    (*vmap).depth = depth;
    return vmap;
}
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_vmap_destroy(mut vmap: *mut LqrVMap) {
    libc::free((*vmap).buffer as *mut libc::c_void);
    libc::free(vmap as *mut libc::c_void);
}
/* dump the visibility level of the image */
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_vmap_internal_dump(mut r: *mut LqrCarver) -> LqrRetVal {
    let mut _x: libc::c_int = 0;

    let mut z0: libc::c_int = 0;
    let mut vs: libc::c_int = 0;

    if ({
        let mut gaig_temp: libc::c_int = 0;
        if 0 as libc::c_int != 0 {
        } else {
        };
        *(&mut gaig_temp as *mut libc::c_int) =
            ::std::intrinsics::atomic_load::<_, {AtomicOrdering::SeqCst}>(&mut (*r).state as *mut libc::c_int as *mut libc::c_int);
        gaig_temp
    }) == LQR_CARVER_STATE_CANCELLED as libc::c_int
    {
        return LQR_USRCANCEL;
    }
    let mut w1: libc::c_int = (*r).w;
    /* temporarily set the size to the original */
    crate::lqr_carver::lqr_carver_set_width(r, (*r).w_start);

    let mut w: libc::c_int = crate::lqr_carver::lqr_carver_get_width(r);
    let mut h: libc::c_int = crate::lqr_carver::lqr_carver_get_height(r);
    let mut depth: libc::c_int = (*r).w0 - (*r).w_start;
    let mut buffer: *mut libc::c_int = ({
        let mut __n: libc::c_ulong = (w * h) as libc::c_ulong;
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
    if buffer.is_null() {
        return LQR_NOMEM;
    }
    crate::lqr_cursor::lqr_cursor_reset((*r).c);
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*r).h {
        for x in 0 as libc::c_int..(*r).w {
            vs = *(*r).vs.offset((*(*r).c).now as isize);

            if (*r).transposed == 0 {
                z0 = y * (*r).w + x
            } else {
                z0 = x * (*r).h + y
            }

            if vs == 0 as libc::c_int {
                *buffer.offset(z0 as isize) = 0 as libc::c_int
            } else {
                *buffer.offset(z0 as isize) = vs - depth
            }

            crate::lqr_cursor::lqr_cursor_next((*r).c);
        }
        y += 1
    }
    /* recover size */
    crate::lqr_carver::lqr_carver_set_width(r, w1);
    crate::lqr_cursor::lqr_cursor_reset((*r).c);
    let mut vmap: *mut LqrVMap = lqr_vmap_new(buffer, w, h, depth, (*r).transposed);
    if vmap.is_null() {
        return LQR_NOMEM;
    }
    (*r).flushed_vs = crate::lqr_vmap_list::lqr_vmap_list_append((*r).flushed_vs, vmap);
    if (*r).flushed_vs.is_null() {
        return LQR_NOMEM;
    }
    return LQR_OK;
}
/* LQR_PUBLIC */
