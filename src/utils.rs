//! ユーティリティ関数を提供するモジュール
//! - get_input: 標準入力を取得
//! - create_pool: sqlxを使用してデータベースのプールを作成
//! - get_records: sqlxを使用してデータベースからレコードを取得
//! - update_record: sqlxを使用してデータベースのレコードを更新
//! - dir_fuctory: データベースから取得した現在の連番を元に、ディレクトリを作成
//!

use crate::structs::Record;
use sqlx::sqlite::SqlitePool;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

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
    let root_name = path.file_name().unwrap().to_string_lossy().into_owned();

    // ディレクトリ新規作成フラグを取得
    let is_new = !path.exists();

    // ルートディレクトリが存在しない場合、新規でディレクトリを作成
    if is_new {
        fs::create_dir_all(&path).unwrap();
    }

    // ルートディレクトリのパスを取得
    let root_path = path.canonicalize().unwrap();

    // ルートディレクトリ名を連番とプロジェクト名に分割
    let initial_number = root_name.split('_').collect::<Vec<&str>>()[0]
        .parse::<i64>()
        .unwrap();

    (root_path, initial_number, is_new)
}

/// sqlxを使用してデータベースのプールを作成
///
/// # Arguments
/// * `db_url` - データベースのURL
///
/// # Returns
/// `Result<SqlitePool, sqlx::Error>` - データベースのプール
///
pub async fn create_pool(db_url: &String) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePool::connect(db_url).await?;
    Ok(pool)
}

/// sqlxを使用してデータベースからレコードを取得
///
/// # Arguments
/// * `pool` - データベースのプール
/// * `table_name` - テーブル名
/// * `dir_name` - ルートディレクトリ名
///
/// # Returns
/// `Result<Record, sqlx::Error>` - レコード
///
/// # Note
/// `dir_name`はディレクトリのパスではなく、ディレクトリ名を表します。
///  
pub async fn get_records(
    pool: &SqlitePool,
    table_name: &str,
    dir_name: &str,
) -> Result<Record, sqlx::Error> {
    let query = format!(
        r#"SELECT dir_id, dir_name, current_number FROM {} WHERE dir_name="{}";"#,
        table_name, dir_name
    );
    let record = sqlx::query_as::<_, Record>(&query).fetch_one(pool).await?;
    Ok(record)
}

/// sqlxを使用してデータベースのレコードを更新
///
/// # Arguments
/// * `pool` - データベースのプール
/// * `table_name` - テーブル名
/// * `dir_name` - ルートディレクトリ名
/// * `current_number` - 現在の番号
///
/// # Note
/// `dir_name`はディレクトリのパスではなく、ディレクトリ名を表します。
///
pub async fn update_record(
    pool: &SqlitePool,
    table_name: &str,
    dir_name: &str,
    current_number: i64,
) -> Result<(), sqlx::Error> {
    let query = format!(
        r#"UPDATE {} SET current_number={} WHERE dir_name="{}";"#,
        table_name, current_number, dir_name
    );
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

/// sqlxを使用してデータベースにレコードを追加
///
/// # Arguments
/// * `pool` - データベースのプール
/// * `table_name` - テーブル名
/// * `dir_name` - ルートディレクトリ名
/// * `current_number` - 現在の番号
///
/// # Note
/// `dir_name`はディレクトリのパスではなく、ディレクトリ名を表します。
///
pub async fn insert_record(
    pool: &SqlitePool,
    table_name: &str,
    dir_name: &str,
    current_number: i64,
) -> Result<(), sqlx::Error> {
    let query = format!(
        r#"INSERT INTO {} (dir_name, current_number) 
        SELECT "{}", {} WHERE NOT EXISTS 
        (SELECT * FROM {} WHERE dir_name = "{}");"#,
        table_name, dir_name, current_number, table_name, dir_name
    );
    sqlx::query(&query).execute(pool).await?;
    Ok(())
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
