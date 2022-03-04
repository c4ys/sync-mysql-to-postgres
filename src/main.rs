use crate::config::load_config;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Column, Row, TypeInfo};
use std::env;
use std::time::Duration;

mod config;

#[derive(Debug, sqlx::FromRow)]
struct ColumnInfo {
    pub column_name: String,
    pub data_type: String,
    pub udt_name: String,
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let config_path = env::current_dir().unwrap();
    let config_path = config_path.join("config.toml");
    if !config_path.exists() {
        panic!("{} is not exist", config_path.as_os_str().to_str().unwrap());
    }
    let config = load_config(&config_path).unwrap();
    let max_connections = config.tables.len();
    let _from = MySqlPoolOptions::new()
        .connect_timeout(Duration::from_millis(1000))
        .max_connections(max_connections as u32)
        .connect(&config.from)
        .await
        .expect("Failed to connect to MySQL");

    let _to = PgPoolOptions::new()
        .connect_timeout(Duration::from_millis(1000))
        .max_connections(max_connections as u32)
        .connect(&config.to)
        .await
        .expect("Failed to connect to PostgreSQL");
    let _database_info: (String,) = sqlx::query_as("SELECT current_database()")
        .fetch_one(&_to)
        .await?;
    for table in config.tables {
        let sql = format!(
            r#"
SELECT
	"column_name","data_type","udt_name"
FROM
	information_schema.COLUMNS 
WHERE
	table_schema = 'public' 
	AND table_catalog = '{}' 
	AND "table_name" = '{}' 
ORDER BY
	ordinal_position
        "#,
            _database_info.0, table,
        );
        println!("{:#}", sql);
        let rows: Vec<ColumnInfo> = sqlx::query_as(&*sql).fetch_all(&_to).await?;
        for row in rows {
            println!("{:#}", row.column_name);
        }
    }
    Ok(())
}
