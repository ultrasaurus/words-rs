use words::html_words;

fn main() -> anyhow::Result<()> {
    let text = "2 much is never enough 4me... let's see how punctuation works?";
    let html_string = html_words(text)?;

    println!("-----");
    println!("{}", html_string);

    println!("-----");

    Ok(())
}
