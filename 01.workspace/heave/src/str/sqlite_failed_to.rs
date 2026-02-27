/// Represents failures that can occur specifically within the SQLite implementation.
#[derive(Debug, PartialEq)]
pub enum FailedTo {
    /// Failed to begin a database transaction.
    BeginTransaction(rusqlite::Error),
    /// Failed to build a SQL statement.
    BuildStatement,
    /// Failed to commit a database transaction.
    CommitTransaction,
    /// Failed to execute a batch of SQL statements.
    ExecuteBatch,
    /// Failed to execute a SQL query.
    ExecuteQuery,
    /// Failed to execute a prepared SQL statement.
    ExecuteStatement,
    /// Failed to open a connection to the SQLite database.
    OpenConnection,
    /// Failed to prepare a SQL statement for execution.
    PrepareStatement,
}
