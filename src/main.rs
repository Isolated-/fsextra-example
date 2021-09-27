use fsextra::crypto::digest::{Digest, SHA_256};
use fsextra::extensions::{FileExtended, MetadataExtended};
use std::fs::File;
use std::io::{Result, Write};

fn main() -> Result<()> {
    let plaintext = b"hello world!";
    let mut digest = Digest::new(SHA_256);

    digest.write(plaintext)?;
    let result = digest.finish();

    assert_eq!(
        hex::encode(&result),
        "7509e5bda0c762d2bac7f90d758b5b2263fa01ccbc542ab5e3df163be08e6ca9"
    );

    let mut file = File::open("plaintext.txt")?;
    let meta = file.metadata()?;
    let digest = file.digest(SHA_256)?;

    assert_eq!(hex::encode(digest), hex::encode(result));
    assert_eq!(meta.is_executable(), false);

    Ok(())
}
