use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "image_processor",
    about = "Applies a processing plugin to a PNG image"
)]
struct Args {
    /// Path to the input image
    #[arg(long)]
    input: PathBuf,

    /// Path where the processed image will be saved
    #[arg(long)]
    output: PathBuf,

    /// Plugin name without OS prefix/extension (e.g. "mirror")
    #[arg(long)]
    plugin: String,

    /// Path to the text file with processing parameters
    #[arg(long)]
    params: PathBuf,

    /// Directory containing the plugin library
    #[arg(long, default_value = "target/debug")]
    plugin_path: PathBuf,
}

fn main() {
    let mut args = Args::parse();
    if let Err(err) = image_processor::run(
        &args.input,
        &args.output,
        &args.plugin,
        &mut args.plugin_path,
        &args.params,
    ) {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }

    println!("Done: saved to {}", args.output.display())
}
