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
/* *** CARVER LIST FUNCTIONS *** */

pub unsafe extern "C" fn lqr_carver_list_destroy(mut list: *mut LqrCarverList) {
    let mut now: *mut LqrCarverList = list;
    if !now.is_null() {
        lqr_carver_list_destroy((*now).next);
        lqr_carver_list_destroy((*(*now).current).attached_list);
        crate::lqr_carver::lqr_carver_destroy((*now).current);
    };
}
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_carver_list_foreach(
    mut list: *mut LqrCarverList,
    mut func: LqrCarverFunc,
    mut data: LqrDataTok,
) -> LqrRetVal {
    let mut now: *mut LqrCarverList = list;
    if !now.is_null() {
        let mut ret_val: LqrRetVal = LQR_ERROR;
        ret_val = func.expect("non-null function pointer")((*now).current, data);
        if ret_val as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val;
        }
        return lqr_carver_list_foreach((*now).next, func, data);
    }
    return LQR_OK;
}
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_carver_list_foreach_recursive(
    mut list: *mut LqrCarverList,
    mut func: LqrCarverFunc,
    mut data: LqrDataTok,
) -> LqrRetVal {
    let mut now: *mut LqrCarverList = list;
    if !now.is_null() {
        let mut ret_val: LqrRetVal = LQR_ERROR;
        ret_val = func.expect("non-null function pointer")((*now).current, data);
        if ret_val as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val;
        }
        let mut ret_val_0: LqrRetVal = LQR_ERROR;
        ret_val_0 = lqr_carver_list_foreach((*(*now).current).attached_list, func, data);
        if ret_val_0 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
            return ret_val_0;
        }
        return lqr_carver_list_foreach((*now).next, func, data);
    }
    return LQR_OK;
}
