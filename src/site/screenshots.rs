// luflow.net web site
// AGPL-3.0 License (see LICENSE)

use sailfish::Template;
use std::collections::HashMap;

use crate::site::helper::Helper;
use crate::site::screenshot::Screenshot;

#[derive(Clone, Debug)]
pub struct ScreenshotsInfo {
    pub title: String,
    pub url: String,
}

impl ScreenshotsInfo {
    pub fn new() -> ScreenshotsInfo {
        ScreenshotsInfo {
            title: String::new(),
            url: String::new(),
        }
    }
}

#[derive(Template)]
#[template(path = "screenshots.stpl")]
#[derive(Clone, Debug)]
pub struct Screenshots {
    pub base_dir: String,

    pub screenshots: HashMap<String, Vec<Screenshot>>,
    pub screenshot_urls: HashMap<String, String>,
    pub title: String,
    pub url: String,
}

impl Screenshots {
    pub fn new(base_dir: String) -> Screenshots {
        Screenshots {
            base_dir: base_dir,
            screenshots: HashMap::new(),
            screenshot_urls: HashMap::new(),
            title: String::new(),
            url: String::new(),
        }
    }

    pub async fn generate(&mut self) {
        // create output dir needed:
        Helper::create_dir_all(&Helper::get_output_dir().join(&self.base_dir));

        // parse input files:
        self.parse_files().await;

        // generate all individual screenshot pages:
        self.generate_screenshots().await;
    }

    pub fn get_screenshots(&self, num_screenshots: usize, key: &str) -> Vec<Screenshot> {
        let mut screenshots = Vec::new();

        if let Some(value) = self.screenshots.get(key) {
            for i in 0..num_screenshots {
                if let Some(screenshot) = value.get(i) {
                    screenshots.push(screenshot.clone());
                }
            }
        } else {
            panic!("Did not find any screenshots for key: {}", key);
        }

        return screenshots;
    }

    async fn parse_files(&mut self) {
        let mut reader = tokio::fs::read_dir(&self.base_dir).await.unwrap();
        loop {
            if let Some(f) = reader.next_entry().await.unwrap() {
                let contents = tokio::fs::read_to_string(f.path()).await.unwrap();
                let mut screenshots_info = ScreenshotsInfo::new();
                let mut screenshot = Screenshot::new();

                for line in contents.lines() {
                    if line.len() == 0 {
                        continue;
                    }

                    let v: Vec<&str> = line.splitn(2, ':').collect();
                    assert_eq!(v.len(), 2);

                    if let Some(key) = v.get(0) {
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
                    if screenshot.title.len() > 0
                        && screenshot.image_min.len() > 0
                        && screenshot.image_big.len() > 0
                        && screenshot.url.len() > 0
                        && screenshots_info.title.len() > 0
                        && screenshots_info.url.len() > 0
                    {
                        screenshot.screenshots_title = screenshots_info.title.clone();
                        screenshot.screenshots_url = screenshots_info.url.clone();

                        if let Some(value) = self.screenshots.get_mut(&screenshots_info.title) {
                            value.push(screenshot);
                        } else {
                            self.screenshot_urls.insert(
                                screenshots_info.title.clone(),
                                screenshots_info.url.clone(),
                            );
                            self.screenshots
                                .insert(screenshots_info.title.clone(), vec![screenshot]);
                        }

                        // reset for new screenshot:
                        screenshot = Screenshot::new();
                    }
                }
            } else {
                break;
            }
        }

        // TODO: this could be done more efficiently:
        // (need screenshots for each screenshot as they are linked below)
        for (_key, value) in &mut self.screenshots {
            let v = value.clone();
            for s in value {
                s.screenshots = v.clone();
            }
        }
    }

    // generate all individual screenshot pages:
    async fn generate_screenshots(&mut self) {
        for (key, screenshots) in &self.screenshots {
            self.title = String::from(key);
            if let Some(value) = self.screenshot_urls.get(key) {
                self.url = value.clone();
            }

            // create output dir needed:
            Helper::create_dir_all(&Helper::get_output_dir().join(&self.url));

            // write page to disk:
            Helper::write_file_sync(
                &Helper::get_output_dir().join(&self.url).join("index.html"),
                &self.render().unwrap().as_bytes(),
            )
            .unwrap();

            // generate all individual screenshot pages:
            for screenshot in screenshots {
                screenshot.generate().await;
            }
        }
    }
}
