#![allow(unused)]
use byte_unit::{AdjustedByte, Byte};
use chrono::{DateTime, Local};
use colored::Colorize;
use std::{
    fs::{read_dir, Permissions, ReadDir},
    os::unix::prelude::MetadataExt,
    path::{Path, PathBuf},
    process::exit,
};

#[derive(Debug)]
pub(crate) struct Content {
    inner: Directory,
    dirs: Vec<Directory>,
}

impl Content {
    pub(crate) fn print_inner(&self) {
        let inner = &self.inner;
        for file in &inner.files {
            let out = format!(" {} ", file.path.file_name().unwrap().to_str().unwrap());
            print!("{} ", out);
        }
        for link in &inner.links {
            let out = format!("⇒ {} ", link.path.file_name().unwrap().to_str().unwrap());
            print!("{} ", out);
        }
        for dir in &inner.dirs {
            let out = format!(" {} ", dir.path.file_name().unwrap().to_str().unwrap());
            print!("{} ", out.blue());
        }
    }

    pub(crate) fn print_recurse(&self) {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct EntryDetails {
    permissions: Permissions,
    owner: u32,
    group: u32,
    size: AdjustedByte,
    date: DateTime<Local>,
}

impl From<std::fs::File> for EntryDetails {
    fn from(file: std::fs::File) -> Self {
        let metadata = file.metadata().unwrap();
        Self {
            permissions: metadata.permissions(),
            owner: metadata.uid(),
            group: metadata.gid(),
            size: Byte::from_bytes(metadata.size().into()).get_appropriate_unit(true),
            date: metadata.created().unwrap().into(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct File {
    path: PathBuf,
    details: EntryDetails,
}

#[derive(Debug)]
pub(crate) struct Link {
    path: PathBuf,
    details: EntryDetails,
}

#[derive(Debug)]
pub(crate) struct Directory {
    path: PathBuf,
    details: EntryDetails,
    files: Vec<File>,
    links: Vec<Link>,
    dirs: Vec<Directory>,
}

impl Directory {
    fn new(path: PathBuf, details: EntryDetails) -> Self {
        Self {
            path,
            details,
            files: vec![],
            links: vec![],
            dirs: vec![],
        }
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file)
    }

    fn add_link(&mut self, link: Link) {
        self.links.push(link)
    }

    fn add_directory(&mut self, dir: Directory) {
        self.dirs.push(dir)
    }
}

impl Content {
    pub(crate) fn from_path(path: &Path) -> Self {
        let mut dirs = vec![];

        // Parse self "./"
        let inner = match read_dir(path) {
            Ok(read_dir) => Content::get_dir_content(path.to_path_buf(), read_dir),
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        };

        // Parse rest of the directories
        match read_dir(path) {
            Ok(content) => {
                for entry in content {
                    let entry = entry.unwrap();
                    match entry.file_type() {
                        Ok(file_type) => {
                            if file_type.is_dir() {
                                match read_dir(entry.path()) {
                                    Ok(read_dir) => {
                                        dirs.push(Content::get_dir_content(entry.path(), read_dir))
                                    }
                                    Err(e) => eprintln!("{e}"),
                                }
                            }
                        }
                        Err(e) => eprintln!("{e}"),
                    }
                }
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }

        Self { inner, dirs }
    }

    fn get_dir_content(path: PathBuf, dir: ReadDir) -> Directory {
        let details: EntryDetails = std::fs::File::open(path.clone()).unwrap().into();
        let mut directory = Directory::new(path, details);
        for entry in dir {
            match entry {
                Ok(entry) => match entry.file_type() {
                    Ok(file_type) => {
                        let path = entry.path();
                        let details: EntryDetails =
                            std::fs::File::open(path.clone()).unwrap().into();
                        if file_type.is_dir() {
                            match read_dir(entry.path()) {
                                Ok(read_dir) => directory
                                    .add_directory(Content::get_dir_content(path, read_dir)),
                                Err(e) => eprintln!("{e}"),
                            };
                        } else if file_type.is_file() {
                            directory.add_file(File { path, details });
                        } else {
                            directory.add_link(Link { path, details });
                        }
                    }
                    Err(e) => eprintln!("{e}"),
                },
                Err(e) => eprintln!("{e}"),
            }
        }
        directory
    }

    pub(crate) fn get_dirs(&self) -> impl Iterator<Item = &Directory> {
        self.dirs.iter()
    }
}
