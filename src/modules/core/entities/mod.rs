use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serenity::model::id::{ChannelId, GuildId, MessageId, UserId};
use std::io::Write;

// TODO: implement macro to generate trait for discord id wrappers

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::BigInt)]
pub struct ChannelIdWrapper(pub ChannelId);

impl ToSql<diesel::sql_types::BigInt, Pg> for ChannelIdWrapper {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        ToSql::<diesel::sql_types::BigInt, Pg>::to_sql(&i64::from(self.0), out)
    }
}

impl FromSql<diesel::sql_types::BigInt, Pg> for ChannelIdWrapper {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        i64::from_sql(bytes).map(|id| Self(ChannelId::new(id as u64)))
    }
}

impl FromSql<diesel::sql_types::Nullable<diesel::sql_types::BigInt>, Pg> for ChannelIdWrapper {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        i64::from_sql(bytes).map(|id| Self(ChannelId::new(id as u64)))
    }
}

#[derive(Debug, AsExpression, FromSqlRow, Hash, PartialEq, Eq)]
#[diesel(primary_key(id))]
#[diesel(sql_type = diesel::sql_types::BigInt)]
pub struct GuildIdWrapper(pub GuildId);

impl ToSql<diesel::sql_types::BigInt, Pg> for GuildIdWrapper {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        ToSql::<diesel::sql_types::BigInt, Pg>::to_sql(&i64::from(self.0), out)
    }
}

impl FromSql<diesel::sql_types::BigInt, Pg> for GuildIdWrapper {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        i64::from_sql(bytes).map(|id| Self(GuildId::new(id as u64)))
    }
}

impl FromSql<diesel::sql_types::Nullable<diesel::sql_types::BigInt>, Pg> for GuildIdWrapper {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        i64::from_sql(bytes).map(|id| Self(GuildId::new(id as u64)))
    }
}

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::BigInt)]
pub struct MessageIdWrapper(pub MessageId);

impl ToSql<diesel::sql_types::BigInt, Pg> for MessageIdWrapper {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        ToSql::<diesel::sql_types::BigInt, Pg>::to_sql(&i64::from(self.0), out)
    }
}

impl FromSql<diesel::sql_types::BigInt, Pg> for MessageIdWrapper {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        i64::from_sql(bytes).map(|id| Self(MessageId::new(id as u64)))
    }
}

impl FromSql<diesel::sql_types::Nullable<diesel::sql_types::BigInt>, Pg> for MessageIdWrapper {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        i64::from_sql(bytes).map(|id| Self(MessageId::new(id as u64)))
    }
}

#[derive(Debug, AsExpression, FromSqlRow, Hash, PartialEq, Eq)]
#[diesel(primary_key(id))]
#[diesel(sql_type = diesel::sql_types::BigInt)]
pub struct UserIdWrapper(pub UserId);

impl ToSql<diesel::sql_types::BigInt, Pg> for UserIdWrapper {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        ToSql::<diesel::sql_types::BigInt, Pg>::to_sql(&i64::from(self.0), out)
    }
}

impl FromSql<diesel::sql_types::BigInt, Pg> for UserIdWrapper {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        i64::from_sql(bytes).map(|id| Self(UserId::new(id as u64)))
    }
}

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::schema::sql_types::Language)]
pub enum Language {
    En,
    Pt,
}

impl ToSql<crate::schema::sql_types::Language, Pg> for Language {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Language::En => out.write_all(b"en")?,
            Language::Pt => out.write_all(b"pt")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::Language, Pg> for Language {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"en" => Ok(Language::En),
            b"pt" => Ok(Language::Pt),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::schema::sql_types::PollKind)]
pub enum PollKind {
    SingleChoice,
    MultipleChoice,
}

impl ToSql<crate::schema::sql_types::PollKind, Pg> for PollKind {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            PollKind::SingleChoice => out.write_all(b"single_choice")?,
            PollKind::MultipleChoice => out.write_all(b"multiple_choice")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::PollKind, Pg> for PollKind {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"single_choice" => Ok(PollKind::SingleChoice),
            b"multiple_choice" => Ok(PollKind::MultipleChoice),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

pub mod exports {
    pub use super::guild as Guild;
    pub use super::poll as Poll;
    pub use super::user as User;
    pub use super::Language;
    pub use super::PollKind;
}

pub mod guild;
pub mod poll;
pub mod user;
