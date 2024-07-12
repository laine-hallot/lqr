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
/* *** LQR_CURSOR STRUCT FUNTIONS *** */
/* ** constructor and destructor ** */

pub unsafe extern "C" fn lqr_cursor_create(mut owner: *mut LqrCarver) -> *mut LqrCursor {
    let mut c: *mut LqrCursor = ({
        let mut __n: libc::c_ulong = 1 as libc::c_int as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<LqrCursor>() as libc::c_ulong;
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
    }) as *mut LqrCursor;
    if c.is_null() {
        return 0 as *mut LqrCursor;
    }
    (*c).o = owner;
    (*c).eoc = 0 as libc::c_int as libc::c_char;
    lqr_cursor_reset(c);
    return c;
}

pub unsafe extern "C" fn lqr_cursor_destroy(mut c: *mut LqrCursor) {
    libc::free(c as *mut libc::c_void);
}
/* ** functions for moving around ** */
/* resets to starting point */

pub unsafe extern "C" fn lqr_cursor_reset(mut c: *mut LqrCursor) {
    /* make sure the pointers are initialized */
    /* __LQR_DEBUG__ */
    /* reset end of carver flag */
    (*c).eoc = 0 as libc::c_int as libc::c_char;
    /* reset coordinates */
    (*c).x = 0 as libc::c_int;
    (*c).y = 0 as libc::c_int;
    /* set the current point to the beginning of the map */
    (*c).now = 0 as libc::c_int;
    /* skip invisible points */
    while *(*(*c).o).vs.offset((*c).now as isize) != 0 as libc::c_int
        && *(*(*c).o).vs.offset((*c).now as isize) < (*(*c).o).level
    {
        (*c).now += 1
        /* __LQR_DEBUG__ */
    }
}
/* go to next data (first rows, then columns;
 * does nothing if we are already at the top-right corner) */

pub unsafe extern "C" fn lqr_cursor_next(mut c: *mut LqrCursor) {
    /* __LQR_DEBUG__ */
    /* are we at the end? */
    if (*c).eoc != 0 {
        return;
    }
    /* update coordinates */
    if (*c).x == (*(*c).o).w - 1 as libc::c_int {
        if (*c).y == (*(*c).o).h - 1 as libc::c_int {
            /* top-right corner, set eoc flag */
            (*c).eoc = 1 as libc::c_int as libc::c_char;
            return;
        }
        /* end-of-line, carriage return */
        (*c).x = 0 as libc::c_int;
        (*c).y += 1
    } else {
        /* simple right move */
        (*c).x += 1
    }
    /* first move */
    (*c).now += 1;
    /* __LQR_DEBUG__ */
    /* skip invisible points */
    while *(*(*c).o).vs.offset((*c).now as isize) != 0 as libc::c_int
        && *(*(*c).o).vs.offset((*c).now as isize) < (*(*c).o).level
    {
        (*c).now += 1
        /* __LQR_DEBUG__ */
    }
}
/* go to previous data (behaves opposite to next) */
/* ** methods for exploring neighborhoods ** */
/* these return pointers to neighboring data
 * it is an error to ask for out-of-bounds data */

pub unsafe extern "C" fn lqr_cursor_left(mut c: *mut LqrCursor) -> libc::c_int {
    /* create an auxiliary pointer */
    let mut ret: libc::c_int = (*c).now;
    /* __LQR_DEBUG__ */
    /* first move */
    ret -= 1;
    /* __LQR_DEBUG__ */
    /* skip invisible points */
    while *(*(*c).o).vs.offset(ret as isize) != 0 as libc::c_int && *(*(*c).o).vs.offset(ret as isize) < (*(*c).o).level
    {
        ret -= 1
        /* __LQR_DEBUG__ */
    }
    return ret;
}
/* *** END OF LQR_CURSOR_CURSOR CLASS FUNCTIONS *** */
