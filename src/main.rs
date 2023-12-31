use reqwest;
use scraper::{Html, Selector};
use regex::Regex;
use std::collections::HashSet;
use tokio::task;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let re_japanese = Regex::new(r"[\p{Script=Han}\p{Script=Hiragana}\p{Script=Katakana}ー]").unwrap();
    let client = Arc::new(reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(60))
    .build()?);
    
    let skipped_videos = Arc::new(Mutex::new(0));
    let total_videos = Arc::new(Mutex::new(0));


    for i in 1..=37 {
        println!("-----------------------Page: {} -------------------------", i);
        let search_url = format!("https://www.xvideos.com/lang/japanese/{}", i);
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;  // Skip this iteration and proceed with the next one
            }
        };


        let document = Html::parse_document(&html_content);

        let video_url_selector = Selector::parse("div.thumb a").unwrap();
        let mut video_urls = HashSet::new();

        for element in document.select(&video_url_selector) {
            if let Some(href) = element.value().attr("href") {
                video_urls.insert(format!("https://www.xvideos.com{}", href));
            }
        }

        let mut tasks = vec![];
        for url in video_urls {
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);

            let task = task::spawn(async move {
                let video_html = client_clone.get(&url).send().await?.text().await?;
                let video_document = Html::parse_document(&video_html);

                let title_selector = Selector::parse("title").unwrap();
                let keywords_selector = Selector::parse(r#"meta[name="keywords"]"#).unwrap();
               

                if let Some(title_element) = video_document.select(&title_selector).next() {
                    let mut title = title_element.text().collect::<String>();
                    title = title.replace("- XVIDEOS.COM", "").trim().to_string();

                    let mut total_videos_lock = total_videos_clone.lock().unwrap();

                    if !re_japanese_clone.is_match(&title) {
                        let mut skipped_videos_lock = skipped_videos_clone.lock().unwrap();
                        *skipped_videos_lock += 1;
                        return Ok(Some(1));  // Skipped video
                    }
                    
                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next() {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);
                            
                        }
                    }
                }

                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
           let _ = task.await?;
        }
    }

    println!("Skipped Videos: {}", *skipped_videos.lock().unwrap());
    println!("Total Titles: {}", *total_videos.lock().unwrap());

    Ok(())
}

