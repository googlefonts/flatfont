use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::env;
use std::fs::File;
use std::io::Write;

use norad::Font;

//use ufoff_macros::create_from_ufo;

// import the flatbuffers runtime library
extern crate flatbuffers;
 
// import the flatbuffer generated code
#[allow(dead_code, unused_imports)]
#[path = "./flatfont_generated.rs"]
mod flatbuffer_generated;
pub use flatbuffer_generated::flat_font::{FlatFont, FlatFontArgs, MetaInfo, MetaInfoArgs};

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
    let mut mb = flatbuffers::FlatBufferBuilder::with_capacity(1024);

    // WIP: I don't want to hand-write field copies damnit
    //let mm = create_from_ufo!(mb, ufo.meta, MetaInfo);

    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);

    let creator = ufo.meta.creator.map(|c| builder.create_string(&c));

    let meta = MetaInfo::create(&mut builder,
        &MetaInfoArgs {
            creator: creator,
            format_version: ufo.meta.format_version as u16,
            format_version_minor: ufo.meta.format_version_minor,
            ..Default::default()
        });

    let flat_font = FlatFont::create(&mut builder,
        &FlatFontArgs {
            meta: Some(meta),
            ..Default::default()
        });
    builder.finish(flat_font, None);

    // write binary version
    let buf = builder.finished_data(); 
    let mut fout = File::create(args.outpath)?;
    fout.write_all(buf)?;
    Ok(())
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