use std::{str::FromStr};
use strum::EnumString;
use lazy_static::lazy_static;
use regex::Regex;
use anyhow::{Result, bail, Context};

use serde::{Serialize, Deserialize};
use serde_json;

fn main() {
    println!("Hello, world!");

    let test_string = r#"@article{greenwade93,
        author  = "George D. Greenwade",
        title   = "The {C}omprehensive {T}ex {A}rchive {N}etwork ({CTAN})",
        year    = "1993",
        journal = "TUGBoat",
        volume  = "14",
        number  = "3",
        pages   = "342--351"
    }
    "#;

    println!("{:#?}", Bibliography::from_raw(test_string));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}



#[derive(Serialize, Deserialize, Debug)]
struct Bibliography {
    entrytype: EntryType,
    information: BibliographyInformation,
}

impl Bibliography {

    fn from_raw(input: &str) -> Result<Self> {
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
                let key = snailquote::unescape(line_splitted[0].clone().trim()).unwrap();
                let value = snailquote::unescape(line_splitted[1].clone().trim()).unwrap();

                data[key] = value.into();
            }
        }

        println!(" {} ", &data.dump());

        let deserialized: BibliographyInformation = serde_json::from_str(&data.dump()).unwrap();

        return deserialized;
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
struct BibliographyInformation {
    title: String,
    author: Option<String>,
    publisher: Option<String>,
    booktitle: Option<String>,
    chapter: Option<String>,
    edition: Option<String>,
    howpublished: Option<String>,
    institution: Option<String>,
    journal: Option<String>,
    month: Option<String>,
    note: Option<String>,
    number: Option<String>,
    organization: Option<String>,
    pages: Option<String>,
    school: Option<String>,
    series: Option<String>,
    volume: Option<String>,
    year: Option<String>
}


#[derive(Serialize, Deserialize, Debug, PartialEq, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
enum EntryType {
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