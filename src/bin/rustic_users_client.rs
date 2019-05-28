extern crate futures;
extern crate grpc;
extern crate httpbis;
extern crate rustic_users;

extern crate env_logger;

use std::sync::Arc;

use rustic_users::rustic::*;
use rustic_users::rustic_grpc::*;

use grpc::Client;
use grpc::ClientStub;

fn main() {
    env_logger::init();

    let _port = 50051;

    //    let client_conf = Default::default();

    //    let grpc_client = Client::new_plain("::1", 50051, client_conf);
    let client = Arc::new(Client::new_plain("::1", _port, Default::default()).unwrap());

    let user_client = UserServiceClient::with_client(client);

    let req = GetUsersRequest::new();

    let resp = user_client.get_users(grpc::RequestOptions::new(), req);

    println!("{:?}", resp.wait());
}
