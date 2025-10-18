#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum FailedTo {
    BeginTransaction,
    BuildStatement,
    CommitTransaction,
    ExecuteBatch,
    ExecuteQuery,
    ExecuteStatement,
    OpenConnection,
    PrepareStatement,
}
