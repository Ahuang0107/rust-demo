use model::*;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::FileOptions;

fn main() -> Result<(), Box<dyn Error>> {
    // zip_dir("./assets/basic_excel", "./assets/basic_excel.xlsx")?;
    // dbg!(ContentTypes {}.to_xml());
    Ok(())
}

fn zip_dir(src_dir: &str, dst_file: &str) -> zip::result::ZipResult<File> {
    let path = Path::new(dst_file);
    let writer = File::create(path).unwrap();

    let walk_dir = walkdir::WalkDir::new(src_dir);
    let it = walk_dir.into_iter();

    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(src_dir)).unwrap();

        if path.is_file() {
            println!("adding file {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()
}
