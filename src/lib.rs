#![cfg_attr(feature = "cargo-clippy", deny(clippy, clippy_pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(non_ascii_literal))]

//! # chrono systemd.time
//! 
//! [chrono-systemd-time] is a library which parses timestamps following the [systemd.time] specifications into [chrono] types.
//! 
//! [chrono-systemd-time]: https://docs.rs/chrono-systemd-time/
//! [systemd.time]: https://www.freedesktop.org/software/systemd/man/systemd.time.html
//! [chrono]: https://docs.rs/chrono/
//! 
//! ## Usage
//!
//! Put this in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! chrono-systemd-time = "0.1"
//! ```
//! 
//! ## Timestamp Format
//! 
//! The supported timestamp formats are any defined by the systemd.time specifications, with a few exceptions:
//! * time units **must** accompany all time span values.
//! * time zone suffixes are **not** supported.
//! * weekday prefixes are **not** supported.
//!
//! The format of a timestamp may be either a time, a time span, or a combination of a time +/- a time span.
//! * When only a time is given, the parsed time is returned.
//! * When only a time span is given, the time span is added or subtracted from the current time (now).
//! * When a combination of a time and a time span is given, the time span is added or subtracted from the parsed time.
//!
//! Examples of parsing valid timestamps, assuming now is 2018-06-21 01:02:03:
//! ```rust,ignore
//!     parse_timestamp_tz("2018-08-20 09:11:12.123", Utc) == "2018-08-20T09:11:12.000123Z"
//!     parse_timestamp_tz("2018-08-20 09:11:12", Utc) == "2018-08-20T09:11:12Z"
//!     parse_timestamp_tz("18-08-20 09:11:12 +2m", Utc) == "2018-08-20T09:13:12Z"
//!     parse_timestamp_tz("2018-08-20 + 1h2m3s", Utc) == "2018-08-20T01:02:03Z"
//!     parse_timestamp_tz("18-08-20 - 1h 2m 3s", Utc) == "2018-08-19T22:57:57Z"
//!     parse_timestamp_tz("09:11:12 -1day", Utc) == "2018-06-20T09:11:12Z"
//!     parse_timestamp_tz("09:11:12.123", Utc) == "2018-06-21T09:11:12.000123Z"
//!     parse_timestamp_tz("11:12", Utc) == "2018-06-21T11:12:00Z"
//!     parse_timestamp_tz("now", Utc) == "2018-06-21T01:02:03.203918151Z"
//!     parse_timestamp_tz("today", Utc) == "2018-06-21T00:00:00Z"
//!     parse_timestamp_tz("yesterday -2days", Utc) == "2018-06-18T00:00:00Z"
//!     parse_timestamp_tz("tomorrow +1week", Utc) == "2018-06-29T00:00:00Z"
//! 
//!     parse_timestamp_tz("epoch +1529578800s", Utc) == "2018-06-21T11:00:00Z"
//!     parse_timestamp_tz("@1529578800s", Utc) == "2018-06-21T11:00:00Z"
//!     parse_timestamp_tz("now +4h50m", Utc) == "2018-06-21T05:52:03.203918151Z"
//!     parse_timestamp_tz("4h50m left", Utc) == "2018-06-21T05:52:03.203918151Z"
//!     parse_timestamp_tz("+4h50m", Utc) == "2018-06-21T05:52:03.203918151Z"
//!     parse_timestamp_tz("now -3s", Utc) == "2018-06-21T01:02:00.203918151Z"
//!     parse_timestamp_tz("3s ago", Utc) == "2018-06-21T01:02:00.203918151Z"
//!     parse_timestamp_tz("-3s", Utc) == "2018-06-21T01:02:00.203918151Z"
//! ```
//!
//! #### Time
//! The syntax of a time consists of a set of keywords and strftime formats:
//! * `"now"`, `"epoch"`
//! * `"today"`, `"yesterday"`, `"tomorrow"`
//! * `"%y-%m-%d %H:%M:%S"`, `"%Y-%m-%d %H:%M:%S"`
//! * `"%y-%m-%d %H:%M"`, `"%Y-%m-%d %H:%M"`
//! * `"%y-%m-%d"`, `"%Y-%m-%d"`
//! * `"%H:%M:%S"`
//! * `"%H:%M"`
//!
//! Strftime timestamps with a seconds component may also include a microsecond component, separated by a `'.'`.
//! * When the date is omitted, today is assumed.
//! * When the time is omitted, 00:00:00 is assumed.
//!
//! Examples of valid times (assuming now is 2018-06-21 01:02:03):
//! ```rust,ignore
//!     "2018-08-20 09:11:12.123" == "2018-08-20T09:11:12.000123"
//!         "2018-08-20 09:11:12" == "2018-08-20T09:11:12"
//!           "18-08-20 09:11:12" == "2018-08-20T09:11:12"
//!                  "2018-08-20" == "2018-08-20T00:00:00"
//!                    "18-08-20" == "2018-08-20T00:00:00"
//!                    "09:11:12" == "2018-06-21T09:11:12"
//!                "09:11:12.123" == "2018-06-21T09:11:12.000123"
//!                       "11:12" == "2018-06-21T11:12:00"
//!                         "now" == "2018-06-21T01:02:03.203918151"
//!                       "epoch" == "1970-01-01T00:00:00"
//!                       "today" == "2018-06-21T00:00:00"
//!                   "yesterday" == "2018-06-20T00:00:00"
//!                    "tomorrow" == "2018-06-22T00:00:00"
//! ```
//!        
//! #### Time span
//! A time span is made up of a combination of time units, with the following time units understood:
//! * `"usec"`, `"us"`, `"µs"`
//! * `"msec"`, `"ms"`
//! * `"seconds"`, `"second"`, `"sec"`, `"s"`
//! * `"minutes"`, `"minute"`, `"min"`, `"m"`
//! * `"hours"`, `"hour"`, `"hr"`, `"h"`
//! * `"days"`, `"day"`, `"d"`
//! * `"weeks"`, `"week"`, `"w"`
//! * `"months"`, `"month"`, `"M"` (defined as 30.44 days)
//! * `"years"`, `"year"`, `"y"` (defined as 365.25 days)
//!
//! All components of a time span are added to together.
//!
//! Examples of valid time spans:
//! ```rust,ignore
//!           "3hours" == Duration::hours(3)
//!            "2d 5h" == Duration::days(2) + Duration::hours(5)
//!     "1y 10 months" == Duration::years(1) + Duration::months(10)
//!           "30m22s" == Duration::minutes(30) + Duration::seconds(22)
//!        "10m 2s 5m" == Duration::minutes(15) + Duration::seconds(2)
//!         "10d 2 5m" == Duration::days(10) + Duration::minutes(25)
//! ```
extern crate chrono;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::str;

use chrono::{Date, DateTime, Datelike, Duration, NaiveDate, NaiveTime, TimeZone};
use chrono::offset::Utc;

/* 
 * Chrono stores its DateTimes and Durations in i64s, so use that here.
 * Ideally we would use a larger primitive type (and unsigned).
 */

const USEC_PER_USEC: i64 = 1;
const USEC_PER_MSEC: i64 = 1_000 * USEC_PER_USEC;
const USEC_PER_SEC: i64 = 1_000 * USEC_PER_MSEC;
const USEC_PER_MINUTE: i64 = 60 * USEC_PER_SEC;
const USEC_PER_HOUR: i64 = 60 * USEC_PER_MINUTE;
const USEC_PER_DAY: i64 = 24 * USEC_PER_HOUR;
const USEC_PER_WEEK: i64 = 7 * USEC_PER_DAY;
const USEC_PER_MONTH: i64 = 2_629_800 * USEC_PER_SEC;
const USEC_PER_YEAR: i64 = 31_557_600 * USEC_PER_SEC;

lazy_static! {
    static ref USEC_MULTIPLIER: HashMap<&'static str, i64> = hashmap! {
        "us" =>      USEC_PER_USEC,
        "usec" =>    USEC_PER_USEC,
        "µs" =>      USEC_PER_USEC,

        "ms" =>      USEC_PER_MSEC,
        "msec" =>    USEC_PER_MSEC,

        "s" =>       USEC_PER_SEC,
        "sec" =>     USEC_PER_SEC,
        "second" =>  USEC_PER_SEC,
        "seconds" => USEC_PER_SEC,

        "m" =>       USEC_PER_MINUTE,
        "min" =>     USEC_PER_MINUTE,
        "minute" =>  USEC_PER_MINUTE,
        "minutes" => USEC_PER_MINUTE,

        "h" =>       USEC_PER_HOUR,
        "hour" =>    USEC_PER_HOUR,
        "hours" =>   USEC_PER_HOUR,
        "hr" =>      USEC_PER_HOUR,

        "d" =>       USEC_PER_DAY,
        "day" =>     USEC_PER_DAY,
        "days" =>    USEC_PER_DAY,

        "M" =>       USEC_PER_MONTH,
        "month" =>   USEC_PER_MONTH,
        "months" =>  USEC_PER_MONTH,

        "w" =>       USEC_PER_WEEK,
        "week" =>    USEC_PER_WEEK,
        "weeks" =>   USEC_PER_WEEK,

        "y" =>       USEC_PER_YEAR,
        "year" =>    USEC_PER_YEAR,
        "years" =>   USEC_PER_YEAR,
    };
}

/// Describes an error during the parsing of a timestamp.
#[derive(Debug)]
pub enum InvalidTimestamp {
    /// The timestamp is incorrectly formatted. 
    Format(String),
    /// The timestamp contains a component that cannot be parsed into a number, or the number overflowed.
    Number(String),
    /// The timestamp contains a component that cannot be parsed into a time unit.
    TimeUnit(String),
}

impl Error for InvalidTimestamp {
    fn description(&self) -> &str {
        match *self {
            InvalidTimestamp::Format(ref err_msg)
            | InvalidTimestamp::Number(ref err_msg)
            | InvalidTimestamp::TimeUnit(ref err_msg) => err_msg,
        }
    }
}

impl fmt::Display for InvalidTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvalidTimestamp::Format(_) => f.write_str("invalid timestamp format"),
            InvalidTimestamp::Number(_) => f.write_str("invalid timestamp number"),
            InvalidTimestamp::TimeUnit(_) => f.write_str("invalid timestamp time unit"),
        }
    }
}

/// Parse a timestamp returning a `DateTime` with the specified timezone.
/// 
/// # Examples
/// ```rust
///     # extern crate chrono;
///     # extern crate chrono_systemd_time;
///     # use chrono_systemd_time::parse_timestamp_tz;
///     use chrono::{Duration, Local, Utc};
/// 
///     assert_eq!(parse_timestamp_tz("today + 2h", Utc).expect(""),
///                parse_timestamp_tz("today", Utc).expect("") + Duration::hours(2));
///     assert_eq!(parse_timestamp_tz("yesterday", Local).expect(""),
///                parse_timestamp_tz("today - 1d", Local).expect(""));
///     assert_eq!(parse_timestamp_tz("2018-06-21", Utc).expect(""),
///                parse_timestamp_tz("18-06-21 1:00 - 1h", Utc).expect(""));
/// ```
pub fn parse_timestamp_tz<S, T, Tz>(timestamp: S, time_zone: T) -> Result<DateTime<Tz>, InvalidTimestamp>
    where S: AsRef<str>, T: Borrow<Tz>, Tz: TimeZone {
    let tz = time_zone.borrow();
    let ts = timestamp.as_ref();
    let ts_nw = ts.chars().filter(|&c| !c.is_whitespace()).collect::<String>();

    if ts_nw.is_empty() {
        return Err(InvalidTimestamp::Format("Timestamp cannot be empty".to_string()));
    }

    /* 
     * A timestamp is composed of two parts: a time and an offset relative to that time.
     * 
     * In the general case, the time is separated from the offset by either a '+' or '-' 
     * character which denotes how the offset is relative to that time.
     * 
     * There are a few special cases which are not handled by the general case.
     * These are detected, and handled, before applying the general case algorithm.
     */

    // Special Case 1 - a suffix of " left" or " ago", or a prefix of '+' or '-':
    //  - the time is now.
    //  - the offset consists of the remaining characters added to or subtracted from the current time, respectively.
    if ts.starts_with('+') {
        let now = Utc::now().with_timezone(tz);
        let offset = parse_offset(&ts_nw[1..])?;
        return Ok(now + offset)
    }
    if ts.ends_with(" left") {
        let now = Utc::now().with_timezone(tz);
        let offset = parse_offset(&ts_nw[..(ts_nw.len() - 4)])?;
        return Ok(now + offset)
    }

    if ts.starts_with('-') {
        let now = Utc::now().with_timezone(tz);
        let offset = parse_offset(&ts_nw[1..])?;
        return Ok(now - offset)
    }
    if ts.ends_with(" ago") {
        let now = Utc::now().with_timezone(tz);
        let offset = parse_offset(&ts_nw[..(ts_nw.len() - 3)])?;
        return Ok(now - offset)
    }

    // Special Case 2 - a prefix of '@':
    //  - the time is the unix epoch.
    //  - the offset consists of the remaining characters added to the epoch time.
    if ts.starts_with('@') {
        let epoch = tz.timestamp(0, 0);
        let offset = parse_offset(&ts_nw[1..])?;
        return Ok(epoch + offset)
    }

    // General Case - the time is separated from the offset by either a '+' or '-'.
    // Note: need to find " +" and " -" here because strftime date formats may contain the '-' character,
    //       but with no leading whitespaces.
    match (ts.find(" +"), ts.find(" -")) {
        (Some(_), Some(_)) => {
            Err(InvalidTimestamp::Format("Timestamp cannot contain both a `+` and `-`".to_string()))
        },
        (Some(p), None) => {
            let p_nw = ts_nw.find('+').unwrap();
            let time = parse_time(&ts[..p], tz)?;
            let offset = parse_offset(&ts_nw[(p_nw + 1)..])?;
            Ok(time + offset)
        },
        (None, Some(m)) => {
            let m_nw = ts_nw.rfind('-').unwrap();
            let time = parse_time(&ts[..m], tz)?;
            let offset = parse_offset(&ts_nw[(m_nw + 1)..])?;
            Ok(time - offset)
        },
        (None, None) => {
            let time = parse_time(&ts, tz)?;
            Ok(time)
        },
    }
}

/// Parse a point-in-time into a `DateTime` with the given timezone.
/// 
/// * `ts` - a str of a time with whitespace intact.
/// * `tz` - the time zone to use.
fn parse_time<Tz: TimeZone>(ts: &str, tz: &Tz) -> Result<DateTime<Tz>, InvalidTimestamp> {
    if ts == "now" {
        let now = Utc::now().with_timezone(tz);
        return Ok(now);
    }

    if ts == "epoch" {
        let epoch = tz.timestamp(0, 0);
        return Ok(epoch);
    }

    if ts == "today" {
        let today = today_with_timezone(tz).and_hms(0, 0, 0);
        return Ok(today);
    }
    if ts == "yesterday" {
        let today = today_with_timezone(tz).and_hms(0, 0, 0);
        return Ok(today - Duration::days(1));
    }
    if ts == "tomorrow" {
        let today = today_with_timezone(tz).and_hms(0, 0, 0);
        return Ok(today + Duration::days(1));
    }

    // an optional '.' separates the seconds and microseconds components
    if let Some(p) = ts.find('.') {
        let ts_t = &ts[..p];
        let dt = if let Ok(dt) = tz.datetime_from_str(ts_t, "%y-%m-%d %H:%M:%S") {
            dt
        } else if let Ok(dt) = tz.datetime_from_str(ts_t, "%Y-%m-%d %H:%M:%S") {
            dt
        } else if let Ok(nt) = NaiveTime::parse_from_str(ts_t, "%H:%M:%S") {
            today_with_timezone(tz).and_time(nt).unwrap()
        } else {
            let err_msg = format!("Cannot parse `{}` before '.' into a time", ts_t);
            return Err(InvalidTimestamp::Format(err_msg));
        };

        let ts_u = &ts[(p + 1)..];
        let usecs = match ts_u.parse::<i64>() {
            Ok(v) => v,
            Err(e) => {
                let err_msg = format!("Cannot parse `{}` after '.' into a number: {}", ts_u, e);
                return Err(InvalidTimestamp::Number(err_msg));
            }
        };

        return Ok(dt + Duration::microseconds(usecs));
    }

    if let Ok(dt) = tz.datetime_from_str(ts, "%y-%m-%d %H:%M:%S") {
        return Ok(dt);
    }
    if let Ok(dt) = tz.datetime_from_str(ts, "%Y-%m-%d %H:%M:%S") {
        return Ok(dt);
    }
    if let Ok(dt) = tz.datetime_from_str(ts, "%y-%m-%d %H:%M") {
        return Ok(dt);
    }
    if let Ok(dt) = tz.datetime_from_str(ts, "%Y-%m-%d %H:%M") {
        return Ok(dt);
    }
    if let Ok(nd) = NaiveDate::parse_from_str(ts, "%y-%m-%d") {
        let dt = tz.ymd(nd.year(), nd.month(), nd.day()).and_hms(0, 0, 0);
        return Ok(dt);
    }
    if let Ok(nd) = NaiveDate::parse_from_str(ts, "%Y-%m-%d") {
        let dt = tz.ymd(nd.year(), nd.month(), nd.day()).and_hms(0, 0, 0);
        return Ok(dt);
    }
    if let Ok(nt) = NaiveTime::parse_from_str(ts, "%H:%M:%S") {
        let dt = today_with_timezone(tz).and_time(nt).unwrap();
        return Ok(dt);
    }
    if let Ok(nt) = NaiveTime::parse_from_str(ts, "%H:%M") {
        let dt = today_with_timezone(tz).and_time(nt).unwrap();
        return Ok(dt);
    }

    let err_msg = format!("Cannot parse `{}` into a time", ts);
    Err(InvalidTimestamp::Format(err_msg))
}

/// Parse and combine all time spans into a single duration.
/// 
/// * `ts_nw` - a str of time spans with whitespace removed.
fn parse_offset(mut ts_nw: &str) -> Result<Duration, InvalidTimestamp> {
    let mut total_usecs: i64 = 0;
    loop {
        if ts_nw.is_empty() {
            return Ok(Duration::microseconds(total_usecs)); 
        }

        /*
         * Time spans have the format: "<number><multipler>"
         */

        // look for digit characters to make up the `number`
        // followed by alphabetic characters to make up the `multiplier`
        let (digits, ts_tail) = partition_predicate(ts_nw, |c| c.is_digit(10));
        let (letters, ts_tail) = partition_predicate(ts_tail, |c| c.is_alphabetic());
        ts_nw = ts_tail;

        // parse the `number` and `multipler` strings into i64
        let number = match digits.parse::<i64>() {
            Ok(n) => n,
            Err(e) => {
                let err_msg = format!("Cannot parse `{}` into a number: {}", digits, e);
                return Err(InvalidTimestamp::Number(err_msg));
            }
        };
        let multiplier = if let Some(m) = USEC_MULTIPLIER.get::<str>(&letters) {
            *m
        } else {
            let err_msg = format!("Cannot parse `{}` into a multipler", letters);
            return Err(InvalidTimestamp::TimeUnit(err_msg));
        };

        // increment the total microsecond offset returning a failure on an overflow
        total_usecs = if let Some(usecs) = number.checked_mul(multiplier)
                                                 .and_then(|usec| usec.checked_add(total_usecs)) {
            usecs
        } else {
            let err_msg = format!("Offset microseconds overflowed: total_usecs `{}` number `{}` multiplier `{}`",
                                  total_usecs, number, multiplier);
            return Err(InvalidTimestamp::Number(err_msg));
        }
    }
}

/// Return the date of today with the given timezone.
fn today_with_timezone<Tz: TimeZone>(tz: &Tz) -> Date<Tz> {
    // the obvious method, `Utc::today().with_timezone(tz)`, is incorrect because
    // the date may be different depending on how close the offset is to UTC
    Utc::now().with_timezone(tz).date()
}


/// Partition a str by a given predicate.
/// Returned is a tuple where:
/// - the first element contains the sub-slice of sequential characters that tested true.
/// - the second element contains the remaining characters of the original str.
fn partition_predicate<P>(ts: &str, predicate: P) -> (&str, &str) 
    where P : Fn(char) -> bool {
    if let Some(p) = ts.find(|c: char| !predicate(c)) {
        (&ts[..p], &ts[p..])
    } else {
        (ts, "")
    }
}

#[cfg(test)]
mod tests {

    use chrono::{Duration, TimeZone};
    use chrono::offset::{Utc, Local};

    use super::parse_timestamp_tz;
    use super::{USEC_PER_MONTH, USEC_PER_YEAR};
    use super::InvalidTimestamp;

    /*
     * Positive Tests
     */

    /// Test extracting a time from a keyword.
    #[test]
    fn time_word() {
        let now = Utc::now();
        let epoch = Utc.timestamp(0, 0);
        assert!(parse_timestamp_tz("now", Utc).unwrap() >= now);
        assert_eq!(parse_timestamp_tz("epoch", Utc).unwrap(), epoch);
        assert!(parse_timestamp_tz("now", Local).unwrap().with_timezone(&Utc) >= now);
        assert_eq!(parse_timestamp_tz("epoch", Local).unwrap(), epoch);

        let today_utc = Utc::today().and_hms(0, 0, 0);
        let tomorrow_utc = today_utc + Duration::days(1);
        let yesterday_utc = today_utc - Duration::days(1);
        assert_eq!(parse_timestamp_tz("today", Utc).unwrap(), today_utc);
        assert_eq!(parse_timestamp_tz("tomorrow", Utc).unwrap(), tomorrow_utc);
        assert_eq!(parse_timestamp_tz("yesterday", Utc).unwrap(), yesterday_utc);

        let today_local = Local::today().and_hms(0, 0, 0);
        let tomorrow_local = today_local + Duration::days(1);
        let yesterday_local = today_local - Duration::days(1);
        assert_eq!(parse_timestamp_tz("today", Local).unwrap(), today_local);
        assert_eq!(parse_timestamp_tz("tomorrow", Local).unwrap(), tomorrow_local);
        assert_eq!(parse_timestamp_tz("yesterday", Local).unwrap(), yesterday_local);
    }

    /// Test extracting a time from a strftime formatted timestamp.
    #[test]
    fn time_strftime() {
        assert_eq!(parse_timestamp_tz("2018-08-09 07:06:05", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 5));
        assert_eq!(parse_timestamp_tz("18-08-09 07:06:05", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 5));
        assert_eq!(parse_timestamp_tz("2018-08-09 07:06", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 0));
        assert_eq!(parse_timestamp_tz("18-08-09 07:06", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 0));
        assert_eq!(parse_timestamp_tz("2018-08-09", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(0, 0, 0));
        assert_eq!(parse_timestamp_tz("18-08-09", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(0, 0, 0));
        assert_eq!(parse_timestamp_tz("10:11:12", Utc).unwrap(), Utc::today().and_hms(10, 11, 12));
        assert_eq!(parse_timestamp_tz("10:11", Utc).unwrap(), Utc::today().and_hms(10, 11, 0));

        assert_eq!(parse_timestamp_tz("2018-08-09 07:06:05.123", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 5) + Duration::microseconds(123));
        assert_eq!(parse_timestamp_tz("18-08-09 07:06:05.1", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 5) + Duration::microseconds(1));
        assert_eq!(parse_timestamp_tz("10:11:12.1234", Utc).unwrap(), Utc::today().and_hms(10, 11, 12) + Duration::microseconds(1234));

        assert_eq!(parse_timestamp_tz("2018-08-09 07:06:05", Local).unwrap(), Local.ymd(2018, 8, 9).and_hms(7, 6, 5));
        assert_eq!(parse_timestamp_tz("18-08-09 07:06:05", Local).unwrap(), Local.ymd(2018, 8, 9).and_hms(7, 6, 5));
        assert_eq!(parse_timestamp_tz("2018-08-09 07:06", Local).unwrap(), Local.ymd(2018, 8, 9).and_hms(7, 6, 0));
        assert_eq!(parse_timestamp_tz("18-08-09 07:06", Local).unwrap(), Local.ymd(2018, 8, 9).and_hms(7, 6, 0));
        assert_eq!(parse_timestamp_tz("2018-08-09", Local).unwrap(), Local.ymd(2018, 8, 9).and_hms(0, 0, 0));
        assert_eq!(parse_timestamp_tz("18-08-09", Local).unwrap(), Local.ymd(2018, 8, 9).and_hms(0, 0, 0));
        assert_eq!(parse_timestamp_tz("10:11:12", Local).unwrap(), Local::today().and_hms(10, 11, 12));
        assert_eq!(parse_timestamp_tz("10:11", Local).unwrap(), Local::today().and_hms(10, 11, 0));

        assert_eq!(parse_timestamp_tz("2018-08-09 07:06:05.123", Local).unwrap(), Local.ymd(2018, 8, 9).and_hms(7, 6, 5) + Duration::microseconds(123));
        assert_eq!(parse_timestamp_tz("18-08-09 07:06:05.1", Local).unwrap(), Local.ymd(2018, 8, 9).and_hms(7, 6, 5) + Duration::microseconds(1));
        assert_eq!(parse_timestamp_tz("10:11:12.1234", Local).unwrap(), Local::today().and_hms(10, 11, 12) + Duration::microseconds(1234));
    }

    /// Test applying an offset to time keywords.
    #[test]
    fn offset_word() {
        let today = parse_timestamp_tz("today", Utc).unwrap();
        assert_eq!(parse_timestamp_tz("today +1s", Utc).unwrap(), today + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("today +1s2m", Utc).unwrap(), today + Duration::seconds(1) + Duration::minutes(2));
        assert_eq!(parse_timestamp_tz("today -1s", Utc).unwrap(), today - Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("today -1s2m", Utc).unwrap(), today - Duration::seconds(1) - Duration::minutes(2));

        let tomorrow = parse_timestamp_tz("tomorrow", Utc).unwrap();
        assert_eq!(parse_timestamp_tz("tomorrow +1s", Utc).unwrap(), tomorrow + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("tomorrow +1s2m", Utc).unwrap(), tomorrow + Duration::seconds(1) + Duration::minutes(2));
        assert_eq!(parse_timestamp_tz("tomorrow -1s", Utc).unwrap(), tomorrow - Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("tomorrow -1s2m", Utc).unwrap(), tomorrow - Duration::seconds(1) - Duration::minutes(2));

        let yesterday = parse_timestamp_tz("yesterday", Utc).unwrap();
        assert_eq!(parse_timestamp_tz("yesterday +1s", Utc).unwrap(), yesterday + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("yesterday +1s2m", Utc).unwrap(), yesterday + Duration::seconds(1) + Duration::minutes(2));
        assert_eq!(parse_timestamp_tz("yesterday -1s", Utc).unwrap(), yesterday - Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("yesterday -1s2m", Utc).unwrap(), yesterday - Duration::seconds(1) - Duration::minutes(2));

        let epoch = parse_timestamp_tz("epoch", Utc).unwrap();
        assert_eq!(parse_timestamp_tz("epoch +1s", Utc).unwrap(), epoch + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("epoch +1s2m", Utc).unwrap(), epoch + Duration::seconds(1) + Duration::minutes(2));
        assert_eq!(parse_timestamp_tz("epoch -1s", Utc).unwrap(), epoch - Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("epoch -1s2m", Utc).unwrap(), epoch - Duration::seconds(1) - Duration::minutes(2));

        let now = parse_timestamp_tz("now", Utc).unwrap();
        assert!(parse_timestamp_tz("now +1s", Utc).unwrap() >= now + Duration::seconds(1));
        assert!(parse_timestamp_tz("now +1s2m", Utc).unwrap() >= now + Duration::seconds(1) + Duration::minutes(2));
        assert!(parse_timestamp_tz("now -1s", Utc).unwrap() >= now - Duration::seconds(1));
        assert!(parse_timestamp_tz("now -1s2m", Utc).unwrap() >= now - Duration::seconds(1) - Duration::minutes(2));
    }

    /// Test applying an offset to strftime formatted timestamps.
    #[test]
    fn offset_strftime() {
        assert_eq!(parse_timestamp_tz("2018-08-09 07:06:05 +1s", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 5) + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("2018-08-09 07:06:05 +1s2m", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 5) + Duration::seconds(1) + Duration::minutes(2));
        assert_eq!(parse_timestamp_tz("2018-08-09 07:06:05 -1s", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 5) - Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("2018-08-09 07:06:05 -1s2m", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 5) - Duration::seconds(1) - Duration::minutes(2));

        assert_eq!(parse_timestamp_tz("18-08-09 07:06:05 +1s", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 5) + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("18-08-09 07:06:05 +1s2m", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 5) + Duration::seconds(1) + Duration::minutes(2));
        assert_eq!(parse_timestamp_tz("18-08-09 07:06:05 -1s", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 5) - Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("18-08-09 07:06:05 -1s2m", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 5) - Duration::seconds(1) - Duration::minutes(2));

        assert_eq!(parse_timestamp_tz("2018-08-09 07:06 +1s", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 0) + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("2018-08-09 07:06 +1s2m", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 0) + Duration::seconds(1) + Duration::minutes(2));
        assert_eq!(parse_timestamp_tz("2018-08-09 07:06 -1s", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 0) - Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("2018-08-09 07:06 -1s2m", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 0) - Duration::seconds(1) - Duration::minutes(2));

        assert_eq!(parse_timestamp_tz("18-08-09 07:06 +1s", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 0) + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("18-08-09 07:06 +1s2m", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 0) + Duration::seconds(1) + Duration::minutes(2));
        assert_eq!(parse_timestamp_tz("18-08-09 07:06 -1s", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 0) - Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("18-08-09 07:06 -1s2m", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(7, 6, 0) - Duration::seconds(1) - Duration::minutes(2));

        assert_eq!(parse_timestamp_tz("2018-08-09 +1s", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(0, 0, 0) + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("2018-08-09 +1s2m", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(0, 0, 0) + Duration::seconds(1) + Duration::minutes(2));
        assert_eq!(parse_timestamp_tz("2018-08-09 -1s", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(0, 0, 0) - Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("2018-08-09 -1s2m", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(0, 0, 0) - Duration::seconds(1) - Duration::minutes(2));

        assert_eq!(parse_timestamp_tz("18-08-09 +1s", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(0, 0, 0) + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("18-08-09 +1s2m", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(0, 0, 0) + Duration::seconds(1) + Duration::minutes(2));
        assert_eq!(parse_timestamp_tz("18-08-09 -1s", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(0, 0, 0) - Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("18-08-09 -1s2m", Utc).unwrap(), Utc.ymd(2018, 8, 9).and_hms(0, 0, 0) - Duration::seconds(1) - Duration::minutes(2));

        assert_eq!(parse_timestamp_tz("10:11:12 +1s", Utc).unwrap(), Utc::today().and_hms(10, 11, 12) + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("10:11:12 +1s2m", Utc).unwrap(), Utc::today().and_hms(10, 11, 12) + Duration::seconds(1) + Duration::minutes(2));
        assert_eq!(parse_timestamp_tz("10:11:12 -1s", Utc).unwrap(), Utc::today().and_hms(10, 11, 12) - Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("10:11:12 -1s2m", Utc).unwrap(), Utc::today().and_hms(10, 11, 12) - Duration::seconds(1) - Duration::minutes(2));

        assert_eq!(parse_timestamp_tz("10:11 +1s", Utc).unwrap(), Utc::today().and_hms(10, 11, 0) + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("10:11 +1s2m", Utc).unwrap(), Utc::today().and_hms(10, 11, 0) + Duration::seconds(1) + Duration::minutes(2));
        assert_eq!(parse_timestamp_tz("10:11 -1s", Utc).unwrap(), Utc::today().and_hms(10, 11, 0) - Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("10:11 -1s2m", Utc).unwrap(), Utc::today().and_hms(10, 11, 0) - Duration::seconds(1) - Duration::minutes(2));
    }

    /// Test the various offset time unit keywords.
    #[test]
    fn offset_time_unit() {
        let today = parse_timestamp_tz("today", Utc).unwrap();
        assert_eq!(parse_timestamp_tz("today + 1 seconds", Utc).unwrap(), today + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("today + 1 second", Utc).unwrap(), today + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("today + 1 sec", Utc).unwrap(), today + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("today + 1 s", Utc).unwrap(), today + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("today + 1 minutes", Utc).unwrap(), today + Duration::minutes(1));
        assert_eq!(parse_timestamp_tz("today + 1 minute", Utc).unwrap(), today + Duration::minutes(1));
        assert_eq!(parse_timestamp_tz("today + 1 min", Utc).unwrap(), today + Duration::minutes(1));
        assert_eq!(parse_timestamp_tz("today + 1 months", Utc).unwrap(), today + Duration::microseconds(USEC_PER_MONTH));
        assert_eq!(parse_timestamp_tz("today + 1 month", Utc).unwrap(), today + Duration::microseconds(USEC_PER_MONTH));
        assert_eq!(parse_timestamp_tz("today + 1 M", Utc).unwrap(), today + Duration::microseconds(USEC_PER_MONTH));
        assert_eq!(parse_timestamp_tz("today + 1 msec", Utc).unwrap(), today + Duration::milliseconds(1));
        assert_eq!(parse_timestamp_tz("today + 1 ms", Utc).unwrap(), today + Duration::milliseconds(1));
        assert_eq!(parse_timestamp_tz("today + 1 m", Utc).unwrap(), today + Duration::minutes(1));
        assert_eq!(parse_timestamp_tz("today + 1 hours", Utc).unwrap(), today + Duration::hours(1));
        assert_eq!(parse_timestamp_tz("today + 1 hour", Utc).unwrap(), today + Duration::hours(1));
        assert_eq!(parse_timestamp_tz("today + 1 hr", Utc).unwrap(), today + Duration::hours(1));
        assert_eq!(parse_timestamp_tz("today + 1 h", Utc).unwrap(), today + Duration::hours(1));
        assert_eq!(parse_timestamp_tz("today + 1 days", Utc).unwrap(), today + Duration::days(1));
        assert_eq!(parse_timestamp_tz("today + 1 day", Utc).unwrap(), today + Duration::days(1));
        assert_eq!(parse_timestamp_tz("today + 1 d", Utc).unwrap(), today + Duration::days(1));
        assert_eq!(parse_timestamp_tz("today + 1 weeks", Utc).unwrap(), today + Duration::days(7));
        assert_eq!(parse_timestamp_tz("today + 1 week", Utc).unwrap(), today + Duration::days(7));
        assert_eq!(parse_timestamp_tz("today + 1 w", Utc).unwrap(), today + Duration::days(7));
        assert_eq!(parse_timestamp_tz("today + 1 years", Utc).unwrap(), today + Duration::microseconds(USEC_PER_YEAR));
        assert_eq!(parse_timestamp_tz("today + 1 year", Utc).unwrap(), today + Duration::microseconds(USEC_PER_YEAR));
        assert_eq!(parse_timestamp_tz("today + 1 y", Utc).unwrap(), today + Duration::microseconds(USEC_PER_YEAR));
        assert_eq!(parse_timestamp_tz("today + 1 usec", Utc).unwrap(), today + Duration::microseconds(1));
        assert_eq!(parse_timestamp_tz("today + 1 us", Utc).unwrap(), today + Duration::microseconds(1));
        assert_eq!(parse_timestamp_tz("today + 1 µs", Utc).unwrap(), today + Duration::microseconds(1));
    }

    /// Test the special cases of the parsing algorithm.
    #[test]
    fn offset_special_case() {
        let now = parse_timestamp_tz("now", Utc).unwrap();
        assert!(parse_timestamp_tz("+1s", Utc).unwrap() >= now + Duration::seconds(1));
        assert!(parse_timestamp_tz("1s left", Utc).unwrap() >= now + Duration::seconds(1));
        assert!(parse_timestamp_tz("-1s", Utc).unwrap() >= now - Duration::seconds(1));
        assert!(parse_timestamp_tz("1s ago", Utc).unwrap() >= now - Duration::seconds(1));

        assert!(parse_timestamp_tz("+1s 2m", Utc).unwrap() >= now + Duration::seconds(1) + Duration::minutes(2));
        assert!(parse_timestamp_tz("1s 2m left", Utc).unwrap() >= now + Duration::seconds(1) + Duration::minutes(2));
        assert!(parse_timestamp_tz("-1s 2m", Utc).unwrap() >= now - Duration::seconds(1) - Duration::minutes(2));
        assert!(parse_timestamp_tz("1s 2m ago", Utc).unwrap() >= now - Duration::seconds(1) - Duration::minutes(2));

        let epoch = parse_timestamp_tz("epoch", Utc).unwrap();
        assert_eq!(parse_timestamp_tz("@1s", Utc).unwrap(), epoch + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("@1s 2m", Utc).unwrap(), epoch + Duration::seconds(1) + Duration::minutes(2));
    }

    /// Test whitespace in the timestamp.
    #[test]
    fn timestamp_whitespace() {
        let today = parse_timestamp_tz("today", Utc).unwrap();
        assert_eq!(parse_timestamp_tz("today +1s", Utc).unwrap(), today + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("today + 1s", Utc).unwrap(), today + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("today +1 s", Utc).unwrap(), today + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("today + 1 s", Utc).unwrap(), today + Duration::seconds(1));

        assert_eq!(parse_timestamp_tz("today +1s2m", Utc).unwrap(), today + Duration::seconds(1) + Duration::minutes(2));
        assert_eq!(parse_timestamp_tz("today + 1s2m", Utc).unwrap(), today + Duration::seconds(1) + Duration::minutes(2));
        assert_eq!(parse_timestamp_tz("today +1 s2m", Utc).unwrap(), today + Duration::seconds(1) + Duration::minutes(2));
        assert_eq!(parse_timestamp_tz("today + 1 s2m", Utc).unwrap(), today + Duration::seconds(1) + Duration::minutes(2));
        assert_eq!(parse_timestamp_tz("today + 1 s 2m", Utc).unwrap(), today + Duration::seconds(1) + Duration::minutes(2));
        assert_eq!(parse_timestamp_tz("today + 1 s 2 m", Utc).unwrap(), today + Duration::seconds(1) + Duration::minutes(2));

        let now = parse_timestamp_tz("now", Utc).unwrap();
        assert!(parse_timestamp_tz("+ 1s", Utc).unwrap() >= now + Duration::seconds(1));
        assert!(parse_timestamp_tz("+ 1 s", Utc).unwrap() >= now + Duration::seconds(1));
        assert!(parse_timestamp_tz("1 s left", Utc).unwrap() >= now + Duration::seconds(1));
        assert!(parse_timestamp_tz("1  s  left", Utc).unwrap() >= now + Duration::seconds(1));

        let epoch = parse_timestamp_tz("epoch", Utc).unwrap();
        assert_eq!(parse_timestamp_tz("@ 1 s", Utc).unwrap(), epoch + Duration::seconds(1));
        assert_eq!(parse_timestamp_tz("@  1s", Utc).unwrap(), epoch + Duration::seconds(1));
    }

    /// Test edge cases are parsed a certain way.
    #[test]
    fn timestamp_edge_cases() {
        let epoch = parse_timestamp_tz("epoch", Utc).unwrap();
        assert_eq!(parse_timestamp_tz("@", Utc).unwrap(), epoch);

        let today = parse_timestamp_tz("today", Utc).unwrap();
        // ensure like offsets are combined
        assert_eq!(parse_timestamp_tz("today + 1s 2s", Utc).unwrap(), today + Duration::seconds(3));
        assert_eq!(parse_timestamp_tz("today + 1s 4m 2s", Utc).unwrap(), today + Duration::seconds(3) + Duration::minutes(4));

        // ensure whitespace is removed and is right associative
        assert_eq!(parse_timestamp_tz("today + 1 1s", Utc).unwrap(), today + Duration::seconds(11));
        assert_eq!(parse_timestamp_tz("today + 4m 1 1s", Utc).unwrap(), today + Duration::seconds(11) + Duration::minutes(4));
    }

    /*
     * Negative Tests
     */

    #[test]
    fn invalid_format() {
        // space required before modifer
        match parse_timestamp_tz("today+1s", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::Format(_)) => (),
            Err(_) => assert!(false),
        };

        match parse_timestamp_tz("today-1s", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::Format(_)) => (),
            Err(_) => assert!(false),
        };

        match parse_timestamp_tz("1sleft", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::Format(_)) => (),
            Err(_) => assert!(false),
        };

        match parse_timestamp_tz("1sago", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::Format(_)) => (),
            Err(_) => assert!(false),
        };

        // both modifiers
        match parse_timestamp_tz("today + - 1s", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::Format(_)) => (),
            Err(_) => assert!(false),
        };

        match parse_timestamp_tz("today - 1s + 5m", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::Format(_)) => (),
            Err(_) => assert!(false),
        };

        // unsupported strftime format
        match parse_timestamp_tz("2018/08/12 01:02:03.1234", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::Format(_)) => (),
            Err(_) => assert!(false),
        };

        match parse_timestamp_tz("2018/08/12 01:02:03", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::Format(_)) => (),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn invalid_number() {
        // numbers that would overflow fail
        match parse_timestamp_tz("2018-08-09 07:06:05.123456789123456789123456789", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::Number(_)) => (),
            Err(_) => assert!(false),
        };

        match parse_timestamp_tz("+1000000000d 100s", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::Number(_)) => (),
            Err(_) => assert!(false),
        };

        match parse_timestamp_tz("+100s 1000000000d", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::Number(_)) => (),
            Err(_) => assert!(false),
        };

        // number contains whitespace
        match parse_timestamp_tz("2018-08-09 07:06:05.123 4", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::Number(_)) => (),
            Err(_) => assert!(false),
        };

        // number contains characters
        match parse_timestamp_tz("2018-08-09 07:06:05.123a4", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::Number(_)) => (),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn invalid_timeunit() {
        // missing time unit
        match parse_timestamp_tz("+5", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::TimeUnit(_)) => (),
            Err(_) => assert!(false),
        };

        match parse_timestamp_tz("5 ago", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::TimeUnit(_)) => (),
            Err(_) => assert!(false),
        };

        match parse_timestamp_tz("today +5", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::TimeUnit(_)) => (),
            Err(_) => assert!(false),
        };

        match parse_timestamp_tz("today -5s 6", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::TimeUnit(_)) => (),
            Err(_) => assert!(false),
        };

        // unknown time unit
        match parse_timestamp_tz("+5 bad", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::TimeUnit(_)) => (),
            Err(_) => assert!(false),
        };

        match parse_timestamp_tz("5 bad ago", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::TimeUnit(_)) => (),
            Err(_) => assert!(false),
        };

        match parse_timestamp_tz("today +5 bad", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::TimeUnit(_)) => (),
            Err(_) => assert!(false),
        };

        match parse_timestamp_tz("today -5s 6 bad", Utc) {
            Ok(_) => assert!(false),
            Err(InvalidTimestamp::TimeUnit(_)) => (),
            Err(_) => assert!(false),
        };
    }
}
