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
                .next()
                .ok_or_else(|| Error::HtmlParseError)?
                .to_owned();

            println!("{}", title);
            unimplemented!()
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
    }
}
