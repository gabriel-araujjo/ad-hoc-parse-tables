use std::fmt::Display;
use std::str::FromStr;

pub use self::ata::Ata;
pub use self::document_type::{DocumentType, DocumentTypes};
pub use self::field::Field;
pub use self::number_volume::NumberVolume;
use self::page::Page;
pub use self::range::Range;

mod range;

mod ata;
mod document_type;
mod field;
mod number_volume;
mod page;

mod index;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Row {
    pub year: Option<Field<Range>>,                 // Ano da Revista: 1903
    pub number_volume: Option<Field<NumberVolume>>, // Número/volume da revista: N.2 V.2
    pub ata: Option<Field<Ata>>, // Número e data da Ata de sessão: 5ª sessão de 17 de agosto de 1902
    pub page: Option<Field<Page>>, // Página: p.138
    pub kind: Option<DocumentTypes>, // Tipo de documento: Revista
    pub title: Option<Field<String>>, // Título completo do manuscrito: Revista do IAGP
    pub doner: Option<Field<String>>, // Quem doou?: 1º secretário do IAGP
    pub time: Option<Field<String>>, // Qual a temporalidade do documento: Não mencionado
}

impl Row {
    pub fn new() -> Self {
        Self {
            year: None,
            number_volume: None,
            ata: None,
            page: None,
            kind: None,
            title: None,
            doner: None,
            time: None,
        }
    }

    pub fn set_field(&mut self, field: (String, String)) -> Result<(), (String, String)> {
        let (key, value) = field;

        let key = key.trim().to_lowercase();
        let value = value.trim().to_owned();

        fn set_field_internal<F: FromStr>(
            field: &mut Option<F>,
            key: String,
            value: String,
        ) -> Result<(), (String, String)> {
            match field {
                None => match value.as_str().parse() {
                    Ok(f) => {
                        field.replace(f);
                        Ok(())
                    }
                    _ => Err((key, value)),
                },
                Some(_) => Err((key, value)),
            }
        }

        if key.starts_with("ano") {
            set_field_internal(&mut self.year, key, value)
        } else if key.contains("ata") {
            set_field_internal(&mut self.ata, key, value)
        } else if key.starts_with("numero") || key.starts_with("número") {
            set_field_internal(&mut self.number_volume, key, value)
        } else if key.starts_with("página") || key.starts_with("pagina") {
            set_field_internal(&mut self.page, key, value)
        } else if key.starts_with("tipo") {
            set_field_internal(&mut self.kind, key, value.to_lowercase())
        } else if key.starts_with("título") || key.starts_with("titulo") {
            set_field_internal(&mut self.title, key, value.replace("$", r"\$"))
        } else if key.starts_with("quem") {
            set_field_internal(&mut self.doner, key, value)
        } else if key.starts_with("qual") {
            set_field_internal(&mut self.time, key, value)
        } else if !key.is_empty() || !value.is_empty() {
            eprintln!("Warning: unhandled key {} with value {}", key, value);
            eprintln!("Row current state {}", self);
            Ok(())
        } else {
            Ok(())
        }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, r"\begin{{tabularx}}{{\linewidth}}{{| p{{.4545\linewidth}} | p{{.4625\linewidth}} |}}")?;
        writeln!(f, r"    \hline")?;

        if let Some(year) = self.year.as_ref() {
            writeln!(f, r"    \hfill Ano da revista & {} \\", year)?;
            writeln!(f, r"    \hline")?;
        }

        if let Some(nv) = self.number_volume.as_ref() {
            writeln!(f, r"    \hfill Número/volume da revista & {} \\", nv)?;
            writeln!(f, r"    \hline")?;
        }

        if let Some(ata) = self.ata.as_ref() {
            writeln!(f, r"    \hfill Nº e data da ata de sessão & {} \\", ata)?;
            writeln!(f, r"    \hline")?;
        }

        if let Some(pages) = self.page.as_ref() {
            writeln!(f, r"    \hfill Página(s) & {} \\", pages)?;
            writeln!(f, r"    \hline")?;
        }

        if let Some(kind) = self.kind.as_ref() {
            writeln!(f, r"    \hfill Tipo de documento & {} \\", kind)?;
            writeln!(f, r"    \hline")?;
        }

        if let Some(title) = self.title.as_ref() {
            writeln!(f, r"    \hfill Título completo do documento & {} \\", title)?;
            writeln!(f, r"    \hline")?;
        }

        if let Some(doner) = self.doner.as_ref() {
            writeln!(f, r"    \hfill Quem doou? & {}{} \\", doner, doner.index())?;
            writeln!(f, r"    \hline")?;
        }

        if let Some(time) = self.time.as_ref() {
            writeln!(f, r"    \hfill Temporalidade do documento & {} \\", time)?;
            writeln!(f, r"    \hline")?;
        }

        write!(f, r"\end{{tabularx}}")?;

        Ok(())
    }
}
