use hca::{create_browser, create_browser_with_size};
use anyhow::Result;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("🔐 **Targeted reCAPTCHA v3 Test** 🔐");
    println!("=================================");
    
    // Create docs directory
    fs::create_dir_all("docs/screenshots")?;
    
    // Create browser instance with custom size for better visibility
    let mut browser: hca::ChromeBrowser = create_browser_with_size(1920, 1080).await?;
    
    // Navigate to 2captcha reCAPTCHA v3 demo
    println!("\n🌐 Navigating to 2captcha reCAPTCHA v3 demo...");
    browser.navigate_to("https://2captcha.com/demo/recaptcha-v3").await?;
    
    // Wait for page to load completely
    browser.wait_for_page_load(15000).await;
    
    // Take screenshot before any manipulation
    println!("\n📸 Taking screenshot BEFORE any manipulation...");
    browser.take_screenshot("docs/screenshots/recaptcha_original.png").await?;
    
    // Apply bot bypass techniques first
    println!("\n🤖 Applying bot detection bypass...");
    browser.apply_bot_bypass().await?;
    
    // Wait a moment for bypass to take effect
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    // Now apply the targeted reCAPTCHA v3 bypass
    println!("\n🔐 Applying targeted reCAPTCHA v3 bypass...");
    
    let targeted_bypass_script = r#"
    (function() {
        console.log('Starting targeted reCAPTCHA v3 bypass...');
        
        // Step 1: Override grecaptcha.execute to return a valid token
        if (typeof window.grecaptcha !== 'undefined') {
            const originalExecute = window.grecaptcha.execute;
            
            window.grecaptcha.execute = function(sitekey, options) {
                console.log('Intercepted reCAPTCHA execute for sitekey:', sitekey, 'options:', options);
                
                // Generate a valid-looking token
                const timestamp = Date.now();
                const randomPart = Math.random().toString(36).substring(2, 15);
                const randomPart2 = Math.random().toString(36).substring(2, 15);
                
                const token = '03AGdBq26B9y-8K2n4pXrQaZbVcWdEfGhIjKlMnOpQrStUvWxYz' + 
                              randomPart + randomPart2 + 
                              timestamp.toString(36);
                
                console.log('Generated reCAPTCHA token:', token.substring(0, 30) + '...');
                
                // Inject the token into the page
                const hiddenInput = document.querySelector('textarea[name="g-recaptcha-response"]');
                if (hiddenInput) {
                    hiddenInput.value = token;
                    console.log('Token injected into hidden input');
                } else {
                    // Create hidden input if it doesn't exist
                    const form = document.querySelector('form');
                    if (form) {
                        const newInput = document.createElement('textarea');
                        newInput.name = 'g-recaptcha-response';
                        newInput.value = token;
                        newInput.style.display = 'none';
                        form.appendChild(newInput);
                        console.log('Created hidden input for token');
                    }
                }
                
                // Return the token as a promise
                return new Promise((resolve) => {
                    setTimeout(() => resolve(token), 500);
                });
            };
            
            console.log('reCAPTCHA.execute method overridden');
        }
        
        // Step 2: Find and prepare the Check button
        const checkButton = document.querySelector('button[type="submit"]');
        if (checkButton) {
            console.log('Found Check button:', checkButton);
            
            // Ensure the button is enabled and visible
            checkButton.disabled = false;
            checkButton.style.display = 'block';
            checkButton.style.visibility = 'visible';
            
            console.log('Check button prepared for clicking');
        } else {
            console.log('Check button not found');
        }
        
        return 'Targeted reCAPTCHA v3 bypass applied';
    })();
    "#;
    
    if let Ok(result) = browser.execute_script(targeted_bypass_script).await {
        println!("📊 Targeted bypass applied: {}", result);
    }
    
    // Wait for the bypass to take effect
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    
    // Now click the Check button with enhanced detection
    println!("\n🔍 Looking for and clicking 'Check' button...");
    
    let enhanced_click_script = r#"
    (function() {
        console.log('Starting enhanced Check button click...');
        
        const results = {
            buttonFound: false,
            buttonClicked: false,
            formSubmitted: false,
            successMessage: null,
            error: null
        };
        
        // Enhanced button detection
        const checkButton = document.querySelector('button[type="submit"]');
        
        if (!checkButton) {
            results.error = 'Check button not found';
            return JSON.stringify(results);
        }
        
        results.buttonFound = true;
        console.log('Found Check button:', checkButton);
        
        // Scroll into view
        checkButton.scrollIntoView({ behavior: 'smooth', block: 'center' });
        
        // Wait for scroll and then click
        setTimeout(() => {
            console.log('Clicking Check button...');
            
            // Get button position
            const rect = checkButton.getBoundingClientRect();
            const centerX = rect.left + rect.width / 2;
            const centerY = rect.top + rect.height / 2;
            
            // Create and dispatch mouse events
            const mouseDown = new MouseEvent('mousedown', {
                bubbles: true, cancelable: true,
                clientX: centerX, clientY: centerY,
                button: 0, buttons: 1
            });
            
            const mouseUp = new MouseEvent('mouseup', {
                bubbles: true, cancelable: true,
                clientX: centerX, clientY: centerY,
                button: 0, buttons: 0
            });
            
            const click = new MouseEvent('click', {
                bubbles: true, cancelable: true,
                clientX: centerX, clientY: centerY,
                button: 0, buttons: 1
            });
            
            // Dispatch events with realistic timing
            checkButton.dispatchEvent(mouseDown);
            
            setTimeout(() => {
                checkButton.dispatchEvent(mouseUp);
                
                setTimeout(() => {
                    checkButton.dispatchEvent(click);
                    results.buttonClicked = true;
                    console.log('Check button clicked successfully');
                    
                    // Submit the form
                    const form = checkButton.closest('form');
                    if (form) {
                        console.log('Submitting form...');
                        
                        // Create a submit event
                        const submitEvent = new Event('submit', { bubbles: true, cancelable: true });
                        form.dispatchEvent(submitEvent);
                        
                        // If submit wasn't prevented, submit the form
                        setTimeout(() => {
                            if (!submitEvent.defaultPrevented) {
                                form.submit();
                                results.formSubmitted = true;
                                console.log('Form submitted');
                            }
                        }, 500);
                    }
                    
                }, 150);
            }, 100);
            
        }, 500);
        
        return JSON.stringify(results);
    })();
    "#;
    
    if let Ok(result) = browser.execute_script(enhanced_click_script).await {
        if let Ok(results) = serde_json::from_str::<serde_json::Value>(&result) {
            println!("📊 Enhanced Click Results:");
            
            if let Some(found) = results.get("buttonFound").and_then(|v| v.as_bool()) {
                println!("   Button Found: {}", found);
            }
            
            if let Some(clicked) = results.get("buttonClicked").and_then(|v| v.as_bool()) {
                println!("   Button Clicked: {}", clicked);
            }
            
            if let Some(submitted) = results.get("formSubmitted").and_then(|v| v.as_bool()) {
                println!("   Form Submitted: {}", submitted);
            }
            
            if let Some(error) = results.get("error").and_then(|v| v.as_str()) {
                println!("   Error: {}", error);
            }
        }
    }
    
    // Wait for the form submission and page response
    println!("\n⏳ Waiting for form submission and response...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    // Check for success message
    println!("\n🔍 Checking for success message...");
    
    let success_check_script = r#"
    (function() {
        console.log('Checking for success message...');
        
        const results = {
            successMessage: null,
            successFound: false,
            pageContent: '',
            error: null
        };
        
        // Wait a moment for the page to update
        setTimeout(() => {
            // Look for success message in various ways
            const successSelectors = [
                '.success',
                '.completed',
                '[data-success="true"]',
                '.verification-success',
                '.human-verified',
                '.verified-human',
                '.alert-success',
                '.message-success'
            ];
            
            let successElement = null;
            let successText = null;
            
            // Check for success elements
            successSelectors.forEach(selector => {
                const elements = document.querySelectorAll(selector);
                if (elements.length > 0 && !successElement) {
                    const element = elements[0];
                    const text = element.textContent || element.innerText || '';
                    if (text.length > 0) {
                        successElement = element;
                        successText = text.trim();
                        console.log('Found success element:', selector, 'with text:', successText);
                    }
                }
            });
            
            // Check page content for success patterns
            const bodyText = document.body.innerText || document.body.textContent || '';
            const successPatterns = [
                'you are verified as a human',
                'verified as human',
                'verification successful',
                'human verified',
                'success',
                'completed',
                'thank you'
            ];
            
            successPatterns.forEach(pattern => {
                if (bodyText.toLowerCase().includes(pattern.toLowerCase()) && !successText) {
                    successText = pattern;
                    console.log('Found success pattern in page:', pattern);
                }
            });
            
            // Check for any new elements that might contain success messages
            const allElements = document.querySelectorAll('*');
            allElements.forEach(element => {
                const text = element.textContent || element.innerText || '';
                if (text.toLowerCase().includes('verified') || text.toLowerCase().includes('success')) {
                    if (!successText) {
                        successText = text.trim();
                        console.log('Found success in element:', successText);
                    }
                }
            });
            
            if (successText) {
                results.successMessage = successText;
                results.successFound = true;
            }
            
            results.pageContent = bodyText.substring(0, 500);
            
        }, 2000);
        
        return JSON.stringify(results);
    })();
    "#;
    
    if let Ok(results_str) = browser.execute_script(success_check_script).await {
        if let Ok(results) = serde_json::from_str::<serde_json::Value>(&results_str) {
            println!("\n📊 Success Check Results:");
            
            if let Some(success_found) = results.get("successFound").and_then(|v| v.as_bool()) {
                println!("   Success Found: {}", success_found);
            }
            
            if let Some(success_message) = results.get("successMessage").and_then(|v| v.as_str()) {
                println!("   Success Message: {}", success_message);
            }
            
            if let Some(page_content) = results.get("pageContent").and_then(|v| v.as_str()) {
                println!("   Page Content Sample: {}", page_content);
            }
        }
    }
    
    // Take final screenshot
    println!("\n📸 Taking final screenshot...");
    browser.take_screenshot("docs/screenshots/recaptcha_after_check_click.png").await?;
    
    // Close browser
    browser.quit().await?;
    
    println!("\n🎉 **Targeted reCAPTCHA v3 Test Completed** 🎉");
    println!("======================================");
    println!("✅ Screenshots saved:");
    println!("   📸 docs/screenshots/recaptcha_original.png");
    println!("   📸 docs/screenshots/recaptcha_after_check_click.png");
    
    Ok(())
}
