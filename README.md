# rustnum

データベースに保存されている現在のディレクトリ番号を取得し、連番でディレクトリを作成します。  

## What's this?

sqlite3に保存されている現在のディレクトリ番号を取得し、連番でディレクトリを作成します。作成後、データベースに新しいディレクトリ番号を保存します。  
プロダクトとかでプレフィックスが被らないように、一意の命名規則を設定します。  

## Usage

### データベースの準備
まずは、データベースを準備します。データベースの操作にはcliも含めて、[sqlx](https://github.com/launchbadge/sqlx)を使用します。

#### CLIのインストール

グローバルにsqlx-cliをインストール。
```bash
cargo install sqlx-cli --no-default-features --features sqlite
```

#### データベースの作成

sqlxコマンドを使用してSQLiteデータベースを作成。
```bash
sqlx database create --database-url "sqlite:.db/db.sqlite"
```

#### マイグレーションの作成

以下のコマンドを実行するとmigrationsディレクトリにSQLファイルが生成されます。
```bash
sqlx migrate add -r create_num_table
```

生成されたSQLファイルにDDLを記述します。
```sql
-- ${datetime}_create_num_table.up.sql
CREATE TABLE "NUMBERS" (
 "dir_id" INTEGER NOT NULL UNIQUE,
 "dir_name" TEXT NOT NULL UNIQUE,
 "current_number" INTEGER NOT NULL,
 "created_at" DATETIME NOT NULL DEFAULT (DATETIME('now', 'localtime')),
 "updated_at" DATETIME NOT NULL DEFAULT (DATETIME('now', 'localtime')),
 PRIMARY KEY("dir_id" AUTOINCREMENT)
);

INSERT INTO NUMBERS (dir_name, current_number) VALUES
("00000_test", 0),
("10000_test", 0),
("20000_test", 0);

CREATE TRIGGER trigger_NUMBERS_updated_at AFTER UPDATE ON NUMBERS
BEGIN
    UPDATE NUMBERS SET updated_at = DATETIME('now', 'localtime') WHERE rowid == NEW.rowid;
END;
```

```sql
-- ${datetime}_create_num_table.down.sql
DROP TABLE NUMBERS;
```

#### マイグレーションの実行

以下のコマンドを実行するとマイグレーションが実行されます。
```bash
sqlx migrate run --database-url "sqlite:.db/db.sqlite"

```

### 実行ファイルのビルド

以下のコマンドで実行ファイルをビルドします。
```bash
cargo build --release
```

### 実行

以下のコマンドで実行します。

```bash
./target/release/rustnum -r 10000_test -d .db/db.sqlite -t "NUMBERS"
```

-d, --database データベースまでのパス  
-t, --table テーブル名  
-r, --root ルートディレクトリ  

## Note

環境変数にセットすると、引数を省略できます。

```bash
RUSTNUM_DATABASE_URL=10000_test
RUSTNUM_TABLE_NAME=.db/db.sqlite # 別フォルダで実行する場合は絶対パスで指定
RUSTNUM_CREATE_ROOT="NUMBERS"
```

また、PATHを通しておくと、実行ファイル名だけでどこからでも実行できて便利。

```bash
export PATH=$PATH:/path/to/rustnum

rustnum -r 10000_test -d .db/db.sqlite -t "NUMBERS"
```

