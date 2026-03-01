//! Axum DDD Template - A Domain-Driven Design template using Axum framework.

mod features;
mod shared;

use axum::{routing::get, Router};
use features::task::application::{
    CompleteTaskUseCase, CreateTaskUseCase, DeleteTaskUseCase, GetTaskUseCase, ListTasksUseCase,
};
use features::task::application::{CompleteTask, CreateTask, DeleteTask, GetTask, ListTasks};
use features::task::infrastructure::{http as task_http, PgTaskRepository};
use features::user::application::{
    CreateUserUseCase, DeleteUserUseCase, GetUserUseCase, ListUsersUseCase, UpdateUserUseCase,
};
use features::user::application::{CreateUser, DeleteUser, GetUser, ListUsers, UpdateUser};
use features::user::infrastructure::{http as user_http, PgUserRepository};
use shared::infrastructure::{
    config::Config,
    database,
    http::health_check,
};
use std::sync::Arc;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;

/// User feature state
pub struct UserState {
    pub(crate) create: Box<dyn CreateUser>,
    pub(crate) get: Box<dyn GetUser>,
    pub(crate) list: Box<dyn ListUsers>,
    pub(crate) update: Box<dyn UpdateUser>,
    pub(crate) delete: Box<dyn DeleteUser>,
}

/// Task feature state
pub struct TaskState {
    pub(crate) create: Box<dyn CreateTask>,
    pub(crate) get: Box<dyn GetTask>,
    pub(crate) list: Box<dyn ListTasks>,
    pub(crate) complete: Box<dyn CompleteTask>,
    pub(crate) delete: Box<dyn DeleteTask>,
}

/// Application state shared across handlers
pub struct AppState {
    pub(crate) user: UserState,
    pub(crate) task: TaskState,
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
        user: UserState {
            create: Box::new(CreateUserUseCase::new(Arc::clone(&user_repo))),
            get: Box::new(GetUserUseCase::new(Arc::clone(&user_repo))),
            list: Box::new(ListUsersUseCase::new(Arc::clone(&user_repo))),
            update: Box::new(UpdateUserUseCase::new(Arc::clone(&user_repo))),
            delete: Box::new(DeleteUserUseCase::new(Arc::clone(&user_repo))),
        },
        task: TaskState {
            create: Box::new(CreateTaskUseCase::new(Arc::clone(&task_repo))),
            get: Box::new(GetTaskUseCase::new(Arc::clone(&task_repo))),
            list: Box::new(ListTasksUseCase::new(Arc::clone(&task_repo))),
            complete: Box::new(CompleteTaskUseCase::new(Arc::clone(&task_repo))),
            delete: Box::new(DeleteTaskUseCase::new(Arc::clone(&task_repo))),
        },
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
