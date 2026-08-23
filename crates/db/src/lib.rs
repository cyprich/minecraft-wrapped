mod models;

mod insert;
mod manager;
mod select;
mod update;

pub use insert::*;
pub use manager::*;
pub use select::*;
pub use update::*;

pub(crate) type Pool = sqlx::Pool<sqlx::Postgres>;
pub(crate) type Builder = sqlx::QueryBuilder<sqlx::Postgres>;
