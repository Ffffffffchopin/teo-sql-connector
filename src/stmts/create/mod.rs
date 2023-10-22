use crate::stmts::create::database::SQLCreateDatabaseStatement;
use crate::stmts::create::index::SQLCreateIndexStatement;
use crate::stmts::create::table::SQLCreateTableStatement;

pub mod database;
pub mod table;
pub mod index;

pub(crate) struct SQLCreateStatement { }

impl SQLCreateStatement {

    pub(crate) fn database(&self, database: impl Into<String>) -> SQLCreateDatabaseStatement {
        SQLCreateDatabaseStatement { database: database.into(), if_not_exists: false }
    }

    pub(crate) fn table(&self, table: impl Into<String>) -> SQLCreateTableStatement {
        SQLCreateTableStatement { table: table.into(), if_not_exists: false, columns: vec![], primary: None }
    }

    pub(crate) fn index(&self, index: impl Into<String>) -> SQLCreateIndexStatement {
        SQLCreateIndexStatement { unique: false, index: index.into() }
    }

    pub(crate) fn unique_index(&self, index: impl Into<String>) -> SQLCreateIndexStatement {
        SQLCreateIndexStatement { unique: true, index: index.into() }
    }
}
