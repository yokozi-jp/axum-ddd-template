//! Axum DDD Template - A Domain-Driven Design template using Axum framework.

mod features;
mod shared;

use axum::{routing::get, Router};
use features::task::application::{
    CompleteTaskUseCase, CreateTaskUseCase, DeleteTaskUseCase, GetTaskUseCase, ListTasksUseCase,
};
use features::task::infrastructure::{http as task_http, PgTaskRepository};
use features::user::application::{
    CreateUserUseCase, DeleteUserUseCase, GetUserUseCase, ListUsersUseCase, UpdateUserUseCase,
};
use features::user::infrastructure::{http as user_http, PgUserRepository};
use shared::infrastructure::{config::Config, database, http::health_check};
use std::sync::Arc;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;

/// Application state shared across handlers
pub struct AppState {
    pub(crate) create_user: CreateUserUseCase,
    pub(crate) get_user: GetUserUseCase,
    pub(crate) list_users: ListUsersUseCase,
    pub(crate) update_user: UpdateUserUseCase,
    pub(crate) delete_user: DeleteUserUseCase,
    pub(crate) create_task: CreateTaskUseCase,
    pub(crate) get_task: GetTaskUseCase,
    pub(crate) list_tasks: ListTasksUseCase,
    pub(crate) complete_task: CompleteTaskUseCase,
    pub(crate) delete_task: DeleteTaskUseCase,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = Config::from_env()?;
    let pool = database::create_pool(&config).await?;
    database::run_migrations(&pool).await?;

    let user_repo: Arc<dyn features::user::domain::UserRepository> =
        Arc::new(PgUserRepository::new(pool.clone()));
    let task_repo: Arc<dyn features::task::domain::TaskRepository> =
        Arc::new(PgTaskRepository::new(pool));

    let state = Arc::new(AppState {
        create_user: CreateUserUseCase::new(Arc::clone(&user_repo)),
        get_user: GetUserUseCase::new(Arc::clone(&user_repo)),
        list_users: ListUsersUseCase::new(Arc::clone(&user_repo)),
        update_user: UpdateUserUseCase::new(Arc::clone(&user_repo)),
        delete_user: DeleteUserUseCase::new(Arc::clone(&user_repo)),
        create_task: CreateTaskUseCase::new(Arc::clone(&task_repo)),
        get_task: GetTaskUseCase::new(Arc::clone(&task_repo)),
        list_tasks: ListTasksUseCase::new(Arc::clone(&task_repo)),
        complete_task: CompleteTaskUseCase::new(Arc::clone(&task_repo)),
        delete_task: DeleteTaskUseCase::new(Arc::clone(&task_repo)),
    });

    let app = Router::new()
        .route("/health", get(health_check))
        .merge(user_http::router())
        .merge(task_http::router())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::with_status_code(
                    axum::http::StatusCode::SERVICE_UNAVAILABLE,
                    Duration::from_secs(30),
                )),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(config.server_addr).await?;
    info!("Server running on http://{}", config.server_addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.ok();
    };

    #[cfg(unix)]
    let terminate = async {
        #[expect(clippy::expect_used, reason = "SIGTERM handler is critical for graceful shutdown")]
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {}
        () = terminate => {}
    }
}
