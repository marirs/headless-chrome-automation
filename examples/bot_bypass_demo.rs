use hca::{create_browser, create_scraper};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    println!("🤖 **Bot Bypass Demo** 🤖");
    println!("=====================================");

    // Create browser instance
    let mut browser = create_browser().await?;

    // Navigate to a test website
    println!("🌐 Navigating to test website...");
    browser.navigate_to("https://www.google.com").await?;

    // Wait for page to load
    browser.wait_for_page_load(10000).await?;

    // Apply bot bypass techniques
    browser.apply_bot_bypass().await?;

    // Handle potential Cloudflare challenges
    browser.handle_cloudflare(30000).await?;

    // Bypass Google reCAPTCHA v3
    browser.bypass_google_recaptcha3().await?;

    // Take a screenshot to verify everything worked
    browser.take_screenshot("bot_bypass_screenshot.png").await?;

    // Test JavaScript execution with retry
    println!("🔍 Testing JavaScript execution with retry...");
    let title = browser.execute_script_with_retry("document.title", 3).await?;
    println!("Page Title: {}", title);

    // Test scraping with bot bypass
    let mut scraper = create_scraper(&mut browser);
    let content = scraper.scrape_page_content().await?;

    println!("\n📊 **Scraping Results:**");
    println!("Title: {}", content.title);
    println!("Content Length: {} chars", content.body.len());
    println!("Links Found: {}", content.links.len());
    println!("Images Found: {}", content.images.len());
    println!("Forms Found: {}", content.forms.len());

    // Close browser
    browser.quit().await?;

    println!("\n🎉 **Bot Bypass Demo Completed Successfully!** 🎉");
    println!("=====================================");
    println!("✅ Bot detection bypass applied");
    println!("✅ Cloudflare handling completed");
    println!("✅ reCAPTCHA v3 bypass applied");
    println!("✅ Screenshot captured");
    println!("✅ JavaScript execution working");
    println!("✅ Web scraping completed");
    println!("✅ Browser closed successfully");

    Ok(())
}
