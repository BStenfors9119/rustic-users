pub mod user_proto;

pub fn user_to_proto(user: &rustic_users::users::User) -> rustic_users::rustic::User {
    user_proto::to_proto(user)
}
