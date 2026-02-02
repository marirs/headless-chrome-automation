use anyhow::Result;
use hca::{
    browser::ChromeBrowser,
    scraper::WebScraper,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    println!("Starting basic web scraping example...");

    // Create browser instance
    let mut browser = ChromeBrowser::new(true).await?;

    // Navigate to a test website
    browser.navigate_to("https://www.google.com").await?;

    // Wait a moment for the page to load
    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;

    // Scrape page content
    let mut scraper = WebScraper::new(&mut browser);
    let content = scraper.scrape_page_content().await?;

    println!("Page Title: {}", content.title);
    println!("Page Content (first 200 chars):");
    println!("{}", &content.body[..content.body.len().min(200)]);
    println!("\nFound {} links", content.links.len());
    println!("Found {} images", content.images.len());

    // Take a screenshot
    browser.take_screenshot("google_screenshot.png").await?;

    // Close browser
    browser.quit().await?;

    println!("Basic scraping example completed successfully!");
    Ok(())
}
