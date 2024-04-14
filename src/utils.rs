//! ユーティリティ関数を提供するモジュール
//! - get_input: 標準入力を取得
//! - create_pool: sqlxを使用してデータベースのプールを作成
//! - get_records: sqlxを使用してデータベースからレコードを取得
//! - update_record: sqlxを使用してデータベースのレコードを更新
//! - dir_fuctory: データベースから取得した現在の連番を元に、ディレクトリを作成
//!

use crate::structs::Number;
use sqlx::sqlite::SqlitePool;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

/// 標準入力を取得
///
/// # Returns
///
/// `String` - 標準入力
pub fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
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
/// `Result<Number, sqlx::Error>` - レコード
///
/// # Note
/// `dir_name`はディレクトリのパスではなく、ディレクトリ名を表します。
///  
pub async fn get_records(
    pool: &SqlitePool,
    table_name: &String,
    dir_name: &String,
) -> Result<Number, sqlx::Error> {
    let query = format!(
        r#"SELECT dir_id, dir_name, current_number FROM {} WHERE dir_name="{}";"#,
        table_name, dir_name
    );
    let number = sqlx::query_as::<_, Number>(&query).fetch_one(pool).await?;
    Ok(number)
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
    table_name: &String,
    dir_name: &String,
    current_number: &i64,
) -> Result<(), sqlx::Error> {
    let query = format!(
        r#"UPDATE {} SET current_number={} WHERE dir_name="{}";"#,
        table_name, current_number, dir_name
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
pub fn dir_fuctory(current_number: i64, root_dir: &String) -> i64 {
    let mut new_number = current_number.clone();
    let update_num = loop {
        print!("Enter your project name... \n-> ");
        stdout().flush().unwrap();

        let ipt = get_input();
        if ipt == "exit" {
            println!("\nGoodbye！\n");
            stdout().flush().unwrap();
            break new_number;
        }

        new_number += 1;

        // 連番を5桁でゼロパディングしてパスを作成し、ディレクトリを作成
        let dir_name = format!("{:05}_{}", new_number, ipt);
        let mut path = PathBuf::from(root_dir);
        path.push(&dir_name);
        let dir_path = path.to_string_lossy().into_owned();

        match fs::create_dir_all(&dir_path) {
            Ok(_) => println!("Created directry is: {}\n", &dir_path),
            Err(e) => eprintln!("An error has occurred: {:?}", e),
        };
    };

    update_num
}
