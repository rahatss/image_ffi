use crate::error::AppError;
use libloading::os::unix::Library;
use std::ffi::CString;
use std::os::raw::c_char;
use std::path::Path;

type ProcessImageFn = unsafe extern "C" fn(u32, u32, *mut u8, *const c_char);

pub struct PluginLoader {
    lib: Library,
    process_fn: ProcessImageFn,
}

impl PluginLoader {
    pub fn load(plugin_path: &Path) -> Result<Self, AppError> {
        if !plugin_path.exists() {
            return Err(AppError::InvalidInput(format!(
                "Plugin not found: {}",
                plugin_path.display()
            )));
        }

        let lib = unsafe { Library::new(plugin_path)? };
        let process_fn: ProcessImageFn = unsafe { *lib.get::<ProcessImageFn>(b"process_image\0")? };

        Ok(Self { lib, process_fn })
    }

    pub(crate) fn process(
        &self,
        width: u32,
        height: u32,
        rgba_data: &mut [u8],
        params: &str,
    ) -> Result<(), AppError> {
        let c_params = CString::new(params)
            .map_err(|err| AppError::InvalidInput(format!("Params contain a null byte: {err}")))?;

        unsafe {
            (self.process_fn)(width, height, rgba_data.as_mut_ptr(), c_params.as_ptr());
        }

        Ok(())
    }
}
