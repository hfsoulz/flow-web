// luflow.net web site
// AGPL-3.0 License (see LICENSE)

use chrono::{Datelike, NaiveDateTime};
use sailfish::Template;
use std::path::PathBuf;

use crate::site::helper::Helper;

#[derive(Template)]
#[template(path = "blog_post.stpl")]
#[derive(Clone, Debug)]
pub struct BlogPost {
    pub base_dir: String,

    pub author: String,
    pub published: NaiveDateTime,
    pub published_for_feed: String,
    pub updated: NaiveDateTime,
    pub updated_for_feed: String,
    pub topics_comma_separated: String,
    pub topics: Vec<String>,
    pub topics_sanitized: Vec<String>,
    pub topic_base_dir: String,
    pub title: String,
    pub snippet: String,
    pub html: String,
    pub url: String,
}

impl BlogPost {
    pub fn new(base_dir: String) -> BlogPost {
        BlogPost {
            base_dir: base_dir,

            author: String::new(),
            published: NaiveDateTime::parse_from_str("2000-01-01 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            published_for_feed: String::new(),
            updated: NaiveDateTime::parse_from_str("2000-01-01 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            updated_for_feed: String::new(),
            topics_comma_separated: String::new(),
            topics: Vec::new(),
            topics_sanitized: Vec::new(),
            topic_base_dir: String::from("topic"),
            title: String::new(),
            snippet: String::new(),
            html: String::new(),
            url: String::new(),
        }
    }

    pub fn get_topics(&self) -> &Vec<String> {
        return &self.topics;
    }

    pub fn get_year(&self) -> String {
        return self.published.year().to_string();
    }

    pub fn get_published_date(&self) -> &NaiveDateTime {
        return &self.published;
    }

    fn get_date_for_feed(&self, date: &NaiveDateTime) -> String {
        let month = date.month();
        let day = date.day();

        let mut month_str = month.to_string();
        let mut day_str = day.to_string();

        if month < 10 {
            month_str = format!("0{}", month);
        }

        if day < 10 {
            day_str = format!("0{}", day);
        }

        return format!(
            "{}-{}-{}T{}.000Z",
            date.year(),
            month_str,
            day_str,
            date.time()
        );
    }

    pub async fn parse_markdown_file(&mut self, path: PathBuf) {
        let contents = tokio::fs::read_to_string(path).await.unwrap();
        let mut post_start_found = false;
        let mut markdown = String::new();

        for line in contents.lines() {
            if !post_start_found && line.len() > 0 {
                if line == "---" {
                    post_start_found = true;
                    continue;
                }

                let v: Vec<&str> = line.splitn(2, ':').collect();
                assert_eq!(v.len(), 2);

                if let Some(key) = v.get(0) {
                    if *key == "author" {
                        if let Some(value) = v.get(1) {
                            self.author = String::from(value.trim());
                        } else {
                            panic!("Unable to parse field: 'author'.");
                        }
                    } else if *key == "published" {
                        if let Some(value) = v.get(1) {
                            self.published =
                                NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S").unwrap();
                            self.published_for_feed = self.get_date_for_feed(&self.published);
                        } else {
                            panic!("Unable to parse field: 'published'.");
                        }
                    } else if *key == "updated" {
                        if let Some(value) = v.get(1) {
                            self.updated =
                                NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S").unwrap();
                            self.updated_for_feed = self.get_date_for_feed(&self.updated);
                        } else {
                            panic!("Unable to parse field: 'updated'.");
                        }
                    } else if *key == "topics" {
                        if let Some(value) = v.get(1) {
                            self.topics_comma_separated = String::from(value.trim());
                            let v = self.topics_comma_separated.split(",");
                            for topic in v {
                                let topic_trimmed = topic.trim();
                                self.topics.push(String::from(topic_trimmed));
                                self.topics_sanitized
                                    .push(Helper::sanitize_string(topic_trimmed));
                            }
                        } else {
                            panic!("Unable to parse field: 'topics'.");
                        }
                    } else if *key == "title" {
                        if let Some(value) = v.get(1) {
                            self.title = String::from(value.trim());
                            self.url = Helper::sanitize_string(&self.title);
                        } else {
                            panic!("Unable to parse field: 'title'.");
                        }
                    } else if *key == "snippet" {
                        if let Some(value) = v.get(1) {
                            self.snippet = String::from(value.trim());
                        } else {
                            panic!("Unable to parse field: 'snippet'.");
                        }
                    }
                }
            } else {
                if markdown.len() > 0 {
                    markdown += "\n";
                }
                markdown += line;
            }
        }

        // convert markdown to html:
        self.html = markdown::to_html(&markdown);
    }

    pub fn create_output_dir(&self) {
        Helper::create_dir_all(
            &Helper::get_output_dir()
                .join(&self.base_dir)
                .join(&self.url),
        );
    }

    pub async fn generate(&self) {
        Helper::write_file(
            &Helper::get_output_dir()
                .join(&self.base_dir)
                .join(&self.url)
                .join("index.html"),
            &self.render().unwrap().as_bytes(),
        )
        .await
        .unwrap();
    }
}
