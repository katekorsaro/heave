#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum E {
    ExecuteSQLiteBatch,
    InitDatabase,
    OpenSQLiteConnection,
}
