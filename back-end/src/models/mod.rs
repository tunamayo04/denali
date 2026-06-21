pub mod budget;
pub mod transactions;
pub mod vendors;

use diesel::prelude::*;
use diesel::pg::Pg;
use diesel::sql_types::Bool;
use diesel::sql_types::SingleValue;
use diesel::expression::AsExpression;
use diesel::expression::ValidGrouping;
use diesel::query_builder::QueryFragment;
use diesel::{AppearsOnTable, SelectableExpression};
use serde::{de, Deserialize, Deserializer};
use serde::de::{DeserializeOwned, IntoDeserializer};
use serde::de::value::{Error as ValError, StrDeserializer};

#[derive(Clone)]
pub enum Filter<T> {
    Eq(T),
    Gt(T),
    Ge(T),
    Lt(T),
    Le(T),
    RangeInclusive(T, T),
    RangeExclusive(T, T),
}

impl<T> Filter<T> {
    pub fn apply<C, QS>(self, column: C) -> Box<dyn BoxableExpression<QS, Pg, SqlType = Bool>>
    where
        C: Column + ExpressionMethods + Copy
        + ValidGrouping<(), IsAggregate = diesel::expression::is_aggregate::No>
        + AppearsOnTable<QS> + SelectableExpression<QS>
        + QueryFragment<Pg> + Send + 'static,
        C::SqlType: SingleValue<IsNull = diesel::sql_types::is_nullable::NotNull>,
        T: AsExpression<C::SqlType> + Send + 'static,
        T::Expression: AppearsOnTable<QS>
        + ValidGrouping<(), IsAggregate = diesel::expression::is_aggregate::Never>
        + SelectableExpression<QS> + QueryFragment<Pg> + Send + 'static,
        QS: Send + 'static,
    {
        match self {
            Filter::Eq(value) => Box::new(column.eq(value)),
            Filter::Gt(value) => Box::new(column.gt(value)),
            Filter::Ge(value) => Box::new(column.ge(value)),
            Filter::Lt(value) => Box::new(column.lt(value)),
            Filter::Le(value) => Box::new(column.le(value)),
            Filter::RangeInclusive(from, to) => Box::new(column.ge(from).and(column.le(to))),
            Filter::RangeExclusive(from, to) => Box::new(column.gt(from).and(column.lt(to))),
        }
    }
}


fn parse_one<T: DeserializeOwned>(s: &str) -> Result<T, String> {
    let d: StrDeserializer<ValError> = s.into_deserializer();
    T::deserialize(d).map_err(|e| e.to_string())
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for Filter<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        let (op, rest) = raw
            .split_once(':')
            .ok_or_else(|| de::Error::custom("expected `op:value`, e.g. `gt:2026-06-19`"))?;

        match op {
            "eq" => Ok(Filter::Eq(parse_one(rest).map_err(de::Error::custom)?)),
            "gt" => Ok(Filter::Gt(parse_one(rest).map_err(de::Error::custom)?)),
            "ge" => Ok(Filter::Ge(parse_one(rest).map_err(de::Error::custom)?)),
            "lt" => Ok(Filter::Lt(parse_one(rest).map_err(de::Error::custom)?)),
            "le" => Ok(Filter::Le(parse_one(rest).map_err(de::Error::custom)?)),
            "range_inclusive" => {
                let (from, to) = rest.split_once(',')
                    .ok_or_else(|| de::Error::custom("expected `from,to`"))?;
                Ok(Filter::RangeInclusive(
                    parse_one(from).map_err(de::Error::custom)?,
                    parse_one(to).map_err(de::Error::custom)?,
                ))
            }
            "range_exclusive" => {
                let (from, to) = rest.split_once(',')
                    .ok_or_else(|| de::Error::custom("expected `from,to`"))?;
                Ok(Filter::RangeExclusive(
                    parse_one(from).map_err(de::Error::custom)?,
                    parse_one(to).map_err(de::Error::custom)?,
                ))
            }
            other => Err(de::Error::custom(format!("unknown filter op `{other}`"))),
        }
    }
}