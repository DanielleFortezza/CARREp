use crate::entity::tag;
use rweb::*;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};

#[post("/api/tag/")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn create(
    #[data] db: DatabaseConnection,
    #[data] token: String,
    #[header = "authorization"] authorization: String,
    body: Json<shared::Tag>,
) -> Result<Json<shared::Tag>, Rejection> {
    if !authorization.ends_with(&token) {
        return Err(reject::custom(super::Unauthorized));
    }

    let body = body.into_inner();

    let value = tag::ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(body.title.clone()),
    }
    .insert(&db)
    .await;

    match value {
        Ok(new) => Ok(shared::Tag {
            id: Some(new.id),
            title: new.title,
        }
        .into()),
        _ => Err(warp::reject::custom(super::InternalError)),
    }
}

#[get("/api/tag/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn detail(
    #[data] db: DatabaseConnection,
    id: i32,
) -> Result<Json<shared::Tag>, Rejection> {
    match tag::Entity::find_by_id(id).one(&db).await {
        Ok(Some(value)) => Ok(shared::Tag {
            id: Some(value.id),
            title: value.title.clone(),
        }
        .into()),
        _ => Err(warp::reject::not_found()),
    }
}

#[get("/api/tag/")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn list(#[data] db: DatabaseConnection) -> Result<Json<Vec<shared::Tag>>, Rejection> {
    match tag::Entity::find().all(&db).await {
        Ok(chain_list) => Ok(chain_list
            .iter()
            .map(|t| shared::Tag {
                id: Some(t.id),
                title: t.title.clone(),
            })
            .collect::<Vec<shared::Tag>>()
            .into()),
        _ => Err(reject::not_found()),
    }
}

#[post("/api/tag/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn update(
    #[data] token: String,
    #[data] db: DatabaseConnection,
    #[header = "authorization"] authorization: String,

    body: Json<shared::Tag>,
    id: i32,
) -> Result<Json<shared::Tag>, Rejection> {
    if !authorization.ends_with(&token) {
        return Err(reject::custom(super::Unauthorized));
    }

    let body = body.into_inner();

    match tag::Entity::find_by_id(id).one(&db).await {
        Ok(Some(value)) => {
            let mut value: tag::ActiveModel = value.into();

            value.title = ActiveValue::Set(body.title.clone());
            let value: tag::Model = value.update(&db).await.unwrap();

            Ok(shared::Tag {
                title: value.title.clone(),
                id: Some(value.id),
            }
            .into())
        }
        _ => Err(reject::not_found()),
    }
}

#[delete("/api/tag/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn delete(
    #[data] token: String,
    #[data] db: DatabaseConnection,
    #[header = "authorization"] authorization: String,
    id: i32,
) -> Result<Json<()>, Rejection> {
    if !authorization.ends_with(&token) {
        return Err(reject::custom(super::Unauthorized));
    }

    match tag::Entity::find_by_id(id).one(&db).await {
        Ok(Some(value)) => {
            value.delete(&db).await.unwrap();
            Ok(().into())
        }
        _ => Err(reject::not_found()),
    }
}
