use actix_web::{web, HttpResponse, Error};
use diesel::prelude::*;
use crate::db::DbPool;
use crate::models::{User, NewUser};

pub async fn create_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get().unwrap();
        diesel::insert_into(crate::schema::users::table)
            .values(&new_user.into_inner())
            .returning(User::as_select())
            .get_result::<User>(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn get_users(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let users = web::block(move || {
        let mut conn = pool.get().unwrap();
        crate::schema::users::table
            .select(User::as_select())
            .load::<User>(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_user(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get().unwrap();
        crate::schema::users::table
            .find(id.into_inner())
            .select(User::as_select())
            .get_result::<User>(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn update_user(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
    updated_user: web::Json<NewUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get().unwrap();
        diesel::update(crate::schema::users::table.find(id.into_inner()))
            .set((
                crate::schema::users::name.eq(&updated_user.name),
                crate::schema::users::email.eq(&updated_user.email),
            ))
            .returning(User::as_select())
            .get_result::<User>(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn delete_user(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let _ = web::block(move || {
        let mut conn = pool.get().unwrap();
        diesel::delete(crate::schema::users::table.find(id.into_inner()))
            .execute(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json("User deleted successfully"))
}