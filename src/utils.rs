//! ユーティリティ関数を提供するモジュール
use core::panic;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

/// データベース操作モジュール
pub mod db_manager;

/// 標準入力を取得
///
/// # Returns
/// `String` - 標準入力
///
pub fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// ルートディレクトリ情報を取得
///
/// # Arguments
/// * `path_str` - ルートディレクトリのパス
///
/// # Returns
/// `(PathBuf, i64, bool)` - (ルートディレクトリ名, ルートディレクトリの絶対パス, 初期値, ディレクトリ新規作成フラグ)
///
pub fn props_provider(path_str: &str) -> (PathBuf, i64, bool) {
    let path = std::path::Path::new(path_str);

    // ルートディレクトリ名を取得
    let root_name = match path.file_name() {
        Some(name) => name.to_str().unwrap(),
        None => panic!("Root directory name is not found."),
    };

    // ディレクトリ新規作成フラグを取得
    let is_new = !path.exists();

    // ルートディレクトリが存在しない場合、新規でディレクトリを作成
    #[allow(unused_must_use)]
    if is_new {
        fs::create_dir_all(&path);
    }

    // ルートディレクトリのパスを取得
    let root_path = path.canonicalize().unwrap();

    // ルートディレクトリ名を連番とプロジェクト名に分割
    let initial_number = root_name.split('_').collect::<Vec<&str>>()[0]
        .parse::<i64>()
        .unwrap();

    (root_path, initial_number, is_new)
}

/// データベースから取得した現在の連番を元に、ディレクトリを作成
///
/// # Arguments
/// * `current_number` - 現在の番号
/// * `root_dir` - ルートディレクトリ
///
/// # Returns
/// `i64` - 更新された番号
///
pub fn dir_fuctory(current_number: i64, root_dir: &PathBuf) -> i64 {
    let mut new_number = current_number;
    let update_num = loop {
        print!("Enter your project name... \n-> ");
        stdout().flush().unwrap();

        let ipt = get_input();
        if ipt == "exit" {
            println!("\nGoodbye!\n");
            stdout().flush().unwrap();
            break new_number;
        }

        new_number += 1;

        // 連番を5桁でゼロパディングしてパスを作成し、ディレクトリを作成
        let dir_name = format!("{:05}_{}", new_number, ipt);
        let mut path = root_dir.clone();
        path.push(&dir_name);
        // let dir_path = path.to_string_lossy().into_owned();

        match fs::create_dir_all(&path) {
            Ok(_) => println!("Created directry is: {}\n", &path.display()),
            Err(e) => eprintln!("An error has occurred: {:?}", e),
        };
    };

    update_num
}
