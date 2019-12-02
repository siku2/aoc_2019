use std::io;

pub struct Input {
    raw: String
}

impl Input {
    pub fn new(raw: &str) -> Input {
        Input { raw: String::from(raw) }
    }

    pub fn from_reader(reader: &mut impl io::Read) -> io::Result<Input> {
        let mut s = String::new();
        reader.read_to_string(&mut s)?;

        io::Result::Ok(Input { raw: s })
    }

    pub fn lines<'a: 'a>(&'a self) -> impl Iterator<Item=&str> + 'a {
        self.raw.lines().filter(|s| !s.is_empty())
    }

    pub fn map_lines<'a, T: 'a>(&'a self, f: fn(&str) -> T) -> impl Iterator<Item=T> + 'a {
        self.lines().map(f)
    }

    pub fn usize_lines<'a>(&'a self) -> impl Iterator<Item=usize> + 'a {
        self.map_lines(|l| l.parse().expect("couldn't parse"))
    }

    pub fn isize_lines<'a>(&'a self) -> impl Iterator<Item=isize> + 'a {
        self.map_lines(|l| l.parse().expect("couldn't parse"))
    }
}