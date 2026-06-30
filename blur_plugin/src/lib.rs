use image::RgbaImage;
use imageproc::filter::gaussian_blur_f32;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice::from_raw_parts_mut;

#[derive(Debug)]
struct BlurParams {
    sigma: f32,
}

impl BlurParams {
    fn default() -> BlurParams {
        Self { sigma: 1.0 }
    }
}

fn parse_params(s: &str) -> BlurParams {
    let mut p = BlurParams::default();

    for pair in s.split(",") {
        let pair = pair.trim();
        let Some((key, value)) = pair.split_once('=') else {
            return p;
        };

        match key {
            "sigma" => {
                if let Ok(v) = value.trim().parse::<f32>() {
                    p.sigma = v;
                    break;
                }
            }
            _ => {}
        }
    }

    p
}

fn blur(buffer: &mut [u8], width: u32, height: u32, params: BlurParams) {
    if let Some(img) = RgbaImage::from_raw(width, height, buffer.to_vec()) {
        let blurred = gaussian_blur_f32(&img, params.sigma);
        buffer.copy_from_slice(&blurred.into_raw());
    }
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
        blur(buffer_slice, width, height, params)
    }));

    if result.is_err() {
        eprintln!("blur_plugin: panic caught image left unchanged")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn make_buffer(width: u32, height: u32) -> Vec<u8> {
        let mut buf = vec![0u8; (width * height * 4) as usize];
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize * 4;
                buf[idx] = x as u8;
                buf[idx + 1] = y as u8;
                buf[idx + 2] = 0;
                buf[idx + 3] = 255;
            }
        }
        buf
    }

    #[test]
    fn parse_params_default_sigma() {
        let p = parse_params("");
        assert_eq!(p.sigma, 1.0);
    }

    #[test]
    fn parse_params_custom_sigma() {
        let p = parse_params("sigma=3.5");
        assert_eq!(p.sigma, 3.5);
    }

    #[test]
    fn parse_params_invalid_sigma_uses_default() {
        let p = parse_params("sigma=abc");
        assert_eq!(p.sigma, 1.0);
    }

    #[test]
    fn parse_params_ignores_unknown_keys() {
        let p = parse_params("radius=10,sigma=2.0");
        assert_eq!(p.sigma, 2.0);
    }

    #[test]
    fn blur_preserves_buffer_size() {
        let width = 8u32;
        let height = 8u32;
        let mut buf = make_buffer(width, height);
        blur(&mut buf, width, height, BlurParams { sigma: 1.0 });
        assert_eq!(buf.len(), (width * height * 4) as usize);
    }
}
