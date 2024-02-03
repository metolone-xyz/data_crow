use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::sync::Semaphore;
use tokio::task;

#[derive(Serialize, Deserialize, Debug)]
struct VideoData {
    index: String,
    instruction: String,
    input: String,
    output: String,
    category: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let re_japanese =
        Regex::new(r"[\p{Script=Han}\p{Script=Hiragana}\p{Script=Katakana}ー]").unwrap();
    let client = Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            // HTTP/2の有効化
            //.http2_prior_knowledge()
            .build()?,
    );

    let skipped_videos = Arc::new(Mutex::new(0));
    let total_videos = Arc::new(Mutex::new(0));

    //データ保存用ベクター
    let video_data_list = Arc::new(Mutex::new(Vec::new()));

    // セマフォの数を環境に応じて調整
    let semaphore = Arc::new(Semaphore::new(1000));
    //トップページから動画を取得

    //日本のホームページ

    
    for i in 1..=37 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!("https://www.xvideos.com/lang/japanese/{}", i);
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<Option<i32>, reqwest::Error>(None), // Specify the type for `None`
                };
                let video_document = Html::parse_document(&video_html);

                let title_selector = Selector::parse("title").unwrap();
                let keywords_selector = Selector::parse(r#"meta[name="keywords"]"#).unwrap();

                if let Some(title_element) = video_document.select(&title_selector).next() {
                    let mut title = title_element.text().collect::<String>();
                    title = title.replace("- XVIDEOS.COM", "").trim().to_string();

                    let mut total_videos_lock = total_videos_clone.lock().unwrap();

                    /*
                    if !re_japanese_clone.is_match(&title) || title.chars().count() >= 16{
                        let mut skipped_videos_lock = skipped_videos_clone.lock().unwrap();
                        *skipped_videos_lock += 1;
                        return Ok(Some(1)); // Skipped video
                    }
                    */

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }
    
    //Japanese 日本　無修正　高画質

    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!("https://www.xvideos.com/?k=japanese+%E6%97%A5%E6%9C%AC+%E7%84%A1%E4%BF%AE%E6%AD%A3+%E9%AB%98%E7%94%BB%E8%B3%AA&p={}", i);
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<Option<i32>, reqwest::Error>(None), // Specify the type for `None`
                };
                let video_document = Html::parse_document(&video_html);

                let title_selector = Selector::parse("title").unwrap();
                let keywords_selector = Selector::parse(r#"meta[name="keywords"]"#).unwrap();

                if let Some(title_element) = video_document.select(&title_selector).next() {
                    let mut title = title_element.text().collect::<String>();
                    title = title.replace("- XVIDEOS.COM", "").trim().to_string();

                    let mut total_videos_lock = total_videos_clone.lock().unwrap();

                    /* 
                    if !re_japanese_clone.is_match(&title) {
                        let mut skipped_videos_lock = skipped_videos_clone.lock().unwrap();
                        *skipped_videos_lock += 1;
                        return Ok(Some(1)); // Skipped video
                    }*/

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //Japanese milf
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!("https://www.xvideos.com/?k=japanese+milf&p={}", i);
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //Japanese Teen
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!("https://www.xvideos.com/?k=japanese+teen&p={}", i);
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //日本語中出し
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!("https://www.xvideos.com/?k=%E6%97%A5%E6%9C%AC%E4%BA%BA+%E7%84%A1%E4%BF%AE%E6%AD%A3+%E4%B8%AD%E5%87%BA%E3%81%97&p={}", i);
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //しろうと
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!(
            "https://www.xvideos.com/?k=%E3%81%97%E3%82%8D%E3%81%86%E3%81%A8&p={}",
            i
        );
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //おなにー
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!(
            "https://www.xvideos.com/?k=%E3%81%8A%E3%81%AA%E3%81%AB%E3%83%BC&p={}",
            i
        );
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //JK
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!("https://www.xvideos.com/?k=JK&p={}", i);
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }
    //巨乳
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!("https://www.xvideos.com/tags/%E5%B7%A8%E4%B9%B3/{}", i);
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //むしゅうせい japanese
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!("https://www.xvideos.com/?k=%E3%82%80%E3%81%97%E3%82%85%E3%81%86%E3%81%9B%E3%81%84+japanese&p={}", i);
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }
    //---------------だいたい4000後半から5000前半までの動画が日本語になっている-----------------

    //ふぇらちお japanese
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!("https://www.xvideos.com/?k=%E3%81%B5%E3%81%87%E3%82%89%E3%81%A1%E3%81%8A+japanese&p={}", i);
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //あにめ
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!(
            "https://www.xvideos.com/?k=%E3%81%82%E3%81%AB%E3%82%81&p={}",
            i
        );
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //貧乳
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!("https://www.xvideos.com/?k=%E8%B2%A7%E4%B9%B3&p={}", i);
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //NTR
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!("https://www.xvideos.com/?k=NTR&p={}", i);
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //寝取られ japanese
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!(
            "https://www.xvideos.com/?k=%E5%AF%9D%E5%8F%96%E3%82%89%E3%82%8C&p={}",
            i
        );
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //企画もの japanese
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!(
            "https://www.xvideos.com/?k=%E4%BC%81%E7%94%BB%E3%82%82%E3%81%AE&p={}",
            i
        );
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //ギャル https://www.xvideos.com/?k=%E3%82%AE%E3%83%A3%E3%83%AB&p=1
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!(
            "https://www.xvideos.com/?k=%E3%82%AE%E3%83%A3%E3%83%AB&p={}",
            i
        );
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    // セックス https://www.xvideos.com/?k=%E3%82%BB%E3%83%83%E3%82%AF%E3%82%B9&p=1
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!(
            "https://www.xvideos.com/?k=%E3%82%BB%E3%83%83%E3%82%AF%E3%82%B9&p={}",
            i
        );
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //爆乳 https://www.xvideos.com/?k=%E7%88%86%E4%B9%B3&p=1
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!("https://www.xvideos.com/?k=%E7%88%86%E4%B9%B3&p={}", i);
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //手コキ https://www.xvideos.com/?k=%E6%89%8B%E3%82%B3%E3%82%AD&p=1
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!(
            "https://www.xvideos.com/?k=%E6%89%8B%E3%82%B3%E3%82%AD&p={}",
            i
        );
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }
    
    //個人撮影 https://www.xvideos.com/?k=%E5%80%8B%E4%BA%BA%E6%92%AE%E5%BD%B1&p=1
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!(
            "https://www.xvideos.com/?k=%E5%80%8B%E4%BA%BA%E6%92%AE%E5%BD%B1&p={}",
            i
        );
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //japanese wife https://www.xvideos.com/?k=japanese+wife&p=1
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!(
            "https://www.xvideos.com/?k=japanese+wife&p={}",
            i
        );
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //japanese amateur creampie
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!(
            "https://www.xvideos.com/?k=japanese+amateur+creampie&p={}",
            i
        );
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }
    
    //japanese amateur homemade https://www.xvideos.com/?k=japanese+amateur+homemade&p=1
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!(
            "https://www.xvideos.com/?k=japanese+amateur+homemade&p={}",
            i
        );
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //japanese amateur enko https://www.xvideos.com/?k=japanese+amateur+enko&p=1
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!(
            "https://www.xvideos.com/?k=japanese+amateur+enko&p={}",
            i
        );
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }
    
    //japanese+amateur+handjob https://www.xvideos.com/?k=japanese+amateur+handjob&p=1
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!(
            "https://www.xvideos.com/?k=japanese+amateur+handjob&p={}",
            i
        );
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //japanese ntr https://www.xvideos.com/?k=japanese+ntr&p=1
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!(
            "https://www.xvideos.com/?k=japanese+ntr&p={}",
            i
        );
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<Option<i32>, reqwest::Error>(None), // Specify the type for `None`
                };
                let video_document = Html::parse_document(&video_html);

                let title_selector = Selector::parse("title").unwrap();
                let keywords_selector = Selector::parse(r#"meta[name="keywords"]"#).unwrap();

                if let Some(title_element) = video_document.select(&title_selector).next() {
                    let mut title = title_element.text().collect::<String>();
                    title = title.replace("- XVIDEOS.COM", "").trim().to_string();

                    let mut total_videos_lock = total_videos_clone.lock().unwrap();

                    /*
                    if !re_japanese_clone.is_match(&title) {
                        let mut skipped_videos_lock = skipped_videos_clone.lock().unwrap();
                        *skipped_videos_lock += 1;
                        return Ok(Some(1)); // Skipped video
                    }
                    */

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
                Ok::<_, reqwest::Error>(None)
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await?;
        }
    }

    //hentai https://www.xvideos.com/?k=hentai
    for i in 1..149 {
        println!(
            "-----------------------Page: {} -------------------------",
            i
        );
        let search_url = format!(
            "https://www.xvideos.com/?k=hentai&p=1{}",
            i
        );
        let html_content = match client.get(&search_url).send().await {
            Ok(response) => response.text().await?,
            Err(e) => {
                eprintln!("Error occurred while fetching {}: {}", search_url, e);
                continue;
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
            // タスクの生成と同時にセマフォの許可を取得
            let permit = semaphore.clone().acquire_owned().await?;
            let client_clone = Arc::clone(&client);
            let re_japanese_clone = re_japanese.clone();
            let skipped_videos_clone = Arc::clone(&skipped_videos);
            let total_videos_clone = Arc::clone(&total_videos);
            let video_data_list = Arc::clone(&video_data_list);

            let task = task::spawn(async move {
                let video_html = match client_clone.get(&url).send().await {
                    Ok(response) => response.text().await?,
                    Err(_) => return Ok::<_, reqwest::Error>(None), // エラー時はタスクを終了
                };
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
                        return Ok(Some(1)); // Skipped video
                    }

                    *total_videos_lock += 1;
                    let video_num = *total_videos_lock;

                    if let Some(keywords_element) = video_document.select(&keywords_selector).next()
                    {
                        if let Some(keywords) = keywords_element.value().attr("content") {
                            println!("Title {}: {}", video_num, title);
                            println!("Keywords: {}", keywords);

                            let video_data = VideoData {
                                index: video_num.to_string(),
                                instruction: keywords.to_string(),
                                input: "".to_string(),
                                output: title.to_string(),
                                category: "tag2text".to_string(),
                            };

                            // 共有データリストに動画データを追加
                            let mut video_data_list = video_data_list.lock().unwrap();
                            video_data_list.push(video_data);
                        }
                    }
                }
                drop(permit);
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

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    let video_data_list = Arc::try_unwrap(video_data_list)
        .unwrap()
        .into_inner()
        .unwrap();
    let json_data = serde_json::to_string(&video_data_list)?;
    std::fs::write("video_data.json", json_data)?;

    Ok(())
}
