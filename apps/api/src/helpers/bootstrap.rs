//! Composition root: parse configuration, install the logger, create the database pool, and
//! build the app. Every entrypoint starts here.

use axum::Router;
use rustinit_core::BoxError;
use rustinit_database::{DbPool, Env};
use rustinit_logger::LoggerOptions;

use crate::config::AppConfig;
use crate::{AppDependencies, create_app};

pub struct ApiContext {
    pub config: AppConfig,
    pub app: Router,
    pub db: DbPool,
}

impl ApiContext {
    /// Builds the context from the process environment. Must run inside a Tokio runtime.
    pub fn create() -> Result<Self, BoxError> {
        let config = AppConfig::parse(&Env::load())?;
        rustinit_logger::init(LoggerOptions {
            environment: config.environment,
            level: config.log_level,
        })?;

        let db = rustinit_database::create_pool(&config.db);
        let app = create_app(AppDependencies {
            config: config.clone(),
            db: db.clone(),
            token_verifier: None,
        });

        Ok(Self { config, app, db })
    }

    /// Releases resources. Call once the server has stopped accepting requests.
    pub async fn close(self) {
        rustinit_database::close(&self.db).await;
    }
}
