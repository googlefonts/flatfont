use norad::Font;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::env;


fn main() {
    let args = Args::get_from_env_or_exit();

    let start = Instant::now();
    let ufo = Font::load(&args.path).expect("failed to load file");

    let duration = start.elapsed();
    let time_str = seconds_str(duration);
    let font_name =
        ufo.font_info.family_name.as_ref().cloned().unwrap_or_else(|| "an unnamed font".into());

    println!("loaded {} glyphs from {} in {}.", ufo.glyph_count(), font_name, time_str);

}

fn seconds_str(duration: Duration) -> String {
    format!("{}.{}s", duration.as_secs(), duration.subsec_millis())
}



struct Args {
    path: PathBuf,
    outpath: PathBuf,
}

impl Args {
    fn get_from_env_or_exit() -> Self {
        let mut args = env::args().skip(1);
        let path = args.next().map(PathBuf::from).expect("Must have input path");

        let outpath = args.next().map(PathBuf::from).expect("Must have outpath");
        if outpath.exists() {
            std::process::exit(1);
        }

        Args { path, outpath }
    }
}