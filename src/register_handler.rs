use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;

use crate::{
    auth_handler::AuthData,
    errors::ServiceError,
    models::{NewUser, Pool, SlimUser, User},
    utils::hash_password,
};

pub async fn register(
    user_data: web::Json<AuthData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || query(user_data.into_inner(), pool)).await;
    match res {
        Ok(user) => Ok(HttpResponse::Ok().json(&user)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

fn query(user_data: AuthData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    use crate::schema::users::dsl::users;
    let conn = &pool.get().unwrap();
    let hashed_password = hash_password(&user_data.password)?;
    let new_user = NewUser::from_details(user_data.email, hashed_password);
    let inserted_user: User = diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)?;
    dbg!(&inserted_user);
    Ok(inserted_user.into())
}
