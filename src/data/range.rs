use lazy_static::lazy_static;
use regex::Regex;
use std::{
    convert::TryFrom,
    fmt::{self, Debug, Display},
    ops::{Add, Div, Mul, RangeInclusive},
    str::FromStr,
};

#[derive(PartialEq, Eq, Hash)]
pub struct Range<Idx = u32>(RangeInclusive<Idx>);

impl<Idx> From<RangeInclusive<Idx>> for Range<Idx> {
    fn from(r: RangeInclusive<Idx>) -> Self {
        Range(r)
    }
}

impl<Idx: Clone> From<Idx> for Range<Idx> {
    fn from(s: Idx) -> Self {
        Range(s.clone()..=s)
    }
}

impl<Idx: Display + PartialEq> Display for Range<Idx> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_range(&self.0, f)
    }
}

impl<Idx: Display + PartialEq> Debug for Range<Idx> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_range(&self.0, f)
    }
}

impl<Idx: PartialOrd> PartialOrd for Range<Idx> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.start().partial_cmp(other.0.start()) {
            Some(std::cmp::Ordering::Equal) => self.0.end().partial_cmp(other.0.end()),
            o => o,
        }
    }
}

impl<Idx: Ord> Ord for Range<Idx> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.start().cmp(other.0.start()) {
            std::cmp::Ordering::Equal => self.0.end().cmp(other.0.end()),
            o => o,
        }
    }
}

impl<Idx> FromStr for Range<Idx>
where
    Idx: Eq
        + TryFrom<isize>
        + From<u8>
        + Div<Output = Idx>
        + Add<Output = Idx>
        + Mul<Output = Idx>
        + Ord
        + FromStr
        + Copy,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_range(s).map(Into::into)
    }
}

fn format_range<Idx: Display + PartialEq>(
    range: &RangeInclusive<Idx>,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    if range.start() == range.end() {
        write!(f, "{}", range.start())
    } else {
        write!(f, "{}--{}", range.start(), range.end())
    }
}

fn parse_range<Idx>(s: &str) -> Result<RangeInclusive<Idx>, ()>
where
    Idx: Eq
        + TryFrom<isize>
        + From<u8>
        + Div<Output = Idx>
        + Add<Output = Idx>
        + Mul<Output = Idx>
        + Ord
        + FromStr
        + Copy,
{
    lazy_static! {
        static ref NUMBER: Regex = Regex::new(r"(?:\d+|[MDCLXVI]+)").unwrap();
    }

    let mut min = None;
    let mut max = None;

    for m in NUMBER.captures_iter(s) {
        let s = m.get(0).unwrap().as_str();
        let num = match s.parse() {
            Ok(n) => Some(n),
            _ => parse_roman(s),
        }
        .ok_or(())?;

        if min.is_none() {
            min = Some(num);
        }

        match max.take() {
            None => max = Some(num),
            Some(m) => {
                if num >= m {
                    max = Some(num)
                } else {
                    let num_digits = mask_digits(num);
                    max = Some(m / num_digits * num_digits + num)
                }
            }
        }
    }

    match (min, max) {
        (Some(min), Some(max)) => Ok(min..=max),
        _ => Err(()),
    }
}

fn mask_digits<Idx>(mut n: Idx) -> Idx
where
    Idx: PartialEq + From<u8> + Div<Output = Idx> + Mul<Output = Idx>,
{
    let mut mask = Idx::from(1);
    while n != Idx::from(0) {
        n = n / Idx::from(10);
        mask = mask * Idx::from(10);
    }

    mask
}

fn parse_roman<Idx: TryFrom<isize>>(s: &str) -> Option<Idx> {
    let mut total = 0;
    let mut max = 0;

    fn char_to_number(c: char) -> isize {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => unreachable!(),
        }
    }

    for d in s.chars().rev().map(char_to_number) {
        total += if d >= max { d } else { -d };

        if max < d {
            max = d;
        }
    }

    if total > 0 {
        <Idx as TryFrom<isize>>::try_from(total).ok()
    } else {
        None
    }
}

#[test]
fn it_works() {
    let cases = [
        ("1 e 2", Ok(1..=2)),
        ("18", Ok(18..=18)),
        ("XVIII- XIX", Ok(18..=19)),
        ("16", Ok(16..=16)),
        ("XI-XII-XIII", Ok(11..=13)),
        ("7", Ok(7..=7)),
        ("18-19", Ok(18..=19)),
        ("IX", Ok(9..=9)),
        ("2", Ok(2..=2)),
        ("5", Ok(5..=5)),
        ("8", Ok(8..=8)),
        ("XIV", Ok(14..=14)),
        ("2", Ok(2..=2)),
        ("6", Ok(6..=6)),
        ("X", Ok(10..=10)),
        ("4", Ok(4..=4)),
        ("1", Ok(1..=1)),
        ("5", Ok(5..=5)),
        ("9", Ok(9..=9)),
        ("1234-3333", Ok(1234..=3333)),
        ("1234 - 3333", Ok(1234..=3333)),
        (" 1234 - 3333 ", Ok(1234..=3333)),
        (" 1234 ", Ok(1234..=1234)),
        (" 1234 -", Ok(1234..=1234)),
        (" 1234 - 3333 - ", Ok(1234..=3333)),
        ("327/328", Ok(327..=328)),
        ("241-2", Ok(241..=242)),
        ("241/52", Ok(241..=252)),
        ("MCMLXXXVIII-MCMLXXXIX", Ok(1988..=1989)),
    ];

    for (res, expec) in cases.iter().map(|(s, e)| (parse_range(s), e.clone())) {
        assert_eq!(res, expec);
    }
}
