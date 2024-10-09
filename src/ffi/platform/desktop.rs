use crate::ffi::ffi_trait::{Kv, KvTrait};

use directories::ProjectDirs;
use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

const QUALIFIER: &str = "qualifier";
const ORGANIZATION: &str = "organization";
const APPLICATION: &str = "lucky_ball";
const KV_DIR: &str = "kv";

impl KvTrait for Kv {
    fn set(key: &str, value: &str) {
        if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            let path = proj_dirs.data_dir();
            let mut file_path = PathBuf::from(path);
            file_path.push(KV_DIR);

            if let Err(e) = std::fs::create_dir_all(&file_path) {
                println!("dir create fail: {}", e);
                return;
            }

            file_path.push(format!("{key}"));

            let mut file = match std::fs::File::create(&file_path) {
                Ok(file) => file,
                Err(e) => {
                    println!("file create fail err: {}", e);
                    return;
                }
            };

            if let Err(e) = file.write_all(value.as_bytes()) {
                println!("file write fail err: {}", e);
                return;
            }
        } else {
            println!("proj dir fail");
        }
    }

    fn get(key: &str) -> String {
        if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            let path = proj_dirs.data_dir();
            let mut file_path = PathBuf::from(path);
            file_path.push(KV_DIR);
            file_path.push(key);

            let mut file = match std::fs::File::open(&file_path) {
                Ok(file) => file,
                Err(e) => {
                    println!("file open fail err: {}", e);
                    return String::new();
                }
            };

            let mut contents = String::new();
            if let Err(e) = file.read_to_string(&mut contents) {
                println!("file read fail err: {}", e);
                return String::new();
            }

            contents
        } else {
            println!("proj dir fail");
            String::new()
        }
    }

    fn delete(key: &str) {
        if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            let path = proj_dirs.data_dir();
            let mut file_path = PathBuf::from(path);
            file_path.push(KV_DIR);
            file_path.push(key);

            // let mut file = match std::fs::File::open(&file_path) {
            //     Ok(file) => file,
            //     Err(e) => {
            //         println!("file open fail err: {}", e);
            //         return ();
            //     }
            // };

            match fs::remove_file(file_path.clone()) {
                Ok(_) => {
                    println!("파일이 성공적으로 삭제되었습니다: {:?}", file_path);
                }
                Err(e) => {
                    println!("파일을 삭제하는 데 실패했습니다: {}", e);
                }
            }

            // let mut contents = String::new();
            // if let Err(e) = file.read_to_string(&mut contents) {
            //     println!("file read fail err: {}", e);
            //     return String::new();
            // }

            // contents
        } else {
            println!("proj dir fail");
        }
    }

    fn exists(key: &str) -> bool {
        if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            let path = proj_dirs.data_dir();
            let mut file_path = PathBuf::from(path);
            file_path.push(KV_DIR);
            file_path.push(key);

            let mut file = match std::fs::File::open(&file_path) {
                Ok(file) => file,
                Err(e) => {
                    println!("file open fail err: {}", e);
                    return false;
                }
            };

            // let mut contents = String::new();
            // if let Err(e) = file.read_to_string(&mut contents) {
            //     println!("file read fail err: {}", e);
            //     return String::new();
            // }

            true
        } else {
            println!("proj dir fail");
            false
        }
    }
}

#[test]
pub fn get_test() {
    let a = Kv::get("test");
    println!("a: {a:?}");
}
#[test]
pub fn set_test() {
    Kv::set("test", "value");
}
#[test]
pub fn delete_test() {
    Kv::delete("test");
}
#[test]
pub fn exists_test() {
    let a = Kv::exists("test");
    println!("a: {a:?}");
}
