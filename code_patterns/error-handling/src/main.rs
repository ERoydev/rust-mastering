// ERROR HANDLING GOOD PRACTICES

mod error;
mod fs;

pub use self::error::{Error, Result};
use crate::fs::list_files;

fn main() -> Result<()> {
    let files = list_files(".")?;
    println!("{files:#?}");

    Ok(())
}