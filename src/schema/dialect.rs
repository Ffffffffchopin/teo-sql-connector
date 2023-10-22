#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SQLDialect {
    MySQL,
    PostgreSQL,
    SQLite,
    MSSQL,
}

impl SQLDialect {
    pub(crate) fn escape(&self) -> &'static str {
        match self {
            SQLDialect::PostgreSQL => "\"",
            _ => "`",
        }
    }

    pub(crate) fn is_postgres(&self) -> bool {
        match self {
            SQLDialect::PostgreSQL => true,
            _ => false,
        }
    }

    pub(crate) fn is_mysql(&self) -> bool {
        match self {
            SQLDialect::MySQL => true,
            _ => false,
        }
    }
}
