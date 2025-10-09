#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum FailedTo {
    BeginTransaction,
    CommitTransaction,
    ExecuteBatch,
    ExecuteQuery,
    ExecuteStatement,
    OpenConnection,
    PrepareStatement,
}
