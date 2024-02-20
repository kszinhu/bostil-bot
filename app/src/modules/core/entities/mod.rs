use diesel::{
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    serialize::{self, ToSql},
    sql_types::{BigInt, Integer, Nullable},
};

use serenity::model::id::{ChannelId, GuildId, MessageId, UserId};

use crate::schema::sql_types::{
    Language as LanguageType, PollKind as PollKindType, PollState as PollStateType,
};

// TODO: implement macro to generate trait for discord id wrappers

#[derive(FromSqlRow, Debug, AsExpression, Clone, Copy)]
#[diesel(sql_type = BigInt)]
pub struct ChannelIdWrapper(pub ChannelId);

impl ToSql<BigInt, Pg> for ChannelIdWrapper
where
    i64: ToSql<BigInt, Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        <i64 as ToSql<BigInt, Pg>>::to_sql(&i64::from(self.0), &mut out.reborrow())
    }
}

impl<DB: Backend> FromSql<BigInt, DB> for ChannelIdWrapper
where
    DB: Backend,
    i64: FromSql<BigInt, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let id = <i64 as FromSql<BigInt, DB>>::from_sql(bytes)?;
        Ok(Self(ChannelId::new(id as u64)))
    }

    fn from_nullable_sql(
        bytes: Option<<DB as Backend>::RawValue<'_>>,
    ) -> deserialize::Result<Self> {
        match bytes {
            Some(bytes) => Self::from_sql(bytes),
            None => Err("Unexpected null for non-null column".into()),
        }
    }
}

#[derive(Debug, AsExpression, FromSqlRow, Hash, PartialEq, Eq, Clone, Copy)]
#[diesel(primary_key(id))]
#[diesel(sql_type = BigInt)]
pub struct GuildIdWrapper(pub GuildId);

impl PartialEq<GuildId> for GuildIdWrapper {
    fn eq(&self, other: &GuildId) -> bool {
        self.0 == *other
    }
}

impl std::fmt::Display for GuildIdWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ToSql<BigInt, Pg> for GuildIdWrapper
where
    i64: ToSql<BigInt, Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        <i64 as ToSql<BigInt, Pg>>::to_sql(&i64::from(self.0), &mut out.reborrow())
    }
}

impl<DB: Backend> FromSql<BigInt, DB> for GuildIdWrapper
where
    i64: FromSql<BigInt, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let id = <i64 as FromSql<BigInt, DB>>::from_sql(bytes)?;
        Ok(Self(GuildId::new(id as u64)))
    }

    fn from_nullable_sql(
        bytes: Option<<DB as Backend>::RawValue<'_>>,
    ) -> deserialize::Result<Self> {
        match bytes {
            Some(bytes) => Self::from_sql(bytes),
            None => Err("Unexpected null for non-null column".into()),
        }
    }
}

#[derive(Debug, AsExpression, FromSqlRow, Clone, Copy)]
#[diesel(sql_type = diesel::sql_types::BigInt)]
pub struct MessageIdWrapper(pub MessageId);

impl ToSql<BigInt, Pg> for MessageIdWrapper
where
    i64: ToSql<BigInt, Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        <i64 as ToSql<BigInt, Pg>>::to_sql(&i64::from(self.0), &mut out.reborrow())
    }
}

impl<DB: Backend> FromSql<BigInt, DB> for MessageIdWrapper
where
    i64: FromSql<BigInt, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let id = <i64 as FromSql<BigInt, DB>>::from_sql(bytes)?;
        Ok(Self(MessageId::new(id as u64)))
    }
}

impl<DB: Backend> FromSql<Nullable<BigInt>, DB> for MessageIdWrapper
where
    i64: FromSql<Nullable<BigInt>, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let id = <i64 as FromSql<Nullable<BigInt>, DB>>::from_sql(bytes)?;
        Ok(Self(MessageId::new(id as u64)))
    }
}

#[derive(Debug, AsExpression, FromSqlRow, Hash, PartialEq, Eq, Clone, Copy)]
#[diesel(primary_key(id))]
#[diesel(sql_type = diesel::sql_types::BigInt)]
pub struct UserIdWrapper(pub UserId);

impl PartialEq<UserId> for UserIdWrapper {
    fn eq(&self, other: &UserId) -> bool {
        self.0 == *other
    }
}

impl std::fmt::Display for UserIdWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ToSql<BigInt, Pg> for UserIdWrapper
where
    i64: ToSql<BigInt, Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        <i64 as ToSql<BigInt, Pg>>::to_sql(&i64::from(self.0), &mut out.reborrow())
    }
}

impl<DB: Backend> FromSql<BigInt, DB> for UserIdWrapper
where
    i64: FromSql<BigInt, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let id = <i64 as FromSql<BigInt, DB>>::from_sql(bytes)?;
        Ok(Self(UserId::new(id as u64)))
    }
}

impl<DB: Backend> FromSql<Nullable<BigInt>, DB> for UserIdWrapper
where
    i64: FromSql<Nullable<BigInt>, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let id = <i64 as FromSql<Nullable<BigInt>, DB>>::from_sql(bytes)?;
        Ok(Self(UserId::new(id as u64)))
    }
}

#[derive(FromSqlRow, AsExpression, Debug, Clone, Copy)]
#[diesel(sql_type = crate::schema::sql_types::Language)]
pub enum Language {
    En,
    Pt,
}

impl<DB> FromSql<LanguageType, DB> for Language
where
    DB: Backend,
    String: FromSql<diesel::sql_types::VarChar, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        match String::from_sql(bytes)?.as_str() {
            "en" => Ok(Language::En),
            "pt" => Ok(Language::Pt),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl ToSql<LanguageType, Pg> for Language
where
    String: ToSql<diesel::sql_types::VarChar, Pg>,
{
    fn to_sql(&self, out: &mut serialize::Output<Pg>) -> serialize::Result {
        match self {
            Language::En => <String as ToSql<diesel::sql_types::VarChar, Pg>>::to_sql(
                &"en".to_string(),
                &mut out.reborrow(),
            ),
            Language::Pt => <String as ToSql<diesel::sql_types::VarChar, Pg>>::to_sql(
                &"pt".to_string(),
                &mut out.reborrow(),
            ),
        }
    }
}

#[repr(i32)]
#[derive(FromSqlRow, AsExpression, Debug, Clone, Copy)]
#[diesel(sql_type = crate::schema::sql_types::PollKind)]
pub enum PollKind {
    SingleChoice,
    MultipleChoice,
}

impl PollKind {
    pub fn to_int(&self) -> i32 {
        match self {
            PollKind::SingleChoice => 0,
            PollKind::MultipleChoice => 1,
        }
    }

    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(PollKind::SingleChoice),
            1 => Some(PollKind::MultipleChoice),
            _ => None,
        }
    }
}

impl ToSql<PollKindType, Pg> for PollKind
where
    i32: ToSql<Integer, Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        <i32 as ToSql<Integer, Pg>>::to_sql(&self.to_int(), &mut out.reborrow())
    }
}

impl<DB: Backend> FromSql<crate::schema::sql_types::PollKind, DB> for PollKind
where
    DB: Backend,
    i32: FromSql<diesel::sql_types::Integer, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let value = i32::from_sql(bytes)?;
        Ok(Self::from_i32(value).ok_or("Unrecognized enum variant")?)
    }
}

#[derive(Debug, FromSqlRow, AsExpression, Clone, Copy)]
#[diesel(sql_type = crate::schema::sql_types::PollState)]
pub enum PollState {
    Created,
    Started,
    Stopped,
    Ended,
}

impl PollState {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(PollState::Created),
            1 => Some(PollState::Started),
            2 => Some(PollState::Stopped),
            3 => Some(PollState::Ended),
            _ => None,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            PollState::Created => 0,
            PollState::Started => 1,
            PollState::Stopped => 2,
            PollState::Ended => 3,
        }
    }
}

impl ToSql<PollStateType, Pg> for PollState
where
    i32: ToSql<Integer, Pg>,
{
    fn to_sql(&self, out: &mut serialize::Output<Pg>) -> serialize::Result {
        <i32 as ToSql<Integer, Pg>>::to_sql(&self.to_i32(), &mut out.reborrow())
    }
}

impl<DB: Backend> FromSql<PollStateType, DB> for PollState
where
    DB: Backend,
    i32: FromSql<diesel::sql_types::Integer, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let value = i32::from_sql(bytes)?;
        Ok(Self::from_i32(value).ok_or("Unrecognized enum variant")?)
    }
}

pub mod exports {
    pub use super::guild as Guild;
    pub use super::poll::{Poll, PollChoice, PollVote};
    pub use super::user as User;
    pub use super::Language;
    pub use super::PollKind;
    pub use super::PollState;
}

pub mod guild;
pub mod poll;
pub mod user;
