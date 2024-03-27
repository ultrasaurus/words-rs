use regex::Regex;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// source text 
    #[clap(short, long, value_parser, default_value = "Hello world!")]
    text: String,
}

fn html_words(text: String) -> anyhow::Result<String> {
    let regex = Regex::new(r"([a-zà-ýA-ZÀ-Ý0-9]+?)([[\s$][^a-zà-ýA-ZÀ-Ý0-9]]+)")?;
    let mut nth_word = 0;
    let html_string = regex.captures_iter(&text).map(|c| {
        println!("{:?}", c);
        let range: std::ops::Range<usize> = c.get(0).unwrap().range();
        let s = format!("<span word='{}' char='{}'>{}</span>{}", 
                        nth_word, range.start, &c[1], &c[2]);
        nth_word = nth_word + 1;
        s
    }).collect::<Vec<String>>().join("");
    Ok(html_string)    
}
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let html_string = html_words(cli.text)?;

    println!("-----");
    println!("{}", html_string);

    println!("-----");

    Ok(())
}
