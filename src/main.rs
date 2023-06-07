use clap::{App, Arg};
use reqwest::blocking::get;
use serde::Deserialize;
use serde_json::{Result, Value};
use std::fmt;

#[derive(Debug, Deserialize)]
struct Paper {
    #[serde(rename = "__type")]
    #[serde(alias = "type")]
    bib_type: String,
    title: String,
    authors: Authors,
    venue: String,
    year: String,
    access: String,
    key: String,
    ee: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct Papers {
    items: Option<Vec<Paper>>,
}

#[derive(Debug, Deserialize)]
struct Authors {
    author: Vec<Author>,
}

#[derive(Debug, Deserialize)]
struct Author {
    #[serde(default, rename = "@pid")]
    pid: Option<String>,
    text: String,
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Bib Type: {}", self.bib_type)?;
        writeln!(f, "Title: {}", self.title)?;
        writeln!(f, "Authors: {}", self.authors)?;
        writeln!(f, "Venue: {}", self.venue)?;
        writeln!(f, "Year: {}", self.year)?;
        writeln!(f, "Access: {}", self.access)?;
        writeln!(f, "Key: {}", self.key)?;
        writeln!(f, "EE: {}", self.ee)?;
        writeln!(f, "URL: {}", self.url)?;
        Ok(())
    }
}

impl fmt::Display for Authors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for author in &self.author {
            write!(f, "{}, ", author.text)?;
        }
        Ok(())
    }
}

fn main() {
    let matches = App::new("DBLP Crawler")
        .version("0.1.0")
        .author("Benshan Mei, Institute of Information Engineering")
        .about("Crawls DBLP database")
        .arg(
            Arg::with_name("query_type")
                .short("t")
                .long("type")
                .value_name("TYPE")
                .help("Sets the query type")
                .takes_value(true)
                .possible_values(&["publ", "author", "venue"])
                .required(true),
        )
        .arg(
            Arg::with_name("query")
                .short("q")
                .long("q")
                .value_name("QUERY")
                .help("Sets the query string to search for, as described on a separate page.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .value_name("FORMAT")
                .help("Sets the result format of the search. Recognized values are \"xml\" and \"json\".")
                .takes_value(true)
                .default_value("json")
                .possible_values(&["xml", "json"]),
        )
        .arg(
            Arg::with_name("hits")
                .short("h")
                .long("hits")
                .value_name("HITS")
                .help("Sets the maximum number of search results (hits) to return. For bandwidth reasons, this number is capped at 1000.")
                .takes_value(true)
                .default_value("30"),
        )
        .arg(
            Arg::with_name("first")
                .short("i")
                .long("first")
                .value_name("FIRST")
                .help("Sets the first hit in the numbered sequence of search results (starting with 0) to return. In combination with the h parameter, this parameter can be used for pagination of search results.")
                .takes_value(true)
                .default_value("0"),
        )
        .arg(
            Arg::with_name("completion")
                .short("c")
                .long("completion")
                .value_name("COMPLETION")
                .help("Sets the maximum number of completion terms (see below) to return. For bandwidth reasons, this number is capped at 1000.")
                .takes_value(true)
                .default_value("10"),
        )
        .get_matches();

    let query_type = matches.value_of("query_type").unwrap();
    let query_string = matches.value_of("query").unwrap();
    let format = matches.value_of("format").unwrap();
    let hits = matches.value_of("hits").unwrap();
    let first = matches.value_of("first").unwrap();
    let completion = matches.value_of("completion").unwrap();

    let build_url = || {
        format!(
            "https://dblp.org/search/{query_type}/api?q={query_string}&format={format}&h={hits}&f={first}&c={completion}",
            query_type = query_type,
            query_string = query_string,
            format = format,
            hits = hits,
            first = first,
            completion = completion,
        )
    };

    let handle_result = |url: &str| -> Result<()> {
        let response = get(url)?;
        let body: Value = response.json()?;
        let papers: Papers = serde_json::from_value(body)?;

        if let Some(items) = papers.items {
            for paper in items {
                println!("{}", paper);
                println!("-----------------------------");
            }
        } else {
            println!("No papers found");
        }

        Ok(())
    };

    let result = match query_type {
        "publ" => handle_result(&build_url()),
        "author" => handle_result(&build_url()),
        "venue" => handle_result(&build_url()),
        _ => panic!("Invalid query type"),
    };

    if let Err(err) = result {
        eprintln!("Error: {:?}", err);
    }
}