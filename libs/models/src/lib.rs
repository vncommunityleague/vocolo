pub use vocolo_database_migrations::{Migrator, MigratorTrait};

pub mod error;
pub mod osu;

pub(crate) type Result<T> = std::result::Result<T, error::Error>;
