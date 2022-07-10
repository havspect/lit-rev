use anyhow::Result;
use sqlx::sqlite::SqlitePoolOptions;

mod model;
use crate::model::bibliography::Bibliography;

#[async_std::main]
async fn main() -> Result<()> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("research.db").await?;

    let vec_results = Bibliography::get_from_bib_file("ma.bib");

    let vec_results_ok = vec_results.into_iter()
        .filter(|f| f.is_ok())
        .map(|f| f.unwrap())
        .collect::<Vec<Bibliography>>();


    for entry in vec_results_ok.into_iter() {
        let entrytype_string = entry.entrytype.to_string();

        if entry.information.author.is_some() {
            let author = entry.information.author.unwrap();
            sqlx::query!(
                r#"INSERT INTO bibliography (ENTRY_TYPE, AUTHOR, TITLE) 
                VALUES (?1, ?2, ?3) 
            "#, entrytype_string ,author, entry.information.title)
                .execute(&pool)
                .await
                .ok();
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::model::bibliography::{Bibliography, EntryType};

    #[test]
    fn check_raw_function() {
        let test_string = r#"@article{greenwade93,
            author  = {George D. Greenwade},
            title   = "The {C}omprehensive {T}ex {A}rchive {N}etwork ({CTAN})",
            year    = "1993",
            journal = "TUGBoat",
            volume  = "14",
            number  = "3",
            pages   = "342--351"
        }
        "#;

        let result = Bibliography::from_raw(test_string).unwrap();
        assert_eq!(result.entrytype, EntryType::Article);
        assert_eq!(result.information.author, Some("George D. Greenwade".to_string()));
    }
}



