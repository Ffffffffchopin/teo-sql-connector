use crate::schema::dialect::SQLDialect;
use crate::schema::value::encode::ToSQLString;
use crate::core::database::r#type::DatabaseType;

pub(crate) mod decoder;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub(crate) struct SQLColumn {
    pub(self) name: String,
    pub(self) r#type: DatabaseType,
    pub(self) not_null: bool,
    pub(self) auto_increment: bool,
    pub(self) default: Option<String>,
    pub(self) primary_key: bool,
}

impl SQLColumn {

    pub(crate) fn new(name: String, r#type: DatabaseType, not_null: bool, auto_increment: bool, default: Option<String>, primary_key: bool) -> Self {
        Self {
            name, r#type, not_null, auto_increment, default, primary_key
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn r#type(&self) -> &DatabaseType {
        &self.r#type
    }

    pub(crate) fn not_null(&self) -> bool {
        self.not_null
    }

    pub(crate) fn null(&self) -> bool {
        !self.not_null
    }

    pub(crate) fn auto_increment(&self) -> bool {
        self.auto_increment
    }

    pub(crate) fn default(&self) -> Option<&str> {
        self.default.as_deref()
    }

    pub(crate) fn primary_key(&self) -> bool {
        self.primary_key
    }

    pub(crate) fn set_default(&mut self, default: Option<String>) {
        self.default = default;
    }
}

impl ToSQLString for SQLColumn {
    fn to_string(&self, dialect: SQLDialect) -> String {
        let name = &self.name;
        let t = self.r#type.to_string(dialect);
        let not_null = if self.not_null { " NOT NULL" } else { " NULL" };
        let primary = if self.primary_key { " PRIMARY KEY" } else { "" };
        let default = if self.default.is_some() { " DEFAULT ".to_owned() + self.default.as_ref().unwrap().as_str() } else { "".to_owned() };
        let auto_inc = if self.auto_increment {
            if dialect == SQLDialect::MySQL {
                " AUTO_INCREMENT"
            } else {
                " AUTOINCREMENT"
            }
        } else { "" };
        if dialect == SQLDialect::PostgreSQL {
            let t_with_auto_inc = if self.auto_increment {
                "SERIAL".to_owned()
            } else {
                t
            };
            format!("\"{name}\" {t_with_auto_inc}{default}{not_null}{primary}")
        } else {
            format!("`{name}` {t}{default}{not_null}{primary}{auto_inc}")
        }
    }
}
