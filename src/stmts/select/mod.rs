use crate::query::escape_wisdom;
use crate::schema::dialect::SQLDialect;
use crate::schema::value::encode::ToSQLString;

pub mod r#where;

pub struct SQLSelectStatement<'a> {
    pub(crate) columns: Option<&'a Vec<&'a str>>,
    pub(crate) from: &'a str,
    pub(crate) r#where: Option<String>,
    pub(crate) left_join: Option<String>,
    pub(crate) inner_join: Option<String>,
    pub(crate) order_by: Option<String>,
    pub(crate) limit: Option<(u64, u64)>,
}

impl<'a> SQLSelectStatement<'a> {

    pub fn left_join(&mut self, left_join: String) -> &mut Self {
        self.left_join = Some(left_join);
        self
    }

    pub fn inner_join(&mut self, inner_join: String) -> &mut Self {
        self.inner_join = Some(inner_join);
        self
    }

    pub fn r#where(&mut self, r#where: String) -> &mut Self {
        self.r#where = Some(r#where);
        self
    }

    pub fn order_by(&mut self, order_by: String) -> &mut Self {
        self.order_by = Some(order_by);
        self
    }

    pub fn limit(&mut self, limit: u64, skip: u64) -> &mut Self {
        self.limit = Some((limit, skip));
        self
    }
}

impl<'a> ToSQLString for SQLSelectStatement<'a> {
    fn to_string(&self, dialect: SQLDialect) -> String {
        let columns = if self.columns.is_none() { "*".to_owned() } else { self.columns.unwrap().iter().map(|c| {
            escape_wisdom(c, dialect)
        }).collect::<Vec<_>>().join(", ") };
        let left_join = if let Some(left_join) = &self.left_join {
            " LEFT JOIN ".to_owned() + left_join
        } else {
            "".to_owned()
        };
        let inner_join = if let Some(inner_join) = &self.inner_join {
            " INNER JOIN ".to_owned() + inner_join
        } else {
            "".to_owned()
        };
        let r#where = if let Some(r#where) = &self.r#where {
            " WHERE ".to_owned() + r#where
        } else {
            "".to_owned()
        };
        let order_by = if let Some(order_by) = &self.order_by {
            " ORDER BY ".to_owned() + order_by
        } else {
            "".to_owned()
        };
        let limit = if let Some(limit) = &self.limit {
            if dialect == SQLDialect::PostgreSQL {
                format!(" LIMIT {} OFFSET {}", limit.0, limit.1)
            } else {
                format!(" LIMIT {},{}", limit.1, limit.0)
            }
        } else {
            "".to_owned()
        };
        format!("SELECT {columns} from {}{}{}{}{}{}", self.from, left_join, inner_join, r#where, order_by, limit)
    }
}
