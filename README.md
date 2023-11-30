# chrono systemd.time

[![crates.io](https://img.shields.io/crates/v/chrono-systemd-time.svg)](https://crates.io/crates/chrono-systemd-time)
[![docs.rs](https://docs.rs/chrono-systemd-time/badge.svg)](https://docs.rs/chrono-systemd-time/)
[![Build & Test](https://github.com/Odoh/chrono-systemd-time/actions/workflows/test.yml/badge.svg)](https://github.com/Odoh/chrono-systemd-time/actions/workflows/test.yml)
[![lint](https://github.com/Odoh/chrono-systemd-time/actions/workflows/lint.yml/badge.svg)](https://github.com/Odoh/chrono-systemd-time/actions/workflows/lint.yml)

The library parses timestamps following the [systemd.time] specifications into [chrono] types.

[chrono-systemd-time]: https://docs.rs/chrono-systemd-time/
[systemd.time]: https://www.freedesktop.org/software/systemd/man/systemd.time.html
[chrono]: https://docs.rs/chrono/

## Timestamp Format

The supported timestamp formats are any defined by the systemd.time specifications, with a few exceptions:
* time units **must** accompany all time span values.
* time zone suffixes are **not** supported.
* weekday prefixes are **not** supported.

The format of a timestamp may be either a time, a time span, or a combination of a time +/- a time span.
* When only a time is given, the parsed time is returned.
* When only a time span is given, the time span is added or subtracted from the current time (now).
* When a combination of a time and a time span is given, the time span is added or subtracted from the parsed time.

Examples of parsing valid timestamps, assuming now is 2018-06-21 01:02:03:
```rust,ignore
    parse_timestamp_tz("2018-08-20 09:11:12.123", Utc) == "2018-08-20T09:11:12.000123Z"
    parse_timestamp_tz("2018-08-20 09:11:12", Utc) == "2018-08-20T09:11:12Z"
    parse_timestamp_tz("18-08-20 09:11:12 +2m", Utc) == "2018-08-20T09:13:12Z"
    parse_timestamp_tz("2018-08-20 + 1h2m3s", Utc) == "2018-08-20T01:02:03Z"
    parse_timestamp_tz("18-08-20 - 1h 2m 3s", Utc) == "2018-08-19T22:57:57Z"
    parse_timestamp_tz("09:11:12 -1day", Utc) == "2018-06-20T09:11:12Z"
    parse_timestamp_tz("09:11:12.123", Utc) == "2018-06-21T09:11:12.000123Z"
    parse_timestamp_tz("11:12", Utc) == "2018-06-21T11:12:00Z"
    parse_timestamp_tz("now", Utc) == "2018-06-21T01:02:03.203918151Z"
    parse_timestamp_tz("today", Utc) == "2018-06-21T00:00:00Z"
    parse_timestamp_tz("yesterday -2days", Utc) == "2018-06-18T00:00:00Z"
    parse_timestamp_tz("tomorrow +1week", Utc) == "2018-06-29T00:00:00Z"

    parse_timestamp_tz("epoch +1529578800s", Utc) == "2018-06-21T11:00:00Z"
    parse_timestamp_tz("@1529578800s", Utc) == "2018-06-21T11:00:00Z"
    parse_timestamp_tz("now +4h50m", Utc) == "2018-06-21T05:52:03.203918151Z"
    parse_timestamp_tz("4h50m left", Utc) == "2018-06-21T05:52:03.203918151Z"
    parse_timestamp_tz("+4h50m", Utc) == "2018-06-21T05:52:03.203918151Z"
    parse_timestamp_tz("now -3s", Utc) == "2018-06-21T01:02:00.203918151Z"
    parse_timestamp_tz("3s ago", Utc) == "2018-06-21T01:02:00.203918151Z"
    parse_timestamp_tz("-3s", Utc) == "2018-06-21T01:02:00.203918151Z"
```

#### Time
The syntax of a time consists of a set of keywords and strftime formats:
* `"now"`, `"epoch"`
* `"today"`, `"yesterday"`, `"tomorrow"`
* `"%y-%m-%d %H:%M:%S"`, `"%Y-%m-%d %H:%M:%S"`
* `"%y-%m-%d %H:%M"`, `"%Y-%m-%d %H:%M"`
* `"%y-%m-%d"`, `"%Y-%m-%d"`
* `"%H:%M:%S"`
* `"%H:%M"`

Strftime timestamps with a seconds component may also include a microsecond component, separated by a `'.'`.
* When the date is omitted, today is assumed.
* When the time is omitted, 00:00:00 is assumed.

Examples of valid times (assuming now is 2018-06-21 01:02:03):
```rust,ignore
    "2018-08-20 09:11:12.123" == "2018-08-20T09:11:12.000123"
        "2018-08-20 09:11:12" == "2018-08-20T09:11:12"
          "18-08-20 09:11:12" == "2018-08-20T09:11:12"
                 "2018-08-20" == "2018-08-20T00:00:00"
                   "18-08-20" == "2018-08-20T00:00:00"
                   "09:11:12" == "2018-06-21T09:11:12"
               "09:11:12.123" == "2018-06-21T09:11:12.000123"
                      "11:12" == "2018-06-21T11:12:00"
                        "now" == "2018-06-21T01:02:03.203918151"
                      "epoch" == "1970-01-01T00:00:00"
                      "today" == "2018-06-21T00:00:00"
                  "yesterday" == "2018-06-20T00:00:00"
                   "tomorrow" == "2018-06-22T00:00:00"
```

#### Time span
A time span is made up of a combination of time units, with the following time units understood:
* `"usec"`, `"us"`, `"µs"`
* `"msec"`, `"ms"`
* `"seconds"`, `"second"`, `"sec"`, `"s"`
* `"minutes"`, `"minute"`, `"min"`, `"m"`
* `"hours"`, `"hour"`, `"hr"`, `"h"`
* `"days"`, `"day"`, `"d"`
* `"weeks"`, `"week"`, `"w"`
* `"months"`, `"month"`, `"M"` (defined as 30.44 days)
* `"years"`, `"year"`, `"y"` (defined as 365.25 days)

All components of a time span are added together.

Examples of valid time spans:
```rust,ignore
          "3hours" == Duration::hours(3)
           "2d 5h" == Duration::days(2) + Duration::hours(5)
    "1y 10 months" == Duration::years(1) + Duration::months(10)
          "30m22s" == Duration::minutes(30) + Duration::seconds(22)
       "10m 2s 5m" == Duration::minutes(15) + Duration::seconds(2)
        "10d 2 5m" == Duration::days(10) + Duration::minutes(25)
```
