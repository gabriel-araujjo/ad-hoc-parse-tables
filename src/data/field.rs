use std::{cmp::Ordering, fmt::{Debug, Display}, mem::swap, str::FromStr};

use never::Never;

use super::{Ata, NumberVolume, Range, index::Index, page::Page};

#[derive(PartialEq, Eq, Hash)]
pub enum Field<T> {
    Absent,
    Present(T),
    NotStandard(String),
}

impl<T: AsRef<str>> AsRef<str> for Field<T> {
    fn as_ref(&self) -> &str {
        match self {
            Field::Absent => "",
            Field::Present(p) => p.as_ref(),
            Field::NotStandard(s) => s,
        }
    }
}

impl<T: AsRef<str>> Field<T> {
    pub fn index(&self) -> Index {
        self.as_ref().into()
    }
}

impl<T: AsRef<str> + Eq> Ord for Field<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Field::Absent, Field::Absent) => Ordering::Equal,
            (Field::Absent, _) => Ordering::Less,
            (_, Field::Absent) => Ordering::Greater,
            (lhs, rhs) => lhs.as_ref().cmp(rhs.as_ref()),
        }
    }
}

impl Ord for Field<Ata> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Field::Absent, Field::Absent) => Ordering::Equal,
            (Field::Absent, _) => Ordering::Less,
            (_, Field::Absent) => Ordering::Greater,
            (Field::Present(lhs), Field::Present(rhs)) => lhs.cmp(rhs),
            (Field::Present(lhs), Field::NotStandard(rhs)) => {
                format!("{}", lhs).as_str().cmp(rhs.as_str())
            }
            (Field::NotStandard(lhs), Field::Present(rhs)) => {
                lhs.as_str().cmp(format!("{}", rhs).as_str())
            }
            (Field::NotStandard(lhs), Field::NotStandard(rhs)) => {
                lhs.cmp(rhs)
            }
        }
    }
}

impl Ord for Field<Page> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Field::Absent, Field::Absent) => Ordering::Equal,
            (Field::Absent, _) => Ordering::Less,
            (_, Field::Absent) => Ordering::Greater,
            (Field::Present(lhs), Field::Present(rhs)) => lhs.cmp(rhs),
            (Field::Present(lhs), Field::NotStandard(rhs)) => {
                format!("{}", lhs).as_str().cmp(rhs.as_str())
            }
            (Field::NotStandard(lhs), Field::Present(rhs)) => {
                lhs.as_str().cmp(format!("{}", rhs).as_str())
            }
            (Field::NotStandard(lhs), Field::NotStandard(rhs)) => {
                lhs.cmp(rhs)
            }
        }
    }
}

impl Ord for Field<Range> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Field::Absent, Field::Absent) => Ordering::Equal,
            (Field::Absent, _) => Ordering::Less,
            (_, Field::Absent) => Ordering::Greater,
            (Field::Present(lhs), Field::Present(rhs)) => lhs.cmp(rhs),
            (Field::Present(lhs), Field::NotStandard(rhs)) => {
                format!("{}", lhs).as_str().cmp(rhs.as_str())
            }
            (Field::NotStandard(lhs), Field::Present(rhs)) => {
                lhs.as_str().cmp(format!("{}", rhs).as_str())
            }
            (Field::NotStandard(lhs), Field::NotStandard(rhs)) => {
                lhs.cmp(rhs)
            }
        }
    }
}

impl Ord for Field<NumberVolume> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Field::Absent, Field::Absent) => Ordering::Equal,
            (Field::Absent, _) => Ordering::Less,
            (_, Field::Absent) => Ordering::Greater,
            (Field::Present(lhs), Field::Present(rhs)) => lhs.cmp(rhs),
            (Field::Present(lhs), Field::NotStandard(rhs)) => {
                format!("{}", lhs).as_str().cmp(rhs.as_str())
            }
            (Field::NotStandard(lhs), Field::Present(rhs)) => {
                lhs.as_str().cmp(format!("{}", rhs).as_str())
            }
            (Field::NotStandard(lhs), Field::NotStandard(rhs)) => {
                lhs.cmp(rhs)
            }
        }
    }
}

impl<T> PartialOrd for Field<T>
where
    Field<T>: Ord,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> FromStr for Field<T>
where
    T: FromStr,
{
    type Err = Never;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "-" | "(?)" | "?" | "não mencionado" | "Não mencionado" | "Não mencionada" | "" | "a conferir" | "A conferir" | "x" | "X" | "- -" => {
                Ok(Self::Absent)
            }
            _ => match s.parse() {
                Ok(t) => Ok(Self::Present(t)),
                _ => Ok(Self::NotStandard(s.to_owned())),
            },
        }
    }
}

impl<T> Field<T> {
    #[inline]
    pub fn present(self) -> Option<T> {
        match self {
            Self::Present(s) => Some(s),
            _ => None,
        }
    }

    #[inline]
    pub fn take(&mut self) -> Self {
        let mut result = Self::Absent;
        swap(&mut result, self);
        result
    }
}

impl<T: Debug> Debug for Field<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Absent => write!(f, "---"),
            Field::Present(d) => write!(f, "{:?}", d),
            Field::NotStandard(s) => write!(f, "{:?}", s),
        }
    }
}

impl<T: Display> Display for Field<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Absent => write!(f, "---"),
            Field::Present(d) => write!(f, "{}", d),
            Field::NotStandard(s) => write!(f, "{}", s),
        }
    }
}
