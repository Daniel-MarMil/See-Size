use std::{
    ffi::OsString,
    fs::ReadDir,
    io::{Error, ErrorKind},
    path::{Path, PathBuf, absolute},
};

#[derive(Debug)]
pub enum AnalyzeDirectoryError {
    PermissionDenied,
    NotADirectory,
    NotFound,
    InvalidInput,
    UnexpectedError(Error),
}

pub struct Node {
    pub name: OsString,
    pub parent: Option<PathBuf>,
    pub error: Option<AnalyzeDirectoryError>,
}

fn calculate_error(err: Error) -> AnalyzeDirectoryError {
    match err.kind() {
        ErrorKind::PermissionDenied => AnalyzeDirectoryError::PermissionDenied,
        ErrorKind::NotADirectory => AnalyzeDirectoryError::NotADirectory,
        ErrorKind::NotFound => AnalyzeDirectoryError::NotFound,
        ErrorKind::InvalidInput => AnalyzeDirectoryError::InvalidInput,
        _ => AnalyzeDirectoryError::UnexpectedError(err),
    }
}

fn add_node(node: Node, tree_node: &mut Vec<Node>, sub_father: &mut usize) {
    tree_node.push(node);
    *sub_father = *sub_father + 1;
}

fn create_node(
    name: OsString,
    parent: Option<PathBuf>,
    error: Option<AnalyzeDirectoryError>,
) -> Node {
    Node {
        name: (name),
        parent: (parent),
        error: (error),
    }
}

fn init_create_tree_node(dir: ReadDir, initpath: &OsString, tree_node: &mut Vec<Node>) {
    let mut sub_father_indice: usize = 0;
    add_node(
        create_node(initpath.clone(), None, None),
        tree_node,
        &mut sub_father_indice,
    );
    generate_tree_node(
        dir,
        PathBuf::from(initpath),
        tree_node,
        &mut sub_father_indice,
    )
}

fn reed_sub_dir(
    dir: (PathBuf, usize),
    tree_node: &mut Vec<Node>,
    list_dir: &mut Vec<(PathBuf, usize)>,
    sub_father: &mut usize,
) {
    let path = dir.0;
    let localitation = dir.1;
    match std::fs::read_dir(&path) {
        Ok(dir_read) => {
            for entry in dir_read {
                match entry {
                    Ok(dir_entry) => {
                        if dir_entry.path().is_dir() && !dir_entry.path().is_symlink() {
                            // It's a dir
                            list_dir.push((dir_entry.path(), *sub_father));
                        }
                        add_node(
                            create_node(dir_entry.file_name(), Some(path.clone()), None),
                            tree_node,
                            sub_father,
                        )
                    }
                    Err(err) => match tree_node.get_mut(localitation) {
                        Some(value) => value.error = Some(calculate_error(err)),
                        None => continue,
                    },
                }
            }
        }
        Err(err) => match tree_node.get_mut(localitation) {
            Some(value) => value.error = Some(calculate_error(err)),
            None => return,
        },
    }
}

fn generate_tree_node(
    dir: ReadDir,
    fatherpath: PathBuf,
    tree_node: &mut Vec<Node>,
    sub_father: &mut usize,
) {
    let mut list_dir: Vec<(PathBuf, usize)> = Vec::new();
    for entry in dir {
        match entry {
            Ok(dir_entry) => {
                if dir_entry.path().is_dir() && !dir_entry.path().is_symlink() {
                    // It's a dir
                    list_dir.push((dir_entry.path(), *sub_father));
                }
                add_node(
                    create_node(dir_entry.file_name(), Some(fatherpath.clone()), None),
                    tree_node,
                    sub_father,
                );
            }
            Err(err) => match tree_node.get_mut(0) {
                Some(value) => value.error = Some(calculate_error(err)),
                None => continue,
            },
        }
    }

    while let Some(dir) = list_dir.pop() {
        reed_sub_dir(dir, tree_node, &mut list_dir, sub_father);
    }
}

pub fn analyze_directory_space(path: &Path) -> Result<Vec<Node>, AnalyzeDirectoryError> {
    match absolute(path) {
        Ok(absolute_path) => match std::fs::read_dir(&absolute_path) {
            Ok(dir) => {
                let mut tree_node: Vec<Node> = Vec::new();
                init_create_tree_node(dir, &absolute_path.into_os_string(), &mut tree_node);
                return Ok(tree_node);
            }
            Err(err) => Err(calculate_error(err)),
        },
        Err(err) => Err(calculate_error(err)),
    }
}

#[cfg(windows)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permission_denied() {
        assert!(matches!(
            analyze_directory_space(Path::new(r"C:\System Volume Information")),
            Err(AnalyzeDirectoryError::PermissionDenied)
        ))
    }
    #[test]
    fn not_adirectory() {
        assert!(matches!(
            analyze_directory_space(Path::new(r"C:\Windows\notepad.exe")),
            Err(AnalyzeDirectoryError::NotADirectory)
        ))
    }
    #[test]
    fn not_found() {
        assert!(matches!(
            analyze_directory_space(Path::new(r"C:\no-existe")),
            Err(AnalyzeDirectoryError::NotFound)
        ))
    }

    /*
        #[test]
        fn no_error() {
            assert!(matches!(
                analyze_directory_space(Path::new(r"C:\Windows")),
                Ok(())
            ))
        }
    */
}
