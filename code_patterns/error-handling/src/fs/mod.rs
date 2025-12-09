use crate::{Result, Error};

pub fn list_files(path: &str) -> Result<Vec<String>> {
    let files: Vec<String> = std::fs::read_dir(path)? // instead of putting `?` i can use .map_err() like bellow to add custom error
        // .map_err(|ex| format!("error while reading dir: cause {ex}"))?
        .filter_map(|re| re.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();

    if files.is_empty() {
        return Err(Error::FsEmptyFolderError);
    }
    
    Ok(files)
}

#[cfg(test)]
mod tests{
    // Tipicaly i just set an alias when i test
    type Result<T> = core::result::Result<T, Error>;
    type Error = Box<dyn std::error::Error>;

    use super::*;

    #[test]
    fn test_name() -> Result<()> {
        todo!();
    }

}