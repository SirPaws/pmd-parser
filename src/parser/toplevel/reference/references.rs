use chrono::{DateTime, TimeZone, Utc};

use anyhow::{anyhow, Context, Result};
#[cfg(not(feature = "wasm"))]
use color_print::cprintln;

use super::structured_base_parser::{peek_next_token, eat_token, parse_value};

#[derive(Debug, Clone, PartialEq)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
impl Month {
    pub fn to_string(&self) -> &str {
        match self {
           Month::January   => "january",
           Month::February  => "february",
           Month::March     => "march",
           Month::April     => "april",
           Month::May       => "may",
           Month::June      => "june",
           Month::July      => "july",
           Month::August    => "august",
           Month::September => "september",
           Month::October   => "october",
           Month::November  => "november",
           Month::December  => "december",
        }
    }

    pub fn to_chrono_month(&self) -> chrono::Month {
        match self {
           Month::January   => chrono::Month::January,
           Month::February  => chrono::Month::February,
           Month::March     => chrono::Month::March,
           Month::April     => chrono::Month::April,
           Month::May       => chrono::Month::May,
           Month::June      => chrono::Month::June,
           Month::July      => chrono::Month::July,
           Month::August    => chrono::Month::August,
           Month::September => chrono::Month::September,
           Month::October   => chrono::Month::October,
           Month::November  => chrono::Month::November,
           Month::December  => chrono::Month::December,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum PmdDate {
    #[default] None,
    String(String),
    Split{ day: Option<u32>, month: Option<Month>, year: Option<u32>}
}

fn get_month_from_string(word: &str) -> Option<Month> {
    match word.to_lowercase().as_str() {
        "january"   | "jan" => Some(Month::January),
        "febuary"   | "february"  | "feb" => Some(Month::February),
        "march"     | "mar" => Some(Month::March),
        "april"     | "apr" => Some(Month::April),
        "may"               => Some(Month::May),
        "june"      | "jun" => Some(Month::June),
        "july"      | "jul" => Some(Month::July),
        "august"    | "aug" => Some(Month::August),
        "september" | "sep" => Some(Month::September),
        "october"   | "oct" => Some(Month::October),
        "november"  | "nov" => Some(Month::November),
        "december"  | "dec" => Some(Month::December),
        _                   => None,
    }
}

fn is_number(word: &str) -> bool {
    for character in word.chars() {
        if !character.is_digit(10) { return false; }
    }
    return true;
}

fn get_day_from_string(word: &str) -> Option<u32> {
    if is_number(word) && let Ok(n) = word.parse::<usize>() {
        if n > 31 { None } else { Some(n as u32) }
    } else {
        None
    }
}

fn get_year_from_string(word: &str) -> Option<u32> {
    if is_number(word) && let Ok(n) = word.parse::<usize>() {
        if n > (u32::MAX as usize) { None } else { Some(n as u32) }
    } else {
        None
    }
}

fn get_time_from_string(text: &str) -> (Option<u32>, Option<Month>, Option<u32>) {
    let mut day   : Option<u32>   = None;
    let mut month : Option<Month> = None;
    let mut year  : Option<u32>   = None;
    
    let text = text.replace(",", " ");
    let words = if text.contains("/") { text.split("/").collect::<Vec<_>>() } else { text.split_whitespace().collect::<Vec<_>>() };
    for word in words {
        if let Some(found_month)  = get_month_from_string(&word) { month = Some(found_month); continue }
        if let Some(found_day)    = get_day_from_string(&word)   { day   = Some(found_day)  ; continue }
        if let Some(found_year)   = get_year_from_string(&word)  { year  = Some(found_year) ; continue }
    }

    (day, month, year)
}

impl PmdDate {

    pub fn get_day(&self) -> Option<u32> {
        match self {
            PmdDate::None => { None },
            PmdDate::String(x) => {
                let (day, _month, _year) = get_time_from_string(x);
                day
            },
            PmdDate::Split { day, month: _, year: _ } => { day.clone() },
        }
    }
    
    pub fn get_month(&self) -> Option<Month> {
        match self {
            PmdDate::None => { None },
            PmdDate::String(x) => {
                let (_day, month, _year) = get_time_from_string(x);
                month
            },
            PmdDate::Split { day: _, month, year: _ } => { month.clone() },
        }
    }
    
    pub fn get_year(&self) -> Option<u32> {
        match self {
            PmdDate::None => { None },
            PmdDate::String(x) => {
                let (_day, _month, year) = get_time_from_string(x);
                year
            },
            PmdDate::Split { day: _, month: _, year } => { year.clone() },
        }
    }

    pub fn split_date(&self) -> (Option<u32>, Option<Month>, Option<u32>) {
        ( self.get_day(), self.get_month(), self.get_year() )
    }

    pub fn is_not_none(&self) -> bool {
        match self {
            PmdDate::None => false,
            _ => true
        }
    }
    
    pub fn is_none(&self) -> bool {
        match self {
            PmdDate::None => true,
            _ => false
        }
    }

    pub fn to_date(&self) -> Option<DateTime<Utc>> {
        if self.is_none() { return None }

        let maybe_year  = self.get_year();
        let maybe_month = self.get_month();
        let maybe_day   = self.get_day();
        
        let year  = maybe_year.unwrap_or(2024);
        let month = maybe_month.unwrap_or(Month::January);
        let day   = maybe_day.unwrap_or(1);

        #[cfg(not(feature = "wasm"))]
        cprintln!("<cyan>debug:</> got date {} {:?} {}", year, month, day);

        let month = month.to_chrono_month().number_from_month();

        chrono::Utc.with_ymd_and_hms(year as i32, month, day, 0, 0, 0).earliest()
    }
}


#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReferenceDefinition {
    pub id: String,
    pub authors: Vec<String>,
    pub editors: Vec<String>,
    pub translators: Vec<String>,
    pub title: String,
    pub description: String,
    pub container_title: String,
    pub publisher: String,
    pub date: PmdDate,
    pub date_retrieved: PmdDate,
    pub volume:  String,
    pub edition: String,
    pub version: String,
    pub issue: String,
    pub pages: String,
    pub link: String,
    pub doi: String,
    pub esbn: String,
}

fn parse_ref_day(text: &str)      -> (String, u32) { 
    if let Some(token) = peek_next_token(text) {
        if let Some(day) = get_day_from_string(token.as_str()) {
            (eat_token(text, token.as_str()), day)
        } else {
            (eat_token(text, token.as_str()), 0)
        }
    } else {
        (text.to_string(), 0)
    }
}

fn parse_ref_month(text: &str)    -> (String, Month) {
    if let Some(token) = peek_next_token(text) {
        if let Some(month) = get_month_from_string(token.as_str()) {
            (eat_token(text, token.as_str()), month)
        } else {
            (eat_token(text, token.as_str()), Month::January)
        }
    } else {
        (text.to_string(), Month::January)
    }
}

fn parse_ref_year(text: &str)     -> (String, u32) { 
    if let Some(token) = peek_next_token(text) {
        if let Some(year) = get_year_from_string(token.as_str()) {
            (eat_token(text, token.as_str()), year)
        } else {
            (eat_token(text, token.as_str()), 0)
        }
    } else {
        (text.to_string(), 0)
    }
}

fn parse_ref_date(text: &str)     -> (String, String) {
    let (buf, result) = parse_value(text);
    (buf, if result.is_empty() {
        "".to_string()
    } else {
        result[0].clone()
    })
}

// fn update_date() 

pub fn parse_reference(content: String) -> Result<ReferenceDefinition> {
    let current = content.lines().nth(0).context("expected a line where there was none")?;

    if current.starts_with("£") && current.chars().nth(1).is_some_and(|c| return char::is_alphabetic(c) || c == '-') {
        let mut buf = content.clone();
        let id = if let Some(token) = peek_next_token(&buf) { token } else { return Err(anyhow!("missing name")); };
        buf = eat_token(&buf, &id);
        let mut reference: ReferenceDefinition = ReferenceDefinition::default();
        reference.id = id[2..].to_string();

        if let Some(token) = peek_next_token(&buf) && token == "{" {
            buf = eat_token(&buf, "{");

            loop {
                let opt_token = peek_next_token(&buf);
                if opt_token.is_none() { break }

                let token = opt_token.expect("token was None somehow");
                if token == "}" { break }

                let ident = token.clone();

                buf = eat_token(&buf, &token);
                let opt_token = peek_next_token(&buf);
                if opt_token.is_none() { break }

                let token = opt_token.expect("token was None somehow");
                if token == "}" { break }
                if token != ":" { break }

                buf = eat_token(&buf, &token);

                match ident.to_lowercase().as_str() {
                    "authors" | "author" => {
                        let authors;
                        (buf, authors) = parse_value(&buf);
                        for author in authors {
                            reference.authors.push(author);
                        }
                    },
                    "editors" | "editor" => {
                        let editors;
                        (buf, editors) = parse_value(&buf);
                        for editor in editors {
                            reference.editors.push(editor);
                        }
                    },
                    "translators" | "translator" => {
                        let translators;
                        (buf, translators) = parse_value(&buf);
                        for translator in translators {
                            reference.translators.push(translator);
                        }
                    },
                    "title" | "description" | "container-title" | 
                    "publisher" | "edition" | "version" | "issue" | 
                    "volume" | "pages" | "link" | "doi" | "esbn"
                        => {
                        let value;
                        (buf, value) = parse_value(&buf);
                        if !value.is_empty() {
                            match ident.to_lowercase().as_str() {
                            "title"           => reference.title           = value[0].clone(),
                            "description"     => reference.description     = value[0].clone(),
                            "container-title" => reference.container_title = value[0].clone(),
                            "publisher"       => reference.publisher       = value[0].clone(),
                            "edition"         => reference.edition         = value[0].clone(),
                            "version"         => reference.version         = value[0].clone(),
                            "issue"           => reference.issue           = value[0].clone(),
                            "volume"          => reference.volume          = value[0].clone(),
                            "pages"           => reference.pages           = value[0].clone(),
                            "link"            => reference.link            = value[0].clone(),
                            "doi"             => reference.doi             = value[0].clone(),
                            "esbn"            => reference.esbn            = value[0].clone(),
                            _                 => {},
                            }
                        }
                    },
                    "date"              => {
                        let date;
                        (buf, date) = parse_ref_date(&buf);
                        reference.date = PmdDate::String(date);
                    },
                    "day"               => {
                        let day;
                        (buf, day) = parse_ref_day(&buf);
                        if reference.date.get_day().is_none() {
                            let (_, month, year) = reference.date.split_date();
                            reference.date = PmdDate::Split{day: Some(day), month, year};
                        }
                    },
                    "month"             => {
                        let month;
                        (buf, month) = parse_ref_month(&buf);
                        if reference.date.get_month().is_none() {
                            let (day, _, year) = reference.date.split_date();
                            reference.date = PmdDate::Split{day, month: Some(month), year};
                        }
                    },
                    "year"              => {
                        let year;
                        (buf, year) = parse_ref_year(&buf);
                        if reference.date.get_year().is_none() {
                            let (day, month, _) = reference.date.split_date();
                            reference.date = PmdDate::Split{day, month, year: Some(year)};
                        }
                    },
                    "date-retrieved"    => {
                        let date;
                        (buf, date) = parse_ref_date(&buf);
                        reference.date_retrieved = PmdDate::String(date);
                    },
                    "day-retrieved"     => {
                        let day;
                        (buf, day) = parse_ref_day(&buf);
                        if reference.date.get_day().is_none() {
                            let (_, month, year) = reference.date.split_date();
                            reference.date_retrieved = PmdDate::Split{day: Some(day), month, year};
                        }
                    },
                    "month-retrieved"   => {
                        let month;
                        (buf, month) = parse_ref_month(&buf);
                        if reference.date.get_month().is_none() {
                            let (day, _, year) = reference.date.split_date();
                            reference.date_retrieved = PmdDate::Split{day, month: Some(month), year};
                        }
                    },
                    "year-retrieved"    => {
                        let year;
                        (buf, year) = parse_ref_year(&buf);
                        if reference.date.get_year().is_none() {
                            let (day, month, _) = reference.date.split_date();
                            reference.date_retrieved = PmdDate::Split{day, month, year: Some(year)};
                        }
                    },
                    _ => break
                }

                if peek_next_token(&buf).is_some_and(|x| x == ",") {
                    buf = eat_token(&buf, ",");
                }
            }

            peek_next_token(&buf).expect("expected '}'");
            // content = eat_token(&buf, "}");
            Ok(reference)
        } else { return Err(anyhow!("expected '{{'")) }
    } else { Err(anyhow!("not a valid reference")) }
}

pub fn to_citation(value: &ReferenceDefinition) -> String {
    
    let mut result : String = "(".to_string();
    if value.authors.len() == 0 { return "(INVALID REFERENCE)".to_string() }
    match value.authors.len() {
        1 => {
            let author = value.authors[0].clone();
            let split_name : Vec<_> = author.split_whitespace().collect();
            result += split_name.last().unwrap();
            result += ", "; 
        },
        2 => {
            let first_author  = value.authors[0].clone();
            let second_author = value.authors[1].clone();
            let first_author_split_name : Vec<_>  = first_author.split_whitespace().collect();
            let second_author_split_name : Vec<_> = second_author.split_whitespace().collect();
            result += first_author_split_name.last().unwrap();
            result += " & "; 
            result += second_author_split_name.last().unwrap();
            result += ", "; 

        },
        _ => {
            let author = value.authors[0].clone();
            let split_name : Vec<_> = author.split_whitespace().collect();
            result += split_name.last().unwrap();
            result += " et al., "; 
        },
    }

    if let Some(year) = value.date.get_year() {
        result += year.to_string().as_str();
    }

    result += ")";

    result
}

pub fn bibliograph_name(name: &String) -> String {
    let split_name : Vec<_> = name.split_whitespace().collect();
    let last_name = *split_name.last().unwrap();
    let rest_of_name = &split_name[0..split_name.len() - 1];

    let mut result = last_name.to_string();
    result += ", ";

    for (n, name) in rest_of_name.iter().enumerate() {
        result.push(name.chars().nth(0).unwrap());
        result.push('.');
        if n != rest_of_name.len() - 1 {
            result.push(' ');
        }
    }

    result
}

pub fn any_non_empty(strings: &[&String]) -> bool {
    for string in strings {
        if !string.is_empty() { return true }
    }
    false
}

pub fn to_bibliography(value: &ReferenceDefinition) -> String {
    
    let mut result = "".to_string();

    for (n, author) in value.authors.iter().enumerate() {
        let name = bibliograph_name(&author);
        result += name.as_str();
        if n != value.authors.len() - 1 { result += ", " }
    }


    result += " (";
    let date = &value.date;
    if let Some(year) = date.get_year() {
        result += year.to_string().as_str();
    }
    
    if let Some(month) = date.get_month() {
        result.push(',');
        result.push(' ');
        result += month.to_string();
        result.push(' ');
    }

    if let Some(day) = date.get_day() {
        result += day.to_string().as_str();
    }
    result.push(')');
    result.push('.');
    result.push(' ');

    result.push('"');
    result += value.title.as_str();
    result.push('.');
    result.push('"');
    if !value.description.is_empty() {
        result.push(' ');
        result.push('[');
        result += value.description.as_str();
        result.push(']');
    }

    if value.editors.len() != 0 {
        result += " edited by ";
        if value.editors.len() == 1 {
            let name = &value.editors[0];
            result += name.as_str();
            result.push('.');
        } else {
            for (n, author) in value.editors.iter().enumerate() {
                let name = bibliograph_name(&author);
                result += name.as_str();
                if n != value.editors.len() - 1 { result += ", " }
            }
        }
    }
    
    if value.translators.len() != 0 {
        result += ", translated by ";
        if value.translators.len() == 1 {
            let name = &value.translators[0];
            result += name.as_str();
            result.push('.');
        } else {
            for (n, author) in value.translators.iter().enumerate() {
                let name = bibliograph_name(&author);
                result += name.as_str();
                if n != value.translators.len() - 1 { result += ", " }
            }
        }
    }

    if !value.container_title.is_empty() {
        result += " in ";
        result += value.container_title.as_str();
    }

    if any_non_empty(&[&value.volume, &value.issue, &value.pages, &value.edition, &value.version]) {
        result.push(' ');
        result.push('(');

        let mut found_one = false;
        if !value.version.is_empty() {
            result += value.version.as_str();
            result += " vers.";
            found_one = true;
        }
        if !value.edition.is_empty() {
            if found_one {
                result += ", "
            }
            result += value.edition.as_str();
            result += " ed.";
            found_one = true;
        }
        if !value.volume.is_empty() {
            if found_one {
                result += ", "
            }
            result += value.volume.as_str();
            result += " vol.";
            found_one = true;
        }
        if !value.issue.is_empty() {
            if found_one {
                result += ", "
            }
            result += "issue ";
            result += value.issue.as_str();
            found_one = true;
        }
        if !value.pages.is_empty() {
            if found_one {
                result += ", "
            }
            result += "pp. ";
            result += value.pages.as_str();
        }
    
        result.push(')');
    }
    result.push('.');

    if !value.publisher.is_empty() {
        result.push(' ');
        result += value.publisher.as_str();
        result.push('.');
    }

    let mut has_link = false;
    if value.link == value.doi && !value.link.is_empty() {
        result.push(' ');
        result += value.link.as_str();
        has_link = true;
    } else {
        if !value.doi.is_empty() {
            result.push(' ');
            if !value.link.is_empty() {
                result += "doi: ";
            }
            result += value.doi.as_str();
            has_link = true;
        }

        if !value.link.is_empty() {
            result += ", ";
            result += value.link.as_str();
            has_link = true;
        }
    }

    if !value.esbn.is_empty() {
        if has_link {
            result += ", ";
        } else {
            result.push(' ');
        }

        result += "esbn: ";
        result += value.esbn.as_str();
    }

    if value.date_retrieved.is_not_none() {
        result += " accessed ";
        if let Some(year) = date.get_year() {
            result += year.to_string().as_str();
        }
        
        if let Some(month) = date.get_month() {
            result.push(',');
            result.push(' ');
            result += month.to_string();
            result.push(' ');
            result += " ";
        }
        
        if let Some(day) = date.get_day() {
            result += day.to_string().as_str();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_reference_declaration_simple() {
        let example_ref: String = "£example { 
            title: Simulacra and Simulation,
            author: Jean Baudrillard,
            publisher: University of Michigan Press,
            year: 1994,
            pages: 176,
            esbn: 0-472-06521-1,
        }".to_string();
        
        let example = parse_reference(example_ref);

        assert!(example.is_ok());
        assert_eq!(example.unwrap(), ReferenceDefinition{ 
            id: "example".into(), 
            authors: vec!["Jean Baudrillard".into()],
            editors: vec![],
            translators: vec![],
            title: "Simulacra and Simulation".into(),
            description: "".into(),
            container_title: "".into(),
            publisher: "University of Michigan Press".into(),
            date: PmdDate::Split{ day: None, month: None, year: Some(1994) },
            date_retrieved: PmdDate::None,
            volume: "".into(),
            edition: "".into(),
            version: "".into(),
            issue: "".into(),
            pages: "176".into(),
            link: "".into(),
            doi: "".into(),
            esbn: "0-472-06521-1".into()
        });
    }
    
    #[test]
    fn test_parse_reference_declaration_two_authors() {
        let example_ref: String = "£example { 
            title: Simulacra and Simulation,
            authors: [Jean Baudrillard, Henry Ford],
            publisher: University of Michigan Press,
            year: 1994,
            pages: 176,
            esbn: 0-472-06521-1,
        }".to_string();
        
        let example = parse_reference(example_ref);

        assert!(example.is_ok());
        assert_eq!(example.unwrap(), ReferenceDefinition{ 
            id: "example".into(), 
            authors: vec!["Jean Baudrillard".into(), "Henry Ford".into()],
            editors: vec![],
            translators: vec![],
            title: "Simulacra and Simulation".into(),
            description: "".into(),
            container_title: "".into(),
            publisher: "University of Michigan Press".into(),
            date: PmdDate::Split{ day: None, month: None, year: Some(1994) },
            date_retrieved: PmdDate::None,
            volume: "".into(),
            edition: "".into(),
            version: "".into(),
            issue: "".into(),
            pages: "176".into(),
            link: "".into(),
            doi: "".into(),
            esbn: "0-472-06521-1".into()
        });
    }
    
    #[test]
    fn test_parse_reference_declaration_multiple_authors_misspelling() {
        let example_ref: String = "£example { 
            title: Simulacra and Simulation,
            author: [Jean Baudrillard, Henry Ford],
            publisher: University of Michigan Press,
            year: 1994,
            pages: 176,
            esbn: 0-472-06521-1,
        }".to_string();
        
        let example = parse_reference(example_ref);

        assert!(example.is_ok());
        assert_eq!(example.unwrap(), ReferenceDefinition{ 
            id: "example".into(), 
            authors: vec!["Jean Baudrillard".into(), "Henry Ford".into()],
            editors: vec![],
            translators: vec![],
            title: "Simulacra and Simulation".into(),
            description: "".into(),
            container_title: "".into(),
            publisher: "University of Michigan Press".into(),
            date: PmdDate::Split{ day: None, month: None, year: Some(1994) },
            date_retrieved: PmdDate::None,
            volume: "".into(),
            edition: "".into(),
            version: "".into(),
            issue: "".into(),
            pages: "176".into(),
            link: "".into(),
            doi: "".into(),
            esbn: "0-472-06521-1".into()
        });
    }

    #[test]
    fn test_parse_reference_declaration_complex() {
        let ada_ref : String = "£Ada-Cheung {
            authors: [ Ada S Cheung, Sav Zwickl, Kirsti Miller, Brendan J Nolan, Alex Fang Qi Wong, Patrice Jones, Nir Eynon ],
            title: The Impact of Gender-Affirming Hormone Therapy on Physical Performance,
            container-title: The Journal of Clinical Endocrinology & Metabolism,
            description: A paper on the impact of Gender-Affirming Hormone Therapy,
            publisher: Oxford Academic,
            volume: 109,
            issue: 2,
            pages: e455-e465,
            year: 2023,
            month: july,
            day:   13,
            doi: https://doi.org/10.1210/clinem/dgad414
        }
        ".to_string();
        
        let ada  = parse_reference(ada_ref);

        assert!(ada.is_ok());
        assert_eq!(ada.unwrap(), ReferenceDefinition{ 
            id: "Ada-Cheung".into(), 
            authors: vec![
                "Ada S Cheung".into(),
                "Sav Zwickl".into(),
                "Kirsti Miller".into(),
                "Brendan J Nolan".into(),
                "Alex Fang Qi Wong".into(),
                "Patrice Jones".into(),
                "Nir Eynon".into()
            ],
            editors: vec![],
            translators: vec![],
            title: "The Impact of Gender-Affirming Hormone Therapy on Physical Performance".into(),
            description: "A paper on the impact of Gender-Affirming Hormone Therapy".into(),
            container_title: "The Journal of Clinical Endocrinology & Metabolism".into(),
            publisher: "Oxford Academic".into(),
            date: PmdDate::Split{ day: Some(13), month: Some(Month::July), year: Some(2023) },
            date_retrieved: PmdDate::None,
            volume: "109".into(),
            edition: "".into(),
            version: "".into(),
            issue: "2".into(),
            pages: "e455-e465".into(),
            link: "".into(),
            doi: "https://doi.org/10.1210/clinem/dgad414".into(),
            esbn: "".into()
        });
    }
}
