use crate::error::AppError;
use crate::plugin_loader::PluginLoader;
use std::path::Path;

mod error;
mod plugin_loader;

pub fn run(
    input: &Path,
    output: &Path,
    plugin_name: &str,
    plugin_dir: &mut Path,
    params_file: &Path,
) -> Result<(), AppError> {
    if !input.exists() {
        return Err(AppError::InvalidInput(format!(
            "Input file not found: {}",
            input.display()
        )));
    }

    if !params_file.exists() {
        return Err(AppError::InvalidInput(format!(
            "Params file not found: {}",
            input.display()
        )));
    }

    let image = image::open(input)?.into_rgba8();
    let width = image.width();
    let height = image.height();
    let mut rgba_data = image.into_raw();

    let params = std::fs::read_to_string(params_file)?;
    let params = params.trim().to_string();

    let lib_filename = plugin_filename(plugin_name);
    let plugin_path = plugin_dir.join(&lib_filename);

    let loader = PluginLoader::load(&plugin_path)?;
    loader.process(width, height, &mut rgba_data, &params)?;

    let output_image = image::RgbaImage::from_raw(width, height, rgba_data)
        .ok_or_else(|| AppError::InvalidInput("Failed to reconstruct output image".to_string()))?;
    output_image.save_with_format(output, image::ImageFormat::Png)?;

    Ok(())
}

/// macOS  → lib{name}.dylib
/// Linux  → lib{name}.so
/// Windows→ {name}.dll
fn plugin_filename(name: &str) -> String {
    #[cfg(target_os = "macos")]
    return format!("lib{name}.dylib");

    #[cfg(target_os = "windows")]
    return format!("{name}.dll");

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return format!("lib{name}.so");
}
