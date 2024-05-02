//! データベースに保存されている現在のディレクトリ番号を取得し、連番でディレクトリを作成します。
//!
//! # Example
//!
//! ```bash
//! $ cargo run -- -d sqlite:./test.db -t numbers -r ./test
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
//! * `--database`, `-d` - Database URL
//! * `--table`, `-t` - Table name
//! * `--root`, `-r` - Root directory

mod structs;
mod utils;

use crate::structs::Env;
use clap::Parser;
use utils::props_provider as provider;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let env = Env::parse();

    // sqlxを使用してデータベースのプールを作成
    let pool = utils::create_pool(&env.database).await?;

    let (root_path, init_num, is_new) = provider(&env.root);

    if is_new {
        println!("Target directory does not exist.");
        println!("Create new directory: {}\n", root_path.display());
    } else {
        println!("Target directory is: {}\n", root_path.display());
    }

    // レコードが存在しなければ新規作成
    utils::insert_record(&pool, &env.table, &root_name, init_num).await?;

    // データベースから現在のディレクトリ番号を取得
    let record = utils::get_records(&pool, &env.table, &root_name).await?;

    // ディレクトリを作成
    let update_num = utils::dir_fuctory(record.current_number, &root_path);

    // データベースのレコード現在のディレクトリ番号を更新
    utils::update_record(&pool, &env.table, &root_name, update_num).await?;

    Ok(())
}
