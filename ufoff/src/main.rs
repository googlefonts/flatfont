use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::env;
use std::fs::File;
use std::io::Write;

use norad::Font;

// import the flatbuffers runtime library
extern crate flatbuffers;
 
// import the flatbuffer generated code
#[allow(dead_code, unused_imports)]
mod flatbuffers_generated;
pub use flatbuffers_generated::flat_font::ufo::{FontInfo, FontInfoArgs};

fn main() -> std::io::Result<()> {
    let args = Args::get_from_env_or_exit();

    let start = Instant::now();
    let ufo = Font::load(&args.path).expect("failed to load file");

    let duration = start.elapsed();
    let time_str = seconds_str(duration);
    let font_name =
        ufo.font_info.family_name.as_ref().cloned().unwrap_or_else(|| "an unnamed font".into());

    println!("loaded {} glyphs from {} in {}.", ufo.glyph_count(), font_name, time_str);

    // Build a flat version
    // https://google.github.io/flatbuffers/flatbuffers_guide_tutorial.html

    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);

    let creator = ufo.meta.creator.map(|c| builder.create_string(&c));

    let font_info = FontInfo::create(&mut builder,
        &FontInfoArgs {
            ascender: *ufo.font_info.ascender.expect("ascender") as f32,
            capHeight: *ufo.font_info.cap_height.expect("cap_height") as f32,
            // TODO: so very many more
            ..Default::default()
        });

    builder.finish(font_info, None);

    // write binary version
    let buf = builder.finished_data(); 
    let mut fout = File::create(args.outpath)?;
    fout.write_all(buf)?;
    Ok(())
}

fn seconds_str(duration: Duration) -> String {
    format!("{}.{}s", duration.as_secs(), duration.subsec_millis())
}

fn to_f32(v: f64) -> f32 {
    return v as f32;
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
