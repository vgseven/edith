use reqwest;
use scraper::{Html, Selector};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://vgseven.com/projects/";

    let res = reqwest::get(url).await?;

    let body = res.text().await?;

    let document = Html::parse_document(&body);

    let selector = Selector::parse(".border.dark\\:border-zinc-800.border-zinc-400.bg-transparent.text-card-foreground.shadow.my-4.p-2.rounded-3xl").unwrap();

    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");
        println!("{}", text);
    }

    Ok(())
}
