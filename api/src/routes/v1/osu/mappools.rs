use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_garde::WithValidation;

use vocolo_database::{osu::Mappool, Database};
use vocolo_internal::{Result, UserConnections};
use vocolo_models::v1;

use crate::routes::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", post(mappool_create)).nest(
        "/:mappool_id",
        Router::new()
            .route(
                "/",
                get(mappool_fetch)
                    .patch(mappool_update)
                    .delete(mappool_remove),
            )
            .route(
                "/maps/:pos",
                post(mappool_map_add).delete(mappool_map_remove),
            ),
    )
}

pub async fn mappool_fetch(
    State(db): State<Database>,
    Path(mappool_id): Path<String>,
) -> Result<Json<v1::osu::Mappool>> {
    let mappool = Mappool::fetch(&db, &mappool_id).await?;

    Ok(Json(mappool.into()))
}

pub async fn mappool_create(
    connections: UserConnections,
    State(db): State<Database>,
    WithValidation(data): WithValidation<Json<v1::osu::CreateMappoolRequest>>,
) -> Result<StatusCode> {
    let data = data.into_inner();

    let mappool: Mappool = data.into();
    mappool.create(&db).await?;

    Ok(StatusCode::CREATED)
}

pub async fn mappool_update(
    connections: UserConnections,
    State(db): State<Database>,
    Path(mappool_id): Path<String>,
    WithValidation(data): WithValidation<Json<v1::osu::UpdateMappoolRequest>>,
) -> Result<()> {
    let data = data.into_inner();

    let mappool = Mappool::fetch(&db, &mappool_id).await?;

    let mappool = data.into();
    Mappool::update(&db, &mappool_id, &mappool).await?;

    Ok(())
}

pub async fn mappool_remove(
    connections: UserConnections,
    State(db): State<Database>,
    Path(mappool_id): Path<String>,
) -> Result<StatusCode> {
    Mappool::delete(&db, &mappool_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn mappool_map_add(
    connections: UserConnections,
    State(db): State<Database>,
    Path(mappool_id): Path<String>,
    WithValidation(data): WithValidation<Json<v1::osu::AddMappoolMapRequest>>,
) -> Result<StatusCode> {
    let data = data.into_inner();

    let maps = data.maps.into_iter().map(|map| map.into()).collect();
    Mappool::add_maps(&db, &mappool_id, maps).await?;

    Ok(StatusCode::CREATED)
}

pub async fn mappool_map_remove(
    connections: UserConnections,
    State(db): State<Database>,
    Path((mappool_id, pos)): Path<(String, i32)>,
) -> Result<StatusCode> {
    Mappool::delete_map(&db, &mappool_id, &pos).await?;

    Ok(StatusCode::NO_CONTENT)
}
