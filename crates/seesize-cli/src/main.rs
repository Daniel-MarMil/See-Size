use seesize_core::ErrorOnFile;
use seesize_core::open_directory;
use std::path::PathBuf;

fn print_error(err: &ErrorOnFile) {
    match err {
        ErrorOnFile::PermissionDenied => {
            eprintln!("Permission Denied: directory impossible to access")
        }
        ErrorOnFile::NotADirectory => {
            eprintln!("It's not a directory")
        }
        ErrorOnFile::NotFound => {
            eprintln!("Directory not found")
        }
        ErrorOnFile::UnexpectedError(error) => eprintln!(
            "Unexpected error occurred: directory impossible to access. Error data: {}",
            error
        ),
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut line_readed = String::new();

    println!("Enter the path to open: ");
    stdin
        .read_line(&mut line_readed)
        .expect("Error to read the Path");

    let path: PathBuf = line_readed.trim().into();

    let result = open_directory(&path);

    match result {
        Ok(_) => println!("Directory opened successfully."),
        Err(err) => print_error(&err),
    }
}
