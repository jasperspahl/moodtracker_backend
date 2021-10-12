use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    auth_handler::LoggedUser,
    errors::ServiceError,
    models::{Activity, Entry, EntryActivity, Mood, NewEntry, NewEntryActivity, Pool, User},
};

#[derive(Debug, Serialize)]
pub struct BigEntry {
    pub id: i32,
    pub user_id: i32,
    pub mood: Mood,
    pub desc: Option<String>,
    pub created_at: std::time::SystemTime,
    pub activities: Vec<Activity>,
}

pub async fn get_entrys(
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || get_entrys_query(logged_user, pool)).await;

    match res {
        Ok(entrys) => Ok(HttpResponse::Ok().json(&entrys)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

fn get_entrys_query(
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<Vec<BigEntry>, ServiceError> {
    use crate::schema::{
        activities::dsl::{activities, id},
        entry_activities::dsl::{activity_id, entry_activities, entry_id},
        entrys::dsl::{created_at, entrys, user_id},
        moods::dsl::moods,
        users::dsl::users,
    };

    let conn = &pool.get().unwrap();
    let user: User = users.find(logged_user.id).get_result::<User>(conn)?;
    let entry_vec = entrys
        .filter(user_id.eq(user.id))
        .order(created_at.desc())
        .get_results::<Entry>(conn)?;

    let mut res = Vec::new();
    for entry in entry_vec {
        let mood = moods.find(entry.mood_id).get_result::<Mood>(conn)?;
        let activity_ids = entry_activities
            .filter(entry_id.eq(entry.id))
            .select(activity_id)
            .load::<i32>(conn)?;
        let activity_vec = activities
            .filter(id.eq_any(activity_ids))
            .get_results::<Activity>(conn)?;
        res.push(BigEntry {
            id: entry.id,
            user_id: entry.user_id,
            mood,
            desc: entry.desc,
            created_at: entry.created_at,
            activities: activity_vec,
        })
    }
    dbg!(&res);
    Ok(res)
}

#[derive(Debug, Deserialize)]
pub struct EntryData {
    pub mood_id: i32,
    pub desc: Option<String>,
    pub created_at: Option<std::time::SystemTime>,
    pub activity_ids: Vec<i32>,
}

pub async fn create_entry(
    logged_user: LoggedUser,
    entry_data: web::Json<EntryData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let entry_data = entry_data.into_inner();
    let res = web::block(move || create_entry_query(logged_user, entry_data, pool)).await;

    match res {
        Ok(entry) => Ok(HttpResponse::Ok().json(&entry)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

fn create_entry_query(
    logged_user: LoggedUser,
    entry_data: EntryData,
    pool: web::Data<Pool>,
) -> Result<(Entry, Vec<EntryActivity>), ServiceError> {
    use crate::schema::{entry_activities::dsl::entry_activities, entrys::dsl::entrys};

    let conn = &pool.get().unwrap();
    let mut new_entry = NewEntry {
        user_id: logged_user.id,
        mood_id: entry_data.mood_id,
        desc: None,
        created_at: None,
    };

    if let Some(desc) = entry_data.desc {
        new_entry.desc = Some(desc);
    }
    if let Some(time) = entry_data.created_at {
        new_entry.created_at = Some(time);
    }

    let inserted_entry = diesel::insert_into(entrys)
        .values(new_entry)
        .get_result::<Entry>(conn)?;
    dbg!(&inserted_entry);
    let mut activity_vec: Vec<NewEntryActivity> = Vec::new();

    for activity_id in entry_data.activity_ids {
        activity_vec.push(NewEntryActivity {
            entry_id: inserted_entry.id,
            activity_id,
        })
    }

    let inserted_activities = diesel::insert_into(entry_activities)
        .values(activity_vec)
        .get_results(conn)?;
    dbg!(&inserted_activities);
    Ok((inserted_entry, inserted_activities))
}

pub async fn get_entry_by_id(
    logged_user: LoggedUser,
    id: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let id = id.into_inner().parse::<i32>().unwrap();
    let res = web::block(move || get_entry_by_id_query(id, logged_user, pool)).await;

    match res {
        Ok(res) => Ok(HttpResponse::Ok().json(&res)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

fn get_entry_by_id_query(
    id: i32,
    logged_user: LoggedUser,
    pool: web::Data<Pool>,
) -> Result<BigEntry, ServiceError> {
    use crate::schema::{
        activities::dsl::{activities, id as activities_id},
        entry_activities::dsl::{activity_id, entry_activities, entry_id},
        entrys::dsl::{entrys, user_id},
        moods::dsl::moods,
    };
    let conn = &pool.get().unwrap();
    let entry = entrys
        .find(id)
        .filter(user_id.eq(logged_user.id))
        .get_result::<Entry>(conn)?;
    let mood = moods.find(entry.mood_id).get_result::<Mood>(conn)?;
    let activity_ids = entry_activities
        .filter(entry_id.eq(entry.id))
        .select(activity_id)
        .get_results::<i32>(conn)?;
    let activity_vec = activities
        .filter(activities_id.eq_any(activity_ids))
        .get_results::<Activity>(conn)?;
    Ok(BigEntry {
        id,
        user_id: logged_user.id,
        mood,
        desc: entry.desc,
        created_at: entry.created_at,
        activities: activity_vec,
    })
}
