//! The library parses timestamps following the [systemd.time] specifications into [chrono] types.
//!
//! [systemd.time]: https://www.freedesktop.org/software/systemd/man/systemd.time.html
//! [chrono]: https://docs.rs/chrono/
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

#[cfg(test)]
mod tests;

mod error;
mod local_datetime;

pub use self::{error::Error, local_datetime::LocalDateTime};

use std::borrow::Borrow;
use std::collections::HashMap;
use std::str;

use chrono::offset::Utc;
use chrono::{Days, Duration};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use once_cell::sync::Lazy;

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

#[rustfmt::skip]
static USEC_MULTIPLIER: Lazy<HashMap<&'static str, i64>> = Lazy::new(|| {
    HashMap::from_iter([
        ("us", USEC_PER_USEC),
        ("usec", USEC_PER_USEC),
        ("µs", USEC_PER_USEC),

        ("ms", USEC_PER_MSEC),
        ("msec", USEC_PER_MSEC),

        ("s", USEC_PER_SEC),
        ("sec", USEC_PER_SEC),
        ("second", USEC_PER_SEC),
        ("seconds", USEC_PER_SEC),

        ("m", USEC_PER_MINUTE),
        ("min", USEC_PER_MINUTE),
        ("minute", USEC_PER_MINUTE),
        ("minutes", USEC_PER_MINUTE),

        ("h", USEC_PER_HOUR),
        ("hour", USEC_PER_HOUR),
        ("hours", USEC_PER_HOUR),
        ("hr", USEC_PER_HOUR),

        ("d", USEC_PER_DAY),
        ("day", USEC_PER_DAY),
        ("days", USEC_PER_DAY),

        ("M", USEC_PER_MONTH),
        ("month", USEC_PER_MONTH),
        ("months", USEC_PER_MONTH),

        ("w", USEC_PER_WEEK),
        ("week", USEC_PER_WEEK),
        ("weeks", USEC_PER_WEEK),

        ("y", USEC_PER_YEAR),
        ("year", USEC_PER_YEAR),
        ("years", USEC_PER_YEAR),
    ])
});

/// Parse a timestamp returning a `DateTime` with the specified timezone.
///
/// # Examples
/// ```rust
/// # use chrono_systemd_time::parse_timestamp_tz;
/// use chrono::{DateTime, Duration, Local, TimeZone, Utc};
///
/// fn parse_timestamp_tz_aux<Tz: TimeZone>(timestamp: &str, timezone: Tz) -> DateTime<Tz> {
///     parse_timestamp_tz(timestamp, timezone)
///         .unwrap()
///         .single()
///         .unwrap()
/// }
///
/// assert_eq!(parse_timestamp_tz_aux("today + 2h", Utc),
///             parse_timestamp_tz_aux("today", Utc) + Duration::hours(2));
/// assert_eq!(parse_timestamp_tz_aux("yesterday", Local),
///             parse_timestamp_tz_aux("today - 1d", Local));
/// assert_eq!(parse_timestamp_tz_aux("2018-06-21", Utc),
///             parse_timestamp_tz_aux("18-06-21 1:00 - 1h", Utc));
/// ```
pub fn parse_timestamp_tz<S, T, Tz>(timestamp: S, timezone: T) -> Result<LocalDateTime<Tz>, Error>
where
    S: AsRef<str>,
    T: Borrow<Tz>,
    Tz: TimeZone,
{
    let tz = timezone.borrow();
    let ts = timestamp.as_ref();
    let ts_nw = ts
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<String>();

    if ts_nw.is_empty() {
        return Err(Error::Format("Timestamp cannot be empty".to_owned()));
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
        return Ok(LocalDateTime::Single(now + offset));
    }
    if ts.ends_with(" left") {
        let now = Utc::now().with_timezone(tz);
        let offset = parse_offset(&ts_nw[..(ts_nw.len() - 4)])?;
        return Ok(LocalDateTime::Single(now + offset));
    }

    if ts.starts_with('-') {
        let now = Utc::now().with_timezone(tz);
        let offset = parse_offset(&ts_nw[1..])?;
        return Ok(LocalDateTime::Single(now - offset));
    }
    if ts.ends_with(" ago") {
        let now = Utc::now().with_timezone(tz);
        let offset = parse_offset(&ts_nw[..(ts_nw.len() - 3)])?;
        return Ok(LocalDateTime::Single(now - offset));
    }

    // Special Case 2 - a prefix of '@':
    //  - the time is the unix epoch.
    //  - the offset consists of the remaining characters added to the epoch time.
    if ts.starts_with('@') {
        let epoch = tz.timestamp_opt(0, 0).unwrap();
        let offset = parse_offset(&ts_nw[1..])?;
        return Ok(LocalDateTime::Single(epoch + offset));
    }

    // General Case - the time is separated from the offset by either a '+' or '-'.
    // Note: need to find " +" and " -" here because strftime date formats may contain the '-' character,
    //       but with no leading whitespaces.
    match (ts.find(" +"), ts.find(" -")) {
        (Some(_), Some(_)) => Err(Error::Format(
            "Timestamp cannot contain both a `+` and `-`".to_owned(),
        )),
        (Some(p), None) => {
            let p_nw = ts_nw.find('+').unwrap();
            let time = parse_time(&ts[..p], tz)?;
            let offset = parse_offset(&ts_nw[(p_nw + 1)..])?;
            Ok(time + offset)
        }
        (None, Some(m)) => {
            let m_nw = ts_nw.rfind('-').unwrap();
            let time = parse_time(&ts[..m], tz)?;
            let offset = parse_offset(&ts_nw[(m_nw + 1)..])?;
            Ok(time - offset)
        }
        (None, None) => {
            let time = parse_time(ts, tz)?;
            Ok(time)
        }
    }
}

/// Parse a point-in-time into a `DateTime` with the given timezone.
///
/// * `ts` - a str of a time with whitespace intact.
/// * `tz` - the time zone to use.
fn parse_time<Tz: TimeZone>(ts: &str, tz: &Tz) -> Result<LocalDateTime<Tz>, Error> {
    let dt = match ts {
        "now" => LocalDateTime::Single(Utc::now().with_timezone(tz)),
        "epoch" => LocalDateTime::Single(tz.timestamp_opt(0, 0).unwrap()),
        "today" => LocalDateTime::from_date(naive_today(tz), tz)?,
        "yesterday" => LocalDateTime::from_date(naive_today(tz) - Days::new(1), tz)?,
        "tomorrow" => LocalDateTime::from_date(naive_today(tz) + Days::new(1), tz)?,
        ts => match ts.find('.') {
            // an optional '.' separates the seconds and microseconds components
            Some(p) => {
                let ts_t = &ts[..p];
                let ndt = NaiveDateTime::parse_from_str(ts_t, "%y-%m-%d %H:%M:%S")
                    .or_else(|_| NaiveDateTime::parse_from_str(ts_t, "%Y-%m-%d %H:%M:%S"))
                    .or_else(|_| {
                        NaiveTime::parse_from_str(ts_t, "%H:%M:%S")
                            .map(|nt| naive_today(tz).and_time(nt))
                    })
                    .map_err(|_| {
                        Error::Format(format!("Cannot parse `{ts_t}` before '.' into a time"))
                    })?;

                let ts_u = &ts[(p + 1)..];
                let usecs: i64 = ts_u.parse().map_err(|e| {
                    Error::Number(format!(
                        "Cannot parse `{ts_u}` after '.' into a number: {e}"
                    ))
                })?;

                let ndt = ndt + Duration::microseconds(usecs);
                LocalDateTime::from_datetime(ndt, tz)?
            }
            None => NaiveDateTime::parse_from_str(ts, "%y-%m-%d %H:%M:%S")
                .or_else(|_| NaiveDateTime::parse_from_str(ts, "%Y-%m-%d %H:%M:%S"))
                .or_else(|_| NaiveDateTime::parse_from_str(ts, "%y-%m-%d %H:%M"))
                .or_else(|_| NaiveDateTime::parse_from_str(ts, "%Y-%m-%d %H:%M"))
                .or_else(|_| {
                    NaiveDate::parse_from_str(ts, "%y-%m-%d")
                        .map(|nd| nd.and_hms_opt(0, 0, 0).unwrap())
                })
                .or_else(|_| {
                    NaiveDate::parse_from_str(ts, "%Y-%m-%d")
                        .map(|nd| nd.and_hms_opt(0, 0, 0).unwrap())
                })
                .or_else(|_| {
                    NaiveTime::parse_from_str(ts, "%H:%M:%S").map(|nt| naive_today(tz).and_time(nt))
                })
                .or_else(|_| {
                    NaiveTime::parse_from_str(ts, "%H:%M").map(|nt| naive_today(tz).and_time(nt))
                })
                .map_err(|_| Error::Format(format!("Cannot parse `{ts}` into a time")))
                .and_then(|ndt| LocalDateTime::from_datetime(ndt, tz))?,
        },
    };
    Ok(dt)
}

/// Parse and combine all time spans into a single duration.
///
/// * `ts_nw` - a str of time spans with whitespace removed.
fn parse_offset(mut ts_nw: &str) -> Result<Duration, Error> {
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
        let (digits, ts_tail) = partition_predicate(ts_nw, |c| c.is_ascii_digit());
        let (letters, ts_tail) = partition_predicate(ts_tail, char::is_alphabetic);
        ts_nw = ts_tail;

        // parse the `number` and `multipler` strings into i64
        let number: i64 = digits
            .parse()
            .map_err(|e| Error::Number(format!("Cannot parse `{digits}` into a number: {e}")))?;
        let Some(&multiplier) = USEC_MULTIPLIER.get(letters) else {
            return Err(Error::TimeUnit(letters.to_owned()));
        };

        let Some(usecs) = number
            .checked_mul(multiplier)
            .and_then(|usec| usec.checked_add(total_usecs))
        else {
            return Err(Error::Number(format!(
                "Offset microseconds overflowed: total_usecs `{total_usecs}` number `{number}` multiplier `{multiplier}`"
            )));
        };
        // increment the total microsecond offset returning a failure on an overflow
        total_usecs = usecs;
    }
}

fn naive_today<Tz: TimeZone>(tz: &Tz) -> NaiveDate {
    Utc::now().with_timezone(tz).date_naive()
}

/// Partition a str by a given predicate.
/// Returned is a tuple where:
/// - the first element contains the sub-slice of sequential characters that tested true.
/// - the second element contains the remaining characters of the original str.
fn partition_predicate<P>(ts: &str, predicate: P) -> (&str, &str)
where
    P: Fn(char) -> bool,
{
    ts.find(|c: char| !predicate(c))
        .map(|p| ts.split_at(p))
        .unwrap_or((ts, ""))
}
