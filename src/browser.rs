use anyhow::{anyhow, Result};
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;
use reqwest::Client;
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use base64::Engine;

pub struct ChromeBrowser {
    pub process: std::process::Child,
    port: u16,
    client: Client,
    websocket_url: String,
    target_id: Option<String>,
}

impl ChromeBrowser {
    pub async fn new(headless: bool) -> Result<Self> {
        let chrome_path = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
        let port = 9222 + (rand::random::<u32>() % 1000) as u16;
        let user_data_dir = format!("/tmp/chrome_hca_{}", port);
        
        // Clean up any existing profile
        if std::path::Path::new(&user_data_dir).exists() {
            std::fs::remove_dir_all(&user_data_dir)?;
        }
        std::fs::create_dir_all(&user_data_dir)?;
        
        // Launch Chrome with CDP enabled
        let child = Command::new(chrome_path)
            .args(&[
                format!("--remote-debugging-port={}", port),
                format!("--user-data-dir={}", user_data_dir),
                if headless { "--headless" } else { "--headless=false" }.to_string(),
                "--no-sandbox".to_string(),
                "--disable-dev-shm-usage".to_string(),
                "--disable-blink-features=AutomationControlled".to_string(),
                "--disable-features=VizDisplayCompositor".to_string(),
                "--disable-web-security".to_string(),
                "--disable-features=TranslateUI".to_string(),
                "--disable-ipc-flooding-protection".to_string(),
                "--disable-background-timer-throttling".to_string(),
                "--disable-backgrounding-occluded-windows".to_string(),
                "--disable-renderer-backgrounding".to_string(),
                "--disable-plugins-discovery".to_string(),
                "--disable-default-apps".to_string(),
                "--disable-extensions".to_string(),
                "--disable-component-extensions-with-background-pages".to_string(),
                "--disable-background-networking".to_string(),
                "--disable-sync".to_string(),
                "--disable-default-browser-check".to_string(),
                "--disable-features=TranslateUI,BlinkGenPropertyTrees".to_string(),
                "--user-agent=Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36".to_string(),
                "about:blank".to_string(),
            ])
            .spawn()?;
        
        // Wait for Chrome to start
        sleep(Duration::from_secs(3)).await;
        
        // Get the browser websocket URL from the HTTP endpoint
        let browser_info_response = reqwest::get(&format!("http://localhost:{}/json", port)).await?;
        let browser_info: serde_json::Value = browser_info_response.json().await?;
        
        if let Some(websocket_url) = browser_info.as_array().and_then(|arr| arr.first()).and_then(|info| info.get("webSocketDebuggerUrl")).and_then(|url| url.as_str()) {
            let websocket_url = websocket_url.to_string();
            
            Ok(Self {
                process: child,
                port,
                client: Client::new(),
                websocket_url,
                target_id: None,
            })
        } else {
            Err(anyhow!("Failed to get Chrome websocket URL"))
        }
    }
    
    pub async fn navigate_to(&mut self, url: &str) -> Result<()> {
        // Wait a bit more for Chrome to be fully ready
        sleep(Duration::from_secs(2)).await;
        
        // Create a new target with the URL
        let create_target_request = json!({
            "id": 1,
            "method": "Target.createTarget",
            "params": {
                "url": url
            }
        });
        
        let response = self.send_message(create_target_request).await?;
        
        if let Some(target_id) = response.get("result").and_then(|r| r.get("targetId")).and_then(|id| id.as_str()) {
            self.target_id = Some(target_id.to_string());
            
            // Wait for the page to load
            sleep(Duration::from_secs(3)).await;
            
            return Ok(());
        }
        
        Err(anyhow!("Failed to create Chrome target"))
    }
    
    pub async fn take_screenshot(&mut self, path: &str) -> Result<()> {
        if let Some(target_id) = &self.target_id {
            // First attach to the target
            let attach_request = json!({
                "id": 8,
                "method": "Target.attachToTarget",
                "params": {
                    "targetId": target_id,
                    "flatten": true
                }
            });
            
            let attach_response = self.send_message(attach_request).await?;
            
            if let Some(session_id) = attach_response.get("result").and_then(|r| r.get("sessionId")).and_then(|id| id.as_str()) {
                // Enable Page domain
                let enable_page_request = json!({
                    "id": 9,
                    "method": "Page.enable",
                    "sessionId": session_id
                });
                
                let _enable_response = self.send_message(enable_page_request).await?;
                
                // Take screenshot
                let screenshot_request = json!({
                    "id": 10,
                    "method": "Page.captureScreenshot",
                    "params": {
                        "format": "png"
                    },
                    "sessionId": session_id
                });
                
                let response = self.send_message(screenshot_request).await?;
                
                if let Some(result) = response.get("result") {
                    if let Some(data) = result.get("data").and_then(|d| d.as_str()) {
                        if let Ok(screenshot_bytes) = base64::engine::general_purpose::STANDARD.decode(data) {
                            std::fs::write(path, screenshot_bytes)?;
                            return Ok(());
                        }
                    }
                }
                
                // If we get here, try to get any error information
                if let Some(error) = response.get("error") {
                    return Err(anyhow!("Screenshot error: {}", error));
                }
            }
        }
        
        Err(anyhow!("Failed to take screenshot"))
    }
    
    pub async fn execute_script(&mut self, script: &str) -> Result<String> {
        if let Some(target_id) = &self.target_id {
            // First attach to the target
            let attach_request = json!({
                "id": 5,
                "method": "Target.attachToTarget",
                "params": {
                    "targetId": target_id,
                    "flatten": true
                }
            });
            
            let attach_response = self.send_message(attach_request).await?;
            
            if let Some(session_id) = attach_response.get("result").and_then(|r| r.get("sessionId")).and_then(|id| id.as_str()) {
                // Enable Runtime domain
                let enable_runtime_request = json!({
                    "id": 6,
                    "method": "Runtime.enable",
                    "sessionId": session_id
                });
                
                let _enable_response = self.send_message(enable_runtime_request).await?;
                
                // Execute the script
                let evaluate_request = json!({
                    "id": 7,
                    "method": "Runtime.evaluate",
                    "params": {
                        "expression": script,
                        "returnByValue": true,
                        "awaitPromise": true
                    },
                    "sessionId": session_id
                });
                
                let response = self.send_message(evaluate_request).await?;
                
                if let Some(result) = response.get("result") {
                    if let Some(value) = result.get("value") {
                        if let Some(text) = value.as_str() {
                            return Ok(text.to_string());
                        } else if let Some(num) = value.as_f64() {
                            return Ok(num.to_string());
                        } else if let Some(bool) = value.as_bool() {
                            return Ok(bool.to_string());
                        } else if value.is_null() {
                            return Ok("null".to_string());
                        }
                    }
                }
                
                // If we get here, try to get any error information
                if let Some(error) = response.get("error") {
                    return Err(anyhow!("JavaScript execution error: {}", error));
                }
            }
        }
        
        Ok("null".to_string())
    }
    
    async fn send_message(&mut self, request: serde_json::Value) -> Result<serde_json::Value> {
        // Connect to WebSocket
        let (ws_stream, _) = connect_async(&self.websocket_url).await?;
        let (mut write, mut read) = ws_stream.split();
        
        // Send message
        write.send(Message::Text(request.to_string())).await?;
        
        // Read response
        if let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(response) = serde_json::from_str::<serde_json::Value>(&text) {
                        return Ok(response);
                    }
                }
                _ => {}
            }
        }
        
        Err(anyhow!("Failed to send message"))
    }
    
    pub async fn quit(&mut self) -> Result<()> {
        // Close Chrome process
        self.process.kill()?;
        self.process.wait()?;
        
        // Clean up user data directory
        if std::path::Path::new(&format!("/tmp/chrome_hca_{}", self.port)).exists() {
            std::fs::remove_dir_all(&format!("/tmp/chrome_hca_{}", self.port))?;
        }
        
        Ok(())
    }
}
