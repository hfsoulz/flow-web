// luflow.net web site
// AGPL-3.0 License (see LICENSE)

use sailfish::Template;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::site::helper::Helper;
use crate::site::screenshot::Screenshot;

#[derive(Clone, Debug)]
pub struct ScreenshotsInfo {
    pub title: String,
    pub url: String,
}

impl ScreenshotsInfo {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            url: String::new(),
        }
    }
}

pub struct ScreenshotsShared {
    state: Mutex<ScreenshotsState>,
}

impl ScreenshotsShared {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(ScreenshotsState::new()),
        }
    }

    pub fn shared(self) -> Arc<Self> {
        Arc::new(self)
    }

    pub fn get_screenshots(&self, num_screenshots: usize, key: &str) -> Vec<Screenshot> {
        let lock = self.state.lock().unwrap();
        let mut screenshots = Vec::new();

        if let Some(value) = lock.screenshots.get(key) {
            for i in 0..num_screenshots {
                if let Some(screenshot) = value.get(i) {
                    screenshots.push(screenshot.clone());
                }
            }
        } else {
            panic!("Did not find any screenshots for key: {}", key);
        }

        screenshots
    }
}

#[derive(Template)]
#[template(path = "screenshots.stpl")]
#[derive(Clone, Debug)]
struct ScreenshotsState {
    screenshots: HashMap<String, Vec<Screenshot>>,
    screenshot_urls: HashMap<String, String>,
    title: String,
    url: String,
}

impl ScreenshotsState {
    pub fn new() -> Self {
        Self {
            screenshots: HashMap::new(),
            screenshot_urls: HashMap::new(),
            title: String::new(),
            url: String::new(),
        }
    }
}

pub async fn generate_screenshots(shared: Arc<ScreenshotsShared>) {
    parse_files(shared.clone(), "screenshots").await;
}

async fn parse_files(shared: Arc<ScreenshotsShared>, base_dir: &str) {
    let mut reader = tokio::fs::read_dir(base_dir).await.unwrap();
    while let Some(f) = reader.next_entry().await.unwrap() {
        let contents = tokio::fs::read_to_string(f.path()).await.unwrap();

        parse_file(shared.clone(), contents);
        update_screenshots(shared.clone());
        generate(shared.clone());
    }
}

fn parse_file(shared: Arc<ScreenshotsShared>, contents: String) {
    let mut screenshots_info = ScreenshotsInfo::new();
    let mut screenshot = Screenshot::new();

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }

        let v: Vec<&str> = line.splitn(2, ':').collect();
        assert_eq!(v.len(), 2);

        if let Some(key) = v.first() {
            if *key == "screenshots_title" {
                if let Some(value) = v.get(1) {
                    screenshots_info.title = String::from(value.trim());
                } else {
                    panic!("Unable to parse field: 'screenshots_title'.");
                }
            } else if *key == "screenshots_url" {
                if let Some(value) = v.get(1) {
                    screenshots_info.url = String::from(value.trim());
                } else {
                    panic!("Unable to parse field: 'screenshots_url'.");
                }
            } else if *key == "title" {
                if let Some(value) = v.get(1) {
                    screenshot.title = String::from(value.trim());
                } else {
                    panic!("Unable to parse field: 'title'.");
                }
            } else if *key == "image_min" {
                if let Some(value) = v.get(1) {
                    screenshot.image_min = String::from(value.trim());
                } else {
                    panic!("Unable to parse field: 'image_min'.");
                }
            } else if *key == "image_big" {
                if let Some(value) = v.get(1) {
                    screenshot.image_big = String::from(value.trim());
                } else {
                    panic!("Unable to parse field: 'image_big'.");
                }
            } else if *key == "url" {
                if let Some(value) = v.get(1) {
                    screenshot.url = String::from(value.trim());
                } else {
                    panic!("Unable to parse field: 'url'.");
                }
            }
        }

        // all info needed for one screenshot:
        if !screenshot.title.is_empty()
            && !screenshot.image_min.is_empty()
            && !screenshot.image_big.is_empty()
            && !screenshot.url.is_empty()
            && !screenshots_info.title.is_empty()
            && !screenshots_info.url.is_empty()
        {
            screenshot.screenshots_title = screenshots_info.title.clone();
            screenshot.screenshots_url = screenshots_info.url.clone();

            {
                let mut lock = shared.state.lock().unwrap();

                if let Some(value) = lock.screenshots.get_mut(&screenshots_info.title) {
                    value.push(screenshot);
                } else {
                    lock.screenshot_urls
                        .insert(screenshots_info.title.clone(), screenshots_info.url.clone());
                    lock.screenshots
                        .insert(screenshots_info.title.clone(), vec![screenshot]);
                }
            }

            // reset for new screenshot:
            screenshot = Screenshot::new();
        }
    }

    let mut lock = shared.state.lock().unwrap();

    lock.title = screenshots_info.title.clone();
    lock.url = screenshots_info.url.clone();
}

fn update_screenshots(shared: Arc<ScreenshotsShared>) {
    // TODO: this could be done more efficiently:
    // (need screenshots for each screenshot as they are linked below)
    let mut lock = shared.state.lock().unwrap();
    for value in lock.screenshots.values_mut() {
        let v = value.clone();
        for s in value {
            s.screenshots = v.clone();
        }
    }
}

fn generate(shared: Arc<ScreenshotsShared>) {
    let lock = shared.state.lock().unwrap();

    for screenshots in lock.screenshots.values() {
        // create output dir needed:
        Helper::create_dir_all(&Helper::get_output_dir().join(&lock.url));

        // write page to disk:
        Helper::write_file_sync(
            &Helper::get_output_dir().join(&lock.url).join("index.html"),
            lock.render().unwrap().as_bytes(),
        )
        .unwrap();

        // generate all individual screenshot pages:
        for screenshot in screenshots {
            screenshot.generate();
        }
    }
}
