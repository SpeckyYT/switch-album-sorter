mod game_ids;

use std::fs::read_dir;
use std::{fs, path::Path};
use std::path::PathBuf;

use clap::{Command, arg};
use walkdir::WalkDir;

use crate::game_ids::get_game_name;

fn main() {
    let matches = Command::new("switch-album-sorter")
        .args(&[
            arg!(-i --input <ALBUM> "Input Album folder"),
            arg!(-o --output <FOLDER> "Output Sorted folder"),
        ])
        .get_matches();

    let album_dir = PathBuf::from(
        matches
        .get_one::<String>("input")
        .unwrap_or(&String::from("Album"))
    );
    let sorted_dir = PathBuf::from(
        matches
        .get_one::<String>("output")
        .unwrap_or(
            &album_dir
            .join("../Sorted")
            .to_str()
            .unwrap()
            .to_string()
        )
    );

    create_and_check_dirs(&album_dir, &sorted_dir);
    clean_up_sorted(&sorted_dir);
    sort_album(&album_dir, &sorted_dir);
}

fn create_and_check_dirs(album_dir: &Path, sorted_dir: &Path) {
    if album_dir.exists() && !album_dir.is_dir() { panic!("{:?} isn't a folder", album_dir ) }
    if sorted_dir.exists() && !sorted_dir.is_dir() { panic!("{:?} isn't a folder", sorted_dir ) }

    if !album_dir.exists() { panic!("Album folder {:?} doesn't exist", album_dir) }
    if !sorted_dir.exists() { fs::create_dir(&sorted_dir).unwrap() }
}

fn clean_up_sorted(sorted_dir: &Path) {
    match read_dir(sorted_dir) {
        Ok(folders) => {
            folders.filter_map(|dir| match dir {
                Ok(dir) => match dir.file_type().unwrap().is_dir() {
                    true => Some(dir),
                    false => None,
                }
                Err(_) => None,
            })
            .for_each(|dir| {
                let dir = dir.path();

                if let Some(game_name) = game_ids::get_game_name(&dir.file_name().unwrap().to_string_lossy()) {
                    let new_dir = dir.clone().join(format!("../{}", game_name));

                    if !new_dir.exists() {
                        println!("{:?}", new_dir);
                        fs::rename(dir.as_path(), new_dir).unwrap();
                    } else {
                        fs::remove_dir_all(dir).unwrap();
                    }
                }
            });
        }
        Err(_) => {}
    }
}

fn sort_album(album_dir: &Path, sorted_dir: &Path) {
    let quantity = WalkDir::new(&album_dir).into_iter().count();

    for (index, entry) in WalkDir::new(&album_dir).into_iter().enumerate() {
        if let Ok(entry) = entry {
            println!("{esc}[2J{esc}[1;1H{}/{}", index+1, quantity, esc = 27 as char);

            if !entry.path().is_file() { continue }

            let file = entry.path();

            let file_name = file.file_name().unwrap().to_str().unwrap();
            let file_stem = file.file_stem().unwrap().to_str().unwrap();

            if let Some((_, game)) = file_stem.split_once("-") {
                let game_name = get_game_name(game).unwrap_or(game.to_string());
                let game_folder = sorted_dir.join(game_name);
                if !game_folder.exists() { fs::create_dir(&game_folder).unwrap() }
                let game_file = game_folder.join(file_name);
                if game_file.exists() { continue }

                fs::copy(
                    &file,
                    &game_file,
                ).unwrap();
            }
        }
    }
}
