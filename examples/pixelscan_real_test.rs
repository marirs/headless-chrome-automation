use hca::create_browser;
use anyhow::Result;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("🎨 **PixelScan Real Bot Detection Test** 🎨");
    println!("==========================================");
    
    // Create docs directory
    fs::create_dir_all("docs/screenshots")?;
    
    // Create browser instance
    let mut browser = create_browser().await?;
    
    // Navigate to PixelScan bot detection page
    println!("\n🌐 Navigating to PixelScan bot detection...");
    browser.navigate_to("https://pixelscan.net/bot-check").await?;
    
    // Wait for page to load
    browser.wait_for_page_load(10000).await?;
    
    // Apply bot bypass techniques
    println!("\n🤖 Applying bot detection bypass...");
    browser.apply_bot_bypass().await?;
    
    // Apply PixelScan-specific bypass techniques
    println!("\n🎨 Applying PixelScan-specific bypass...");
    let pixelscan_script = r#"
    (function() {
        // PixelScan-specific bypass techniques
        console.log('PixelScan bypass activated');
        
        // Override screen properties for consistent fingerprinting
        Object.defineProperty(screen, 'availWidth', {
            get: () => 1280
        });
        Object.defineProperty(screen, 'availHeight', {
            get: () => 1024
        });
        Object.defineProperty(screen, 'width', {
            get: () => 1280
        });
        Object.defineProperty(screen, 'height', {
            get: () => 1024
        });
        Object.defineProperty(screen, 'colorDepth', {
            get: () => 24
        });
        Object.defineProperty(screen, 'pixelDepth', {
            get: () => 24
        });
        
        // Override navigator properties
        Object.defineProperty(navigator, 'hardwareConcurrency', {
            get: () => 8
        });
        Object.defineProperty(navigator, 'deviceMemory', {
            get: () => 8
        });
        Object.defineProperty(navigator, 'maxTouchPoints', {
            get: () => 0
        });
        
        // Override timezone
        Object.defineProperty(Intl.DateTimeFormat.prototype, 'resolvedOptions', {
            value: function() {
                return {
                    timeZone: 'America/New_York',
                    locale: 'en-US'
                };
            }
        });
        
        // Override canvas fingerprinting with more sophisticated noise
        const originalGetContext = HTMLCanvasElement.prototype.getContext;
        HTMLCanvasElement.prototype.getContext = function(contextType, ...args) {
            const context = originalGetContext.call(this, contextType, ...args);
            if (contextType === '2d' && context) {
                const originalGetImageData = context.getImageData;
                const originalToDataURL = context.toDataURL;
                
                context.getImageData = function(...args) {
                    const imageData = originalGetImageData.apply(this, args);
                    // Add subtle, realistic noise
                    for (let i = 0; i < imageData.data.length; i += 4) {
                        imageData.data[i] += Math.random() * 3 - 1.5;     // Red
                        imageData.data[i + 1] += Math.random() * 3 - 1.5; // Green
                        imageData.data[i + 2] += Math.random() * 3 - 1.5; // Blue
                    }
                    return imageData;
                };
                
                context.toDataURL = function(...args) {
                    // Add noise before converting to data URL
                    const imageData = this.getImageData(0, 0, this.canvas.width, this.canvas.height);
                    return originalToDataURL.apply(this, args);
                };
            }
            return context;
        };
        
        // Override WebGL fingerprinting
        const getParameter = WebGLRenderingContext.prototype.getParameter;
        WebGLRenderingContext.prototype.getParameter = function(parameter) {
            const overrides = {
                37445: 'Intel Inc.',                           // UNMASKED_VENDOR_WEBGL
                37446: 'Intel Iris OpenGL Engine',             // UNMASKED_RENDERER_WEBGL
                35724: 'WebGL 1.0 (OpenGL ES 2.0 Chromium)', // VERSION
                35723: 'WebGL 1.0',                          // SHADING_LANGUAGE_VERSION
                7936: 'Intel Inc.',                          // VENDOR
                7937: 'Intel Iris OpenGL Engine'              // RENDERER
            };
            
            if (overrides[parameter]) {
                return overrides[parameter];
            }
            return getParameter.call(this, parameter);
        };
        
        // Add realistic mouse movement simulation
        let mouseX = 640, mouseY = 512; // Center of 1280x1024
        let targetX = mouseX, targetY = mouseY;
        
        const moveMouse = () => {
            targetX = Math.random() * 1280;
            targetY = Math.random() * 1024;
            const steps = 15 + Math.floor(Math.random() * 10);
            let step = 0;
            
            const interval = setInterval(() => {
                step++;
                const progress = step / steps;
                const easeProgress = 1 - Math.pow(1 - progress, 3); // Ease out cubic
                
                const currentX = mouseX + (targetX - mouseX) * easeProgress;
                const currentY = mouseY + (targetY - mouseY) * easeProgress;
                
                const event = new MouseEvent('mousemove', {
                    clientX: currentX,
                    clientY: currentY,
                    bubbles: true,
                    cancelable: true
                });
                document.dispatchEvent(event);
                
                if (step >= steps) {
                    mouseX = targetX;
                    mouseY = targetY;
                    clearInterval(interval);
                }
            }, 16 + Math.random() * 16); // Variable timing
        };
        
        // Start mouse movement
        moveMouse();
        setInterval(moveMouse, 2000 + Math.random() * 2000);
        
        // Add keyboard events for realism
        const typeRandom = () => {
            const keys = ['a', 's', 'd', 'f', 'j', 'k', 'l'];
            const randomKey = keys[Math.floor(Math.random() * keys.length)];
            const event = new KeyboardEvent('keydown', {
                key: randomKey,
                code: `Key${randomKey.toUpperCase()}`,
                bubbles: true
            });
            document.dispatchEvent(event);
        };
        
        // Occasional typing
        setInterval(typeRandom, 5000 + Math.random() * 5000);
        
        return 'PixelScan bypass simulation completed';
    })();
    "#;
    
    browser.execute_script(pixelscan_script).await?;
    
    // Wait for the test to run
    println!("\n⏳ Waiting for PixelScan test to complete...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    // Check if test results are visible
    let check_script = r#"
    (function() {
        // Look for PixelScan test results
        const results = {
            hasResults: false,
            score: null,
            status: null,
            details: [],
            botScore: null,
            humanScore: null
        };
        
        // Check for various result elements
        const scoreElements = document.querySelectorAll('[class*="score"], [class*="result"], [data-score], .percentage, .score-value');
        scoreElements.forEach(el => {
            const text = el.textContent || el.innerText;
            if (text && text.match(/[0-9.]+/)) {
                const match = text.match(/([0-9.]+)/);
                if (match) {
                    results.score = match[1];
                    results.hasResults = true;
                }
            }
        });
        
        // Check for status indicators
        const statusElements = document.querySelectorAll('[class*="status"], [class*="human"], [class*="bot"], [class*="detection"]');
        statusElements.forEach(el => {
            const text = el.textContent || el.innerText;
            if (text && (text.includes('Human') || text.includes('Bot') || text.includes('Detection'))) {
                results.status = text;
                results.hasResults = true;
            }
        });
        
        // Look for specific PixelScan result sections
        const resultSections = document.querySelectorAll('[class*="result"], [class*="test"], [class*="detection"], [class*="analysis"]');
        resultSections.forEach(section => {
            const text = section.textContent || section.innerText;
            if (text && text.length > 10) {
                results.details.push(text.substring(0, 100));
            }
        });
        
        // Check for bot/human probability
        const probElements = document.querySelectorAll('[class*="probability"], [class*="confidence"], [class*="likelihood"]');
        probElements.forEach(el => {
            const text = el.textContent || el.innerText;
            if (text && text.match(/[0-9.]+/)) {
                const match = text.match(/([0-9.]+)/);
                if (match) {
                    if (text.includes('bot') || text.includes('Bot')) {
                        results.botScore = match[1];
                    } else if (text.includes('human') || text.includes('Human')) {
                        results.humanScore = match[1];
                    }
                    results.hasResults = true;
                }
            }
        });
        
        return JSON.stringify(results);
    })();
    "#;
    
    if let Ok(results_str) = browser.execute_script(check_script).await {
        if let Ok(results) = serde_json::from_str::<serde_json::Value>(&results_str) {
            println!("📊 PixelScan Test Results Analysis:");
            if let Some(has_results) = results.get("hasResults").and_then(|v| v.as_bool()) {
                if has_results {
                    if let Some(score) = results.get("score").and_then(|v| v.as_str()) {
                        println!("   Score: {}", score);
                    }
                    if let Some(status) = results.get("status").and_then(|v| v.as_str()) {
                        println!("   Status: {}", status);
                    }
                    if let Some(bot_score) = results.get("botScore").and_then(|v| v.as_str()) {
                        println!("   Bot Probability: {}", bot_score);
                    }
                    if let Some(human_score) = results.get("humanScore").and_then(|v| v.as_str()) {
                        println!("   Human Probability: {}", human_score);
                    }
                } else {
                    println!("   ⏳ Test still running or results not yet visible");
                }
            }
        }
    }
    
    // Take screenshot of the results
    println!("\n📸 Capturing PixelScan test results...");
    browser.take_screenshot("docs/screenshots/pixelscan_bot_detection.png").await?;
    
    // Try to scroll down to see more results
    println!("\n📜 Scrolling to see more results...");
    let scroll_script = r#"
    window.scrollTo(0, document.body.scrollHeight / 2);
    return "Scrolled to middle";
    "#;
    browser.execute_script(scroll_script).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    // Close browser
    browser.quit().await?;
    
    println!("\n🎉 **PixelScan Test Completed** 🎉");
    println!("=================================");
    println!("✅ Screenshot saved:");
    println!("   📸 docs/screenshots/pixelscan_bot_detection.png");
    
    Ok(())
}
