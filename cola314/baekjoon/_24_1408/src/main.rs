use std::error::Error;
use std::io;
use std::ops::{Add, Sub};

fn main() -> Result<(), Box<dyn Error>> {
    let current = read_string()?;
    let start = read_string()?;

    let result = solve(&current, &start)?;
    println!("{}", result);
    Ok(())
}

fn solve(current: &str, start: &str) -> Result<String, Box<dyn Error>> {
    let current = parse_date(current)?;
    let start = parse_date(start)?;
    let duration = start + Time::from_hms(24, 0, 0) - current;
    Ok(format!(
        "{:02}:{:02}:{:02}",
        duration.hour, duration.minute, duration.second
    ))
}

fn parse_date(str: &str) -> Result<Time, Box<dyn Error>> {
    let vec: Vec<&str> = str.trim().split(":").collect();
    Ok(Time::from_hms(
        vec.get(0).ok_or("no year")?.parse()?,
        vec.get(1).ok_or("no month")?.parse()?,
        vec.get(2).ok_or("no day")?.parse()?,
    ))
}

#[derive(Debug, PartialEq)]
struct Time {
    hour: i32,
    minute: i32,
    second: i32,
}

impl Time {
    fn from_hms(hour: i32, minute: i32, second: i32) -> Time {
        Time {
            hour,
            minute,
            second,
        }
    }

    fn from_total_seconds(total_second: i32) -> Time {
        Time {
            hour: total_second / 60 / 60 % 24,
            minute: total_second / 60 % 60,
            second: total_second % 60,
        }
    }

    fn total_seconds(&self) -> i32 {
        let total_minutes = self.hour * 60 + self.minute;
        total_minutes * 60 + self.second
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Time::from_total_seconds(self.total_seconds() + other.total_seconds())
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut total = self.total_seconds() - other.total_seconds();
        while total < 0 {
            total += Time::from_hms(24, 0, 0).total_seconds();
        }
        Time::from_total_seconds(total)
    }
}

fn read_string() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_date_test() {
        let expected = Time::from_hms(13, 52, 30);
        let actual = parse_date("13:52:30").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn example1_test() {
        assert_eq!(solve("13:52:30", "14:00:00").unwrap(), "00:07:30");
    }
}
