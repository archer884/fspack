use std::{
    borrow::Cow,
    env,
    fs::File,
    io,
    path::{Path, PathBuf},
    process,
};

mod layout;

use clap::Parser;
use layout::{Content, Layout};

/// Update manifest.json and layout.json
#[derive(Debug, Parser)]
struct Args {
    /// The path to your package. Defaults to your current directory
    path: Option<PathBuf>,

    /// print package size
    ///
    /// In the original version of these packages, there was a member in the
    /// package manifest that contained the total size for the files in the
    /// package. This appears to no longer be necessary, but in the vent you
    /// need to update that value, this will print the appropriate size.
    #[arg(short, long)]
    size: bool,

    /// write layout changes
    ///
    /// By default, we just print the new layout file to stdout, but you can
    /// pass this flag to have us overwrite the file directly.
    #[arg(short, long)]
    layout: bool,
}

impl Args {
    fn path(&self) -> io::Result<Cow<Path>> {
        match &self.path {
            Some(path) => Ok(Cow::from(path)),
            None => Ok(Cow::from(env::current_dir()?)),
        }
    }
}

fn main() {
    if let Err(e) = run(&Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: &Args) -> io::Result<()> {
    let path = args.path()?;
    let layout = Layout::new(walk_files(&path));

    if args.size {
        let size = layout.package_size();
        println!("{size}");
        return Ok(());
    }

    if args.layout {
        write_package_metadata(&path, &layout)
    } else {
        print_package_metadata(&layout)
    }
}

fn walk_files(path: &Path) -> impl Iterator<Item = Content> + '_ {
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_entry(|entry| entry.file_type().is_file() || !entry.path().ends_with(".git"))
        .filter_map(move |entry| {
            let entry = entry.ok()?;
            let meta = entry.metadata().ok()?;

            if !meta.is_file()
                || entry.path().ends_with("manifest.json")
                || entry.path().ends_with("layout.json")
            {
                None
            } else {
                Some(Content::new(path, entry.path(), meta))
            }
        })
}

fn write_package_metadata(path: &Path, layout: &Layout) -> io::Result<()> {
    serde_json::to_writer_pretty(&mut File::create(path.join("layout.json"))?, layout)?;
    Ok(())
}

fn print_package_metadata(layout: &Layout) -> io::Result<()> {
    let layout = serde_json::to_string_pretty(&layout)?;
    println!("{layout}");
    Ok(())
}
