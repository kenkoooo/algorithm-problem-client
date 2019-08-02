use crate::{Error, Result};

use super::*;
use scraper::element_ref::Select;
use scraper::{Html, Selector};

fn get_next_fold_text(x: &mut Select) -> Result<String> {
    Ok(x.next()
        .ok_or_else(|| Error::HtmlParseError)?
        .select(&Selector::parse("a").unwrap())
        .next()
        .ok_or_else(|| Error::HtmlParseError)?
        .text()
        .fold(String::new(), |s, t| s + t)
        .trim()
        .to_string())
}

fn scrape_max_page_count(html: &str) -> Result<u32> {
    Html::parse_document(html)
        .select(&Selector::parse("span.page-index").unwrap())
        .flat_map(|span| span.text().next())
        .flat_map(str::parse::<u32>)
        .max()
        .ok_or_else(|| Error::HtmlParseError)
}

fn scrape_problems(html: &str) -> Result<Vec<CodeforcesProblem>> {
    let table_selector = Selector::parse("table.problems").unwrap();
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
        .skip(1)
        .map(|tr| {
            let mut tds = tr.select(&td_selector);
            let id = get_next_fold_text(&mut tds)?;
            let title = get_next_fold_text(&mut tds)?;
            let difficulty = tds
                .nth(1)
                .ok_or_else(|| Error::HtmlParseError)?
                .text()
                .fold(String::new(), |s, t| s + t)
                .trim()
                .parse::<u32>()
                .ok();

            Ok(CodeforcesProblem {
                id,
                title,
                difficulty,
            })
        })
        .collect()
}

pub fn scrape(html: &str) -> Result<(Vec<CodeforcesProblem>, u32)> {
    let problems = scrape_problems(html)?;
    let count = scrape_max_page_count(html)?;
    Ok((problems, count))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_scrape() {
        let mut file = File::open("test_resources/codeforces_problemset").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let problems = scrape_problems(&contents).unwrap();
        assert_eq!(
            problems[8],
            CodeforcesProblem {
                id: "1197F".to_string(),
                title: "Coloring Game".to_string(),
                difficulty: Some(2500),
            }
        );
    }

    #[test]
    fn test_scrape_max_page_count() {
        let mut file = File::open("test_resources/codeforces_problemset").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let max_count = scrape_max_page_count(&contents).unwrap();
        assert_eq!(max_count, 53);
    }
}
