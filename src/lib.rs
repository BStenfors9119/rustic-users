#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate grpc_protobuf;
extern crate protobuf;
extern crate tls_api;

pub mod rustic;
pub mod rustic_grpc;

pub mod schema;
pub mod users;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
