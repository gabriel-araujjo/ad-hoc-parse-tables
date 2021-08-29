use chrono::{Datelike, NaiveDate};
use lazy_static::lazy_static;
use never::Never;
use regex::{Captures, Regex};
use std::{
    fmt::{Debug, Display},
    hint::unreachable_unchecked,
    str::FromStr,
};

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Session {
    Ordinary,
    Extraordinary,
    Other(String),
}

impl FromStr for Session {
    type Err = Never;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_ref() {
            "extraordinária" => Ok(Self::Extraordinary),
            "ordinária" => Ok(Self::Ordinary),
            s => Ok(Self::Other(s.to_owned())),
        }
    }
}

impl AsRef<str> for Session {
    fn as_ref(&self) -> &str {
        match self {
            Session::Ordinary => "ord.",
            Session::Extraordinary => "ext.",
            Session::Other(s) => s,
        }
    }
}

impl Display for Session {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Debug for Session {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct NaiveYearMonth(NaiveDate);

impl NaiveYearMonth {
    fn from_ym_opt(year: i32, month: u32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, 1).map(|d| NaiveYearMonth(d))
    }

    fn year(&self) -> i32 {
        self.0.year()
    }
    fn month(&self) -> u32 {
        self.0.month()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HistoricalData {
    PreciseMonth(NaiveYearMonth),
    PreciseDay(NaiveDate),
}

impl HistoricalData {
    fn from_ymd_opt(year: i32, month: u32, day: Option<u32>) -> Option<Self> {
        match day {
            Some(day) => NaiveDate::from_ymd_opt(year, month, day).map(|d| Self::PreciseDay(d)),
            None => NaiveYearMonth::from_ym_opt(year, month).map(|d| Self::PreciseMonth(d)),
        }
    }
}

/// # Safify
///
/// Caller must ensure m in in range 1..=12
unsafe fn month(m: u32) -> &'static str {
    match m {
        1 => "jan",
        2 => "fev",
        3 => "mar",
        4 => "abr",
        5 => "mai",
        6 => "jun",
        7 => "jul",
        8 => "ago",
        9 => "set",
        10 => "out",
        11 => "nov",
        12 => "dez",
        _ => unsafe { unreachable_unchecked() },
    }
}

impl Display for HistoricalData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HistoricalData::PreciseMonth(d) => {
                write!(f, "{}. {}", unsafe { month(d.month()) }, d.year())
            }
            HistoricalData::PreciseDay(d) => write!(
                f,
                "{} {}. {}",
                d.day(),
                unsafe { month(d.month()) },
                d.year()
            ),
        }
    }
}

impl Debug for HistoricalData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ata {
    date: HistoricalData,
    number: u16,
    session: Session,
}

impl FromStr for Ata {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(\d+)[ºª°]?\s+(?:sessão\s+)?(?:([\pL\s]+)\s+)?de\s+(?:(\d+)\s+)?(?:de\s+)?(\w+)\s+de\s+(\d+)"
            )
            .unwrap();
        }

        let captures = RE.captures(value).ok_or(value.to_owned())?;

        Ata::parse_captures(captures).ok_or(value.to_owned())
    }
}

impl Ata {
    #[inline]
    fn parse_captures(captures: Captures) -> Option<Ata> {
        let number = captures.get(1)?.as_str().parse().ok()?;
        let session = captures
            .get(2)
            .map(|m| m.as_str().parse().unwrap())
            .unwrap_or(Session::Ordinary);
        let day = captures.get(3).map(|m| m.as_str().parse().ok()).flatten();
        let month = captures.get(4)?.as_str();
        let year = captures.get(5)?.as_str().parse().ok()?;

        let mut month_chars = month.chars();

        let month = match month_chars.next().unwrap() {
            'j' | 'J' => match month_chars.as_str() {
                "aneiro" => Some(1),
                "unho" => Some(6),
                "ulho" => Some(7),
                _ => None,
            },
            'f' | 'F' => match month_chars.as_str() {
                "evereiro" => Some(2),
                _ => None,
            },
            'm' | 'M' => match month_chars.as_str() {
                "arço" => Some(3),
                "aio" => Some(5),
                _ => None,
            },
            'a' | 'A' => match month_chars.as_str() {
                "bril" => Some(4),
                "gosto" => Some(8),
                _ => None,
            },
            's' | 'S' => match month_chars.as_str() {
                "etembro" => Some(9),
                _ => None,
            },
            'o' | 'O' => match month_chars.as_str() {
                "utubro" => Some(10),
                _ => None,
            },
            'n' | 'N' => match month_chars.as_str() {
                "ovembro" => Some(11),
                _ => None,
            },
            'd' | 'D' => match month_chars.as_str() {
                "ezembro" => Some(12),
                _ => None,
            },
            _ => None,
        }?;

        Some(Ata {
            number,
            session,
            date: HistoricalData::from_ymd_opt(year, month, day)?,
        })
    }
}

impl Ata {
    pub fn year(&self) -> i32 {
        match &self.date {
            HistoricalData::PreciseMonth(d) => d.year(),
            HistoricalData::PreciseDay(d) => d.year(),
        }
    }
}

impl Display for Ata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}ª sess. {} de {}",
            self.number, self.session, self.date
        )
    }
}

impl Debug for Ata {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

#[test]
fn it_works() {
    let cases = [
        "190ª de 05 de Maio de 1912",
        "76º de 07 de Janeiro de 1906",
        "74º de 05 de Novembro de 1905",
        "106º de 3 de setembro de 1907",
        "113º de 5 de abril de 1908",
        "151º  de 03 de Abril de 1910",
        "104º de 7 de setembro de 1907",
        "6º sessão extraordinária de 26  de junho de 1904",
        "63º de 21 de Maio de 1905",
        "92º de 21 de Outubro de 1906",
        "131ª de 21 de Janeiro de 1909",
        "76º de 7 de Janeiro de 1906",
        "5º sessão extraordinária de 24 de janeiro de 1904",
        "120ª de 19 de julho de 1908",
        "194ª de 07 de Julho de 1912",
        "200ª de 06 de Outubro de 1912",
        "203ª de 01 de Dezembro de 1912",
        "89º de 02 de Setembro de 1906",
        "119ª de 05 de julho de 1908",
        "198ª de 01 de Setembro de 1912",
        "35º sessão ordinária de 21 de fevereiro de 1904",
        "38º sessão ordinária de 3 de abril de 1904",
        "42º sessão ordinária de 5 de junho de 1904",
        "140ª de 15 de Agosto de 1909",
        "40º sessão ordinária de 1 de maio de 1904",
        "52º de 04 de Dezembro de 1904",
        "103º de 18 de agosto de 1907",
        "128ª de 06 de dezembro de 1908",
        "70º de 03 de Setembro de 1905",
        "55º de 15 de Janeiro de 1905",
        "13 sessão ordinária de 7 de fevereiro de 1904",
        "88º de 05 de Agosto de 1906",
        "50ª de 02 de Outubro de 1904",
        "31º sessão de 15 de novembro de 1903",
        "78º de 04 de Fevereiro de 1906",
        "58º de 05 de Março de 1905",
        "64ª de 04 de Junho de 1905",
        "5º sessão de 17 de agosto de 1902",
        "19º sessão de 19 de abril de 1903",
        "50º de 02 de Outubro de 1904",
        "125ª de 20 de Setembro de 1908",
        "78º de 4 de Fevereiro de 1906",
        "33º sessão de 3 de janeiro de 1904",
        "64º de 04 de Junho de 1905",
        "66º de 02 de Julho de 1905",
        "51º de 16 de Outubro de 1904",
        "126ª de 18 de outubro de 1908",
        "39º sessão ordinária de 17 de abril de 1904",
        "87º de 15 de Julho de 1906",
        "123ª de 16 de Agosto de 1908",
        "11º sessão de 16 de novembro de 1902",
        "37º sessão ordinária de 20 de março de 1904",
        "41º sessão ordinária de 15 de maio de 1904",
        "26º sessão de 17 de agosto de 1903",
        "124ª de 20 de Setembro de 1908",
        "29º sessão de 4 de outubro de 1903",
        "53º de 18 de Dezembro de 1904",
        "193ª de 16 de Junho de 1912",
        "151º  de 3 de Abril de 1910",
        "138ª de 18 de junho de 1909",
        "134ª de 2 de maio de 1909",
        "122ª de 16 de Agosto de 1908",
        "67º de 16 de Julho de 1905",
        "51ª de 16 de Outubro de 1904",
        "129ª de 03 de Janeiro de 1909",
        "28º sessão de 20 de setembro de 1903",
        "53ª de 18 de Dezembro de 1904",
        "141ª de 5 de Setembro de 1909",
        "21º sessão de 17 de maio de 1903",
        "27º sessão de 6 de setembro de 1903",
        "135ª de 6 de junho de 1909",
        "144ª de 17 de Outubro de 1909",
        "60° de 02 de Abril de 1905",
        "145ª de Novembro de 1909",
        "20º sessão de 13 de maio de 1903",
        "137ª de 4 de julho de 1909",
        "132ª de abril de 1909",
        "13º sessão de 21 de dezembro de 1902",
        "X",
        "142ª de 19 de Setembro de 1909",
        "O documento não estava registrado em ata.",
        "6º sessão da Assembléa Geral de 22 de Outubro de 1905",
        "129ª de 03 de Janeiro de 1908",
        "16º sessão de 15 de fevereiro de 1903",
        "143ª de 3 de Outubro de 1909",
        "30º sessão de 18 outubro de 1903",
    ];

    for case in cases.iter() {
        let ata: Result<Ata, _> = case.parse();

        println!("{:?}", ata);
    }
}

#[test]
fn historical_date_order() {
    assert!(
        HistoricalData::PreciseMonth(NaiveYearMonth::from_ym_opt(1970, 1).unwrap())
            < HistoricalData::PreciseDay(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
    )
}
