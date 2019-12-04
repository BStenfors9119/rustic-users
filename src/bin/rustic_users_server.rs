extern crate grpc;
extern crate protobuf;
extern crate rustic_users;

extern crate tls_api;
extern crate tls_api_native_tls;

use actix_web::client::Client;
use futures::Future;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::thread;

use rustic_users::establish_connection;
use rustic_users::rustic::*;
use rustic_users::rustic_grpc::*;
use rustic_users::users::{NewUser, User};

mod proto_converters;

use grpc::ServerHandlerContext;
use grpc::ServerRequestSingle;
use grpc::ServerResponseUnarySink;
use protobuf::{RepeatedField, SingularPtrField};

#[derive(Deserialize)]
struct GithubUser {
    login: String,
    email: Option<String>,
}

struct RusticUsersImpl;

impl UserService for RusticUsersImpl {
    fn get_users(
        &self,
        _: ServerHandlerContext,
        req: ServerRequestSingle<GetUsersRequest>,
        resp: ServerResponseUnarySink<GetUsersResponse>,
    ) -> grpc::Result<()> {
        let mut user_resp = GetUsersResponse::new();
        let connection = establish_connection();
        let results = User::all(&connection);
        let mut user_repeated_field = RepeatedField::new();

        for (i, user) in results.iter().enumerate() {
            user_repeated_field.insert(i, proto_converters::user_to_proto(user))
        }

        println!("Users found: {}", user_repeated_field.len().to_string());

        user_resp.set_users(user_repeated_field);

        resp.finish(user_resp)
    }

    fn get_user_by_email(
        &self,
        o: ServerHandlerContext,
        req: ServerRequestSingle<GetUserByEmailRequest>,
        resp: ServerResponseUnarySink<GetUserResponse>,
    ) -> grpc::Result<()> {
        let mut user_resp = GetUserResponse::new();
        let connection = establish_connection();
        let results = User::get_by_email(req.message.get_email().to_string(), &connection);

        let mut user_by_email = rustic_users::rustic::User::new();
        let small_size = 0;
        if results.len().gt(&small_size) {
            user_by_email = proto_converters::user_to_proto(&results[0]);
        }

        user_resp.set_user(user_by_email);

        resp.finish(user_resp)
    }

    fn get_user_github_info(
        &self,
        o: ServerHandlerContext,
        req: ServerRequestSingle<GetUserGithubInfoRequest>,
        resp: ServerResponseUnarySink<GetUserGithubInfoResponse>,
    ) -> grpc::Result<()> {
        let mut client = reqwest::Client::new();
        let mut user_git_hub = GetUserGithubInfoResponse::new();

        println!(
            "Inside of get_user_github_info: {}",
            req.message.get_token().to_string()
        );

        let res = client
            .get("https://api.github.com/user")
            .header(
                "Authorization",
                format!("Bearer {}", req.message.get_token().to_string()),
            )
            .send();

        match res {
            Ok(mut resp) => {
                println!("Github Auth Res: {}", resp.status());
                match resp.text() {
                    Ok(body) => {
                        println!("Body response: {}", body);
                        let git_hub_user: GithubUser = serde_json::from_str(&body).unwrap();
                        let conn = establish_connection();
                        let user_size: usize = 0;
                        let existing_user_results =
                            User::get_by_username(&git_hub_user.login, &conn);
                        if existing_user_results.is_empty() {
                            let new_rustic_user = NewUser {
                                uname: String::from(git_hub_user.login),
                                email: String::from(match git_hub_user.email {
                                    None => "",
                                    Some(ref x) => x,
                                }),
                                active: 1,
                            };

                            if User::insert(&new_rustic_user, &conn) {
                                println!("Success! We now have {} users!", User::all(&conn).len());

                                let user_resp =
                                    User::get_by_username(&new_rustic_user.uname, &conn);

                                let proto_user = proto_converters::user_to_proto(&user_resp[0]);

                                user_git_hub.set_user(proto_user);
                            }
                        } else {
                            println!("Inside of else statement since we found a user");
                            let existing_user = &existing_user_results[0];

                            let user = NewUser {
                                uname: String::from(git_hub_user.login),
                                email: String::from(match git_hub_user.email {
                                    None => "",
                                    Some(ref x) => x,
                                }),
                                active: existing_user.active,
                            };

                            if User::update_by_id(existing_user.id, &conn, user) {
                                println!("Success! We now have a user!");
                                let git_hub_updated_user: GithubUser =
                                    serde_json::from_str(&body).unwrap();

                                let user_resp =
                                    User::get_by_username(&git_hub_updated_user.login, &conn);

                                let proto_user = proto_converters::user_to_proto(&user_resp[0]);

                                user_git_hub.set_user(proto_user);
                            }
                        }
                    }
                    Err(e) => println!("Error parsing body text: {}", e),
                }
            }
            Err(e) => println!("error executing auth request"),
        }

        resp.finish(user_git_hub)
    }
}

fn main() {
    let _port = 50051;

    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(_port);
    server.add_service(UserServiceServer::new_service_def(RusticUsersImpl));

    let _server = server.build().expect("server");

    println!("users server started on port {}", _port);

    loop {
        thread::park();
    }
}
