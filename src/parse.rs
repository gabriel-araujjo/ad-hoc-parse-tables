use std::{fmt::Display, mem::swap};

#[derive(Debug)]
pub enum ParseStatus {
    SearchingTable,
    SearchingField,
    SearchingCol1,
    Col1(String),
    SearchingCol2(String),
    Col2(String, String),
    Field(String, String),
    Finished,
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error: {}", self.message)
    }
}

impl ParseStatus {
    pub fn start_table(&mut self) -> Result<(), ParseError> {
        // eprintln!("start_table");
        *self = match self {
            ParseStatus::SearchingTable => ParseStatus::SearchingField,
            _ => {
                return Err(ParseError {
                    message: format!("invalid state {:?}", self),
                })
            }
        };
        Ok(())
    }

    pub fn start_field(&mut self) -> Result<(), ParseError> {
        // eprintln!("    start_field");
        *self = match self {
            ParseStatus::SearchingField => ParseStatus::SearchingCol1,
            _ => {
                return Err(ParseError {
                    message: format!("search field on state {:?}", self),
                })
            }
        };
        Ok(())
    }

    pub fn append_text(&mut self, text: &str) -> Result<(), ParseError> {
        // eprintln!("            append_text");
        fn push_str(s: &mut String, text: &str) -> Result<(), ParseError> {
            s.push_str(text);
            Ok(())
        }

        match self {
            ParseStatus::Col1(key) => push_str(key, text),
            ParseStatus::Col2(_, value) => push_str(value, text),
            _ => Err(ParseError {
                message: format!("append text on state {:?}", self),
            }),
        }
    }

    pub fn append_new_line(&mut self) -> Result<(), ParseError> {
        fn push_space(s: &mut String) -> Result<(), ParseError> {
            if !s.is_empty() {
                s.push(' ');
            }
            Ok(())
        }

        match self {
            ParseStatus::Col1(key) => push_space(key),
            ParseStatus::Col2(_, value) => push_space(value),
            _ => Err(ParseError {
                message: format!("append text on state {:?}", self),
            }),
        }
    }

    pub fn start_col(&mut self) -> Result<(), ParseError> {
        // eprintln!("        start_col");
        *self = match self {
            Self::SearchingCol1 => ParseStatus::Col1(String::new()),
            Self::SearchingCol2(key) => {
                let mut aux = String::new();
                swap(&mut aux, key);
                ParseStatus::Col2(aux, String::new())
            }
            _ => {
                return Err(ParseError {
                    message: format!("start col on status {:?}", self),
                })
            }
        };
        Ok(())
    }

    pub fn end_col(&mut self) -> Result<(), ParseError> {
        // eprintln!("        end_col");
        *self = match self {
            Self::Col1(key) => {
                let mut aux = String::new();
                swap(&mut aux, key);
                Self::SearchingCol2(aux)
            }
            Self::Col2(key, value) => {
                let mut aux_key = String::new();
                let mut aux_value = String::new();
                swap(&mut aux_key, key);
                swap(&mut aux_value, value);
                Self::Field(aux_key, aux_value)
            }
            _ => {
                return Err(ParseError {
                    message: format!("end col on status {:?}", self),
                })
            }
        };
        Ok(())
    }

    pub fn end_field(&mut self) -> Result<(String, String), ParseError> {
        // eprintln!("    end_field");
        let mut aux = ParseStatus::SearchingField;
        swap(&mut aux, self);

        match aux {
            Self::SearchingCol2(key) => {
                eprintln!("warning: key: '{}' has no value", key);
                Ok((key, Default::default()))
            }
            Self::Field(key, value) => Ok((key, value)),
            aux => {
                *self = aux;
                Err(ParseError {
                    message: format!("end field on status {:?}", self),
                })
            }
        }
    }

    pub fn end_table(&mut self) -> Result<(), ParseError> {
        // eprintln!("end_table");
        *self = match self {
            ParseStatus::SearchingField => ParseStatus::SearchingTable,
            _ => {
                return Err(ParseError {
                    message: format!("finish table on status {:?}", self),
                })
            }
        };
        Ok(())
    }

    pub fn finish(&mut self) -> Result<(), (String, String)> {
        let mut aux = Self::Finished;
        swap(&mut aux, self);

        match aux {
            ParseStatus::Col1(k) => Err((k, Default::default())),
            ParseStatus::SearchingCol2(k) => Err((k, Default::default())),
            ParseStatus::Col2(k, v) => Err((k, v)),
            ParseStatus::Field(k, v) => Err((k, v)),
            _ => Ok(()),
        }
    }

    pub fn is_finished(&self) -> bool {
        match self {
            ParseStatus::Finished => true,
            _ => false,
        }
    }
}
