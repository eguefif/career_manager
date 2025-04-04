use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./html/website/");
    copy_dir(&path)
}

fn copy_dir(path: &Path) {
    if path.is_dir() {
        let dir_list = fs::read_dir(path).unwrap();
        for file in dir_list {
            let file = file.unwrap().path();
            if file.is_dir() {
                if let Some(new_file) = file.to_str() {
                    let new_path = new_file.replace("website", "dist_test");
                    println!("DIR: {:?}", new_path);
                    let _ = fs::create_dir(new_path);
                }
                copy_dir(&file);
            } else {
                if let Some(file) = file.to_str() {
                    let new_path = file.replace("website", "dist_test");
                    let _ = fs::copy(file, new_path.clone());
                    println!("{:?}", new_path);
                }
            }
        }
    }
}
