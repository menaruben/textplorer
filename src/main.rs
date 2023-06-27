use inquire::Select;
use std::{
    env,
    fs::{self, metadata},
    path::PathBuf,
    process::Command,
};

fn get_absolute_path(path: &str) -> String {
    let abs_path = fs::canonicalize(PathBuf::from(path));
    let mut abs_path_string = abs_path.unwrap().as_path().to_string_lossy().into_owned();

    // remove windows long path prefix (\\?\....) Ex: '\\?\C:\repos\'
    if abs_path_string.starts_with(r"\\?\") {
        abs_path_string = String::from(&abs_path_string[r"\\?\".len()..]);
    }

    abs_path_string
}

fn get_dir_entries(path: &str) -> Vec<String> {
    // println!("{}", path);
    let entries = fs::read_dir(path).unwrap();
    let mut paths: Vec<String> = Vec::new();

    paths.push("..".to_string());

    for entry in entries {
        let path_str = entry.unwrap().path().to_string_lossy().into_owned();
        paths.push(path_str);
    }

    paths
}

#[warn(unused_assignments)]
fn get_cwd() -> String {
    env::current_dir()
        .unwrap()
        .as_path()
        .to_string_lossy()
        .into_owned()
}

fn clearscreen() {
    // clear screen and position cursor at row 1 column 1
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    let mut cwd = get_cwd();
    let mut paths: Vec<String> = Vec::new();

    loop {
        paths = get_dir_entries(&cwd);
        let ans = Select::new("", paths.clone()).prompt();

        match ans {
            Ok(choice) => {
                if metadata(&choice).unwrap().is_dir() {
                    cwd = get_absolute_path(&choice);
                    let _ = env::set_current_dir(cwd.clone());
                } else {
                    if cfg!(target_os = "windows") {
                        let _ = Command::new("cmd")
                            .args(["/C", "start", "hx", &choice])
                            .spawn();
                    } else {
                        let _ = Command::new("sh").args(["-c", "hx", &choice]).spawn();
                    };
                };
            }
            Err(_) => {
                clearscreen();
                std::process::exit(0);
            }
        }
    }
}
