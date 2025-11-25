use crate::api::constant::http_status_phrases as message;
use crate::api::utils::hash::verify_password;
use crate::ui::models::login_model::ApiResponse;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use axum_session::Session;
use axum_session_sqlx::SessionSqlitePool;
use sqlx::{Pool, Sqlite};
use tracing;

#[derive(serde::Deserialize)]
struct UserRequest {
    email: String,
    password: String,
}

#[derive(sqlx::FromRow, Debug)]
struct UserSql {
    id: i64,
    email: String,
    password_hash: String,
    role_id: i64,
}

async fn login(
    session: Session<SessionSqlitePool>,
    State(pool): State<Pool<Sqlite>>,
    Json(user): Json<UserRequest>,
) -> impl IntoResponse {
    let user_result: Result<Vec<UserSql>, _> = sqlx::query_as(
        "SELECT id, email, password_hash, role_id, active_session FROM users WHERE email = ?",
    )
    .bind(&user.email)
    .fetch_all(&pool)
    .await;

    match user_result {
        Ok(users) if !users.is_empty() => {
            let user_data = &users[0];

            match verify_password(&user.password, &user_data.password_hash) {
                Ok(true) => {
                    // Generate a unique session ID first
                    let session_id = format!(
                        "{}-{}",
                        chrono::Utc::now().timestamp_millis(),
                        rand::random::<u64>()
                    );

                    // atomic operation
                    match sqlx::query_scalar::<_, Option<String>>(
                        "UPDATE users 
                         SET active_session = CASE 
                            WHEN active_session IS NULL THEN ? 
                            ELSE active_session 
                         END 
                         WHERE id = ? 
                         RETURNING active_session",
                    )
                    .bind(&session_id)
                    .bind(user_data.id)
                    .fetch_optional(&pool)
                    .await
                    {
                        Ok(Some(Some(existing_session))) if existing_session != session_id => {
                            let response = ApiResponse::<()>::error(message::CONFLICT);
                            (StatusCode::CONFLICT, axum::Json(response)).into_response()
                        }
                        Ok(_) => {
                            // Success - either new session or updating existing one
                            session.set("id", user_data.id);
                            session.set("email", &user_data.email);
                            session.set("role_id", user_data.role_id);
                            session.set("session_id", &session_id);

                            let response = ApiResponse::success((), message::OK);
                            (StatusCode::OK, axum::Json(response)).into_response()
                        }
                        Err(e) => {
                            tracing::error!("Database error during login: {}", e);
                            let response = ApiResponse::<()>::error(message::INTERNAL_SERVER_ERROR);
                            (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(response))
                                .into_response()
                        }
                    }
                }
                Ok(false) => {
                    let response = ApiResponse::<()>::error(message::UNAUTHORIZED);
                    (StatusCode::UNAUTHORIZED, axum::Json(response)).into_response()
                }
                Err(_) => {
                    let response = ApiResponse::<()>::error(message::INTERNAL_SERVER_ERROR);
                    (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(response)).into_response()
                }
            }
        }
        _ => {
            let response = ApiResponse::<()>::error(message::UNAUTHORIZED);
            (StatusCode::UNAUTHORIZED, axum::Json(response)).into_response()
        }
    }
}

pub fn router() -> Router<sqlx::Pool<Sqlite>> {
    use axum::routing::post;
    Router::new().route("/auth/login", post(login))
}
// guide https://github.com/MikeCode00/Axum-Auth-Session/blob/main/src/main.rs
