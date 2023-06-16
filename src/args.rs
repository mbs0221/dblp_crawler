use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "DBLP Crawler 0.1.0", about = "Crawls DBLP database")]
struct Opt {
    #[structopt(short = "c", long = "completion", default_value = "10")]
    completion: usize,

    #[structopt(short = "i", long = "first", default_value = "0")]
    first: usize,

    #[structopt(short = "f", long = "format", default_value = "json", possible_values = &["xml", "json"])]
    format: String,

    #[structopt(short = "h", long = "hits", default_value = "30")]
    hits: usize,

    #[structopt(short = "q", long = "q")]
    query: String,

    #[structopt(short = "t", long = "type", possible_values = &["publ", "author", "venue"])]
    r#type: String,
}