use crate::structs::Record;
use sqlx::sqlite::SqlitePool;

/// データベースハンドラ
///
/// # Fields
/// * `pool` - データベースプール
/// * `table_name` - テーブル名
/// * `dir_name` - ディレクトリ名
///
/// # Note
/// `dir_name`はディレクトリのパスではなく、ディレクトリ名を表します。
///
pub struct Handler<'a> {
    pool: SqlitePool,
    table_name: &'a str,
    dir_name: &'a str,
}

/// データベースハンドラの実装
///
/// # Methods
/// * `new` - データベースハンドラを初期化
/// * `get` - データベースからレコードを取得
/// * `update` - データベースのレコードを更新
/// * `insert` - データベースにレコードを追加
///
impl<'a> Handler<'a> {
    pub async fn new(
        db_url: &'a str,
        table_name: &'a str,
        dir_name: &'a str,
    ) -> Result<Self, sqlx::Error> {
        Ok(Self {
            pool: SqlitePool::connect(db_url).await?,
            table_name,
            dir_name,
        })
    }

    /// sqlxを使用してデータベースからレコードを取得
    ///
    /// # Returns
    /// `Result<Record, sqlx::Error>` - レコード
    ///
    pub async fn get(&self) -> Result<Record, sqlx::Error> {
        let query = format!(
            r#"SELECT dir_id, dir_name, current_number FROM {} WHERE dir_name="{}";"#,
            self.table_name, self.dir_name
        );
        let record = sqlx::query_as::<_, Record>(&query)
            .fetch_one(&self.pool)
            .await?;
        Ok(record)
    }

    /// sqlxを使用してデータベースのレコードを更新
    ///
    /// # Arguments
    /// * `current_number` - 現在の番号
    ///
    pub async fn update(&self, current_number: i64) -> Result<(), sqlx::Error> {
        let query = format!(
            r#"UPDATE {} SET current_number={} WHERE dir_name="{}";"#,
            self.table_name, current_number, self.dir_name
        );
        sqlx::query(&query).execute(&self.pool).await?;
        Ok(())
    }

    /// sqlxを使用してデータベースにレコードを追加
    ///
    /// # Arguments
    /// * `current_number` - 現在の番号
    ///
    pub async fn insert(&self, current_number: i64) -> Result<(), sqlx::Error> {
        let query = format!(
            r#"INSERT INTO {} (dir_name, current_number) 
            SELECT "{}", {} WHERE NOT EXISTS 
            (SELECT * FROM {} WHERE dir_name = "{}");"#,
            self.table_name, self.dir_name, current_number, self.table_name, self.dir_name
        );
        sqlx::query(&query).execute(&self.pool).await?;
        Ok(())
    }
}
