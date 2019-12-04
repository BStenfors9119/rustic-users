extern crate futures;
extern crate grpc;
extern crate httpbis;
extern crate rustic_users;

extern crate env_logger;

use std::sync::Arc;

use rustic_users::rustic::*;
use rustic_users::rustic_grpc::*;

use grpc::ClientStubExt;

fn main() {
    env_logger::init();

    let _port = 50051;

    let client = Arc::new(UserServiceClient::new_plain("::1", _port, Default::default()).unwrap());

    let req = GetUsersRequest::new();

    let resp = client.get_users(grpc::RequestOptions::new(), req);

    println!("{:?}", resp.wait());
}
