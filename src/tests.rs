use chrono::offset::{Local, Utc};
use chrono::{DateTime, Duration, NaiveTime, TimeZone};

use super::naive_today;
use super::parse_timestamp_tz;
use super::Error;
use super::{USEC_PER_MONTH, USEC_PER_YEAR};

/*
 * Positive Tests
 */

/// Test extracting a time from a keyword.
#[test]
fn time_word() {
    let now = Utc::now();
    let epoch = Utc.timestamp_opt(0, 0).unwrap();
    assert!(parse_timestamp_tz_aux("now", Utc) >= now);
    assert_eq!(parse_timestamp_tz_aux("epoch", Utc), epoch);
    assert!(parse_timestamp_tz_aux("now", Local).with_timezone(&Utc) >= now);
    assert_eq!(parse_timestamp_tz_aux("epoch", Local), epoch);

    let today_utc = today_time(&Utc, None);
    let tomorrow_utc = today_utc + Duration::days(1);
    let yesterday_utc = today_utc - Duration::days(1);
    assert_eq!(parse_timestamp_tz_aux("today", Utc), today_utc);
    assert_eq!(parse_timestamp_tz_aux("tomorrow", Utc), tomorrow_utc);
    assert_eq!(parse_timestamp_tz_aux("yesterday", Utc), yesterday_utc);

    let today_local = today_time(&Local, None);
    let tomorrow_local = today_local + Duration::days(1);
    let yesterday_local = today_local - Duration::days(1);
    assert_eq!(parse_timestamp_tz_aux("today", Local), today_local);
    assert_eq!(parse_timestamp_tz_aux("tomorrow", Local), tomorrow_local);
    assert_eq!(parse_timestamp_tz_aux("yesterday", Local), yesterday_local);
}

/// Test extracting a time from a strftime formatted timestamp.
#[test]
fn time_strftime() {
    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 07:06:05", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 07:06:05", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 07:06", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 07:06", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz_aux("18-08-09", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz_aux("10:11:12", Utc),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 12))
    );
    assert_eq!(
        parse_timestamp_tz_aux("10:11", Utc),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 0))
    );

    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 07:06:05.123", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap() + Duration::microseconds(123)
    );
    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 07:06:05.1", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap() + Duration::microseconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("10:11:12.1234", Utc),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 12)) + Duration::microseconds(1234)
    );

    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 07:06:05", Local),
        Local.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 07:06:05", Local),
        Local.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 07:06", Local),
        Local.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 07:06", Local),
        Local.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09", Local),
        Local.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz_aux("18-08-09", Local),
        Local.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap()
    );
    assert_eq!(
        parse_timestamp_tz_aux("10:11:12", Local),
        today_time(&Local, NaiveTime::from_hms_opt(10, 11, 12))
    );
    assert_eq!(
        parse_timestamp_tz_aux("10:11", Local),
        today_time(&Local, NaiveTime::from_hms_opt(10, 11, 0))
    );

    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 07:06:05.123", Local),
        Local.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap() + Duration::microseconds(123)
    );
    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 07:06:05.1", Local),
        Local.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap() + Duration::microseconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("10:11:12.1234", Local),
        today_time(&Local, NaiveTime::from_hms_opt(10, 11, 12)) + Duration::microseconds(1234)
    );
}

/// Test applying an offset to time keywords.
#[test]
fn offset_word() {
    let today = parse_timestamp_tz_aux("today", Utc);
    assert_eq!(
        parse_timestamp_tz_aux("today +1s", Utc),
        today + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today +1s2m", Utc),
        today + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today -1s", Utc),
        today - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today -1s2m", Utc),
        today - Duration::seconds(1) - Duration::minutes(2)
    );

    let tomorrow = parse_timestamp_tz_aux("tomorrow", Utc);
    assert_eq!(
        parse_timestamp_tz_aux("tomorrow +1s", Utc),
        tomorrow + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("tomorrow +1s2m", Utc),
        tomorrow + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz_aux("tomorrow -1s", Utc),
        tomorrow - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("tomorrow -1s2m", Utc),
        tomorrow - Duration::seconds(1) - Duration::minutes(2)
    );

    let yesterday = parse_timestamp_tz_aux("yesterday", Utc);
    assert_eq!(
        parse_timestamp_tz_aux("yesterday +1s", Utc),
        yesterday + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("yesterday +1s2m", Utc),
        yesterday + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz_aux("yesterday -1s", Utc),
        yesterday - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("yesterday -1s2m", Utc),
        yesterday - Duration::seconds(1) - Duration::minutes(2)
    );

    let epoch = parse_timestamp_tz_aux("epoch", Utc);
    assert_eq!(
        parse_timestamp_tz_aux("epoch +1s", Utc),
        epoch + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("epoch +1s2m", Utc),
        epoch + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz_aux("epoch -1s", Utc),
        epoch - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("epoch -1s2m", Utc),
        epoch - Duration::seconds(1) - Duration::minutes(2)
    );

    let now = parse_timestamp_tz_aux("now", Utc);
    assert!(parse_timestamp_tz_aux("now +1s", Utc) >= now + Duration::seconds(1));
    assert!(
        parse_timestamp_tz_aux("now +1s2m", Utc)
            >= now + Duration::seconds(1) + Duration::minutes(2)
    );
    assert!(parse_timestamp_tz_aux("now -1s", Utc) >= now - Duration::seconds(1));
    assert!(
        parse_timestamp_tz_aux("now -1s2m", Utc)
            >= now - Duration::seconds(1) - Duration::minutes(2)
    );
}

/// Test applying an offset to strftime formatted timestamps.
#[test]
fn offset_strftime() {
    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 07:06:05 +1s", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap() + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 07:06:05 +1s2m", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap()
            + Duration::seconds(1)
            + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 07:06:05 -1s", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap() - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 07:06:05 -1s2m", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap()
            - Duration::seconds(1)
            - Duration::minutes(2)
    );

    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 07:06:05 +1s", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap() + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 07:06:05 +1s2m", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap()
            + Duration::seconds(1)
            + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 07:06:05 -1s", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap() - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 07:06:05 -1s2m", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 5).unwrap()
            - Duration::seconds(1)
            - Duration::minutes(2)
    );

    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 07:06 +1s", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap() + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 07:06 +1s2m", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap()
            + Duration::seconds(1)
            + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 07:06 -1s", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap() - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 07:06 -1s2m", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap()
            - Duration::seconds(1)
            - Duration::minutes(2)
    );

    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 07:06 +1s", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap() + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 07:06 +1s2m", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap()
            + Duration::seconds(1)
            + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 07:06 -1s", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap() - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 07:06 -1s2m", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 7, 6, 0).unwrap()
            - Duration::seconds(1)
            - Duration::minutes(2)
    );

    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 +1s", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap() + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 +1s2m", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap()
            + Duration::seconds(1)
            + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 -1s", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap() - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("2018-08-09 -1s2m", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap()
            - Duration::seconds(1)
            - Duration::minutes(2)
    );

    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 +1s", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap() + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 +1s2m", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap()
            + Duration::seconds(1)
            + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 -1s", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap() - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("18-08-09 -1s2m", Utc),
        Utc.with_ymd_and_hms(2018, 8, 9, 0, 0, 0).unwrap()
            - Duration::seconds(1)
            - Duration::minutes(2)
    );

    assert_eq!(
        parse_timestamp_tz_aux("10:11:12 +1s", Utc),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 12)) + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("10:11:12 +1s2m", Utc),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 12))
            + Duration::seconds(1)
            + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz_aux("10:11:12 -1s", Utc),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 12)) - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("10:11:12 -1s2m", Utc),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 12))
            - Duration::seconds(1)
            - Duration::minutes(2)
    );

    assert_eq!(
        parse_timestamp_tz_aux("10:11 +1s", Utc),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 0)) + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("10:11 +1s2m", Utc),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 0))
            + Duration::seconds(1)
            + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz_aux("10:11 -1s", Utc),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 0)) - Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("10:11 -1s2m", Utc),
        today_time(&Utc, NaiveTime::from_hms_opt(10, 11, 0))
            - Duration::seconds(1)
            - Duration::minutes(2)
    );
}

/// Test the various offset time unit keywords.
#[test]
fn offset_time_unit() {
    let today = parse_timestamp_tz_aux("today", Utc);
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 seconds", Utc),
        today + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 second", Utc),
        today + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 sec", Utc),
        today + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 s", Utc),
        today + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 minutes", Utc),
        today + Duration::minutes(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 minute", Utc),
        today + Duration::minutes(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 min", Utc),
        today + Duration::minutes(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 months", Utc),
        today + Duration::microseconds(USEC_PER_MONTH)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 month", Utc),
        today + Duration::microseconds(USEC_PER_MONTH)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 M", Utc),
        today + Duration::microseconds(USEC_PER_MONTH)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 msec", Utc),
        today + Duration::milliseconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 ms", Utc),
        today + Duration::milliseconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 m", Utc),
        today + Duration::minutes(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 hours", Utc),
        today + Duration::hours(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 hour", Utc),
        today + Duration::hours(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 hr", Utc),
        today + Duration::hours(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 h", Utc),
        today + Duration::hours(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 days", Utc),
        today + Duration::days(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 day", Utc),
        today + Duration::days(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 d", Utc),
        today + Duration::days(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 weeks", Utc),
        today + Duration::days(7)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 week", Utc),
        today + Duration::days(7)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 w", Utc),
        today + Duration::days(7)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 years", Utc),
        today + Duration::microseconds(USEC_PER_YEAR)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 year", Utc),
        today + Duration::microseconds(USEC_PER_YEAR)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 y", Utc),
        today + Duration::microseconds(USEC_PER_YEAR)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 usec", Utc),
        today + Duration::microseconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 us", Utc),
        today + Duration::microseconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 µs", Utc),
        today + Duration::microseconds(1)
    );
}

/// Test the special cases of the parsing algorithm.
#[test]
fn offset_special_case() {
    let now = parse_timestamp_tz_aux("now", Utc);
    assert!(parse_timestamp_tz_aux("+1s", Utc) >= now + Duration::seconds(1));
    assert!(parse_timestamp_tz_aux("1s left", Utc) >= now + Duration::seconds(1));
    assert!(parse_timestamp_tz_aux("-1s", Utc) >= now - Duration::seconds(1));
    assert!(parse_timestamp_tz_aux("1s ago", Utc) >= now - Duration::seconds(1));

    assert!(
        parse_timestamp_tz_aux("+1s 2m", Utc) >= now + Duration::seconds(1) + Duration::minutes(2)
    );
    assert!(
        parse_timestamp_tz_aux("1s 2m left", Utc)
            >= now + Duration::seconds(1) + Duration::minutes(2)
    );
    assert!(
        parse_timestamp_tz_aux("-1s 2m", Utc) >= now - Duration::seconds(1) - Duration::minutes(2)
    );
    assert!(
        parse_timestamp_tz_aux("1s 2m ago", Utc)
            >= now - Duration::seconds(1) - Duration::minutes(2)
    );

    let epoch = parse_timestamp_tz_aux("epoch", Utc);
    assert_eq!(
        parse_timestamp_tz_aux("@1s", Utc),
        epoch + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("@1s 2m", Utc),
        epoch + Duration::seconds(1) + Duration::minutes(2)
    );
}

/// Test whitespace in the timestamp.
#[test]
fn timestamp_whitespace() {
    let today = parse_timestamp_tz_aux("today", Utc);
    assert_eq!(
        parse_timestamp_tz_aux("today +1s", Utc),
        today + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1s", Utc),
        today + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today +1 s", Utc),
        today + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 s", Utc),
        today + Duration::seconds(1)
    );

    assert_eq!(
        parse_timestamp_tz_aux("today +1s2m", Utc),
        today + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1s2m", Utc),
        today + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today +1 s2m", Utc),
        today + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 s2m", Utc),
        today + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 s 2m", Utc),
        today + Duration::seconds(1) + Duration::minutes(2)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 s 2 m", Utc),
        today + Duration::seconds(1) + Duration::minutes(2)
    );

    let now = parse_timestamp_tz_aux("now", Utc);
    assert!(parse_timestamp_tz_aux("+ 1s", Utc) >= now + Duration::seconds(1));
    assert!(parse_timestamp_tz_aux("+ 1 s", Utc) >= now + Duration::seconds(1));
    assert!(parse_timestamp_tz_aux("1 s left", Utc) >= now + Duration::seconds(1));
    assert!(parse_timestamp_tz_aux("1  s  left", Utc) >= now + Duration::seconds(1));

    let epoch = parse_timestamp_tz_aux("epoch", Utc);
    assert_eq!(
        parse_timestamp_tz_aux("@ 1 s", Utc),
        epoch + Duration::seconds(1)
    );
    assert_eq!(
        parse_timestamp_tz_aux("@  1s", Utc),
        epoch + Duration::seconds(1)
    );
}

/// Test edge cases are parsed a certain way.
#[test]
fn timestamp_edge_cases() {
    let epoch = parse_timestamp_tz_aux("epoch", Utc);
    assert_eq!(parse_timestamp_tz_aux("@", Utc), epoch);

    let today = parse_timestamp_tz_aux("today", Utc);
    // ensure like offsets are combined
    assert_eq!(
        parse_timestamp_tz_aux("today + 1s 2s", Utc),
        today + Duration::seconds(3)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 1s 4m 2s", Utc),
        today + Duration::seconds(3) + Duration::minutes(4)
    );

    // ensure whitespace is removed and is right associative
    assert_eq!(
        parse_timestamp_tz_aux("today + 1 1s", Utc),
        today + Duration::seconds(11)
    );
    assert_eq!(
        parse_timestamp_tz_aux("today + 4m 1 1s", Utc),
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
        Err(Error::Format(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("today-1s", Utc),
        Err(Error::Format(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("1sleft", Utc),
        Err(Error::Format(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("1sago", Utc),
        Err(Error::Format(_))
    ));

    // both modifiers
    assert!(matches!(
        parse_timestamp_tz("today + - 1s", Utc),
        Err(Error::Format(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("today - 1s + 5m", Utc),
        Err(Error::Format(_))
    ));

    // unsupported strftime format
    assert!(matches!(
        parse_timestamp_tz("2018/08/12 01:02:03.1234", Utc),
        Err(Error::Format(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("2018/08/12 01:02:03", Utc),
        Err(Error::Format(_))
    ));
}

#[test]
fn invalid_number() {
    // numbers that would overflow fail
    assert!(matches!(
        parse_timestamp_tz("2018-08-09 07:06:05.123456789123456789123456789", Utc),
        Err(Error::Number(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("+1000000000d 100s", Utc),
        Err(Error::Number(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("+100s 1000000000d", Utc),
        Err(Error::Number(_))
    ));

    // number contains whitespace
    assert!(matches!(
        parse_timestamp_tz("2018-08-09 07:06:05.123 4", Utc),
        Err(Error::Number(_))
    ));

    // number contains characters
    assert!(matches!(
        parse_timestamp_tz("2018-08-09 07:06:05.123a4", Utc),
        Err(Error::Number(_))
    ));
}

#[test]
fn invalid_timeunit() {
    // missing time unit
    assert!(matches!(
        parse_timestamp_tz("+5", Utc),
        Err(Error::TimeUnit(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("5 ago", Utc),
        Err(Error::TimeUnit(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("today +5", Utc),
        Err(Error::TimeUnit(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("today -5s 6", Utc),
        Err(Error::TimeUnit(_))
    ));

    // unknown time unit
    assert!(matches!(
        parse_timestamp_tz("+5 bad", Utc),
        Err(Error::TimeUnit(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("5 bad ago", Utc),
        Err(Error::TimeUnit(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("today +5 bad", Utc),
        Err(Error::TimeUnit(_))
    ));

    assert!(matches!(
        parse_timestamp_tz("today -5s 6 bad", Utc),
        Err(Error::TimeUnit(_))
    ));
}

fn parse_timestamp_tz_aux<Tz: TimeZone>(timestamp: &str, timezone: Tz) -> DateTime<Tz> {
    parse_timestamp_tz(timestamp, timezone)
        .unwrap()
        .single()
        .unwrap()
}

fn today_time<Tz: TimeZone>(tz: &Tz, t: Option<NaiveTime>) -> DateTime<Tz> {
    let t = naive_today(tz).and_time(t.unwrap_or_default());
    tz.from_local_datetime(&t).unwrap()
}
