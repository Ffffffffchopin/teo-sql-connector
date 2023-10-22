use crate::schema::dialect::SQLDialect;
use crate::schema::value::encode::ToSQLString;

pub(crate) struct SQLShowTablesStatement {
    pub(crate) like: Option<String>
}

impl SQLShowTablesStatement {
    pub(crate) fn like(&mut self, name: impl Into<String>) -> &mut Self {
        self.like = Some(name.into());
        self
    }
}

impl ToSQLString for SQLShowTablesStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let like = match &self.like {
            Some(name) => format!(" like \"{name}\""),
            None => "".to_string()
        };
        format!("SHOW TABLES{like}")
    }
}
