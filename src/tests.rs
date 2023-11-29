use chrono::offset::{Local, Utc};
use chrono::{Duration, NaiveTime, TimeZone};

use crate::today_time;

use super::parse_timestamp_tz;
use super::InvalidTimestamp;
use super::{USEC_PER_MONTH, USEC_PER_YEAR};

/*
 * Positive Tests
 */

/// Test extracting a time from a keyword.
#[test]
fn time_word() {
    let now = Utc::now();
    let epoch = Utc.timestamp_opt(0, 0).unwrap();
    assert!(parse_timestamp_tz("now", Utc).unwrap() >= now);
    assert_eq!(parse_timestamp_tz("epoch", Utc).unwrap(), epoch);
    assert!(
        parse_timestamp_tz("now", Local)
            .unwrap()
            .with_timezone(&Utc)
            >= now
    );
    assert_eq!(parse_timestamp_tz("epoch", Local).unwrap(), epoch);

    let today_utc = today_time(&Utc, None);
    let tomorrow_utc = today_utc + Duration::days(1);
    let yesterday_utc = today_utc - Duration::days(1);
    assert_eq!(parse_timestamp_tz("today", Utc).unwrap(), today_utc);
    assert_eq!(parse_timestamp_tz("tomorrow", Utc).unwrap(), tomorrow_utc);
    assert_eq!(parse_timestamp_tz("yesterday", Utc).unwrap(), yesterday_utc);

    let today_local = today_time(&Local, None);
    let tomorrow_local = today_local + Duration::days(1);
    let yesterday_local = today_local - Duration::days(1);
    assert_eq!(parse_timestamp_tz("today", Local).unwrap(), today_local);
    assert_eq!(
        parse_timestamp_tz("tomorrow", Local).unwrap(),
        tomorrow_local
    );
    assert_eq!(
        parse_timestamp_tz("yesterday", Local).unwrap(),
        yesterday_local
    );
}

/// Test extracting a time from a strftime formatted timestamp.
#[test]
fn time_strftime() {
    assert_eq!(
        parse_timestamp_tz("2018-08-09 07:06:05", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz("18-08-09 07:06:05", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz("2018-08-09 07:06", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz("18-08-09 07:06", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz("2018-08-09", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz("18-08-09", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz("10:11:12", Utc).unwrap(),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 12))
    );
    assert_eq!(
        parse_timestamp_tz("10:11", Utc).unwrap(),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 0))
    );

    assert_eq!(
        parse_timestamp_tz("2018-08-09 07:06:05.123", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap() + Duration::microseconds(123)
    );
    assert_eq!(
        parse_timestamp_tz("18-08-09 07:06:05.1", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap() + Duration::microseconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("10:11:12.1234", Utc).unwrap(),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 12)) + Duration::microseconds(1234)
    );

    assert_eq!(
        parse_timestamp_tz("2018-08-09 07:06:05", Local).unwrap(),
        Local.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz("18-08-09 07:06:05", Local).unwrap(),
        Local.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz("2018-08-09 07:06", Local).unwrap(),
        Local.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz("18-08-09 07:06", Local).unwrap(),
        Local.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz("2018-08-09", Local).unwrap(),
        Local.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz("18-08-09", Local).unwrap(),
        Local.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz("10:11:12", Local).unwrap(),
        today_time(&Local, NaiveTime::from_hms_opt(10, 11, 12))
    );
    assert_eq!(
        parse_timestamp_tz("10:11", Local).unwrap(),
        today_time(&Local, NaiveTime::from_hms_opt(10, 11, 0))
    );

    assert_eq!(
        parse_timestamp_tz("2018-08-09 07:06:05.123", Local).unwrap(),
        Local.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap() + Duration::microseconds(123)
    );
    assert_eq!(
        parse_timestamp_tz("18-08-09 07:06:05.1", Local).unwrap(),
        Local.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap() + Duration::microseconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("10:11:12.1234", Local).unwrap(),
        today_time(&Local, NaiveTime::from_hms_opt(10, 11, 12)) + Duration::microseconds(1234)
    );
}

/// Test applying an offset to time keywords.
#[test]
fn offset_word() {
    let today = parse_timestamp_tz("today", Utc).unwrap();
    assert_eq!(
        parse_timestamp_tz("today +1s", Utc).unwrap(),
        today + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("today +1s2m", Utc).unwrap(),
        today + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz("today -1s", Utc).unwrap(),
        today - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("today -1s2m", Utc).unwrap(),
        today - Duration::seconds(1) - Duration::minutes(2)
    );

    let tomorrow = parse_timestamp_tz("tomorrow", Utc).unwrap();
    assert_eq!(
        parse_timestamp_tz("tomorrow +1s", Utc).unwrap(),
        tomorrow + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("tomorrow +1s2m", Utc).unwrap(),
        tomorrow + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz("tomorrow -1s", Utc).unwrap(),
        tomorrow - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("tomorrow -1s2m", Utc).unwrap(),
        tomorrow - Duration::seconds(1) - Duration::minutes(2)
    );

    let yesterday = parse_timestamp_tz("yesterday", Utc).unwrap();
    assert_eq!(
        parse_timestamp_tz("yesterday +1s", Utc).unwrap(),
        yesterday + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("yesterday +1s2m", Utc).unwrap(),
        yesterday + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz("yesterday -1s", Utc).unwrap(),
        yesterday - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("yesterday -1s2m", Utc).unwrap(),
        yesterday - Duration::seconds(1) - Duration::minutes(2)
    );

    let epoch = parse_timestamp_tz("epoch", Utc).unwrap();
    assert_eq!(
        parse_timestamp_tz("epoch +1s", Utc).unwrap(),
        epoch + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("epoch +1s2m", Utc).unwrap(),
        epoch + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz("epoch -1s", Utc).unwrap(),
        epoch - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("epoch -1s2m", Utc).unwrap(),
        epoch - Duration::seconds(1) - Duration::minutes(2)
    );

    let now = parse_timestamp_tz("now", Utc).unwrap();
    assert!(parse_timestamp_tz("now +1s", Utc).unwrap() >= now + Duration::seconds(1));
    assert!(
        parse_timestamp_tz("now +1s2m", Utc).unwrap()
            >= now + Duration::seconds(1) + Duration::minutes(2)
    );
    assert!(parse_timestamp_tz("now -1s", Utc).unwrap() >= now - Duration::seconds(1));
    assert!(
        parse_timestamp_tz("now -1s2m", Utc).unwrap()
            >= now - Duration::seconds(1) - Duration::minutes(2)
    );
}

/// Test applying an offset to strftime formatted timestamps.
#[test]
fn offset_strftime() {
    assert_eq!(
        parse_timestamp_tz("2018-08-09 07:06:05 +1s", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap() + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("2018-08-09 07:06:05 +1s2m", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap()
            + Duration::seconds(1)
            + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz("2018-08-09 07:06:05 -1s", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap() - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("2018-08-09 07:06:05 -1s2m", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap()
            - Duration::seconds(1)
            - Duration::minutes(2)
    );

    assert_eq!(
        parse_timestamp_tz("18-08-09 07:06:05 +1s", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap() + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("18-08-09 07:06:05 +1s2m", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap()
            + Duration::seconds(1)
            + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz("18-08-09 07:06:05 -1s", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap() - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("18-08-09 07:06:05 -1s2m", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap()
            - Duration::seconds(1)
            - Duration::minutes(2)
    );

    assert_eq!(
        parse_timestamp_tz("2018-08-09 07:06 +1s", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap() + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("2018-08-09 07:06 +1s2m", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap()
            + Duration::seconds(1)
            + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz("2018-08-09 07:06 -1s", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap() - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("2018-08-09 07:06 -1s2m", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap()
            - Duration::seconds(1)
            - Duration::minutes(2)
    );

    assert_eq!(
        parse_timestamp_tz("18-08-09 07:06 +1s", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap() + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("18-08-09 07:06 +1s2m", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap()
            + Duration::seconds(1)
            + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz("18-08-09 07:06 -1s", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap() - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("18-08-09 07:06 -1s2m", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap()
            - Duration::seconds(1)
            - Duration::minutes(2)
    );

    assert_eq!(
        parse_timestamp_tz("2018-08-09 +1s", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap() + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("2018-08-09 +1s2m", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap()
            + Duration::seconds(1)
            + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz("2018-08-09 -1s", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap() - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("2018-08-09 -1s2m", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap()
            - Duration::seconds(1)
            - Duration::minutes(2)
    );

    assert_eq!(
        parse_timestamp_tz("18-08-09 +1s", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap() + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("18-08-09 +1s2m", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap()
            + Duration::seconds(1)
            + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz("18-08-09 -1s", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap() - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("18-08-09 -1s2m", Utc).unwrap(),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap()
            - Duration::seconds(1)
            - Duration::minutes(2)
    );

    assert_eq!(
        parse_timestamp_tz("10:11:12 +1s", Utc).unwrap(),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 12)) + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("10:11:12 +1s2m", Utc).unwrap(),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 12))
            + Duration::seconds(1)
            + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz("10:11:12 -1s", Utc).unwrap(),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 12)) - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("10:11:12 -1s2m", Utc).unwrap(),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 12))
            - Duration::seconds(1)
            - Duration::minutes(2)
    );

    assert_eq!(
        parse_timestamp_tz("10:11 +1s", Utc).unwrap(),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 0)) + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("10:11 +1s2m", Utc).unwrap(),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 0))
            + Duration::seconds(1)
            + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz("10:11 -1s", Utc).unwrap(),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 0)) - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("10:11 -1s2m", Utc).unwrap(),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 0))
            - Duration::seconds(1)
            - Duration::minutes(2)
    );
}

/// Test the various offset time unit keywords.
#[test]
fn offset_time_unit() {
    let today = parse_timestamp_tz("today", Utc).unwrap();
    assert_eq!(
        parse_timestamp_tz("today + 1 seconds", Utc).unwrap(),
        today + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 second", Utc).unwrap(),
        today + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 sec", Utc).unwrap(),
        today + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 s", Utc).unwrap(),
        today + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 minutes", Utc).unwrap(),
        today + Duration::minutes(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 minute", Utc).unwrap(),
        today + Duration::minutes(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 min", Utc).unwrap(),
        today + Duration::minutes(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 months", Utc).unwrap(),
        today + Duration::microseconds(USEC_PER_MONTH)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 month", Utc).unwrap(),
        today + Duration::microseconds(USEC_PER_MONTH)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 M", Utc).unwrap(),
        today + Duration::microseconds(USEC_PER_MONTH)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 msec", Utc).unwrap(),
        today + Duration::milliseconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 ms", Utc).unwrap(),
        today + Duration::milliseconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 m", Utc).unwrap(),
        today + Duration::minutes(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 hours", Utc).unwrap(),
        today + Duration::hours(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 hour", Utc).unwrap(),
        today + Duration::hours(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 hr", Utc).unwrap(),
        today + Duration::hours(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 h", Utc).unwrap(),
        today + Duration::hours(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 days", Utc).unwrap(),
        today + Duration::days(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 day", Utc).unwrap(),
        today + Duration::days(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 d", Utc).unwrap(),
        today + Duration::days(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 weeks", Utc).unwrap(),
        today + Duration::days(7)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 week", Utc).unwrap(),
        today + Duration::days(7)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 w", Utc).unwrap(),
        today + Duration::days(7)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 years", Utc).unwrap(),
        today + Duration::microseconds(USEC_PER_YEAR)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 year", Utc).unwrap(),
        today + Duration::microseconds(USEC_PER_YEAR)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 y", Utc).unwrap(),
        today + Duration::microseconds(USEC_PER_YEAR)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 usec", Utc).unwrap(),
        today + Duration::microseconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 us", Utc).unwrap(),
        today + Duration::microseconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 µs", Utc).unwrap(),
        today + Duration::microseconds(1)
    );
}

/// Test the special cases of the parsing algorithm.
#[test]
fn offset_special_case() {
    let now = parse_timestamp_tz("now", Utc).unwrap();
    assert!(parse_timestamp_tz("+1s", Utc).unwrap() >= now + Duration::seconds(1));
    assert!(parse_timestamp_tz("1s left", Utc).unwrap() >= now + Duration::seconds(1));
    assert!(parse_timestamp_tz("-1s", Utc).unwrap() >= now - Duration::seconds(1));
    assert!(parse_timestamp_tz("1s ago", Utc).unwrap() >= now - Duration::seconds(1));

    assert!(
        parse_timestamp_tz("+1s 2m", Utc).unwrap()
            >= now + Duration::seconds(1) + Duration::minutes(2)
    );
    assert!(
        parse_timestamp_tz("1s 2m left", Utc).unwrap()
            >= now + Duration::seconds(1) + Duration::minutes(2)
    );
    assert!(
        parse_timestamp_tz("-1s 2m", Utc).unwrap()
            >= now - Duration::seconds(1) - Duration::minutes(2)
    );
    assert!(
        parse_timestamp_tz("1s 2m ago", Utc).unwrap()
            >= now - Duration::seconds(1) - Duration::minutes(2)
    );

    let epoch = parse_timestamp_tz("epoch", Utc).unwrap();
    assert_eq!(
        parse_timestamp_tz("@1s", Utc).unwrap(),
        epoch + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("@1s 2m", Utc).unwrap(),
        epoch + Duration::seconds(1) + Duration::minutes(2)
    );
}

/// Test whitespace in the timestamp.
#[test]
fn timestamp_whitespace() {
    let today = parse_timestamp_tz("today", Utc).unwrap();
    assert_eq!(
        parse_timestamp_tz("today +1s", Utc).unwrap(),
        today + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1s", Utc).unwrap(),
        today + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("today +1 s", Utc).unwrap(),
        today + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 s", Utc).unwrap(),
        today + Duration::seconds(1)
    );

    assert_eq!(
        parse_timestamp_tz("today +1s2m", Utc).unwrap(),
        today + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1s2m", Utc).unwrap(),
        today + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz("today +1 s2m", Utc).unwrap(),
        today + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 s2m", Utc).unwrap(),
        today + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 s 2m", Utc).unwrap(),
        today + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1 s 2 m", Utc).unwrap(),
        today + Duration::seconds(1) + Duration::minutes(2)
    );

    let now = parse_timestamp_tz("now", Utc).unwrap();
    assert!(parse_timestamp_tz("+ 1s", Utc).unwrap() >= now + Duration::seconds(1));
    assert!(parse_timestamp_tz("+ 1 s", Utc).unwrap() >= now + Duration::seconds(1));
    assert!(parse_timestamp_tz("1 s left", Utc).unwrap() >= now + Duration::seconds(1));
    assert!(parse_timestamp_tz("1  s  left", Utc).unwrap() >= now + Duration::seconds(1));

    let epoch = parse_timestamp_tz("epoch", Utc).unwrap();
    assert_eq!(
        parse_timestamp_tz("@ 1 s", Utc).unwrap(),
        epoch + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz("@  1s", Utc).unwrap(),
        epoch + Duration::seconds(1)
    );
}

/// Test edge cases are parsed a certain way.
#[test]
fn timestamp_edge_cases() {
    let epoch = parse_timestamp_tz("epoch", Utc).unwrap();
    assert_eq!(parse_timestamp_tz("@", Utc).unwrap(), epoch);

    let today = parse_timestamp_tz("today", Utc).unwrap();
    // ensure like offsets are combined
    assert_eq!(
        parse_timestamp_tz("today + 1s 2s", Utc).unwrap(),
        today + Duration::seconds(3)
    );
    assert_eq!(
        parse_timestamp_tz("today + 1s 4m 2s", Utc).unwrap(),
        today + Duration::seconds(3) + Duration::minutes(4)
    );

    // ensure whitespace is removed and is right associative
    assert_eq!(
        parse_timestamp_tz("today + 1 1s", Utc).unwrap(),
        today + Duration::seconds(11)
    );
    assert_eq!(
        parse_timestamp_tz("today + 4m 1 1s", Utc).unwrap(),
        today + Duration::seconds(11) + Duration::minutes(4)
    );
}

/*
 * Negative Tests
 */

#[test]
fn invalid_format() {
    // space required before modifer
    assert!(matches!(
        parse_timestamp_tz("today+1s", Utc),
        Err(InvalidTimestamp::Format(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("today-1s", Utc),
        Err(InvalidTimestamp::Format(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("1sleft", Utc),
        Err(InvalidTimestamp::Format(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("1sago", Utc),
        Err(InvalidTimestamp::Format(_))
    ));

    // both modifiers
    assert!(matches!(
        parse_timestamp_tz("today + - 1s", Utc),
        Err(InvalidTimestamp::Format(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("today - 1s + 5m", Utc),
        Err(InvalidTimestamp::Format(_))
    ));

    // unsupported strftime format
    assert!(matches!(
        parse_timestamp_tz("2018/08/12 01:02:03.1234", Utc),
        Err(InvalidTimestamp::Format(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("2018/08/12 01:02:03", Utc),
        Err(InvalidTimestamp::Format(_))
    ));
}

#[test]
fn invalid_number() {
    // numbers that would overflow fail
    assert!(matches!(
        parse_timestamp_tz("2018-08-09 07:06:05.123456789123456789123456789", Utc),
        Err(InvalidTimestamp::Number(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("+1000000000d 100s", Utc),
        Err(InvalidTimestamp::Number(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("+100s 1000000000d", Utc),
        Err(InvalidTimestamp::Number(_))
    ));

    // number contains whitespace
    assert!(matches!(
        parse_timestamp_tz("2018-08-09 07:06:05.123 4", Utc),
        Err(InvalidTimestamp::Number(_))
    ));

    // number contains characters
    assert!(matches!(
        parse_timestamp_tz("2018-08-09 07:06:05.123a4", Utc),
        Err(InvalidTimestamp::Number(_))
    ));
}

#[test]
fn invalid_timeunit() {
    // missing time unit
    assert!(matches!(
        parse_timestamp_tz("+5", Utc),
        Err(InvalidTimestamp::TimeUnit(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("5 ago", Utc),
        Err(InvalidTimestamp::TimeUnit(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("today +5", Utc),
        Err(InvalidTimestamp::TimeUnit(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("today -5s 6", Utc),
        Err(InvalidTimestamp::TimeUnit(_))
    ));

    // unknown time unit
    assert!(matches!(
        parse_timestamp_tz("+5 bad", Utc),
        Err(InvalidTimestamp::TimeUnit(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("5 bad ago", Utc),
        Err(InvalidTimestamp::TimeUnit(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("today +5 bad", Utc),
        Err(InvalidTimestamp::TimeUnit(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("today -5s 6 bad", Utc),
        Err(InvalidTimestamp::TimeUnit(_))
    ));
}
