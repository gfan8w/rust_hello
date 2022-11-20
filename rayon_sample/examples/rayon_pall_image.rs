// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

use std::fs::create_dir_all;
use std::io::Error;
use std::path::Path;
use error_chain::{bail, quick_main};
use glob::{glob_with, MatchOptions, PatternError};
use image::imageops::FilterType;
use rayon::prelude::*;
use std::io::Write;
use std::time::Instant; // trait which holds `display_chain`


// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    use error_chain::error_chain;
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {}
}

// This only gives access within this module. Make this `pub use errors::*;`
// instead if the types must be accessible from other modules (e.g., within
// a `links` section).
use errors::*;



fn main_1() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

// The above main gives you maximum control over how the error is
// formatted. If you don't care (i.e. you want to display the full
// error during an assert) you can just call the `display_chain` method
// on the error object
#[allow(dead_code)]
fn alternative_main() {
    if let Err(ref e) = run() {
        use error_chain::ChainedError; // trait which holds `display_chain`
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "{}", e.display_chain()).expect(errmsg);
        ::std::process::exit(1);
    }
}

// Use this macro to auto-generate the main above. You may want to
// set the `RUST_BACKTRACE` env variable to see a backtrace.
// quick_main!(run);

// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.
fn run() -> Result<()> {
    use std::fs::File;

    // This operation will fail
    File::open("tretrete").chain_err(|| "unable to open tretrete file")?;

    Ok(())
}

fn main() -> errors::Result<()> {
    rayon_parallel()
}

/*并行任务处理*/
fn rayon_parallel() -> errors::Result<()>{
    use error_chain::ChainedError;

    let opt: MatchOptions = MatchOptions{
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };



    let files: Vec<_> = glob_with("images/*.jpg", opt).unwrap()
        .flat_map(|x| x.ok())
        .collect();

    if files.is_empty() {
        bail!("No .jpg files found in current directory")
    }

    let thumb_dir = "thumbnails";
    create_dir_all(thumb_dir.to_string()+"/images").unwrap();

    println!("Saving {} thumbnails into '{}'...", files.len(), thumb_dir);
    let start = Instant::now();
    let image_failures: Vec<_> = files
        .par_iter()// 使用rayon的并行库 处理
        // .iter() // 非并行处理
        .map(|path|{
            make_thumbnail(path, thumb_dir, 100)
                .map_err(|e|e.chain_err(||path.display().to_string()))
        }).filter_map(|x|x.err())
        .collect();
    let elapsed = (Instant::now() - start).as_millis(); // start.elapsed().as_millis()
    println!("cost time:{}ms", elapsed);

    image_failures.iter().for_each(|x| println!("{}", x.display_chain()));

    Ok(())
}

fn make_thumbnail<PA, PB>(original: PA,
                          thumb_dir: PB,
                          longest_edge: u32) -> Result<()>
where
  PA: AsRef<Path>,
  PB: AsRef<Path>,
{
    let file_path = thumb_dir.as_ref().join(original.as_ref());
    println!("process:{:?} save to :{:?}", original.as_ref(), file_path);
    let img = image::open(original.as_ref()).unwrap();
     img.resize(longest_edge, longest_edge, FilterType::Nearest)
         .save(file_path).unwrap();
    Ok(())
}