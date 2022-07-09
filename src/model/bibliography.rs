use std::{str::FromStr};
use strum::EnumString;
use lazy_static::lazy_static;
use regex::Regex;
use anyhow::{Result, bail, Context};

use serde::{Serialize, Deserialize};
use serde_json;


#[derive(Serialize, Deserialize, Debug)]
pub struct Bibliography {
    pub entrytype: EntryType,
    pub information: BibliographyInformation,
}

impl Bibliography {

    /// from_raw
    /// The function from_raw allows you to extract Bibliography Inforamtion from a bib-tex file.
    /// 
    /// # Errors
    /// ...
    /// This function will return an error if .
    pub fn from_raw(input: &str) -> Result<Self> {
        let entrytype = Self::get_entry_type(input)
            .context("Failed to extract the entry type!")?;

        let information = Self::get_bibliography_informations(input);

        Ok(Self{entrytype, information })
    }

    fn get_bibliography_informations(input: &str) -> BibliographyInformation {
        let mut data = json::JsonValue::new_object();

        let splitted_result = input.split(',');

        for line in splitted_result {
            let line_splitted = line.split(" = ").collect::<Vec<&str>>();
            if line_splitted.len() != 1 {
                let key = snailquote::unescape(line_splitted[0].clone().trim()).unwrap().to_uppercase();
                let value = Self::clean_string(line_splitted[1].clone());

                data[key] = value.into();
            }
        }

        let deserialized: BibliographyInformation = serde_json::from_str(&data.dump()).unwrap();

        return deserialized;
    }

    fn clean_string(input: &str) -> String {
        // Unescape and Trim
        let unescaped = snailquote::unescape(input.trim()).unwrap();
        // Remove Parenthesis
        return unescaped.replace("{", "").replace("}", "");

    }


    fn get_entry_type(input: &str) -> Result<EntryType> {
        // Layz static allows rust to compile the regex term only once.
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"\w+"#).unwrap();
        }
        let capture = RE.captures(input).unwrap();
    
        // The first value 0 is equal to the entry type. Uppercase to improve the matching process
        let entry_type_raw = capture.get(0).map_or("", |m| m.as_str());
        let entry_type_raw = &entry_type_raw.to_uppercase();
    
        return match EntryType::from_str(entry_type_raw) {
            Ok(s) => Ok(s),
            Err(_) => bail!("Provided Entrytype is not in the list of known entry types. Given {}", entry_type_raw)
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct BibliographyInformation {
    pub title: String,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub booktitle: Option<String>,
    pub chapter: Option<String>,
    pub edition: Option<String>,
    pub howpublished: Option<String>,
    pub institution: Option<String>,
    pub journal: Option<String>,
    pub month: Option<String>,
    pub note: Option<String>,
    pub number: Option<String>,
    pub organization: Option<String>,
    pub pages: Option<String>,
    pub school: Option<String>,
    pub series: Option<String>,
    pub volume: Option<String>,
    pub year: Option<String>
}


#[derive(Serialize, Deserialize, Debug, PartialEq, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
pub enum EntryType {
    Book,
    Article,
    Booklet,
    Conference,
    InBook,
    InCollection,
    InProceedings,
    Manual,
    Masterthesis,
    Misc,
    Phdthessis,
    Proceedings,
    Techreport,
    Unpublished,
    #[strum(serialize = "")]
    NoneType,
}