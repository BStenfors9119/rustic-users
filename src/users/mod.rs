extern crate reqwest;

use diesel;
use diesel::prelude::*;
use diesel::MysqlConnection;

use super::schema::users;
use super::schema::users::dsl::users as all_users;

use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, PartialEq, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub uname: Option<String>,
    pub email: Option<String>,
    pub active: i32,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub uname: String,
    pub email: String,
    pub active: i32,
}
impl User {
    pub fn all(conn: &MysqlConnection) -> Vec<User> {
        all_users
            .load::<User>(conn)
            .expect("Error loading all users")
    }

    pub fn get_by_email(email: String, conn: &MysqlConnection) -> Vec<User> {
        all_users
            .filter(users::email.eq(email))
            .load::<User>(conn)
            .expect("Error finding user by email")
    }

    pub fn get_by_username(username: &String, conn: &MysqlConnection) -> Vec<User> {
        all_users
            .filter(users::uname.eq(username))
            .load::<User>(conn)
            .expect("Error finding user by username")
    }

    pub fn insert(user: &NewUser, conn: &MysqlConnection) -> bool {
        diesel::insert_into(users::table)
            .values(user)
            .execute(conn)
            .is_ok()
    }

    pub fn update_by_id(id: i32, conn: &MysqlConnection, user: NewUser) -> bool {
        use super::schema::users::dsl::{active as a, email as e, uname as u};

        let NewUser {
            uname,
            email,
            active,
        } = user;

        diesel::update(all_users.find(id))
            .set((u.eq(uname), e.eq(email), a.eq(active)))
            .execute(conn)
            .is_ok()
    }

    pub fn update_by_email(
        current_user_email: String,
        conn: &MysqlConnection,
        user: NewUser,
    ) -> bool {
        use super::schema::users::dsl::{active as a, email as e, uname as u};

        let NewUser {
            uname,
            email,
            active,
        } = user;

        diesel::update(all_users.filter(users::email.eq(current_user_email)))
            .set((u.eq(uname), e.eq(email), a.eq(active)))
            .execute(conn)
            .is_ok()
    }
}
