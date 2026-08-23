//! The `importer` crate responsible for importing snapshots and player UUID's from files
//!
//! Snapshots are just read as `shared::PlayerSnapshotJson`,
//! meaning the snapshots themselves are not parsed into structs,

mod players;
mod snapshots;

pub use players::*;
pub use snapshots::*;
