use crate::schema::dialect::SQLDialect;
use crate::schema::value::encode::ToSQLString;

pub(crate) struct SQLDropTableStatement {
    pub(crate) table: String,
    pub(crate) if_exists: bool,
}

impl SQLDropTableStatement {
    pub fn if_exists(&mut self) -> &mut Self {
        self.if_exists = true;
        self
    }
}

impl ToSQLString for SQLDropTableStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let table = &self.table;
        let if_exists = if self.if_exists { " IF EXISTS" } else { "" };
        format!("DROP TABLE{if_exists} '{table}';")
    }
}
