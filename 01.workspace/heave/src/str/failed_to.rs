#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum E {
    BeginSQLiteTransaction,
    CommitSQLiteTransaction,
    ExecuteSQLiteBatch,
    ExecuteSQLiteQuery,
    ExecuteSQLiteStatement,
    InitDatabase,
    LoadFromDB,
    MapAttribute,
    MapEntity,
    OpenSQLiteConnection,
    PersistCatalog,
    PrepareSQLiteStatement,
}
