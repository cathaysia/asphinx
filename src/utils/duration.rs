use std::{fmt::Display, time::SystemTime};

pub struct Counter {
    start_time: SystemTime,
    time: SystemTime,
}

impl Counter {
    pub fn new() -> Self {
        let time = SystemTime::now();
        Self {
            start_time: time.clone(),
            time,
        }
    }

    pub fn elapsed(&self) -> Option<PrintableDuration> {
        Some(self.time.elapsed().ok()?.into())
    }

    pub fn reset(&mut self) {
        self.time = SystemTime::now();
    }

    pub fn from_start(&self) -> Option<PrintableDuration> {
        Some(self.start_time.elapsed().ok()?.into())
    }
}

pub struct PrintableDuration {
    duration: u128,
}

impl PrintableDuration {
    pub fn new(duration: u128) -> Self {
        assert!(duration != 0);
        PrintableDuration { duration }
    }
}

impl From<std::time::Duration> for PrintableDuration {
    fn from(value: std::time::Duration) -> Self {
        PrintableDuration {
            duration: value.as_millis(),
        }
    }
}

impl Display for PrintableDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = Vec::<String>::new();
        let t = self.duration / 60000;
        if t > 0 {
            res.push(format!("{}m", t));
        }
        let t = self.duration / 1000 % 60;
        if t > 0 {
            res.push(format!("{}s", t));
        }
        let t = self.duration % 1000;
        if t > 0 {
            res.push(format!("{}ms", t));
        }

        write!(f, "{}", res.join(" "))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fmt_1min() {
        let res = format!("{}", PrintableDuration::new(60 * 1000));
        assert_eq!(&res, "1m")
    }

    #[test]
    fn test_fmt_1min20s() {
        let res = format!("{}", PrintableDuration::new(60 * 1000 + 20 * 1000));
        assert_eq!(&res, "1m 20s")
    }

    #[test]
    fn test_fmt_1min20s30ms() {
        let res = format!("{}", PrintableDuration::new(60 * 1000 + 20 * 1000 + 30));
        assert_eq!(&res, "1m 20s 30ms")
    }

    #[test]
    fn test_fmt_20s() {
        let res = format!("{}", PrintableDuration::new(20 * 1000));
        assert_eq!(&res, "20s")
    }

    #[test]
    fn test_fmt_20s30ms() {
        let res = format!("{}", PrintableDuration::new(20 * 1000 + 30));
        assert_eq!(&res, "20s 30ms")
    }

    #[test]
    fn test_fmt_30ms() {
        let res = format!("{}", PrintableDuration::new(30));
        assert_eq!(&res, "30ms")
    }
}
