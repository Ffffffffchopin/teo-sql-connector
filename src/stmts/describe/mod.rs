use crate::schema::dialect::SQLDialect;
use crate::schema::value::encode::ToSQLString;

pub struct SQLDescribeStatement {
    pub(crate) table: String
}

impl ToSQLString for SQLDescribeStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let table = &self.table;
        format!("DESCRIBE `{table}`")
    }
}
