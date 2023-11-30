use std::ops::{Add, Sub};

use chrono::LocalResult;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, TimeZone};

use crate::Error;

/// The conversion time returned by [`NaiveDateTime::and_local_timezone`]
///
/// [`NaiveDateTime::and_local_timezone`]: chrono::NaiveDateTime::and_local_timezone
#[derive(Debug, PartialEq, Eq)]
pub enum LocalDateTime<Tz: TimeZone> {
    Single(DateTime<Tz>),
    Ambiguous(DateTime<Tz>, DateTime<Tz>),
}

impl<Tz: TimeZone> TryFrom<LocalResult<DateTime<Tz>>> for LocalDateTime<Tz> {
    type Error = Error;

    fn try_from(res: LocalResult<DateTime<Tz>>) -> Result<Self, Self::Error> {
        match res {
            LocalResult::None => Err(Error::Never),
            LocalResult::Single(dt) => Ok(LocalDateTime::Single(dt)),
            LocalResult::Ambiguous(dt1, dt2) => Ok(LocalDateTime::Ambiguous(dt1, dt2)),
        }
    }
}

impl<Tz: TimeZone> LocalDateTime<Tz> {
    /// Returns `Some` when the conversion time is unique, or `None` otherwise.
    pub fn single(self) -> Option<DateTime<Tz>> {
        match self {
            Self::Single(dt) => Some(dt),
            _ => None,
        }
    }

    /// Returns the earliest possible conversion time.
    pub fn earliest(self) -> DateTime<Tz> {
        match self {
            Self::Single(dt) | Self::Ambiguous(dt, _) => dt,
        }
    }

    /// Returns the latest possible conversion time.
    pub fn latest(self) -> DateTime<Tz> {
        match self {
            Self::Single(dt) | Self::Ambiguous(_, dt) => dt,
        }
    }
}

impl<Tz: TimeZone> LocalDateTime<Tz> {
    pub(super) fn from_date(date: NaiveDate, tz: &Tz) -> Result<LocalDateTime<Tz>, Error> {
        tz.from_local_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
            .try_into()
    }

    pub(super) fn from_datetime(
        datetime: NaiveDateTime,
        tz: &Tz,
    ) -> Result<LocalDateTime<Tz>, Error> {
        tz.from_local_datetime(&datetime).try_into()
    }
}

impl<Tz: TimeZone> Add<Duration> for LocalDateTime<Tz> {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        match self {
            Self::Single(dt) => Self::Single(dt + rhs),
            Self::Ambiguous(dt1, dt2) => Self::Ambiguous(dt1 + rhs, dt2 + rhs),
        }
    }
}

impl<Tz: TimeZone> Sub<Duration> for LocalDateTime<Tz> {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        match self {
            Self::Single(dt) => Self::Single(dt - rhs),
            Self::Ambiguous(dt1, dt2) => Self::Ambiguous(dt1 - rhs, dt2 - rhs),
        }
    }
}
