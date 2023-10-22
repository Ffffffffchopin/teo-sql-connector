use crate::schema::column::SQLColumn;
use crate::schema::dialect::SQLDialect;
use crate::schema::value::encode::ToSQLString;

pub struct SQLAlterTableModifyStatement {
    pub(crate) table: String,
    pub(crate) column: SQLColumn,
}

impl ToSQLString for SQLAlterTableModifyStatement {
    fn to_string(&self, dialect: SQLDialect) -> String {
        let table = &self.table;
        let def = self.column.to_string(dialect);
        let escape = dialect.escape();
        if dialect == SQLDialect::SQLite {
            format!("ALTER TABLE {escape}{table}{escape} ({def})")
        } else if dialect == SQLDialect::PostgreSQL {
            let c_name = self.column.name();
            format!("ALTER TABLE {escape}{table}{escape} ALTER COLUMN {escape}{c_name}{escape} TYPE column_definition;")
        } else {
            format!("ALTER TABLE {escape}{table}{escape} MODIFY {def}")
        }
    }
}
