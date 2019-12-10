use std::{io, str};

pub struct Input {
    pub raw: String,
}

impl Input {
    pub fn new(raw: &str) -> Input {
        Input {
            raw: String::from(raw),
        }
    }

    pub fn from_reader(reader: &mut impl io::Read) -> io::Result<Input> {
        let mut s = String::new();
        reader.read_to_string(&mut s)?;

        io::Result::Ok(Input { raw: s })
    }

    pub fn lines<'a>(&'a self) -> impl Iterator<Item = &str> + 'a {
        self.raw.trim().lines().filter(|s| !s.is_empty())
    }

    pub fn map_lines<'a, T>(
        &'a self,
        f: impl FnMut(&'a str) -> T + 'a,
    ) -> impl Iterator<Item = T> + 'a {
        self.lines().map(f)
    }

    pub fn parse_lines<'a, T: str::FromStr + 'a>(
        &'a self,
    ) -> impl Iterator<Item = Result<T, T::Err>> + 'a {
        self.map_lines(|l| l.parse())
    }

    pub fn csv<'a>(&'a self) -> impl Iterator<Item = &str> + 'a {
        self.raw.split(',').filter(|s| !s.is_empty()).map(str::trim)
    }

    pub fn map_csv<'a, T: 'a>(
        &'a self,
        f: impl FnMut(&'a str) -> T + 'a,
    ) -> impl Iterator<Item = T> + 'a {
        self.csv().map(f)
    }

    pub fn parse_csv<'a, T: str::FromStr + 'a>(
        &'a self,
    ) -> impl Iterator<Item = Result<T, T::Err>> + 'a {
        self.map_csv(|s| s.parse())
    }
}
