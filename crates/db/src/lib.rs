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

// approximate runtime in dev profile with ~1.67M stats

// const SELECT_CHUNK_SIZE: i64 = 2048; // 30s
// const SELECT_CHUNK_SIZE: i64 = 4096; // 20s
// const SELECT_CHUNK_SIZE: i64 = 8192; // 14s
// const SELECT_CHUNK_SIZE: i64 = 16384; // 13s
const SELECT_CHUNK_SIZE: i64 = 32768; // 10s
// const SELECT_CHUNK_SIZE: i64 = 65536; // 10s
// const SELECT_CHUNK_SIZE: i64 = 131072; // 11s

// const INSERT_CHUNK_SIZE: i64 = 2048; // 39s
// const INSERT_CHUNK_SIZE: i64 = 4096; // 42s
// const INSERT_CHUNK_SIZE: i64 = 8192; // 39s
// const INSERT_CHUNK_SIZE: i64 = 16384; // 39s
const INSERT_CHUNK_SIZE: i64 = 32768; // 39s
// const INSERT_CHUNK_SIZE: i64 = 65536; // 39s
// const INSERT_CHUNK_SIZE: i64 = 131072; // 45s
