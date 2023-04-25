use std::{fs::File, io, path::PathBuf};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");

#[allow(unused_variables)]
fn main() -> io::Result<()> {
    let file_path = PathBuf::from(format!("{ROOT}/assets/autumn_1440x960.heic"));
    let mut file = File::open(file_path)?;

    heif_lib::parse_file(&mut file)?;

    Ok(())
}
