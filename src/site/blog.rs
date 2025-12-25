// luflow.net web site
// AGPL-3.0 License (see LICENSE)

use chrono::{Datelike, Local};
use sailfish::Template;
use std::cmp;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::site::blog_post::BlogPost;
use crate::site::helper::Helper;

#[derive(Clone)]
pub struct BlogShared {
    inner: Arc<Mutex<BlogSharedInner>>,
}

struct BlogSharedInner {
    posts: Vec<BlogPost>,
}

impl BlogShared {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(BlogSharedInner { posts: Vec::new() })),
        }
    }

    pub fn insert_post(&self, post: BlogPost) {
        let mut lock = self.inner.lock().unwrap();
        lock.posts.push(post);
    }

    pub fn get_posts(&self) -> Vec<BlogPost> {
        let lock = self.inner.lock().unwrap();
        return lock.posts.clone();
    }

    pub fn sort_posts(&self) {
        let mut lock = self.inner.lock().unwrap();
        lock.posts
            .sort_by(|a, b| b.get_published_date().cmp(a.get_published_date()));
    }
}

#[derive(Template)]
#[template(path = "blog_overview.stpl")]
pub struct Blog {
    base_dir: String,
    num_previews_per_page: usize,
    shared_data: BlogShared,
    blog_posts: Vec<BlogPost>,
    topic_blog_indices: HashMap<String, Vec<usize>>,
    topics: Vec<String>,
    topics_sanitized: Vec<String>,
    year_blog_indices: HashMap<String, Vec<usize>>,
    years: Vec<String>,
    overview_page_url: String,
    overview_current_page: usize,
    overview_num_pages: usize,
    overview_offset: usize,
    overview_num_posts: usize,
    overview_keywords: String,
    overview_title: String,
    overview_topic: String,
    overview_topic_sanitized: String,
    overview_year: String,
    overview_type: i32,
}

impl Blog {
    pub fn new(base_dir: String, num_previews_per_page: usize) -> Blog {
        Blog {
            base_dir: base_dir,
            num_previews_per_page: num_previews_per_page,
            shared_data: BlogShared::new(),
            blog_posts: Vec::new(),
            topic_blog_indices: HashMap::new(),
            topics: Vec::new(),
            topics_sanitized: Vec::new(),
            year_blog_indices: HashMap::new(),
            years: Vec::new(),
            overview_page_url: String::new(),
            overview_current_page: 0,
            overview_num_pages: 0,
            overview_offset: 0,
            overview_num_posts: 0,
            overview_keywords: String::new(),
            overview_title: String::new(),
            overview_topic: String::new(),
            overview_topic_sanitized: String::new(),
            overview_year: String::new(),
            overview_type: 0,
        }
    }

    pub async fn generate(&mut self) {
        // create output dirs for topic, year and feeds:
        self.create_output_dirs();

        // parse all the markdown files in 'blog-posts' folder:
        self.parse_markdown_files().await;

        // generate all individual blog posts:
        self.generate_blog_posts().await;

        // generate blog overview:
        self.generate_overview_posts();

        // generate blog overview by topic:
        self.generate_overview_topic();

        // generate blog overview by year:
        self.generate_overview_year();

        // generate blog atom feed:
        self.generate_atom_feed();
    }

    pub fn get_latest_blog_posts(&self, num_posts: usize) -> Vec<BlogPost> {
        let mut blog_posts = Vec::new();

        for i in 0..num_posts {
            if let Some(value) = self.blog_posts.get(i) {
                blog_posts.push(value.clone());
            }
        }

        return blog_posts;
    }

    fn create_output_dirs(&self) {
        Helper::create_dir_all(&Helper::get_output_dir().join(&self.base_dir).join("topic"));
        Helper::create_dir_all(&Helper::get_output_dir().join(&self.base_dir).join("year"));
        Helper::create_dir_all(&Helper::get_output_dir().join("feeds"));
    }

    async fn parse_markdown_files(&mut self) {
        let mut reader = tokio::fs::read_dir("blog-posts").await.unwrap();
        loop {
            if let Some(f) = reader.next_entry().await.unwrap() {
                let mut post = BlogPost::new(self.base_dir.clone());
                post.parse_markdown_file(f.path()).await;

                // store blog post in vector for later lookup:
                self.shared_data.insert_post(post);
            } else {
                break;
            }
        }

        // sort so that posts published latest is first:
        self.shared_data.sort_posts();

        // store locally for faster access:
        self.blog_posts = self.shared_data.get_posts();

        for (i, post) in self.blog_posts.iter().enumerate() {
            // add blog_id for each blog topic for later lookup:
            for topic in post.get_topics() {
                if let Some(indices) = self.topic_blog_indices.get_mut(topic) {
                    indices.push(i);
                } else {
                    self.topic_blog_indices.insert(String::from(topic), vec![i]);
                }
            }

            // add blog_id for each blog year for later lookup:
            let year_str = post.get_year();
            if let Some(indices) = self.year_blog_indices.get_mut(&year_str) {
                indices.push(i);
            } else {
                self.year_blog_indices
                    .insert(String::from(year_str), vec![i]);
            }
        }

        for (key, _value) in &self.topic_blog_indices {
            self.topics.push(String::from(key));
            self.topics_sanitized
                .push(String::from(Helper::sanitize_string(key)));
        }
        // sort by topic name:
        self.topics
            .sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        self.topics_sanitized.sort_by(|a, b| a.cmp(b));

        for (key, _value) in &self.year_blog_indices {
            self.years.push(String::from(key));
        }
        // sort by year:
        self.years.sort_by(|a, b| a.cmp(b));
    }

    async fn generate_blog_posts(&self) {
        for post in &self.blog_posts {
            post.create_output_dir();
            post.generate().await;
        }
    }

    fn generate_overview_posts(&mut self) {
        self.overview_offset = 0;
        let num = self.blog_posts.len() as f32 / self.num_previews_per_page as f32;
        self.overview_num_pages = num.ceil() as usize;
        self.overview_num_posts = self.num_previews_per_page;
        self.overview_keywords = String::from("overview");
        self.overview_title = String::from("Blog Overview");
        self.overview_type = 0;

        for i in 1..self.overview_num_pages + 1 {
            self.overview_current_page = i;
            self.overview_page_url = format!("{}/page/{}", self.base_dir, i);

            if i == self.overview_num_pages {
                self.overview_num_posts = cmp::min(
                    self.blog_posts.len() - self.num_previews_per_page * (i - 1),
                    self.num_previews_per_page,
                );
            }

            if self.overview_current_page == 1 {
                // write page to disk:
                Helper::write_file_sync(
                    &Helper::get_output_dir()
                        .join(&self.base_dir)
                        .join("index.html"),
                    &self.render().unwrap().as_bytes(),
                )
                .unwrap();
            }

            // create dir recursively:
            Helper::create_dir_all(&Helper::get_output_dir().join(&self.overview_page_url));

            // write page to disk:
            Helper::write_file_sync(
                &Helper::get_output_dir()
                    .join(&self.overview_page_url)
                    .join("index.html"),
                &self.render().unwrap().as_bytes(),
            )
            .unwrap();

            self.overview_offset += self.overview_num_posts;
        }
    }

    fn generate_overview_topic(&mut self) {
        for (key, indices) in &self.topic_blog_indices {
            self.overview_offset = 0;
            let num = indices.len() as f32 / self.num_previews_per_page as f32;
            self.overview_num_pages = num.ceil() as usize;
            self.overview_num_posts = self.num_previews_per_page;
            self.overview_keywords = format!("topic, {}", key);
            self.overview_topic = String::from(key);
            self.overview_topic_sanitized = Helper::sanitize_string(key);
            self.overview_type = 1;

            for i in 1..self.overview_num_pages + 1 {
                self.overview_current_page = i;
                self.overview_page_url = format!(
                    "{}/topic/{}/page/{}",
                    self.base_dir,
                    Helper::sanitize_string(key),
                    i
                );
                self.overview_title = format!("Blog posts by topic: {}", key);

                if i == self.overview_num_pages {
                    self.overview_num_posts = cmp::min(
                        indices.len() - self.num_previews_per_page * (i - 1),
                        self.num_previews_per_page,
                    );
                }

                if self.overview_current_page == 1 {
                    // create dir recursively:
                    Helper::create_dir_all(
                        &Helper::get_output_dir()
                            .join(&self.base_dir)
                            .join("topic")
                            .join(Helper::sanitize_string(key)),
                    );

                    // write page to disk:
                    Helper::write_file_sync(
                        &Helper::get_output_dir()
                            .join(&self.base_dir)
                            .join("topic")
                            .join(Helper::sanitize_string(key))
                            .join("index.html"),
                        &self.render().unwrap().as_bytes(),
                    )
                    .unwrap();
                }

                // create dir recursively:
                Helper::create_dir_all(&Helper::get_output_dir().join(&self.overview_page_url));

                // write page to disk:
                Helper::write_file_sync(
                    &Helper::get_output_dir()
                        .join(&self.overview_page_url)
                        .join("index.html"),
                    &self.render().unwrap().as_bytes(),
                )
                .unwrap();

                self.overview_offset += self.overview_num_posts;
            }
        }
    }

    fn generate_overview_year(&mut self) {
        for (key, indices) in &self.year_blog_indices {
            self.overview_offset = 0;
            let num = indices.len() as f32 / self.num_previews_per_page as f32;
            self.overview_num_pages = num.ceil() as usize;
            self.overview_num_posts = self.num_previews_per_page;
            self.overview_keywords = format!("year, {}", key);
            self.overview_year = String::from(key);
            self.overview_type = 2;

            for i in 1..self.overview_num_pages + 1 {
                self.overview_current_page = i;
                self.overview_page_url = format!(
                    "{}/year/{}/page/{}",
                    self.base_dir,
                    Helper::sanitize_string(key),
                    i
                );
                self.overview_title = format!("Blog posts by year: {}", key);

                if i == self.overview_num_pages {
                    self.overview_num_posts = cmp::min(
                        indices.len() - self.num_previews_per_page * (i - 1),
                        self.num_previews_per_page,
                    );
                }

                if self.overview_current_page == 1 {
                    // create dir recursively:
                    Helper::create_dir_all(
                        &Helper::get_output_dir()
                            .join(&self.base_dir)
                            .join("year")
                            .join(Helper::sanitize_string(key)),
                    );

                    // write page to disk:
                    Helper::write_file_sync(
                        &Helper::get_output_dir()
                            .join(&self.base_dir)
                            .join("year")
                            .join(Helper::sanitize_string(key))
                            .join("index.html"),
                        &self.render().unwrap().as_bytes(),
                    )
                    .unwrap();
                }

                // create dir recursively:
                Helper::create_dir_all(&Helper::get_output_dir().join(&self.overview_page_url));

                // write page to disk:
                Helper::write_file_sync(
                    &Helper::get_output_dir()
                        .join(&self.overview_page_url)
                        .join("index.html"),
                    &self.render().unwrap().as_bytes(),
                )
                .unwrap();

                self.overview_offset += self.overview_num_posts;
            }
        }
    }

    fn generate_atom_feed(&self) {
        let mut _feed_data = String::new();
        let date_now = self.get_date_now_for_feed();

        // header info:
        _feed_data = String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
        _feed_data += "<feed xmlns=\"http://www.w3.org/2005/Atom\">\n";
        _feed_data += "    <id>https:/www.luflow.net/feeds/blog.atom</id>\n";
        _feed_data += "    <title>luflow.net Blog</title>\n";
        _feed_data += "    <updated>";
        _feed_data += &date_now;
        _feed_data += "</updated>\n";
        _feed_data += "    <generator>https://codeberg.org/hfsoulz/flow-web.git</generator>\n";
        _feed_data += "    <author>\n";
        _feed_data += "        <name>luflow.net</name>\n";
        _feed_data += "        <uri>https://www.luflow.net/</uri>\n";
        _feed_data += "    </author>\n";
        _feed_data += "    <link rel=\"alternate\" href=\"https:/www.luflow.net/blog/\"/>\n";
        _feed_data += "    <link rel=\"self\" href=\"https:/www.luflow.net/feeds/blog.atom\"/>\n";
        _feed_data +=
            "    <subtitle>This blog is dedicated to free software in general.</subtitle>\n";
        _feed_data += "    <logo>https:/www.luflow.net/static/img/icon.png</logo>\n";
        _feed_data += "    <icon>https:/www.luflow.net/favicon.ico</icon>\n";

        // each blog entry:
        for blog_post in &self.blog_posts {
            _feed_data += "    <entry>\n";

            // author:
            _feed_data += "        <author>\n";
            _feed_data += "            <name>Andreas</name>\n";
            _feed_data += "        </author>\n";

            // title:
            _feed_data += "        <title type=\"html\"><![CDATA[";
            _feed_data += &blog_post.title;
            _feed_data += "]]></title>\n";

            // link href:
            _feed_data += "        <link href=\"https:/www.luflow.net/";
            _feed_data += &self.base_dir;
            _feed_data += "/";
            _feed_data += &blog_post.url;
            _feed_data += "/\"/>\n";

            // id:
            _feed_data += "        <id>https:/www.luflow.net/";
            _feed_data += &self.base_dir;
            _feed_data += "/";
            _feed_data += &blog_post.url;
            _feed_data += "/</id>\n";

            // updated:
            _feed_data += "        <updated>";
            _feed_data += &blog_post.updated_for_feed;
            _feed_data += "</updated>\n";

            // published:
            _feed_data += "        <published>";
            _feed_data += &blog_post.published_for_feed;
            _feed_data += "</published>\n";

            // categories:
            for topic in &blog_post.topics {
                _feed_data += "        <category term=\"";
                _feed_data += &topic;
                _feed_data += "\"/>\n";
            }

            // summary:
            _feed_data += "        <summary type=\"html\"><![CDATA[";
            _feed_data += &blog_post.snippet;
            _feed_data += "]]></summary>\n";

            // content:
            _feed_data += "        <content type=\"html\"><![CDATA[";
            _feed_data += &blog_post.html;
            _feed_data += "]]></content>\n";

            _feed_data += "    </entry>\n";
        }

        // close feed:
        _feed_data += "</feed>";

        // write it out:
        Helper::write_file_sync(
            &Helper::get_output_dir().join("feeds").join("blog.atom"),
            _feed_data.as_bytes(),
        )
        .unwrap();
    }

    fn get_date_now_for_feed(&self) -> String {
        let date = Local::now();

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

        return format!("{}-{}-{}T{}Z", date.year(), month_str, day_str, date.time());
    }
}
