use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use lazy_static::lazy_static;
use never::Never;
use regex::Regex;

use super::{Field, index::Index};

#[derive(PartialEq, Eq, Hash)]
pub enum DocumentType {
    Almanaque,
    Anais,
    Anuario,
    Artigo,
    Ata,
    Atlas,
    Biografia,
    Boletim,
    Carta,
    CartaTopografica,
    Catalogo,
    Conferencia,
    Dicionario,
    Discurso,
    DocumentoJuridico,
    DocumentoOficial,
    Estatuto,
    Folheto,
    Jornal,
    Literatura,
    Livro,
    Manifesto,
    Manuscristo,
    Mapa,
    Medalha,
    Memorias,
    Moeda,
    Obra,
    Periodico,
    Poesia,
    Relatorio,
    Revista,
    Tese,
    Other(String),
}

impl AsRef<str> for DocumentType {
    fn as_ref(&self) -> &str {
        match self {
            DocumentType::Almanaque => "Almanaque",
            DocumentType::Anais => "Anais",
            DocumentType::Anuario => "Anuário",
            DocumentType::Artigo => "Artigo",
            DocumentType::Ata => "Ata",
            DocumentType::Atlas => "Atlas",
            DocumentType::Biografia => "Biografia",
            DocumentType::Boletim => "Boletim",
            DocumentType::Carta => "Carta",
            DocumentType::CartaTopografica => "Carta Topográfica",
            DocumentType::Catalogo => "Catalogo",
            DocumentType::Conferencia => "Conferência",
            DocumentType::Dicionario => "Dicionário",
            DocumentType::Discurso => "Discurso",
            DocumentType::DocumentoJuridico => "Documento Juridico",
            DocumentType::DocumentoOficial => "Documento Oficial",
            DocumentType::Estatuto => "Estatuto",
            DocumentType::Folheto => "Folheto",
            DocumentType::Jornal => "Jornal",
            DocumentType::Literatura => "Literatura",
            DocumentType::Livro => "Livro",
            DocumentType::Manifesto => "Manifesto",
            DocumentType::Manuscristo => "Manuscristo",
            DocumentType::Mapa => "Mapa",
            DocumentType::Medalha => "Medalha",
            DocumentType::Memorias => "Memórias",
            DocumentType::Moeda => "Moeda",
            DocumentType::Obra => "Obra",
            DocumentType::Periodico => "Periódico",
            DocumentType::Poesia => "Poesia",
            DocumentType::Relatorio => "Relatório",
            DocumentType::Revista => "Revista",
            DocumentType::Tese => "Tese",
            DocumentType::Other(s) => s,
        }
    }
}

impl Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentType::Other(s) => {
                let (index, fst) = match s.chars().next() {
                    Some(c) => (c.len_utf8(), c),
                    None => return Ok(()),
                };

                let (_, tail) = s.as_str().split_at(index);

                write!(f, "{}{}", fst.to_uppercase(), tail)
            }
            _ => write!(f, "{}", self.as_ref()),
        }
    }
}

impl Debug for DocumentType {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl PartialOrd for DocumentType {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DocumentType {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl DocumentType {
    fn index(&self) -> Index {
        self.as_ref().into()
    }
}

impl FromStr for DocumentType {
    type Err = Never;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "periódico" | "jornais e revistas" | "jornal e revistas" => Self::Periodico,
            "jornal" | "jornais" => Self::Jornal,
            "revista" | "revivsta" | "revistas" => Self::Revista,
            "livro" | "livros" => Self::Livro,
            "documento oficial"
            | "documentos oficiais"
            | "documento official"
            | "documento  oficial" => Self::DocumentoOficial,
            "relatório" | "relatorio" => Self::Relatorio,
            "boletim" => Self::Boletim,
            "medalha comemorativa" | "medalha" => Self::Medalha,
            "catálogo" | "catalogo" => Self::Catalogo,
            "anuário" | "anuario" => Self::Anuario,
            "manuscrito" | "manuscritos" | "manuscristo" => Self::Manuscristo,
            "anais" => Self::Anais,
            "discurso" | "discuro" | "discurso oficial" => Self::Discurso,
            "dicionário" | "dicionario" => Self::Dicionario,
            "moeda" => Self::Moeda,
            "ata" | "acta" => Self::Ata,
            "mapa" | "mapas" => Self::Mapa,
            "carta" | "cartas" => Self::Carta,
            "obra" => Self::Obra,
            "biografia" => Self::Biografia,
            "almanaque" => Self::Almanaque,
            "estatuto" | "estatutos" => Self::Estatuto,
            "conferência" => Self::Conferencia,
            "folheto" => Self::Folheto,
            "manifesto" | "folhetos" => Self::Manifesto,
            "tese" => Self::Tese,
            "atlas" => Self::Atlas,
            "documento jurídico" => Self::DocumentoJuridico,
            "folha topographica" | "planta topográfica" | "carta topográfica" => {
                Self::CartaTopografica
            }
            "artigos" | "artigo" | "artigos de jornal" => Self::Artigo,
            "memórias" | "memória" => Self::Memorias,
            "textos literários" | "literatura" => Self::Literatura,
            "poesias" | "poema" => Self::Poesia,
            s => Self::Other(s.to_owned()),
        })
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DocumentTypes {
    types: Vec<Field<DocumentType>>,
}

impl Into<Vec<Field<DocumentType>>> for DocumentTypes {
    fn into(self) -> Vec<Field<DocumentType>> {
        self.types
    }
}

impl Display for DocumentTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.types.iter();

        fn kind_and_index(
            t: &Field<DocumentType>,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            write!(f, "{}", t)?;
            match t {
                Field::Absent => Ok(()),
                Field::Present(k) => write!(f, "{}", k.index()),
                Field::NotStandard(_) => unreachable!(),
            }
        }

        for t in iter.next() {
            kind_and_index(t, f)?;
        }

        for t in iter {
            f.write_str("/")?;
            kind_and_index(t, f)?;
        }

        Ok(())
    }
}

impl Debug for DocumentTypes {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl FromStr for DocumentTypes {
    type Err = Never;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([^(?]+)?(?:\(([^)]+)\))?(\?)?").unwrap();
        }

        let mut types = Vec::new();

        for part in s.split('/') {
            for part in RE.captures_iter(part) {
                for cap in part.iter().filter(Option::is_some).flatten().skip(1) {
                    types.push(cap.as_str().trim().parse().unwrap());
                }
            }
        }

        Ok(DocumentTypes { types })
    }
}

#[test]
fn parse_string() {
    let types: DocumentTypes = "periódico  (drama)".parse().unwrap();

    assert_eq!(
        types,
        DocumentTypes {
            types: vec![
                Field::Present(DocumentType::Periodico),
                Field::Present(DocumentType::Other("drama".into()))
            ]
        }
    );

    let types: DocumentTypes = "boletim / documentos oficiais".parse().unwrap();

    assert_eq!(
        types,
        DocumentTypes {
            types: vec![
                Field::Present(DocumentType::Boletim),
                Field::Present(DocumentType::DocumentoOficial)
            ]
        }
    );

    let types: DocumentTypes = "boletim ( a conferir )".parse().unwrap();

    assert_eq!(
        types,
        DocumentTypes {
            types: vec![Field::Present(DocumentType::Boletim), Field::Absent]
        }
    );
}
