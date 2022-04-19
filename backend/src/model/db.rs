use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use super::credentials::*;

const PG_APP_MAX_CON: u32 = 5;

pub type Db = Pool<Postgres>;

async fn new_db_pool(
  host: &str,
  db: &str,
  user: &str,
  pwd: &str,
  max_con: u32,
) -> Result<Db, sqlx::Error> {
  let con_string = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
  
  PgPoolOptions::new()
    .max_connections(max_con)
    .connect_timeout(Duration::from_millis(500))
    .connect(&con_string)
    .await
}

pub async fn init_db() -> Result<Db, sqlx::Error> {
  // // -- Create the db with PG_ROOT (dev only)
  // {
  //   let root_db = new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PWD, 1).await?;
  //   pexec(&root_db, SQL_RECREATE).await?;
  // }

  // // -- Run the app sql files
  // let app_db = new_db_pool(PG_HOST, PG_APP_DB, PG_APP_USER, PG_APP_PWD, 1).await?;
  // let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
  //   .into_iter()
  //   .filter_map(|e| e.ok().map(|e| e.path()))
  //   .collect();
  // paths.sort();
  // // execute each file
  // for path in paths {
  //   if let Some(path) = path.to_str() {
  //     // only .sql and not the recreate
  //     if path.ends_with(".sql") && path != SQL_RECREATE {
  //       pexec(&app_db, &path).await?;
  //     }
  //   }
  // }

  // returning the app db
  new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PWD, 1).await
}


// region:    Test
#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;
// endregion: Test