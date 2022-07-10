use anyhow::{bail, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs;
use std::str::FromStr;
use strum::{EnumString, Display};

use serde::{Deserialize, Serialize};
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
        let entrytype = Self::get_entry_type(input).context("Failed to extract the entry type!")?;

        let information = Self::get_bibliography_informations(input)
            .context("Failed to extract the entry type!")?;

        Ok(Self {
            entrytype,
            information,
        })
    }

    pub fn get_from_bib_file(filename: &str) -> Vec<Result<Bibliography>> {
        let contents = fs::read_to_string(filename).expect("failed to tead file");

        let mut splited = contents.split("@").collect::<Vec<&str>>();
        splited.remove(0);

        let bibliography_list = splited
            .into_iter()
            .map(|f| Bibliography::from_raw(f))
            .collect::<Vec<Result<Bibliography>>>();
        return bibliography_list;
    }

    fn get_bibliography_informations(input: &str) -> Result<BibliographyInformation> {
        let mut data = json::JsonValue::new_object();

        let splitted_result = input.lines();

        for line in splitted_result {
            if line == "\"\"" {
                continue;
            } 

            let line_splitted = line.split(" = ").collect::<Vec<&str>>();
            if line_splitted.len() != 1 {
                let key = snailquote::unescape(line_splitted[0].clone().trim())
                    .unwrap()
                    .to_uppercase();
                let value = Self::clean_string(line_splitted[1].clone());

                data[key] = value.into();
            }
        }

        let deserialized: BibliographyInformation = serde_json::from_str(&data.dump())?;

        return Ok(deserialized);
    }

    fn clean_string(input: &str) -> String {
        // Unescape and Trim
        let unescaped = match snailquote::unescape(input.trim()) {
            Ok(s) => s,
            Err(_) => "".to_string(),
        };
        // Remove Parenthesis
        return unescaped.replace("{", "").replace("}", "");
    }

    fn get_authors(input: &str) -> Author {
        todo!()
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
            Err(_) => bail!(
                "Provided Entrytype is not in the list of known entry types. Given {}",
                entry_type_raw
            ),
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
    pub year: Option<String>,
    #[serde(alias = "abstract")]
    pub abstract_: Option<String>,
    pub doi: Option<String>,
    pub isbn: Option<String>,
    pub issn: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, EnumString, Display)]
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

struct Author {
    name: String,
    surname: String,
    middelname: Option<String>,
}