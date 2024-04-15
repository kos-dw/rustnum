use clap::Parser;
use sqlx::FromRow;

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

#[derive(FromRow, Debug)]
pub struct Record {
    /// Directory ID
    pub dir_id: i64,

    /// Directory name
    pub dir_name: String,

    /// Current number
    pub current_number: i64,
}
