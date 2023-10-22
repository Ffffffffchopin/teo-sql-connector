use std::str::FromStr;
use regex::Regex;
use snailquote::unescape;
use crate::schema::dialect::SQLDialect;
use crate::core::database::r#type::DatabaseType;
use crate::core::r#enum::DbEnum;

pub(crate) struct SQLTypeDecoder { }

impl SQLTypeDecoder {
    pub(crate) fn decode(r#type: &str, dialect: SQLDialect) -> DatabaseType {
        match dialect {
            SQLDialect::MySQL => mysql_type_to_database_type(r#type),
            SQLDialect::PostgreSQL => postgresql_type_to_database_type(r#type),
            SQLDialect::SQLite => sqlite_type_to_database_type(r#type),
            SQLDialect::MSSQL => mssql_type_to_database_type(r#type),
        }
    }
}

fn mysql_type_to_database_type(r#type: &str) -> DatabaseType {
    let r#type_string = r#type.to_lowercase();
    let r#type: &str = r#type_string.as_str();
    let regex = Regex::new("([^ \\(\\)]+)( (.+))?(\\((.+)\\))?").unwrap();
    match regex.captures(r#type) {
        None => panic!("Unhandled database type '{}' '{}'.", r#type, regex),
        Some(captures) => {
            let name = captures.get(1).unwrap().as_str();
            let trailing1 = captures.get(3).map(|m| m.as_str());
            let arg = captures.get(5).map(|m| m.as_str());
            match name {
                "bit" => DatabaseType::Bit { m: arg.map(|a| u8::from_str(a).unwrap()) },
                "tinyint" => DatabaseType::TinyInt { m: arg.map(|a| u8::from_str(a).unwrap()), u: trailing1.is_some() },
                "smallint" => DatabaseType::SmallInt { m: arg.map(|a| u8::from_str(a).unwrap()), u: trailing1.is_some() },
                "mediumint" => DatabaseType::MediumInt { m: arg.map(|a| u8::from_str(a).unwrap()), u: trailing1.is_some() },
                "int" => DatabaseType::Int { m: arg.map(|a| u8::from_str(a).unwrap()), u: trailing1.is_some() },
                "bigint" => DatabaseType::BigInt { m: arg.map(|a| u8::from_str(a).unwrap()), u: trailing1.is_some() },
                "float" => DatabaseType::Float { m: None, d: None },
                "double" => DatabaseType::Double { m: None, d: None },
                "char" => DatabaseType::Char { m: arg.map(|a| u8::from_str(a).unwrap()), n: None, c: None },
                "varchar" => DatabaseType::VarChar { m: arg.map(|a| u16::from_str(a).unwrap()).unwrap(), n: None, c: None },
                "text" => DatabaseType::Text { m: None, n: None, c: None },
                "mediumtext" => DatabaseType::MediumText { n: None, c: None },
                "longtext" => DatabaseType::LongText { n: None, c: None },
                "date" => DatabaseType::Date,
                "datetime" => DatabaseType::DateTime(u8::from_str(arg.unwrap()).unwrap()),
                "decimal" => {
                    if let Some(args) = arg {
                        let args = args.split(",").into_iter().collect::<Vec<&str>>();
                        DatabaseType::Decimal { m: Some(args.get(0).unwrap().parse().unwrap()), d: Some(args.get(1).unwrap().parse().unwrap()) }
                    } else {
                        DatabaseType::Decimal { m: None, d: None }
                    }
                }
                "enum" => {
                    let choices = arg.unwrap();
                    let choices_vec = choices.split(",");
                    let unescaped: Vec<String> = choices_vec.map(|s| unescape(s).unwrap()).collect();
                    DatabaseType::Enum(DbEnum { choices: unescaped })
                }
                _ => panic!("Unhandled type '{}' '{:?}' '{:?}'.", name, trailing1, arg)
            }
        }
    }
}

fn postgresql_type_to_database_type(r#type: &str) -> DatabaseType {
    let lower = r#type.to_lowercase();
    let lower_str = lower.as_str();
    match lower_str {
        "integer" | "int4" => DatabaseType::Int { m: None, u: false },
        "text" => DatabaseType::Text { m: None, n: None, c: None },
        "timestamp without time zone" | "timestamp" => DatabaseType::Timestamp { p: 3, z: false },
        "boolean" | "bool" => DatabaseType::Bool,
        "bigint" | "int8" => DatabaseType::BigInt { m: None, u: false },
        "double precision" | "float8" => DatabaseType::Double { m: None, d: None },
        "real" | "float4" => DatabaseType::Real,
        "date" => DatabaseType::Date,
        "numeric" => DatabaseType::Decimal { m: Some(65), d: Some(30) },
        _ => if lower_str.starts_with("array|") {
            let inner = &lower_str[6..];
            DatabaseType::Vec(Box::new(postgresql_type_to_database_type(inner)))
        } else {
            panic!("Unhandled database type {}", r#type)
        }
    }
}

fn sqlite_type_to_database_type(r#type: &str) -> DatabaseType {
    let r#type_string = r#type.to_lowercase();
    let r#type: &str = r#type_string.as_str();
    let regex = Regex::new("([^ \\(\\)]+)( (.+))?(\\((.+)\\))?").unwrap();
    match regex.captures(r#type) {
        None => panic!("Unhandled database type '{}' '{}'.", r#type, regex),
        Some(captures) => {
            let name = captures.get(1).unwrap().as_str();
            let trailing1 = captures.get(3).map(|m| m.as_str());
            let arg = captures.get(5).map(|m| m.as_str());
            match name {
                "integer" => DatabaseType::Int { m: None, u: false },
                "text" => DatabaseType::Text { m: None, n: None, c: None },
                "real" => DatabaseType::Real,
                "double" => DatabaseType::Double { m: None, d: None },
                "decimal" => {
                    if let Some(args) = arg {
                        let args = args.split(",").into_iter().collect::<Vec<&str>>();
                        DatabaseType::Decimal { m: Some(args.get(0).unwrap().parse().unwrap()), d: Some(args.get(1).unwrap().parse().unwrap()) }
                    } else {
                        DatabaseType::Decimal { m: None, d: None }
                    }
                }
                _ => panic!("Unhandled type '{}' '{:?}' '{:?}'.", name, trailing1, arg)
            }
        }
    }
}

fn mssql_type_to_database_type(r#type: &str) -> DatabaseType {
    match r#type.to_lowercase().as_str() {
        _ => panic!("Unhandled database type.")
    }
}
