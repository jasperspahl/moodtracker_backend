use crate::{schema::*, utils::hash_password};
use diesel::{r2d2::ConnectionManager, PgConnection};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub hash: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub hash: String,
}

impl NewUser {
    fn from_details<T: Into<String>>(username: T, email: T, password: T) -> Self {
        let hash: String = hash_password(&password.into()).unwrap();
        NewUser {
            username: username.into(),
            email: email.into(),
            hash
        }
    }
}

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[belongs_to(User)]
pub struct Mood {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub value: i32,
    pub icon: String,
}