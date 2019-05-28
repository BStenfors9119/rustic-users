use rustic_users::rustic::User;

pub fn to_proto(user: &rustic_users::users::User) -> User {
    let mut users_resp = User::new();

    let username = user.uname.as_ref().unwrap().to_string();
    let email = user.email.as_ref().unwrap().to_string();

    users_resp.set_id(user.id);
    users_resp.set_uname(username);
    users_resp.set_email(email);
    users_resp.set_active(user.active);

    return users_resp;
}
