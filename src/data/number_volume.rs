use std::{
    fmt::{Debug, Display},
    hint::unreachable_unchecked,
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

use super::Range;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NumberVolume {
    number: Option<Range>,
    volume: Option<Range>,
}

impl FromStr for NumberVolume {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref NUM_RE: Regex =
                Regex::new(r"N(?:úmeros?)?[.:]? ?((?:[\-e]? *\d+ *)+)").unwrap();
            static ref VOL_RE: Regex =
                Regex::new(r"V(?:olumes?)?[.:]? ?((?:[\-e]? *(?:\d+|[XIV]+))+)").unwrap();
        }

        let num_captures = NUM_RE.captures(value);
        let vol_captures = VOL_RE.captures(value);

        match (num_captures, vol_captures) {
            (None, None) => Err(()),
            (num, vol) => Ok(NumberVolume {
                number: num
                    .map(|cap| cap.get(1).unwrap().as_str().parse().ok())
                    .flatten(),
                volume: vol
                    .map(|cap| cap.get(1).unwrap().as_str().parse().ok())
                    .flatten(),
            }),
        }
    }
}

impl Display for NumberVolume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.volume, &self.number) {
            // Safity: NumberVolume struct ensure at least one of number or volume is Some
            (None, None) => unsafe { unreachable_unchecked() },
            (Some(n), Some(v)) => write!(f, "n.~{}, v.~{}", n, v),
            (None, Some(v)) => write!(f, "v.~{}", v),
            (Some(n), None) => write!(f, "n.~{}", n),
        }
    }
}

impl Debug for NumberVolume {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

#[test]
fn it_works() {
    let cases = [
        (
            "Número 1 e 2. Volume 18",
            Ok(NumberVolume {
                number: Some((1..=2).into()),
                volume: Some(18.into()),
            }),
        ),
        (
            "Números: 1 e 2 Volume: XVIII- XIX",
            Ok(NumberVolume {
                number: Some((1..=2).into()),
                volume: Some((18..=19).into()),
            }),
        ),
        (
            "Número 1 e 2. Volume 16",
            Ok(NumberVolume {
                number: Some((1..=2).into()),
                volume: Some(16.into()),
            }),
        ),
        (
            "Volumes XI-XII-XIII",
            Ok(NumberVolume {
                number: None,
                volume: Some((11..=13).into()),
            }),
        ),
        (
            "Números 1 e 2. Volume 7",
            Ok(NumberVolume {
                number: Some((1..=2).into()),
                volume: Some(7.into()),
            }),
        ),
        (
            "N.1-2 V.18-19",
            Ok(NumberVolume {
                number: Some((1..=2).into()),
                volume: Some((18..=19).into()),
            }),
        ),
        (
            "Volume IX - Número 1 e 2",
            Ok(NumberVolume {
                number: Some((1..=2).into()),
                volume: Some(9.into()),
            }),
        ),
        (
            "N.2 V.5",
            Ok(NumberVolume {
                number: Some(2.into()),
                volume: Some(5.into()),
            }),
        ),
        (
            "Número 1 e 2. Volume 8",
            Ok(NumberVolume {
                number: Some((1..=2).into()),
                volume: Some(8.into()),
            }),
        ),
        (
            "Números: 1 e 2 Volume: XIV",
            Ok(NumberVolume {
                number: Some((1..=2).into()),
                volume: Some(14.into()),
            }),
        ),
        (
            "Número 2. Volume 6",
            Ok(NumberVolume {
                number: Some(2.into()),
                volume: Some(6.into()),
            }),
        ),
        (
            "Volume X - Número 1 e 2",
            Ok(NumberVolume {
                number: Some((1..=2).into()),
                volume: Some(10.into()),
            }),
        ),
        (
            "N.1 e 2 V.4",
            Ok(NumberVolume {
                number: Some((1..=2).into()),
                volume: Some(4.into()),
            }),
        ),
        (
            "N.1 V.5",
            Ok(NumberVolume {
                number: Some(1.into()),
                volume: Some(5.into()),
            }),
        ),
        (
            "N.2 V.2",
            Ok(NumberVolume {
                number: Some(2.into()),
                volume: Some(2.into()),
            }),
        ),
        (
            "N.1 V.2",
            Ok(NumberVolume {
                number: Some(1.into()),
                volume: Some(2.into()),
            }),
        ),
        (
            "Números 1 e 2. Volume 9",
            Ok(NumberVolume {
                number: Some((1..=2).into()),
                volume: Some(9.into()),
            }),
        ),
        (
            "Número 1 e 2. Volume 9",
            Ok(NumberVolume {
                number: Some((1..=2).into()),
                volume: Some(9.into()),
            }),
        ),
        (
            "N.1 V.1",
            Ok(NumberVolume {
                number: Some(1.into()),
                volume: Some(1.into()),
            }),
        ),
    ];

    for (res, expec) in cases.iter().map(|(s, exp)| (s.parse(), exp)) {
        assert_eq!(res, *expec);
    }
}
