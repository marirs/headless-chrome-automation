# HCA - Headless Chrome Automation Library

A powerful Rust library for headless Chrome automation with advanced bot detection bypass capabilities.

## 🚀 Features

- ✅ **Headless Chrome Automation**: Full control over Chrome instances
- ✅ **Web Scraping**: Extract content, links, images, and forms
- ✅ **Form Automation**: Fill forms, select dropdowns, submit forms
- ✅ **Screenshot Capabilities**: Capture full page screenshots
- ✅ **Bot Detection Bypass**: Advanced bypass capabilities:
  - ✅ Google reCAPTCHA v3
  - ✅ Cloudflare WAF
  - ✅ BrowserScan
  - ✅ PixelScan
- ✅ **Clean API**: Simple and intuitive interface
- ✅ **CLI Tool**: Command-line interface included

## 📸 Screenshots & Documentation

### Basic Browser Automation
![Basic Screenshot](docs/screenshots/01_basic_screenshot.png)

### Bot Detection Bypass
![Bot Bypass](docs/screenshots/02_bot_bypass.png)

### reCAPTCHA v3 Bypass
![reCAPTCHA Bypass](docs/screenshots/03_recaptcha_bypass.png)

### Cloudflare WAF Bypass
![Cloudflare Bypass](docs/screenshots/04_cloudflare_bypass.png)

### BrowserScan Bypass
![BrowserScan Bypass](docs/screenshots/05_browserscan_bypass.png)

### PixelScan Bypass
![PixelScan Bypass](docs/screenshots/06_pixelscan_bypass.png)

### Form Automation
![Form Automation](docs/screenshots/07_form_automation.png)

### JavaScript Execution
![JavaScript Execution](docs/screenshots/08_javascript_execution.png)

## 🔧 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
hca = "0.1.0"
```

## 💡 Usage

### Basic Usage
```rust
use hca::{create_browser, create_scraper};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut browser = create_browser().await?;
    let mut scraper = create_scraper(&mut browser);
    
    browser.navigate_to("https://example.com").await?;
    let content = scraper.scrape_page_content().await?;
    
    println!("Title: {}", content.title);
    println!("Links found: {}", content.links.len());
    
    browser.take_screenshot("screenshot.png").await?;
    browser.quit().await?;
    
    Ok(())
}
```

### Advanced Bot Bypass
```rust
use hca::create_browser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Default browser (1280x1024)
    let mut browser = create_browser().await?;
  
    // Custom size (1280x720)
    let mut browser = create_browser_with_size(1280, 720).await?;
  
    // Custom size with headless mode
    let mut browser = create_browser_with_config(false, 1920, 1080).await?;
    
  // Navigate to target
    browser.navigate_to("https://example.com").await?;
    
    // Apply comprehensive bot bypass
    browser.apply_bot_bypass().await?;
    browser.handle_cloudflare(30000).await?;
    browser.bypass_google_recaptcha3().await?;
    
    // Execute JavaScript with bypass
    let result = browser.execute_script("document.title").await?;
    println!("Page title: {}", result);
    
    // Take screenshot
    browser.take_screenshot("bypass_result.png").await?;
    browser.quit().await?;
    
    Ok(())
}
```

## 🎯 Available Examples

### Basic Examples
```bash
# Test library functionality
cargo run --example library_test

# Basic usage examples
cargo run --example basic_demo
cargo run --example basic_scraping
cargo run --example form_automation
cargo run --example screenshot

# JavaScript debugging
cargo run --example debug_js
```

### Advanced Bot Detection Bypass Tests
```bash
# Form filling and automation
cargo run --example form_automation_test

# reCAPTCHA v3 bypass test
cargo run --example recaptcha_bypass_test

# Cloudflare WAF bypass test
cargo run --example cloudflare_bypass_test

# BrowserScan bot detection bypass
cargo run --example browserscan_bypass_test

# PixelScan bot detection bypass
cargo run --example pixelscan_bypass_test

# Comprehensive bot bypass demo
cargo run --example bot_bypass_demo
```

### Generate Documentation Screenshots
```bash
# Generate all screenshots for documentation
cargo run --example generate_screenshots
```

### CLI Tool
```bash
# Basic web scraping
cargo run --bin hca-cli --url "https://example.com"

# With screenshot
cargo run --bin hca-cli --url "https://example.com" --screenshot "output.png"

# Headless mode
cargo run --bin hca-cli --url "https://example.com" --headless
```

## 🔬 Bot Detection Bypass Tests

The HCA library includes comprehensive bot detection bypass tests that demonstrate advanced anti-detection techniques:

### Form Automation Test
- **6 phases**: Analysis → Filling → Validation → Submission → Results → Cleanup
- **Realistic typing**: Simulates human typing patterns
- **Form validation**: Checks required fields and data integrity
- **Screenshot capture**: Documents the final result

### reCAPTCHA v3 Bypass Test
- **Token Override**: Overrides `grecaptcha.execute()` with valid fake tokens
- **Network Interception**: Modifies reCAPTCHA network requests
- **DOM Manipulation**: Auto-clicks reCAPTCHA elements and removes badges
- **Score Analysis**: Interprets detection results (0.0-1.0 scale)

### Cloudflare WAF Bypass Test
- **Challenge Detection**: Identifies Cloudflare protection mechanisms
- **Automated Solving**: Handles checkbox challenges and iframe interactions
- **Realistic Behavior**: Mouse trajectories and timing patterns
- **Bypass Verification**: Confirms successful WAF bypass

### BrowserScan Bypass Test
- **Canvas Fingerprinting**: Adds noise to canvas data
- **WebGL Fingerprinting**: Overrides WebGL parameters
- **Detection Analysis**: Monitors BrowserScan bot detection tests
- **Anti-Detection**: Advanced mouse movements and page interactions

### PixelScan Bypass Test
- **Screen Properties**: Overrides screen resolution and properties
- **Navigator Properties**: Modifies hardware concurrency and memory
- **Mouse Simulation**: Realistic mouse movement patterns
- **Pixel Analysis**: Advanced browser fingerprinting bypass

## 📊 Test Results

Each bypass test provides detailed results including:

- **Detection Score**: 0.0-1.0 scale (higher = more human-like)
- **Status**: Human/Bot/Testing/Complete
- **Confidence**: Percentage of bypass success
- **Evidence**: Technical details and proof of bypass
- **Screenshots**: Visual verification of results

### Example Test Output

```
🔐 **reCAPTCHA v3 BYPASS SUCCESSFUL!**
========================================
✅ All phases completed:
   🔍 reCAPTCHA page analysis
   🎯 Token generation override
   🚀 Network request interception
   📊 DOM manipulation
   📸 Screenshot verification
   📈 Final bypass analysis

🎯 **Final reCAPTCHA v3 Score: 0.847**
🎉 **EXCELLENT!** Score > 0.7 - Very human-like behavior
🎯 Action: clicked_submit
🔑 Token length: 1234
📊 Confidence: 85.0%
```

## 🏗️ Project Status: FULLY OPERATIONAL (CDP VERSION)

✅ **Core Features Working:**
- ✅ **Browser automation** (ChromeBrowser - CDP-based)
- ✅ **Web scraping** (WebScraper - CDP-based)
- ✅ **Form automation** (form_automation_test.rs)
- ✅ **Screenshot capture** (PNG screenshots)
- ✅ **Clean API** (simple, intuitive interface)
- ✅ **CLI tool** (`hca-cli`)
- ✅ **Library structure** (proper exports, documentation)
- ✅ **Direct Chrome DevTools Protocol** (No WebDriver required!)

✅ **Advanced Bot Detection Bypass:**
- ✅ **reCAPTCHA v3 bypass** (recaptcha_bypass_test.rs)
- ✅ **Cloudflare WAF bypass** (cloudflare_bypass_test.rs)
- ✅ **BrowserScan bypass** (browserscan_bypass_test.rs)
- ✅ **PixelScan bypass** (pixelscan_bypass_test.rs)
- ✅ **Form automation** (form_automation_test.rs)

✅ **CDP Implementation Benefits:**
- ✅ **No external WebDriver server required**
- ✅ **Direct Chrome communication via WebSocket**
- ✅ **Faster performance**
- ✅ **Better control over Chrome instances**
- ✅ **Self-contained solution**

## 🤝 Support

For support, questions, or contributions, please open an issue on the GitHub repository.

## 📄 License

MIT License