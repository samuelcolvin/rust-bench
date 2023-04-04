use std::fmt;
use std::io::Write;

fn main() {
    println!("benchmarks");
    let mut s = StackString::new("foobar");
    println!("StackString: '{}', {:?}", s, s);
    s.push_str(" more");
    println!("StackString: '{}', {:?}", s, s);
    s.push_str(" and more");
    println!("StackString: '{}', {:?}", s, s);
}

const MAX_LENGTH: usize = 31;

#[derive(Clone)]
pub enum StackString {
    Empty,
    Short([u8; MAX_LENGTH], usize),
    // Long(String),
}

impl Default for StackString {
    fn default() -> Self {
        Self::Empty
    }
}

impl fmt::Debug for StackString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty"),
            Self::Short(array, len) => write!(f, "Short({:?}, {})", slice_to_str(array, *len), len),
            // Self::Long(s) => write!(f, "Long({:?})", s),
        }
    }
}

impl fmt::Display for StackString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => Ok(()),
            Self::Short(array, len) => {
                write!(f, "{}", slice_to_str(array, *len))
            },
            // Self::Long(s) => write!(f, "{}", s),
        }
    }
}

impl StackString {
    pub fn new(s: &str) -> Self {
        let len = s.len();
        match len {
            0 => Self::Empty,
            1..=MAX_LENGTH => Self::Short(str_to_array(s), len),
            // _ => Self::Long(s.to_string()),
            _ => panic!("string too long"),
        }
    }

    pub fn push_str(&mut self, s: &str) {
        match self {
            Self::Empty => {
                let s_len = s.len();
                if s_len > MAX_LENGTH {
                    // *self = Self::Long(s.to_string());
                    panic!("string too long");
                } else {
                    *self = Self::Short(str_to_array(s), s_len);
                }
            },
            Self::Short(array, len) => {
                let mut str = slice_to_str(array, *len).to_string();
                str.push_str(s);
                let s_len = str.len();
                if s_len > MAX_LENGTH {
                    // *self = Self::Long(str);
                    panic!("string too long");
                } else {
                    *self = Self::Short(str_to_array(&str), s_len);
                }
            }
            // Self::Long(str) => {
            //     str.push_str(s);
            // }
        }
    }
}

fn slice_to_str(s: &[u8], len: usize) -> &str {
    unsafe { std::str::from_utf8_unchecked(&s[..len]) }
}

fn str_to_array(s: &str) -> [u8; MAX_LENGTH] {
    let mut bytes: [u8; MAX_LENGTH] = [0; MAX_LENGTH];
    let mut slice: &mut [u8] = &mut bytes;
    slice.write(s.as_bytes()).unwrap();
    bytes
}
