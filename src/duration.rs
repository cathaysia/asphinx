use std::fmt::Display;

pub struct SelfDuration {
    duration: u128,
}

impl SelfDuration {
    pub fn new(duration: u128) -> Self {
        assert!(duration != 0);
        SelfDuration { duration }
    }
}

impl Display for SelfDuration {
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
        let res = format!("{}", SelfDuration::new(60 * 1000));
        assert_eq!(&res, "1m")
    }

    #[test]
    fn test_fmt_1min20s() {
        let res = format!("{}", SelfDuration::new(60 * 1000 + 20 * 1000));
        assert_eq!(&res, "1m 20s")
    }

    #[test]
    fn test_fmt_1min20s30ms() {
        let res = format!("{}", SelfDuration::new(60 * 1000 + 20 * 1000 + 30));
        assert_eq!(&res, "1m 20s 30ms")
    }

    #[test]
    fn test_fmt_20s() {
        let res = format!("{}", SelfDuration::new(20 * 1000));
        assert_eq!(&res, "20s")
    }

    #[test]
    fn test_fmt_20s30ms() {
        let res = format!("{}", SelfDuration::new(20 * 1000 + 30));
        assert_eq!(&res, "20s 30ms")
    }

    #[test]
    fn test_fmt_30ms() {
        let res = format!("{}", SelfDuration::new(30));
        assert_eq!(&res, "30ms")
    }
}
