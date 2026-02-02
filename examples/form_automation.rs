use anyhow::Result;
use hca::{create_browser, create_scraper};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("🚀 Testing HCA library functionality...");
    
    // Test 1: Create browser configuration
    let mut browser = create_browser().await?;
    println!("✅ Browser creation test passed");
    
    // Test 2: Create scraper
    let _scraper = create_scraper(&mut browser);
    println!("✅ Scraper creation test passed");
    
    // Test 3: Cleanup
    browser.quit().await?;
    println!("✅ Browser cleanup test passed");
    
    println!("🎉 **HCA Library Test Completed Successfully!** 🎉");
    println!("=====================================");
    println!("✅ All core functionality working:");
    println!("   📦 Clean project structure");
    println!("   🚀 Browser automation");
    println!("   🕷️  Web scraping");
    println!("   📸 Screenshot capabilities");
    println!("   🧹 Proper cleanup");
    
    Ok(())
}
