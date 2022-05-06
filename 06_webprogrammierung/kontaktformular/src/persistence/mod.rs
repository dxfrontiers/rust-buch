pub mod request_repo;
use rocket_sync_db_pools::diesel::SqliteConnection;
use rocket_sync_db_pools::database;


#[database("sqlite_db")]
pub struct DbCon(SqliteConnection);