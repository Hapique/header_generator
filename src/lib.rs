use std::{
    fs::{read_dir, File},
    io,
    io::prelude::*,
    path::PathBuf,
};

pub enum Paths {
    Files(Vec<PathBuf>),
    Folder(PathBuf),
    RecursiveFolder(PathBuf),
}

pub enum GuardStyle {
    Prama,
    Ifndef,
}

pub struct HeaderGenerator {
    paths: Paths,
    guard_style: GuardStyle,
}

impl HeaderGenerator {
    pub fn new(paths: Paths, guard_style: GuardStyle) -> Self {
        Self { paths, guard_style }
    }

    pub fn run(self) {
        match self.paths {
            Paths::Files(files) => println!("{:?}", files),
            Paths::Folder(folder) => println!("{:?}", folder),
            Paths::RecursiveFolder(folder) => println!("{:?}", folder),
        };
    }

    fn read_file(&self, path: PathBuf) -> io::Result<String> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    fn read_files(&self, paths: Vec<PathBuf>) -> Vec<io::Result<String>> {
        paths.into_iter().map(|path| self.read_file(path)).collect()
    }

    fn read_folder(&self, dir: PathBuf) -> io::Result<Vec<io::Result<String>>> {
        let mut files: Vec<io::Result<String>> = vec![];

        if dir.is_dir() {
            for entry in read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    files.extend(self.read_folder(path)?);
                } else {
                    files.push(self.read_file(path));
                }
            }
        }

        Ok(files)
    }
}

impl Default for HeaderGenerator {
    fn default() -> Self {
        Self::new(
            Paths::RecursiveFolder(PathBuf::from(".")),
            GuardStyle::Prama,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default() {
        HeaderGenerator::default();
    }
}
