#![allow(
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use libm::fabs;
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
/* *** GRADIENT FUNCTIONS *** */

pub unsafe extern "C" fn lqr_grad_norm(mut x: libc::c_double, mut y: libc::c_double) -> libc::c_float {
    return libm::sqrt(x * x + y * y) as libc::c_float;
}

pub unsafe extern "C" fn lqr_grad_sumabs(mut x: libc::c_double, mut y: libc::c_double) -> libc::c_float {
    return ((fabs(x) + fabs(y)) / 2 as libc::c_int as libc::c_double) as libc::c_float;
}

pub unsafe extern "C" fn lqr_grad_xabs(mut x: libc::c_double, mut _y: libc::c_double) -> libc::c_float {
    return fabs(x) as libc::c_float;
}
/* *** END OF GRADIENT FUNCTIONS *** */
