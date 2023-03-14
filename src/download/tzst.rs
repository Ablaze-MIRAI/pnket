use anyhow::Result;
use tar::Archive;
use zstd::stream::read::Decoder;

pub fn tzst(archive: &str, out: &str) -> Result<()> {
    let file = std::fs::File::open(archive)?;
    let decoder = Decoder::new(file)?;
    let mut archive = Archive::new(decoder);
    archive.unpack(out)?;

    Ok(())
}
