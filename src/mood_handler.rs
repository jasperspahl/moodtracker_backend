use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;
use log::info;
use serde::Deserialize;

use crate::{
    auth_handler::LoggedUser,
    errors::ServiceError,
    models::{Mood, NewMood, Pool, User},
};

#[derive(Debug, Deserialize)]
pub struct MoodData {
    pub name: String,
    pub icon: String,
    pub value: i32,
}

pub async fn create_mood(
    logged_user: LoggedUser,
    mood_data: web::Json<MoodData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    info!("Request to create mood by {}", logged_user.email);
    let res =
        web::block(move || create_mood_query(logged_user, mood_data.into_inner(), pool)).await;

    match res {
        Ok(mood) => Ok(HttpResponse::Ok().json(&mood)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

fn create_mood_query(
    logged_user: LoggedUser,
    mood_data: MoodData,
    pool: web::Data<Pool>,
) -> Result<Mood, ServiceError> {
    use crate::schema::moods::dsl::moods;

    let conn = &pool.get().unwrap();
    let new_mood = NewMood {
        user_id: logged_user.id,
        name: mood_data.name,
        value: mood_data.value,
        icon: mood_data.icon,
    };
    let inserted_mood = diesel::insert_into(moods)
        .values(&new_mood)
        .get_result(conn)?;
    dbg!(&&inserted_mood);
    Ok(inserted_mood)
}

pub async fn get_moods(
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    info!("Request to get moods by {}", logged_user.email);
    let res = web::block(move || get_moods_query(logged_user, pool)).await;

    match res {
        Ok(moods) => Ok(HttpResponse::Ok().json(&moods)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

fn get_moods_query(
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<Vec<Mood>, ServiceError> {
    use crate::schema::moods::dsl::value;
    use crate::schema::users::dsl::users;

    let conn = &pool.get().unwrap();
    let user = users.find(logged_user.id).get_result::<User>(conn)?;
    let moods = Mood::belonging_to(&user)
        .order(value.desc())
        .get_results(conn)?;

    Ok(moods)
}
