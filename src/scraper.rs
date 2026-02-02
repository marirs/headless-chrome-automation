use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScrapedContent {
    pub title: String,
    pub body: String,
    pub links: Vec<String>,
    pub images: Vec<String>,
    pub forms: Vec<String>,
}

pub struct WebScraper<'a> {
    browser: &'a mut crate::browser::ChromeBrowser,
}

impl<'a> WebScraper<'a> {
    pub fn new(browser: &'a mut crate::browser::ChromeBrowser) -> Self {
        Self { browser }
    }
    
    pub async fn scrape_page_content(&mut self) -> Result<ScrapedContent> {
        // Wait a moment for the page to fully load
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        
        // Use a simple script that should work
        let title_script = r#"document.title"#;
        let title = self.browser.execute_script(title_script).await?;
        
        // Use a simple script for body content
        let body_script = r#"document.body ? document.body.innerText : 'No body found'"#;
        let body = self.browser.execute_script(body_script).await?;
        
        // Use simple scripts for links and images
        let links_script = r#"
        (function() {
            var links = [];
            var linkElements = document.querySelectorAll('a');
            for (var i = 0; i < linkElements.length; i++) {
                var href = linkElements[i].href;
                if (href && href.trim() !== '') {
                    links.push(href);
                }
            }
            return JSON.stringify(links);
        })()
        "#;
        
        let links_result = self.browser.execute_script(links_script).await?;
        let links: Vec<String> = serde_json::from_str(&links_result).unwrap_or_default();
        
        let images_script = r#"
        (function() {
            var images = [];
            var imgElements = document.querySelectorAll('img');
            for (var i = 0; i < imgElements.length; i++) {
                var src = imgElements[i].src;
                if (src && src.trim() !== '') {
                    images.push(src);
                }
            }
            return JSON.stringify(images);
        })()
        "#;
        
        let images_result = self.browser.execute_script(images_script).await?;
        let images: Vec<String> = serde_json::from_str(&images_result).unwrap_or_default();
        
        let forms_script = r#"
        (function() {
            var forms = [];
            var formElements = document.querySelectorAll('form');
            for (var i = 0; i < formElements.length; i++) {
                var form = formElements[i];
                var id = form.id || form.name || 'form-' + i;
                forms.push(id);
            }
            return JSON.stringify(forms);
        })()
        "#;
        
        let forms_result = self.browser.execute_script(forms_script).await?;
        let forms: Vec<String> = serde_json::from_str(&forms_result).unwrap_or_default();
        
        Ok(ScrapedContent {
            title,
            body,
            links,
            images,
            forms,
        })
    }
}
