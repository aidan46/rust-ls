#![allow(unused)]
use byte_unit::{AdjustedByte, Byte};
use chrono::{DateTime, Local};
use colored::{ColoredString, Colorize};
use std::{
    cmp::Ordering,
    fs::{read_dir, Permissions, ReadDir},
    os::unix::prelude::MetadataExt,
    path::{Path, PathBuf},
    process::exit,
};

use crate::Sorting;

#[derive(Debug)]
pub(crate) struct Content {
    inner: Directory,
    dirs: Vec<Directory>,
}

impl Content {
    pub(crate) fn print_short(&self, all: bool, sorting: Sorting) {
        if let Sorting::Reverse = sorting {
            self.print_short_reverse(all);
            return;
        }
        let inner = &self.inner;
        let hidden = self.collect_hidden();
        let files = self.collect_files();
        let links = self.collect_links();
        let dirs = self.collect_dirs();
        if all {
            for elem in hidden {
                print!("{elem} ");
            }
        }
        for file in files {
            print!("{} ", file);
        }
        for link in links {
            print!("{} ", link);
        }
        for dir in dirs {
            print!("{} ", dir);
        }
    }

    fn print_short_reverse(&self, all: bool) {
        let inner = &self.inner;
        let hidden = {
            let mut hidden = self.collect_hidden();
            hidden.reverse();
            hidden
        };
        let files = {
            let mut files = self.collect_files();
            files.reverse();
            files
        };

        let links = {
            let mut links = self.collect_links();
            links.reverse();
            links
        };
        let dirs = {
            let mut dirs = self.collect_dirs();
            dirs.reverse();
            dirs
        };

        for dir in dirs {
            print!("{} ", dir);
        }
        for link in links {
            print!("{} ", link);
        }
        for file in files {
            print!("{} ", file);
        }
        if all {
            for elem in hidden {
                print!("{elem} ");
            }
        }
    }

    fn collect_hidden(&self) -> Vec<ColoredString> {
        let mut hidden = vec![];
        let inner = &self.inner;
        hidden.push(" .".blue().bold());
        hidden.push(" ..".blue().bold());

        for dir in &inner.dirs {
            let name = dir.path.file_name().unwrap().to_str().unwrap();
            if name.starts_with('.') {
                let out = format!(" {}", name);
                hidden.push(out.blue().bold());
            }
        }
        for file in &inner.files {
            let name = file.path.file_name().unwrap().to_str().unwrap();
            if name.starts_with('.') {
                let out = format!(" {}", name);
                hidden.push(out.normal());
            }
        }
        hidden
    }

    fn collect_files(&self) -> Vec<ColoredString> {
        let mut files = vec![];

        for file in &self.inner.files {
            let name = file.path.file_name().unwrap().to_str().unwrap();
            if !name.starts_with('.') {
                let out = format!(" {}", name);
                files.push(out.normal());
            }
        }
        files
    }

    fn collect_links(&self) -> Vec<ColoredString> {
        let mut links = vec![];

        for link in &self.inner.links {
            let name = link.path.file_name().unwrap().to_str().unwrap();
            if !name.starts_with('.') {
                let out = format!("⇒ {} ", name);
                links.push(out.yellow());
            }
        }
        links
    }

    fn collect_dirs(&self) -> Vec<ColoredString> {
        let mut dirs = vec![];

        for dir in &self.inner.dirs {
            let name = dir.path.file_name().unwrap().to_str().unwrap();
            if !name.starts_with('.') {
                let out = format!(" {} ", name);
                dirs.push(out.blue().bold());
            }
        }
        dirs
    }

    pub(crate) fn print_long(&self, all: bool) {
        todo!()
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
        let mut inner = match read_dir(path) {
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

        inner
            .files
            .sort_by(|a, b| a.path.file_name().unwrap().cmp(b.path.file_name().unwrap()));
        inner
            .dirs
            .sort_by(|a, b| a.path.file_name().unwrap().cmp(b.path.file_name().unwrap()));
        dirs.sort_by(|a, b| a.path.file_name().unwrap().cmp(b.path.file_name().unwrap()));

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
