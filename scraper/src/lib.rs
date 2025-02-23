use std::error::Error;
use std::thread;
use std::time::Duration;
use thirtyfour::prelude::*;
use rand::Rng;
use url::Url;

pub async fn scrape_debank(address: &str) -> Result<(), Box<dyn Error>> {
    let driver = initialize_driver().await?;

    let base = Url::parse("https://debank.com/profile/")?;

    let url = base.join(address)?;
    println!("url:{}", url);

    driver.goto(url.as_str()).await?;

    thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(10..30)));

    let elem = driver.find(By::XPath("//*[starts-with(@class,'HeaderInfo_totalAssetInner')]")).await?;
    let text = elem.text().await?;
    println!("\tvalue: {:?}", text);

    driver.quit().await?;

    Ok(())
}

pub async fn scrape_bitcoin_explorer(address: &str) -> Result<(), Box<dyn Error>> {
    let driver = initialize_driver().await?;

    let base = Url::parse("https://bitcoinexplorer.org/address/")?;

    let url = base.join(address)?;
    println!("url:{}", url);

    driver.goto(url.as_str()).await?;

    thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(10..30)));

    let elem = driver.find(By::XPath("/html/body/div/div/div[1]/div[3]/div[1]/div[3]/div/div/div/div[3]/div[2]/span/span[1]")).await?;
    let text = elem.text().await?;
    println!("\tvalue: {:?}", text);

    driver.quit().await?;

    Ok(())
}

async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:50520", caps).await?;
    driver.maximize_window().await?;
    Ok(driver)
}

