mod model;

use model::{Content, Layout, Manifest};
use std::borrow::Cow;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::{env, io};
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
struct Opt {
    path: Option<PathBuf>,

    #[structopt(short, long)]
    force: bool,
}

impl Opt {
    fn path(&self) -> io::Result<Cow<Path>> {
        match &self.path {
            Some(path) => Ok(Cow::from(path)),
            None => Ok(Cow::from(env::current_dir()?)),
        }
    }
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let path = opt.path()?;

    let (mut manifest, mut layout) = read_manifest_and_layout(&path)?;
    manifest.set_total_package_size(layout.set_content(walk_files(&path)));

    if opt.force {
        write_package_metadata(&path, &manifest, &layout)?;
    } else {
        print_package_metadata(&manifest, &layout)?;
    }

    Ok(())
}

fn read_manifest_and_layout(path: &Path) -> io::Result<(Manifest, Layout)> {
    let manifest = serde_json::from_reader(File::open(path.join("manifest.json"))?)?;
    let layout = serde_json::from_reader(File::open(path.join("layout.json"))?)?;
    Ok((manifest, layout))
}

fn walk_files<'a>(path: &'a Path) -> impl Iterator<Item = Content> + 'a {
    walkdir::WalkDir::new(path)
        .into_iter()
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
    println!("Manifest:\n{}", manifest);
    println!("Layout:\n{}", layout);
    Ok(())
}
