#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_take_screenshot() {
        let result = take_screenshot().await;
        match result {
            Ok(pixbuf) => {
                assert!(pixbuf.width() > 0);
                assert!(pixbuf.height() > 0);
                println!("Screenshot test passed: {}x{}", pixbuf.width(), pixbuf.height());
            }
            Err(e) => {
                assert!(false, "Screenshot test failed (expected in headless): {:#?}", e);
            }
        }
    }
}