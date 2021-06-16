use sqlx::FromRow;

/// External provider service which can be hooked to
#[derive(FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Provider {
    /// Provider name acting as the primary key
    pub name: String,
}
