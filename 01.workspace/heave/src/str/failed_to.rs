#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum E {
    BeginSQLiteTransaction,
    CommitSQLiteTransaction,
    ExecuteSQLiteBatch,
    ExecuteSQLiteStatement,
    InitDatabase,
    OpenSQLiteConnection,
    PersistCatalog,
}
