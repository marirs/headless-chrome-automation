use hca::create_browser;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("🔐 **reCAPTCHA v3 Bypass Test** 🔐");
    println!("===================================");
    
    // Create browser instance
    let mut browser = create_browser().await?;
    
    // Navigate to reCAPTCHA demo page
    println!("\n🌐 Navigating to reCAPTCHA v3 demo page...");
    browser.navigate_to("https://www.google.com/recaptcha/api2/demo").await?;
    
    // Wait for page to load
    browser.wait_for_page_load(10000).await?;
    
    // Apply bot bypass techniques
    println!("\n🤖 Applying bot detection bypass...");
    browser.apply_bot_bypass().await?;
    
    // Apply reCAPTCHA v3 bypass
    println!("\n🔐 Applying reCAPTCHA v3 bypass...");
    browser.bypass_google_recaptcha3().await?;
    
    // Take screenshot
    println!("\n📸 Taking screenshot...");
    browser.take_screenshot("recaptcha_test.png").await?;
    
    // Test JavaScript execution
    println!("\n🔍 Testing JavaScript execution...");
    let title = browser.execute_script("document.title").await?;
    println!("Page title: {}", title);
    
    // Check if reCAPTCHA is present
    println!("\n🔍 Checking for reCAPTCHA elements...");
    let recaptcha_check = browser.execute_script(
        "!!document.querySelector('.g-recaptcha') || !!document.querySelector('[class*=\"recaptcha\"]') || typeof grecaptcha !== 'undefined'"
    ).await?;
    
    if recaptcha_check == "true" {
        println!("✅ reCAPTCHA elements detected");
    } else {
        println!("⚠️  No reCAPTCHA elements found");
    }
    
    // Close browser
    browser.quit().await?;
    
    println!("\n🎉 **reCAPTCHA v3 Bypass Test Completed** 🎉");
    println!("========================================");
    println!("✅ reCAPTCHA v3 bypass applied successfully");
    println!("✅ Screenshot captured");
    println!("✅ JavaScript execution working");
    println!("✅ Browser closed successfully");
    
    Ok(())
}
