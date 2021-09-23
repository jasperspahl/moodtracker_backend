use crate::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(
    Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Associations, AsChangeset,
)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub email: String,
    pub hash: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub hash: String,
}

impl NewUser {
    pub fn from_details<T: Into<String>>(email: T, hash: T) -> Self {
        NewUser {
            email: email.into(),
            hash: hash.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub id: i32,
    pub email: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            id: user.id,
            email: user.email,
        }
    }
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations, AsChangeset)]
#[belongs_to(User)]
pub struct Mood {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub value: i32,
    pub icon: String,
}

#[derive(Debug, Insertable)]
#[table_name = "moods"]
pub struct NewMood {
    pub user_id: i32,
    pub name: String,
    pub value: i32,
    pub icon: String,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations, AsChangeset)]
#[belongs_to(User)]
#[table_name = "activities"]
pub struct Activity {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub icon: String,
}

#[derive(Debug, Insertable)]
#[table_name = "activities"]
pub struct NewActivity<'a> {
    pub user_id: i32,
    pub name: &'a str,
    pub icon: String,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Mood)]
#[table_name = "entrys"]
pub struct Entry {
    pub id: i32,
    pub user_id: i32,
    pub mood_id: i32,
    pub desc: Option<String>,
    pub created_at: std::time::SystemTime,
}

#[derive(Debug, Insertable)]
#[table_name = "entrys"]
pub struct NewEntry {
    pub user_id: i32,
    pub mood_id: i32,
    pub desc: Option<String>,
    pub created_at: Option<std::time::SystemTime>,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Entry)]
#[table_name = "entry_images"]
pub struct EnrtyImage {
    pub id: i32,
    pub user_id: i32,
    pub entry_id: i32,
    pub image_url: String,
}

#[derive(Debug, Insertable)]
#[table_name = "entry_images"]
pub struct NewEntryImage<'a> {
    pub user_id: i32,
    pub entry_id: i32,
    pub image_url: &'a str,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Entry)]
#[belongs_to(Activity)]
#[table_name = "entry_activities"]
pub struct EntryActivity {
    pub id: i32,
    pub entry_id: i32,
    pub activity_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "entry_activities"]
pub struct NewEntryActivity {
    pub entry_id: i32,
    pub activity_id: i32,
}
