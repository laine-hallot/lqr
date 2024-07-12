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
/* *** VMAP LIST FUNCTIONS *** */

pub unsafe extern "C" fn lqr_vmap_list_append(
    mut list: *mut LqrVMapList,
    mut buffer: *mut LqrVMap,
) -> *mut LqrVMapList {
    let mut prev: *mut LqrVMapList = 0 as *mut LqrVMapList;
    let mut now: *mut LqrVMapList = list;
    while !now.is_null() {
        prev = now;
        now = (*now).next
    }
    now = ({
        let mut __n: libc::c_ulong = 1 as libc::c_int as libc::c_ulong;
        let mut __s: libc::c_ulong = ::std::mem::size_of::<LqrVMapList>() as libc::c_ulong;
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
    }) as *mut LqrVMapList;
    if now.is_null() {
        return 0 as *mut LqrVMapList;
    }
    (*now).next = 0 as *mut LqrVMapList;
    (*now).current = buffer;
    if !prev.is_null() {
        (*prev).next = now
    }
    if list.is_null() {
        return now;
    } else {
        return list;
    };
}

pub unsafe extern "C" fn lqr_vmap_list_destroy(mut list: *mut LqrVMapList) {
    let mut now: *mut LqrVMapList = list;
    if !now.is_null() {
        lqr_vmap_list_destroy((*now).next);
        crate::lqr_vmap::lqr_vmap_destroy((*now).current);
    };
}
/* LQR_PUBLIC */
