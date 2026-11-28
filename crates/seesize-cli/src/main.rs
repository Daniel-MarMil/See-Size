use seesize_core::AnalyzeDirectoryError;
use seesize_core::analyze_directory_space;
use std::path::PathBuf;

fn print_error(err: &AnalyzeDirectoryError) {
    match err {
        AnalyzeDirectoryError::PermissionDenied => {
            eprintln!("Permission Denied: directory impossible to access")
        }
        AnalyzeDirectoryError::NotADirectory => {
            eprintln!("It's not a directory")
        }
        AnalyzeDirectoryError::NotFound => {
            eprintln!("Directory not found")
        }
        AnalyzeDirectoryError::InvalidInput => {
            eprintln!("Invalid input path")
        }
        AnalyzeDirectoryError::UnexpectedError(error) => eprintln!(
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

    let result = analyze_directory_space(&path);

    match result {
        Ok(_) => println!("Directory opened successfully."),
        Err(err) => print_error(&err),
    }
}
