use std::{
    borrow::Cow,
    env,
    fs::File,
    io,
    path::{Path, PathBuf},
    process,
};

mod model;

use clap::Parser;
use model::{Content, Layout, Manifest};

/// Update manifest.json and layout.json
#[derive(Debug, Parser)]
struct Args {
    /// The path to your package. Defaults to your current directory
    path: Option<PathBuf>,

    /// Set this flag to overwrite existing manifest and layout files. Defaults to stdout
    #[arg(short, long)]
    force: bool,
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
    let (manifest, mut layout) = read_manifest_and_layout(&path)?;
    layout.set_content(walk_files(&path));
    // manifest.set_total_package_size(layout.set_content(walk_files(&path)));

    if args.force {
        write_package_metadata(&path, &manifest, &layout)?;
    } else {
        print_package_metadata(&manifest, &layout)?;
    }

    Ok(())
}

fn read_manifest_and_layout(path: &Path) -> io::Result<(Manifest, Layout)> {
    let manifest = serde_json::from_reader(File::open(path.join("manifest.json"))?)?;
    
    let layout = path.join("layout.json");
    let layout = if layout.exists() {
        serde_json::from_reader(File::open(path.join("layout.json"))?)?
    } else {
        Layout::default()
    };
    
    Ok((manifest, layout))
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

fn write_package_metadata(path: &Path, manifest: &Manifest, layout: &Layout) -> io::Result<()> {
    serde_json::to_writer_pretty(&mut File::create(path.join("manifest.json"))?, manifest)?;
    serde_json::to_writer_pretty(&mut File::create(path.join("layout.json"))?, layout)?;
    Ok(())
}

fn print_package_metadata(manifest: &Manifest, layout: &Layout) -> io::Result<()> {
    let manifest = serde_json::to_string_pretty(&manifest)?;
    let layout = serde_json::to_string_pretty(&layout)?;

    println!("Manifest:\n{manifest}");
    println!("Layout:\n{layout}");

    Ok(())
}
