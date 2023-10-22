use crate::schema::dialect::SQLDialect;
use crate::schema::value::encode::ToSQLString;
use crate::core::model::index::{ModelIndex, ModelIndexItem};

pub(crate) struct SQLCreateIndexOnStatement {
    unique: bool,
    index: String,
    table: String,
    columns: Vec<ModelIndexItem>
}

impl SQLCreateIndexOnStatement {
    pub(crate) fn column(&mut self, column: ModelIndexItem) -> &mut Self {
        self.columns.push(column);
        self
    }

    pub(crate) fn columns(&mut self, columns: Vec<ModelIndexItem>) -> &mut Self {
        self.columns.extend(columns);
        self
    }
}

impl ToSQLString for SQLCreateIndexOnStatement {
    fn to_string(&self, dialect: SQLDialect) -> String {
        let unique = if self.unique { " UNIQUE" } else { "" };
        let index = &self.index;
        let table = &self.table;
        let def = self.columns.iter().map(|c| ModelIndex::sql_format_item(dialect, c, false)).collect::<Vec<String>>().join(", ");
        format!("CREATE{unique} INDEX `{index}` ON `{table}`({def})")
    }
}

pub(crate) struct SQLCreateIndexStatement {
    pub(crate) unique: bool,
    pub(crate) index: String,
}

impl SQLCreateIndexStatement {
    pub fn on(&self, table: impl Into<String>) -> SQLCreateIndexOnStatement {
        SQLCreateIndexOnStatement { unique: self.unique, index: self.index.clone(), table: table.into(), columns: vec![] }
    }
}
