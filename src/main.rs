mod model;
use crate::model::bibliography;

fn main() {
    println!("Hello, world!");
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



