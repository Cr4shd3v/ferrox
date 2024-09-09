//! Contains the following custom data types implementations for [diesel]:
//! - [TypedJson]: used to represent JSON as a typed struct instead of [Value].

use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use diesel::backend::Backend;
use diesel::query_builder::bind_collector::RawBytesBindCollector;
use diesel::serialize::{IsNull, Output};
use diesel::sql_types::Json;
use diesel::{deserialize, serialize, AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Custom SQL Type for diesel for storing json with type information.
///
/// Stores the given type as JSON in the database.
///
/// This type will always parse the json from the database when loaded.
#[derive(Debug, FromSqlRow, AsExpression)]
#[diesel(sql_type = diesel::sql_types::Json)]
pub struct TypedJson<T: Serialize + for<'a> Deserialize<'a> + Debug>(pub T);

impl<T: Serialize + for<'a> Deserialize<'a> + Debug> Deref for TypedJson<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Serialize + for<'a> Deserialize<'a> + Debug> DerefMut for TypedJson<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<B: Backend, T: Serialize + for<'a> Deserialize<'a> + Debug> serialize::ToSql<Json, B> for TypedJson<T> where Value: serialize::ToSql<Json, B>,
                                                                                                                  for<'a> B: Backend<BindCollector<'a> = RawBytesBindCollector<B>>
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, B>) -> serialize::Result {
        serde_json::to_writer(out, &self.0)
            .map(|_| IsNull::No)
            .map_err(Into::into)
    }
}

impl<B: Backend, T: Serialize + for<'a> Deserialize<'a> + Debug> deserialize::FromSql<Json, B> for TypedJson<T> where Value: deserialize::FromSql<Json, B> {
    fn from_sql(bytes: B::RawValue<'_>) -> deserialize::Result<Self> {
        <Value as deserialize::FromSql<Json, B>>::from_sql(bytes).map(|v| TypedJson(serde_json::from_value::<T>(v).unwrap()))
    }
}