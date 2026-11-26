use std::{
    io::{Error, ErrorKind},
    path::Path,
};

#[derive(Debug)]
pub enum ErrorOnFile {
    PermissionDenied,
    NotADirectory,
    NotFound,
    UnexpectedError(Error),
}

fn transform_to_error_on_file(err: Error) -> ErrorOnFile {
    match err.kind() {
        ErrorKind::PermissionDenied => ErrorOnFile::PermissionDenied,
        ErrorKind::NotADirectory => ErrorOnFile::NotADirectory,
        ErrorKind::NotFound => ErrorOnFile::NotFound,
        _ => ErrorOnFile::UnexpectedError(err),
    }
}

pub fn open_directory(path: &Path) -> Result<(), ErrorOnFile> {
    match std::fs::read_dir(path) {
        Ok(_) => Ok(()),
        Err(err) => Err(transform_to_error_on_file(err)),
    }
}

#[cfg(windows)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permission_denied() {
        assert!(matches!(
            open_directory(Path::new(r"C:\System Volume Information")),
            Err(ErrorOnFile::PermissionDenied)
        ))
    }
    #[test]
    fn not_adirectory() {
        assert!(matches!(
            open_directory(Path::new(r"C:\Windows\notepad.exe")),
            Err(ErrorOnFile::NotADirectory)
        ))
    }
    #[test]
    fn not_found() {
        assert!(matches!(
            open_directory(Path::new(r"C:\no-existe")),
            Err(ErrorOnFile::NotFound)
        ))
    }
    #[test]
    fn no_error() {
        assert!(matches!(
            open_directory(Path::new(r"C:\Windows")),
            Ok(())
        ))
    }
}
