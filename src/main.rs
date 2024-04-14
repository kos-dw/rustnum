//! データベースに保存されている現在のディレクトリ番号を取得し、連番でディレクトリを作成します。
//!
//! # Example
//!
//! ```bash
//! $ cargo run -- --database sqlite:./test.db --table numbers --root ./test
//! ```
//!
//! # Environment Variables
//!
//! * `RUSTNUM_DATABASE_URL` - Database URL
//! * `RUSTNUM_TABLE_NAME` - Table name
//! * `RUSTNUM_CREATE_ROOT` - Root directory
//!
//! # Arguments
//!
//! * `--database` - Database URL
//! * `--table` - Table name
//! * `--root` - Root directory

mod structs;
mod utils;

use crate::structs::Env;
use clap::Parser;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let env = Env::parse();

    // sqlxを使用してデータベースのプールを作成
    let pool = utils::create_pool(&env.database).await?;

    // ターゲットディレクトリとディレクトリ名を取得
    let root = Path::new(&env.root);
    let root = root.canonicalize().unwrap();
    let root_filename = root.file_name().unwrap().to_string_lossy().into_owned();
    let root_path = root.to_string_lossy().into_owned();
    println!("Target directory is: {}\n", root_path);

    // データベースから現在のディレクトリ番号を取得
    let number = utils::get_records(&pool, &env.table, &root_filename).await?;

    // ディレクトリを作成
    let update_num = utils::dir_fuctory(number.current_number, &root_path);

    // データベースのレコード現在のディレクトリ番号を更新
    utils::update_record(&pool, &env.table, &root_filename, &update_num).await?;

    Ok(())
}
