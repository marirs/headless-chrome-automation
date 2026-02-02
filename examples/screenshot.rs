use anyhow::Result;
use hca::{create_browser, create_scraper};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("📸 **Screenshot Test** 📸");
    println!("====================");
    
    // Create browser instance
    let mut browser = create_browser().await?;
    println!("✅ Browser created successfully");
    
    // Navigate to a test website
    browser.navigate_to("https://www.google.com").await?;
    println!("✅ Navigated to google.com");
    
    // Wait for page to load
    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
    
    // Take screenshot
    let filename = "google_screenshot.png";
    browser.take_screenshot(filename).await?;
    println!("✅ Screenshot saved: {}", filename);
    
    // Cleanup
    browser.quit().await?;
    println!("✅ Browser closed successfully");
    
    println!("🎉 **Screenshot Test Completed Successfully!** 🎉");
    println!("=====================================");
    
    Ok(())
}
