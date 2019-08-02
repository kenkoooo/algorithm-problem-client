use crate::{Error, Result};

use super::*;
use scraper::{Html, Selector};

pub fn scrape(html: &str) -> Result<Vec<CodeChefProblem>> {
    let table_selector = Selector::parse("table.dataTable").unwrap();
    let tbody_selector = Selector::parse("tbody").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    Html::parse_document(html)
        .select(&table_selector)
        .next()
        .ok_or_else(|| Error::HtmlParseError)?
        .select(&tbody_selector)
        .next()
        .ok_or_else(|| Error::HtmlParseError)?
        .select(&tr_selector)
        .map(|tr| {
            let mut tds = tr.select(&td_selector);
            let title = tds
                .next()
                .ok_or_else(|| Error::HtmlParseError)?
                .text()
                .fold(String::new(), |title, part| title + part)
                .trim()
                .to_string();
            let code = tds
                .next()
                .ok_or_else(|| Error::HtmlParseError)?
                .text()
                .fold(String::new(), |a, part| a + part);
            let successful_counts = tds
                .next()
                .ok_or_else(|| Error::HtmlParseError)?
                .text()
                .fold(String::new(), |a, part| a + part)
                .parse::<u32>()?;
            Ok(CodeChefProblem {
                title,
                code,
                successful_counts,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_scrape() {
        let mut file = File::open("test_resources/code_chef_problems").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let problems = scrape(&contents).unwrap();
        assert_eq!(problems.len(), 8186);
        assert_eq!(
            problems[0],
            CodeChefProblem {
                title: "Getting Rid of the Holidays (Act I)".to_owned(),
                code: "HOLIDAY1".to_owned(),
                successful_counts: 0
            }
        );
    }
}
