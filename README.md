# HCA - Headless Chrome Automation Library

A powerful Rust library for headless Chrome automation with advanced bot detection bypass capabilities.

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
hca = "0.1.0"
```

## Usage

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

## Features

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

## Available Examples

### Basic Examples
```bash
# Test library functionality
cargo run --example library_test

# Basic usage examples
cargo run --example basic_demo
cargo run --example basic_scraping
cargo run --example form_automation
cargo run --example screenshot
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

## Bot Detection Bypass Tests

The HCA library includes comprehensive bot detection bypass tests that demonstrate advanced anti-detection techniques:

### Form Automation Test
- **6 phases**: Analysis → Filling → Validation → Submission → Results → Cleanup
- **Realistic typing**: Simulates human typing patterns
- **Form validation**: Checks required fields and data integrity
- **Screenshot capture**: Documents the final result

### reCAPTCHA v3 Bypass Test
- **Score extraction**: Extracts reCAPTCHA v3 scores from tokens
- **Automated interaction**: Realistic mouse movements and clicks
- **Score analysis**: Interprets detection results (0.0-1.0 scale)
- **Success indicators**: Verifies bypass completion

### Cloudflare WAF Bypass Test
- **Challenge detection**: Identifies Cloudflare protection mechanisms
- **Automated solving**: Handles checkbox challenges and iframe interactions
- **Realistic behavior**: Mouse trajectories and timing patterns
- **Bypass verification**: Confirms successful WAF bypass

### BrowserScan Bypass Test
- **Detection analysis**: Monitors BrowserScan bot detection tests
- **Anti-detection**: Advanced mouse movements and page interactions
- **Score interpretation**: Analyzes detection confidence levels
- **Human simulation**: Complex behavioral patterns

### PixelScan Bypass Test
- **Pixel analysis**: Advanced browser fingerprinting bypass
- **Enhanced interactions**: Complex mouse and keyboard patterns
- **Score extraction**: Interprets PixelScan detection results
- **Behavior simulation**: Realistic user activity patterns

## 🔧 Dependencies

- `tokio` - Async runtime
- `serde_json` - JSON serialization
- `anyhow` - Error handling
- `tracing` - Structured logging
- `reqwest` - HTTP client
- `base64` - Base64 encoding
- `regex` - Regular expressions
- `clap` - Command line parsing
- `tokio-tungstenite` - WebSocket client for CDP
- `futures-util` - Async utilities
- `rand` - Random number generation
- `tempfile` - Temporary file handling
- `url` - URL parsing
- `scraper` - HTML parsing

## Test Results

Each bypass test provides detailed results including:

- **Detection Score**: 0.0-1.0 scale (higher = more human-like)
- **Status**: Human/Bot/Testing/Complete
- **Confidence**: Percentage of bypass success
- **Evidence**: Technical details and proof of bypass
- **Screenshots**: Visual verification of results

## Example Test Output

```
**reCAPTCHA v3 BYPASS SUCCESSFUL!**
========================================
✅ All phases completed:
   🔍 reCAPTCHA page analysis
   🎯 Score extraction
   🚀 reCAPTCHA v3 trigger
   📊 Post-trigger score analysis
   📸 Screenshot verification
   📈 Final bypass analysis

🎯 **Final reCAPTCHA v3 Score: 0.847**
🎉 **EXCELLENT!** Score > 0.7 - Very human-like behavior
🎯 Action: clicked_submit
🔑 Token length: 1234
📊 Confidence: 85.0%
```

## Project Status: FULLY OPERATIONAL (CDP VERSION)

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

✅ **Ready for Distribution:**
- Clean, maintainable codebase
- Professional API design
- Comprehensive documentation
- Working examples

## Support

For support, questions, or contributions, please open an issue on the GitHub repository.


## License

MIT License