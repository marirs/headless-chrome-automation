//! HCA - Headless Chrome Automation Library
//! 
//! A powerful Rust library for headless Chrome automation with bot detection bypass capabilities.
//! 
//! ## Quick Start
//! 
//! ```rust
//! use hca::{create_browser, create_scraper};
//! 
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let mut browser = create_browser().await?;
//!     let mut scraper = create_scraper(&mut browser);
//!     
//!     browser.navigate_to("https://example.com").await?;
//!     let content = scraper.scrape_page_content().await?;
//!     
//!     println!("Title: {}", content.title);
//!     println!("Links found: {}", content.links.len());
//!     
//!     // Take screenshot
//!     browser.take_screenshot("screenshot.png").await?;
//!     
//!     // Cleanup
//!     browser.quit().await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod browser;
pub mod scraper;

// Re-export commonly used types for convenience
pub use browser::ChromeBrowser;
pub use scraper::{WebScraper, ScrapedContent};

#[derive(Debug, Clone)]
pub struct ChromeConfig {
    pub headless: bool,
    pub user_data_dir: Option<String>,
}

impl Default for ChromeConfig {
    fn default() -> Self {
        Self {
            headless: true,
            user_data_dir: None,
        }
    }
}

/// Main HCA library interface
pub struct HCA {
    config: ChromeConfig,
}

impl HCA {
    /// Create a new HCA instance with default configuration
    pub fn new() -> Self {
        Self {
            config: ChromeConfig::default(),
        }
    }
    
    /// Create a new HCA instance with custom configuration
    pub fn with_config(config: ChromeConfig) -> Self {
        Self { config }
    }
    
    /// Get the current configuration
    pub fn config(&self) -> &ChromeConfig {
        &self.config
    }
    
    /// Set headless mode
    pub fn headless(mut self, headless: bool) -> Self {
        self.config.headless = headless;
        self
    }
    
    /// Set user data directory
    pub fn user_data_dir(mut self, dir: String) -> Self {
        self.config.user_data_dir = Some(dir);
        self
    }
}

/// Convenience function to create a browser instance
pub async fn create_browser() -> anyhow::Result<ChromeBrowser> {
    let hca = HCA::new();
    ChromeBrowser::new(hca.config.headless).await
}

/// Convenience function to create a web scraper
pub fn create_scraper(browser: &mut ChromeBrowser) -> WebScraper<'_> {
    WebScraper::new(browser)
}
