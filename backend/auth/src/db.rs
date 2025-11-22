//! Database models and migrations

use sqlx::PgPool;
use anyhow::Result;

/// Initialize database connection pool
pub async fn init_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPool::connect(database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}

// TODO: Add database models for:
// - users
// - tenants
// - authenticators (WebAuthn, TOTP)
// - sessions
// - audit_logs
// - upstream_apps
// - policies

