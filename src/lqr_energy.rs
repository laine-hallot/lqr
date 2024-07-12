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
/* read normalised pixel value from
 * rgb buffer at the given index */

pub unsafe extern "C" fn lqr_pixel_get_norm(
    mut rgb: *mut libc::c_void,
    mut rgb_ind: libc::c_int,
    mut col_depth: LqrColDepth,
) -> libc::c_double {
    match col_depth as libc::c_uint {
        0 => {
            return *(rgb as *mut libc::c_uchar).offset(rgb_ind as isize) as libc::c_double
                / 0xff as libc::c_int as libc::c_double;
        },
        1 => {
            return *(rgb as *mut libc::c_ushort).offset(rgb_ind as isize) as libc::c_double
                / 0xffff as libc::c_int as libc::c_double;
        },
        2 => return *(rgb as *mut libc::c_float).offset(rgb_ind as isize) as libc::c_double,
        3 => return *(rgb as *mut libc::c_double).offset(rgb_ind as isize),
        _ => {
            /* __LQR_DEBUG__ */
            return 0 as libc::c_int as libc::c_double;
        },
    };
}

pub unsafe extern "C" fn lqr_pixel_get_rgbcol(
    mut rgb: *mut libc::c_void,
    mut rgb_ind: libc::c_int,
    mut col_depth: LqrColDepth,
    mut image_type: LqrImageType,
    mut channel: libc::c_int,
) -> libc::c_double {
    let mut black_fact: libc::c_double = 0 as libc::c_int as libc::c_double;
    match image_type as libc::c_uint {
        0 | 1 => return lqr_pixel_get_norm(rgb, rgb_ind + channel, col_depth),
        4 => return 1.0f64 - lqr_pixel_get_norm(rgb, rgb_ind + channel, col_depth),
        5 | 6 => {
            black_fact =
                1 as libc::c_int as libc::c_double - lqr_pixel_get_norm(rgb, rgb_ind + 3 as libc::c_int, col_depth);
            return black_fact * (1.0f64 - lqr_pixel_get_norm(rgb, rgb_ind + channel, col_depth));
        },
        7 => return 0 as libc::c_int as libc::c_double,
        _ => {
            /* __LQR_DEBUG__ */
            return 0 as libc::c_int as libc::c_double;
        },
    };
}

pub unsafe extern "C" fn lqr_carver_read_brightness_grey(
    mut r: *mut LqrCarver,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_double {
    let mut now: libc::c_int = *(*(*r).raw.offset(y as isize)).offset(x as isize);
    let mut rgb_ind: libc::c_int = now * (*r).channels;
    return lqr_pixel_get_norm((*r).rgb, rgb_ind, (*r).col_depth);
}

pub unsafe extern "C" fn lqr_carver_read_brightness_std(
    mut r: *mut LqrCarver,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_double {
    let mut now: libc::c_int = *(*(*r).raw.offset(y as isize)).offset(x as isize);
    let mut rgb_ind: libc::c_int = now * (*r).channels;

    let mut red: libc::c_double =
        lqr_pixel_get_rgbcol((*r).rgb, rgb_ind, (*r).col_depth, (*r).image_type, 0 as libc::c_int);
    let mut green: libc::c_double =
        lqr_pixel_get_rgbcol((*r).rgb, rgb_ind, (*r).col_depth, (*r).image_type, 1 as libc::c_int);
    let mut blue: libc::c_double =
        lqr_pixel_get_rgbcol((*r).rgb, rgb_ind, (*r).col_depth, (*r).image_type, 2 as libc::c_int);
    return (red + green + blue) / 3 as libc::c_int as libc::c_double;
}

pub unsafe extern "C" fn lqr_carver_read_brightness_custom(
    mut r: *mut LqrCarver,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;

    let mut has_alpha: libc::c_int = if (*r).alpha_channel >= 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut has_black: libc::c_int = if (*r).black_channel >= 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut col_channels: libc::c_uint = ((*r).channels - has_alpha - has_black) as libc::c_uint;
    let mut black_fact: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut now: libc::c_int = *(*(*r).raw.offset(y as isize)).offset(x as isize);
    if has_black != 0 {
        black_fact = lqr_pixel_get_norm((*r).rgb, now * (*r).channels + (*r).black_channel, (*r).col_depth)
    }
    let mut k: libc::c_int = 0 as libc::c_int;
    while k < (*r).channels {
        if k != (*r).alpha_channel && k != (*r).black_channel {
            let mut col: libc::c_double = lqr_pixel_get_norm((*r).rgb, now * (*r).channels + k, (*r).col_depth);
            sum += 1.0f64 - (1.0f64 - col) * (1.0f64 - black_fact)
        }
        k += 1
    }
    sum /= col_channels as libc::c_double;
    if has_black != 0 {
        sum = 1 as libc::c_int as libc::c_double - sum
    }
    return sum;
}
/* read average pixel value at x, y
 * for energy computation */

pub unsafe extern "C" fn lqr_carver_read_brightness(
    mut r: *mut LqrCarver,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_double {
    let mut has_alpha: libc::c_int = if (*r).alpha_channel >= 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut alpha_fact: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut now: libc::c_int = *(*(*r).raw.offset(y as isize)).offset(x as isize);
    let mut bright: libc::c_double = 0 as libc::c_int as libc::c_double;
    match (*r).image_type as libc::c_uint {
        2 | 3 => bright = lqr_carver_read_brightness_grey(r, x, y),
        0 | 1 | 4 | 5 | 6 => bright = lqr_carver_read_brightness_std(r, x, y),
        7 => bright = lqr_carver_read_brightness_custom(r, x, y),
        _ => {},
    }
    if has_alpha != 0 {
        alpha_fact = lqr_pixel_get_norm((*r).rgb, now * (*r).channels + (*r).alpha_channel, (*r).col_depth)
    }
    return bright * alpha_fact;
}

pub unsafe extern "C" fn lqr_carver_read_luma_std(
    mut r: *mut LqrCarver,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_double {
    let mut now: libc::c_int = *(*(*r).raw.offset(y as isize)).offset(x as isize);
    let mut rgb_ind: libc::c_int = now * (*r).channels;

    let mut red: libc::c_double =
        lqr_pixel_get_rgbcol((*r).rgb, rgb_ind, (*r).col_depth, (*r).image_type, 0 as libc::c_int);
    let mut green: libc::c_double =
        lqr_pixel_get_rgbcol((*r).rgb, rgb_ind, (*r).col_depth, (*r).image_type, 1 as libc::c_int);
    let mut blue: libc::c_double =
        lqr_pixel_get_rgbcol((*r).rgb, rgb_ind, (*r).col_depth, (*r).image_type, 2 as libc::c_int);
    return 0.2126f64 * red + 0.7152f64 * green + 0.0722f64 * blue;
}

pub unsafe extern "C" fn lqr_carver_read_luma(
    mut r: *mut LqrCarver,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_double {
    let mut has_alpha: libc::c_int = if (*r).alpha_channel >= 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut alpha_fact: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut now: libc::c_int = *(*(*r).raw.offset(y as isize)).offset(x as isize);
    let mut bright: libc::c_double = 0 as libc::c_int as libc::c_double;
    match (*r).image_type as libc::c_uint {
        2 | 3 => bright = lqr_carver_read_brightness_grey(r, x, y),
        0 | 1 | 4 | 5 | 6 => bright = lqr_carver_read_luma_std(r, x, y),
        7 => bright = lqr_carver_read_brightness_custom(r, x, y),
        _ => {},
    }
    if has_alpha != 0 {
        alpha_fact = lqr_pixel_get_norm((*r).rgb, now * (*r).channels + (*r).alpha_channel, (*r).col_depth)
    }
    return bright * alpha_fact;
}

pub unsafe extern "C" fn lqr_carver_read_rgba(
    mut r: *mut LqrCarver,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut channel: libc::c_int,
) -> libc::c_double {
    let mut has_alpha: libc::c_int = if (*r).alpha_channel >= 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut now: libc::c_int = *(*(*r).raw.offset(y as isize)).offset(x as isize);
    /* __LQR_DEBUG__ */
    if channel < 3 as libc::c_int {
        match (*r).image_type as libc::c_uint {
            2 | 3 => return lqr_carver_read_brightness_grey(r, x, y),
            0 | 1 | 4 | 5 | 6 => {
                return lqr_pixel_get_rgbcol((*r).rgb, now * (*r).channels, (*r).col_depth, (*r).image_type, channel);
            },
            7 => return 0 as libc::c_int as libc::c_double,
            _ => {
                /* __LQR_DEBUG__ */
                return 0 as libc::c_int as libc::c_double;
            },
        }
    } else if has_alpha != 0 {
        return lqr_pixel_get_norm((*r).rgb, now * (*r).channels + (*r).alpha_channel, (*r).col_depth);
    } else {
        return 1 as libc::c_int as libc::c_double;
    };
}

pub unsafe extern "C" fn lqr_carver_read_custom(
    mut r: *mut LqrCarver,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut channel: libc::c_int,
) -> libc::c_double {
    let mut now: libc::c_int = *(*(*r).raw.offset(y as isize)).offset(x as isize);
    return lqr_pixel_get_norm((*r).rgb, now * (*r).channels + channel, (*r).col_depth);
}

pub unsafe extern "C" fn lqr_carver_read_cached_std(
    mut r: *mut LqrCarver,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_double {
    let mut z0: libc::c_int = *(*(*r).raw.offset(y as isize)).offset(x as isize);
    return *(*r).rcache.offset(z0 as isize);
}

pub unsafe extern "C" fn lqr_energy_builtin_grad_all(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut img_width: libc::c_int,
    mut img_height: libc::c_int,
    mut rwindow: *mut LqrReadingWindow,
    mut gf: LqrGradFunc,
) -> libc::c_float {
    let mut gx: libc::c_double = 0.;
    let mut gy: libc::c_double = 0.;
    let mut bread_func: Option<
        unsafe extern "C" fn(_: *mut LqrReadingWindow, _: libc::c_int, _: libc::c_int) -> libc::c_double,
    > = None;
    match crate::lqr_rwindow::lqr_rwindow_get_read_t(rwindow) as libc::c_uint {
        0 => {
            bread_func = Some(
                crate::lqr_rwindow::lqr_rwindow_read_bright
                    as unsafe extern "C" fn(_: *mut LqrReadingWindow, _: libc::c_int, _: libc::c_int) -> libc::c_double,
            )
        },
        1 => {
            bread_func = Some(
                crate::lqr_rwindow::lqr_rwindow_read_luma
                    as unsafe extern "C" fn(_: *mut LqrReadingWindow, _: libc::c_int, _: libc::c_int) -> libc::c_double,
            )
        },
        _ => {
            /* __LQR_DEBUG__ */
            return 0 as libc::c_int as libc::c_float;
        },
    }
    if y == 0 as libc::c_int {
        gy = bread_func.expect("non-null function pointer")(rwindow, 0 as libc::c_int, 1 as libc::c_int)
            - bread_func.expect("non-null function pointer")(rwindow, 0 as libc::c_int, 0 as libc::c_int)
    } else if y < img_height - 1 as libc::c_int {
        gy = (bread_func.expect("non-null function pointer")(rwindow, 0 as libc::c_int, 1 as libc::c_int)
            - bread_func.expect("non-null function pointer")(rwindow, 0 as libc::c_int, -(1 as libc::c_int)))
            / 2 as libc::c_int as libc::c_double
    } else {
        gy = bread_func.expect("non-null function pointer")(rwindow, 0 as libc::c_int, 0 as libc::c_int)
            - bread_func.expect("non-null function pointer")(rwindow, 0 as libc::c_int, -(1 as libc::c_int))
    }
    if x == 0 as libc::c_int {
        gx = bread_func.expect("non-null function pointer")(rwindow, 1 as libc::c_int, 0 as libc::c_int)
            - bread_func.expect("non-null function pointer")(rwindow, 0 as libc::c_int, 0 as libc::c_int)
    } else if x < img_width - 1 as libc::c_int {
        gx = (bread_func.expect("non-null function pointer")(rwindow, 1 as libc::c_int, 0 as libc::c_int)
            - bread_func.expect("non-null function pointer")(rwindow, -(1 as libc::c_int), 0 as libc::c_int))
            / 2 as libc::c_int as libc::c_double
    } else {
        gx = bread_func.expect("non-null function pointer")(rwindow, 0 as libc::c_int, 0 as libc::c_int)
            - bread_func.expect("non-null function pointer")(rwindow, -(1 as libc::c_int), 0 as libc::c_int)
    }
    return gf.expect("non-null function pointer")(gx, gy);
}

pub unsafe extern "C" fn lqr_energy_builtin_grad_norm(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut img_width: libc::c_int,
    mut img_height: libc::c_int,
    mut rwindow: *mut LqrReadingWindow,
    mut _extra_data: *mut libc::c_void,
) -> libc::c_float {
    return lqr_energy_builtin_grad_all(
        x,
        y,
        img_width,
        img_height,
        rwindow,
        Some(
            crate::lqr_gradient::lqr_grad_norm
                as unsafe extern "C" fn(_: libc::c_double, _: libc::c_double) -> libc::c_float,
        ),
    );
}

pub unsafe extern "C" fn lqr_energy_builtin_grad_sumabs(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut img_width: libc::c_int,
    mut img_height: libc::c_int,
    mut rwindow: *mut LqrReadingWindow,
    mut _extra_data: *mut libc::c_void,
) -> libc::c_float {
    return lqr_energy_builtin_grad_all(
        x,
        y,
        img_width,
        img_height,
        rwindow,
        Some(
            crate::lqr_gradient::lqr_grad_sumabs
                as unsafe extern "C" fn(_: libc::c_double, _: libc::c_double) -> libc::c_float,
        ),
    );
}

pub unsafe extern "C" fn lqr_energy_builtin_grad_xabs(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut img_width: libc::c_int,
    mut img_height: libc::c_int,
    mut rwindow: *mut LqrReadingWindow,
    mut _extra_data: *mut libc::c_void,
) -> libc::c_float {
    return lqr_energy_builtin_grad_all(
        x,
        y,
        img_width,
        img_height,
        rwindow,
        Some(
            crate::lqr_gradient::lqr_grad_xabs
                as unsafe extern "C" fn(_: libc::c_double, _: libc::c_double) -> libc::c_float,
        ),
    );
}

pub unsafe extern "C" fn lqr_energy_builtin_null(
    mut _x: libc::c_int,
    mut _y: libc::c_int,
    mut _img_width: libc::c_int,
    mut _img_height: libc::c_int,
    mut _rwindow: *mut LqrReadingWindow,
    mut _extra_data: *mut libc::c_void,
) -> libc::c_float {
    return 0 as libc::c_int as libc::c_float;
}
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_carver_set_energy_function_builtin(
    mut r: *mut LqrCarver,
    mut ef_ind: LqrEnergyFuncBuiltinType,
) -> LqrRetVal {
    match ef_ind as libc::c_uint {
        0 => {
            let mut ret_val: LqrRetVal = LQR_ERROR;
            ret_val = lqr_carver_set_energy_function(
                r,
                Some(
                    lqr_energy_builtin_grad_norm
                        as unsafe extern "C" fn(
                            _: libc::c_int,
                            _: libc::c_int,
                            _: libc::c_int,
                            _: libc::c_int,
                            _: *mut LqrReadingWindow,
                            _: *mut libc::c_void,
                        ) -> libc::c_float,
                ),
                1 as libc::c_int,
                LQR_ER_BRIGHTNESS,
                0 as *mut libc::c_void,
            );
            if ret_val as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val;
            }
        },
        1 => {
            let mut ret_val_0: LqrRetVal = LQR_ERROR;
            ret_val_0 = lqr_carver_set_energy_function(
                r,
                Some(
                    lqr_energy_builtin_grad_sumabs
                        as unsafe extern "C" fn(
                            _: libc::c_int,
                            _: libc::c_int,
                            _: libc::c_int,
                            _: libc::c_int,
                            _: *mut LqrReadingWindow,
                            _: *mut libc::c_void,
                        ) -> libc::c_float,
                ),
                1 as libc::c_int,
                LQR_ER_BRIGHTNESS,
                0 as *mut libc::c_void,
            );
            if ret_val_0 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val_0;
            }
        },
        2 => {
            let mut ret_val_1: LqrRetVal = LQR_ERROR;
            ret_val_1 = lqr_carver_set_energy_function(
                r,
                Some(
                    lqr_energy_builtin_grad_xabs
                        as unsafe extern "C" fn(
                            _: libc::c_int,
                            _: libc::c_int,
                            _: libc::c_int,
                            _: libc::c_int,
                            _: *mut LqrReadingWindow,
                            _: *mut libc::c_void,
                        ) -> libc::c_float,
                ),
                1 as libc::c_int,
                LQR_ER_BRIGHTNESS,
                0 as *mut libc::c_void,
            );
            if ret_val_1 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val_1;
            }
        },
        3 => {
            let mut ret_val_2: LqrRetVal = LQR_ERROR;
            ret_val_2 = lqr_carver_set_energy_function(
                r,
                Some(
                    lqr_energy_builtin_grad_norm
                        as unsafe extern "C" fn(
                            _: libc::c_int,
                            _: libc::c_int,
                            _: libc::c_int,
                            _: libc::c_int,
                            _: *mut LqrReadingWindow,
                            _: *mut libc::c_void,
                        ) -> libc::c_float,
                ),
                1 as libc::c_int,
                LQR_ER_LUMA,
                0 as *mut libc::c_void,
            );
            if ret_val_2 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val_2;
            }
        },
        4 => {
            let mut ret_val_3: LqrRetVal = LQR_ERROR;
            ret_val_3 = lqr_carver_set_energy_function(
                r,
                Some(
                    lqr_energy_builtin_grad_sumabs
                        as unsafe extern "C" fn(
                            _: libc::c_int,
                            _: libc::c_int,
                            _: libc::c_int,
                            _: libc::c_int,
                            _: *mut LqrReadingWindow,
                            _: *mut libc::c_void,
                        ) -> libc::c_float,
                ),
                1 as libc::c_int,
                LQR_ER_LUMA,
                0 as *mut libc::c_void,
            );
            if ret_val_3 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val_3;
            }
        },
        5 => {
            let mut ret_val_4: LqrRetVal = LQR_ERROR;
            ret_val_4 = lqr_carver_set_energy_function(
                r,
                Some(
                    lqr_energy_builtin_grad_xabs
                        as unsafe extern "C" fn(
                            _: libc::c_int,
                            _: libc::c_int,
                            _: libc::c_int,
                            _: libc::c_int,
                            _: *mut LqrReadingWindow,
                            _: *mut libc::c_void,
                        ) -> libc::c_float,
                ),
                1 as libc::c_int,
                LQR_ER_LUMA,
                0 as *mut libc::c_void,
            );
            if ret_val_4 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val_4;
            }
        },
        6 => {
            let mut ret_val_5: LqrRetVal = LQR_ERROR;
            ret_val_5 = lqr_carver_set_energy_function(
                r,
                Some(
                    lqr_energy_builtin_null
                        as unsafe extern "C" fn(
                            _: libc::c_int,
                            _: libc::c_int,
                            _: libc::c_int,
                            _: libc::c_int,
                            _: *mut LqrReadingWindow,
                            _: *mut libc::c_void,
                        ) -> libc::c_float,
                ),
                0 as libc::c_int,
                LQR_ER_BRIGHTNESS,
                0 as *mut libc::c_void,
            );
            if ret_val_5 as libc::c_uint != LQR_OK as libc::c_int as libc::c_uint {
                return ret_val_5;
            }
        },
        _ => return LQR_ERROR,
    }
    return LQR_OK;
}
/* LQR_PUBLIC */

pub unsafe extern "C" fn lqr_carver_set_energy_function(
    mut r: *mut LqrCarver,
    mut en_func: LqrEnergyFunc,
    mut radius: libc::c_int,
    mut reader_type: LqrEnergyReaderType,
    mut extra_data: *mut libc::c_void,
) -> LqrRetVal {
    if ((*r).root == 0 as *mut libc::c_void as *mut LqrCarver) as libc::c_int == 0 as libc::c_int {
        return LQR_ERROR;
    }
    (*r).nrg = en_func;
    (*r).nrg_radius = radius;
    (*r).nrg_read_t = reader_type;
    (*r).nrg_extra_data = extra_data;
    libc::free((*r).rcache as *mut libc::c_void);
    (*r).rcache = 0 as *mut libc::c_double;
    (*r).nrg_uptodate = 0 as libc::c_int;
    crate::lqr_rwindow::lqr_rwindow_destroy((*r).rwindow);
    if reader_type as libc::c_uint == LQR_ER_CUSTOM as libc::c_int as libc::c_uint {
        (*r).rwindow = crate::lqr_rwindow::lqr_rwindow_new_custom(radius, (*r).use_rcache, (*r).channels);
        if (*r).rwindow.is_null() {
            return LQR_NOMEM;
        }
    } else {
        (*r).rwindow = crate::lqr_rwindow::lqr_rwindow_new(radius, reader_type, (*r).use_rcache);
        if (*r).rwindow.is_null() {
            return LQR_NOMEM;
        }
    }
    return LQR_OK;
}

pub unsafe extern "C" fn lqr_carver_generate_rcache_bright(mut r: *mut LqrCarver) -> *mut libc::c_double {
    let mut _x: libc::c_int = 0;

    let mut z0: libc::c_int = 0;
    let mut buffer: *mut libc::c_double = ({
        let mut __n: libc::c_ulong = ((*r).w0 * (*r).h0) as libc::c_ulong;
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
    }) as *mut libc::c_double;
    if buffer.is_null() {
        return 0 as *mut libc::c_double;
    }
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*r).h {
        for x in 0 as libc::c_int..(*r).w {
            z0 = *(*(*r).raw.offset(y as isize)).offset(x as isize);

            *buffer.offset(z0 as isize) = lqr_carver_read_brightness(r, x, y);
        }
        y += 1
    }
    return buffer;
}

pub unsafe extern "C" fn lqr_carver_generate_rcache_luma(mut r: *mut LqrCarver) -> *mut libc::c_double {
    let mut _x: libc::c_int = 0;

    let mut z0: libc::c_int = 0;
    let mut buffer: *mut libc::c_double = ({
        let mut __n: libc::c_ulong = ((*r).w0 * (*r).h0) as libc::c_ulong;
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
    }) as *mut libc::c_double;
    if buffer.is_null() {
        return 0 as *mut libc::c_double;
    }
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*r).h {
        for x in 0 as libc::c_int..(*r).w {
            z0 = *(*(*r).raw.offset(y as isize)).offset(x as isize);

            *buffer.offset(z0 as isize) = lqr_carver_read_luma(r, x, y);
        }
        y += 1
    }
    return buffer;
}

pub unsafe extern "C" fn lqr_carver_generate_rcache_rgba(mut r: *mut LqrCarver) -> *mut libc::c_double {
    let mut _x: libc::c_int = 0;

    let mut _k: libc::c_int = 0;
    let mut z0: libc::c_int = 0;
    let mut buffer: *mut libc::c_double = ({
        let mut __n: libc::c_ulong = ((*r).w0 * (*r).h0 * 4 as libc::c_int) as libc::c_ulong;
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
    }) as *mut libc::c_double;
    if buffer.is_null() {
        return 0 as *mut libc::c_double;
    }
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*r).h {
        for x in 0 as libc::c_int..(*r).w {
            z0 = *(*(*r).raw.offset(y as isize)).offset(x as isize);
            for k in 0 as libc::c_int..4 as libc::c_int {
                *buffer.offset((z0 * 4 as libc::c_int + k) as isize) = lqr_carver_read_rgba(r, x, y, k);
            }
        }
        y += 1
    }
    return buffer;
}

pub unsafe extern "C" fn lqr_carver_generate_rcache_custom(mut r: *mut LqrCarver) -> *mut libc::c_double {
    let mut _x: libc::c_int = 0;

    let mut _k: libc::c_int = 0;
    let mut z0: libc::c_int = 0;
    let mut buffer: *mut libc::c_double = ({
        let mut __n: libc::c_ulong = ((*r).w0 * (*r).h0 * (*r).channels) as libc::c_ulong;
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
    }) as *mut libc::c_double;
    if buffer.is_null() {
        return 0 as *mut libc::c_double;
    }
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*r).h {
        for x in 0 as libc::c_int..(*r).w {
            z0 = *(*(*r).raw.offset(y as isize)).offset(x as isize);
            for k in 0 as libc::c_int..(*r).channels {
                *buffer.offset((z0 * (*r).channels + k) as isize) = lqr_carver_read_custom(r, x, y, k);
            }
        }
        y += 1
    }
    return buffer;
}

pub unsafe extern "C" fn lqr_carver_generate_rcache(mut r: *mut LqrCarver) -> *mut libc::c_double {
    /* __LQR_DEBUG__ */
    match (*r).nrg_read_t as libc::c_uint {
        0 => return lqr_carver_generate_rcache_bright(r),
        1 => return lqr_carver_generate_rcache_luma(r),
        2 => return lqr_carver_generate_rcache_rgba(r),
        3 => return lqr_carver_generate_rcache_custom(r),
        _ => {
            /* __LQR_DEBUG__ */
            return 0 as *mut libc::c_double;
        },
    };
}
/* LQR_PUBLIC */
