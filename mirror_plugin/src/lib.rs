use image::{RgbaImage, imageops};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice::from_raw_parts_mut;

struct RotateParams {
    vertical: bool,
    horizontal: bool,
}

impl RotateParams {
    fn default() -> Self {
        Self {
            vertical: false,
            horizontal: false,
        }
    }
}

fn parse_params(s: &str) -> RotateParams {
    let mut params = RotateParams::default();

    for pair in s.split(',') {
        let pair = pair.trim();
        let Some((key, value)) = pair.split_once('=') else {
            continue;
        };

        if key.trim() == "vertical" {
            match value.trim().parse::<bool>() {
                Ok(v) => params.vertical = v,
                _ => {},
            }
        }

        if key.trim() == "horizontal" {
            match value.trim().parse::<bool>() {
                Ok(v) => params.horizontal = v,
                _ => {},
            }
        }
    }

    params
}

fn rotate(buffer: &mut [u8], width: u32, height: u32, params: RotateParams) {
    let Some(mut img) = RgbaImage::from_raw(width, height, buffer.to_vec()) else {
        return;
    };

    if params.vertical {
        img = imageops::flip_vertical(&img);
    }

    if params.horizontal {
        img = imageops::flip_horizontal(&img);
    }

    buffer.copy_from_slice(&img.into_raw());
}

#[unsafe(no_mangle)]
pub extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let params_str = if params.is_null() {
            ""
        } else {
            unsafe { CStr::from_ptr(params).to_str().unwrap_or("") }
        };

        let params = parse_params(params_str);
        let total_bytes = width as usize * height as usize * 4;
        let buffer_slice = unsafe { from_raw_parts_mut(rgba_data, total_bytes) };
        rotate(buffer_slice, width, height, params)
    }));

    if result.is_err() {
        eprintln!("mirror_plugin: panic caught image left unchanged")
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_vertical_true() {
        let p = parse_params("vertical=true");
        assert!(p.vertical);
        assert!(!p.horizontal);
    }

    #[test]
    fn parse_horizontal_true() {
        let p = parse_params("horizontal=true");
        assert!(!p.vertical);
        assert!(p.horizontal);
    }

    #[test]
    fn parse_both_true() {
        let p = parse_params("vertical=true,horizontal=true");
        assert!(p.vertical);
        assert!(p.horizontal);
    }

    #[test]
    fn parse_false_overrides_default() {
        let p = parse_params("vertical=false,horizontal=false");
        assert!(!p.vertical);
        assert!(!p.horizontal);
    }

    #[test]
    fn parse_empty_defaults_to_false() {
        let p = parse_params("");
        assert!(!p.vertical);
        assert!(!p.horizontal);
    }

    #[test]
    fn parse_invalid_value_ignored() {
        let p = parse_params("vertical=yes,horizontal=on");
        assert!(!p.vertical);
        assert!(!p.horizontal);
    }

    fn make_2x2() -> Vec<u8> {
        vec![
            10, 20,  30,  255,
            40, 50,  60,  255,
            70, 80,  90,  255,
           100, 110, 120, 255,
        ]
    }

    #[test]
    fn horizontal_flip_2x2() {
        let mut buf = make_2x2();
        rotate(&mut buf, 2, 2, parse_params("horizontal=true"));
        assert_eq!(buf, vec![
            40, 50,  60,  255,
            10, 20,  30,  255,
           100, 110, 120, 255,
            70, 80,  90,  255,
        ]);
    }

    #[test]
    fn vertical_flip_2x2() {
        let mut buf = make_2x2();
        rotate(&mut buf, 2, 2, parse_params("vertical=true"));
        assert_eq!(buf, vec![
            70, 80,  90,  255, // (0,0) ← BL
           100, 110, 120, 255, // (1,0) ← BR
            10, 20,  30,  255, // (0,1) ← TL
            40, 50,  60,  255, // (1,1) ← TR
        ]);
    }

    #[test]
    fn both_flips_2x2() {
        let mut buf = make_2x2();
        rotate(&mut buf, 2, 2, parse_params("vertical=true,horizontal=true"));
        assert_eq!(buf, vec![
           100, 110, 120, 255,
            70, 80,  90,  255,
            40, 50,  60,  255,
            10, 20,  30,  255,
        ]);
    }
}
