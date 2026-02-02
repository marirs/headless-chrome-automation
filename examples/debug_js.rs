use hca::create_browser;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    println!("🔍 **JavaScript Debug Test** 🔍");
    println!("=================================");

    // Create browser instance
    let mut browser = create_browser().await?;

    // Navigate to a simple data URL
    println!("🌐 Navigating to simple test page...");
    browser.navigate_to("data:text/html,<html><head><title>Test Page</title></head><body><h1>Hello World</h1></body></html>").await?;

    // Wait a bit
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Test simple JavaScript
    println!("🔍 Testing simple JavaScript...");
    match browser.execute_script("document.title").await {
        Ok(result) => println!("✅ Title: {}", result),
        Err(e) => println!("❌ Error: {}", e),
    }

    // Test another simple script
    println!("🔍 Testing document.body.innerHTML...");
    match browser.execute_script("document.body.innerHTML").await {
        Ok(result) => println!("✅ Body: {}", result),
        Err(e) => println!("❌ Error: {}", e),
    }

    // Test a simple number
    println!("🔍 Testing number return...");
    match browser.execute_script("42").await {
        Ok(result) => println!("✅ Number: {}", result),
        Err(e) => println!("❌ Error: {}", e),
    }

    // Close browser
    browser.quit().await?;

    println!("\n🎉 **Debug Test Completed** 🎉");
    Ok(())
}
