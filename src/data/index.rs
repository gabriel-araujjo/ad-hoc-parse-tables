use std::fmt::Display;

pub struct Index<'a>(&'a str);

impl<'a> From<&'a str> for Index<'a> {
    fn from(s: &'a str) -> Self {
        Self(s)
    }
}

struct Capitalize<'a>(&'a str);

impl<'a> Display for Capitalize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (index, fst) = match self.0.chars().next() {
            Some(c) => (c.len_utf8(), c),
            None => return Ok(()),
        };

        let (_, tail) = self.0.split_at(index);

        write!(f, "{}{}", fst.to_uppercase(), tail)
    }
}

impl<'a> Display for Index<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }

        let s = match self.0 {
            "1º secretário do IAGP" => "IAGP 1º secretário do",
            "Ribeiro da Silva" => "Silva Ribeiro da",
            s => s,
        };

        f.write_str(r"\index{")?;

        let mut it = s.splitn(2, " ");

        it.next()
            .map(|s| write!(f, "{}", Capitalize(s)))
            .transpose()?;

        for s in it {
            write!(f, "!{}", s)?;
        }

        f.write_str("}")
    }
}
