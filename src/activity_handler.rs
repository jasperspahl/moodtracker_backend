use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;
use log::info;
use serde::Deserialize;

use crate::{
    auth_handler::LoggedUser,
    errors::ServiceError,
    models::{Activity, NewActivity, Pool, User},
};

#[derive(Debug, Deserialize)]
pub struct ActivityData {
    pub name: String,
    pub icon: String,
}

pub async fn create_activity(
    logged_user: LoggedUser,
    activity_data: web::Json<ActivityData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    info!("Request to create activity by {}", logged_user.email);
    let res =
        web::block(move || create_activity_query(logged_user, activity_data.into_inner(), pool))
            .await;

    match res {
        Ok(activity) => Ok(HttpResponse::Ok().json(&activity)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

fn create_activity_query(
    logged_user: LoggedUser,
    activity_data: ActivityData,
    pool: web::Data<Pool>,
) -> Result<Activity, ServiceError> {
    use crate::schema::{activities::dsl::activities, users::dsl::users};

    let conn = &pool.get().unwrap();
    let user: User = users.find(logged_user.id).get_result::<User>(conn)?;
    let new_activity = NewActivity {
        user_id: user.id,
        name: &activity_data.name,
        icon: activity_data.icon,
    };
    let inserted_activity = diesel::insert_into(activities)
        .values(&new_activity)
        .get_result(conn)?;
    dbg!(&&inserted_activity);
    Ok(inserted_activity)
}

pub async fn get_activities(
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    info!("Request to get activities by {}", logged_user.email);
    let res = web::block(move || get_activities_query(logged_user, pool)).await;

    match res {
        Ok(activities) => Ok(HttpResponse::Ok().json(&activities)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

fn get_activities_query(
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<Vec<Activity>, ServiceError> {
    use crate::schema::users::dsl::users;

    let conn = &pool.get().unwrap();
    let user = users.find(logged_user.id).get_result::<User>(conn)?;
    let activities = Activity::belonging_to(&user).get_results(conn)?;

    Ok(activities)
}
