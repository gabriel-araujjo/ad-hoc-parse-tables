use crate::parse::{ParseError, ParseStatus};
use data::Row;
use regex::Regex;
use std::io::{BufReader, Error as IoError, Read};
use std::{env, fs::File};
use xml::reader::{Error as XmlError, EventReader, XmlEvent};
use zip::result::ZipError;
use zip::ZipArchive;

mod data;
mod parse;

#[derive(Debug)]
enum CallError {
    NoArgument,
    Io(IoError),
    Xml(XmlError),
    Parse(ParseError),
    Zip(ZipError),
    AlreadyFinished,
    Paragraph(String),
}

impl From<IoError> for CallError {
    fn from(e: IoError) -> Self {
        CallError::Io(e)
    }
}

impl From<XmlError> for CallError {
    fn from(e: XmlError) -> Self {
        CallError::Xml(e)
    }
}

impl From<ParseError> for CallError {
    fn from(e: ParseError) -> Self {
        CallError::Parse(e)
    }
}

impl From<ZipError> for CallError {
    fn from(e: ZipError) -> Self {
        CallError::Zip(e)
    }
}

struct ParseXml<R: Read> {
    parser: EventReader<R>,
    status: ParseStatus,
    row: Option<Row>,
}

impl<R: Read> ParseXml<R> {
    fn new(r: R) -> Self {
        Self {
            parser: EventReader::new(r),
            status: ParseStatus::SearchingTable,
            row: None,
        }
    }

    fn next(&mut self) -> Result<Row, CallError> {
        let status = &mut self.status;

        if status.is_finished() {
            return self.row.take().ok_or(CallError::AlreadyFinished);
        }

        let mut row = self.row.take().unwrap_or_else(Row::new);

        loop {
            match self.parser.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => match name.local_name.as_str() {
                    "tbl" => status.start_table()?,
                    "tr" => status.start_field()?,
                    "tc" => status.start_col()?,
                    "p" => {
                        status.append_new_line().ok();
                    }
                    "t" => {
                        if let Some(att) =
                            attributes.iter().find(|att| att.name.local_name == "space")
                        {
                            if att.value == "preserve" {
                                status
                                    .append_text(" ")
                                    .map_err(|_| CallError::Paragraph(" ".to_owned()))?;
                            }
                        }
                    }
                    _ => {}
                },
                Ok(XmlEvent::EndElement { name, .. }) => match name.local_name.as_str() {
                    "tc" => status.end_col()?,
                    "tr" => {
                        let f = status.end_field()?;

                        if let Err(f) = row.set_field(f) {
                            let mut new_row = Row::new();
                            new_row.set_field(f).ok();
                            self.row = Some(new_row);

                            return Ok(row);
                        }
                    }
                    "tbl" => {
                        status.end_table()?;
                        return Ok(row);
                    }
                    _ => {}
                },
                Ok(XmlEvent::Characters(text)) => {
                    status
                        .append_text(&text)
                        .map_err(|_| CallError::Paragraph(text))?;
                }
                Ok(XmlEvent::EndDocument) => {
                    return match status.finish() {
                        Ok(_) => Err(CallError::AlreadyFinished),
                        Err(f) => {
                            row.set_field(f).unwrap_or_else(|f| {
                                let mut new_row = Row::new();
                                new_row.set_field(f).ok();
                                self.row = Some(new_row);
                            });

                            Ok(row)
                        }
                    };
                }
                Err(e) => {
                    status.finish().ok();
                    return Err(e.into());
                }
                _ => {}
            }
        }
    }
}

enum RowOrParagraph {
    Row(Row),
    Paragraph(String),
}

impl<R: Read> IntoIterator for ParseXml<R> {
    type Item = RowOrParagraph;

    type IntoIter = Rows<R>;

    fn into_iter(self) -> Self::IntoIter {
        Rows { parser: self }
    }
}

struct Rows<R: Read> {
    parser: ParseXml<R>,
}

impl<R: Read> Iterator for Rows<R> {
    type Item = RowOrParagraph;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.parser.next() {
                Ok(row) => break Some(RowOrParagraph::Row(row)),
                Err(CallError::Paragraph(p)) => break Some(RowOrParagraph::Paragraph(p)),
                Err(CallError::AlreadyFinished) => break None,
                _ => {}
            }
        }
    }
}

fn main() -> Result<(), CallError> {
    let path = env::args().skip(1).next().ok_or(CallError::NoArgument)?;
    // let path = "/home/gabriel/Downloads/Tabelas completas livro de bruno(3).docx";
    let mut zip = ZipArchive::new(File::open(path)?)?;

    let entry = BufReader::new(zip.by_name("word/document.xml")?);

    println!(r"\chapter{{Catálogo}}");
    println!();
    println!(r"\section{{Revistas publicadas em 1903}}");
    println!();

    #[derive(PartialEq, Eq)]
    enum LastItem {
        Text,
        Row,
    }

    let mut last_item = None;

    let number_regex = Regex::new(r"^\d+$").unwrap();
    let tabela_regex = Regex::new(r"^Tabela \d+$").unwrap();

    for row_or_p in ParseXml::new(entry).into_iter() {
        match row_or_p {
            RowOrParagraph::Row(row) => {
                if last_item.is_some() {
                    println!();
                    println!(r"\bigskip");
                    println!();
                }
                println!(r"\noindent{}", row);
                last_item = Some(LastItem::Row);
            }
            RowOrParagraph::Paragraph(p) => {
                let trimmed = p.trim();
                match trimmed {
                    "Tabela" | r"SEQ Tabela \* ARABIC" => continue,
                    _ => (),
                };

                if number_regex.is_match(trimmed) || tabela_regex.is_match(trimmed) {
                    continue;
                }

                if Some(LastItem::Row) == last_item {
                    println!();
                    eprintln!();
                }
                eprintln!("paragraph: {}", p);

                println!("{}", p.replace("$", r"\$"));
                last_item = Some(LastItem::Text);
            }
        }
    }

    Ok(())
}
