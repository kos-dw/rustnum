use clap::Parser;
use sqlx::FromRow;

/// 環境変数、及びコマンドライン引数をパースするための構造体
///
/// # Fields
/// * `database` - データベースURL
/// * `table` - テーブル名
/// * `root` - ルートディレクトリ
///
/// # Note
/// `Parser`を使用してコマンドライン引数をパースしています。
///
#[derive(Debug, Parser)]
#[command(
    version,
    about = "A tool that numbers directory names based on database counts."
)]
pub struct Env {
    /// Database URL
    #[arg(short, long, env = "RUSTNUM_DATABASE_URL", hide_env_values = true)]
    pub database: String,

    /// Table name
    #[arg(short, long, env = "RUSTNUM_TABLE_NAME", hide_env_values = true)]
    pub table: String,

    /// Root directory
    #[arg(
        short,
        long,
        env = "RUSTNUM_CREATE_ROOT",
        hide_env_values = true,
        default_value = "."
    )]
    pub root: String,
}

///  テーブルのスキーマを表すための構造体
///
/// # Fields
/// * `dir_id` - ディレクトリID
/// * `dir_name` - ルートディレクトリ名
/// * `current_number` - 現在の番号
///
/// # Note
/// テーブルのスキーマを表すために`FromRow`を実装しています。  
/// また、`dir_name`はディレクトリのパスではなく、ディレクトリ名を表します。
///
#[derive(FromRow, Debug)]
pub struct Record {
    pub dir_id: i64,
    pub dir_name: String,
    pub current_number: i64,
}
