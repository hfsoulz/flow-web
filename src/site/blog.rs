// luflow.net web site
// AGPL-3.0 License (see LICENSE)

use chrono::{Datelike, Local};
use sailfish::Template;
use std::cmp;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::site::blog_post::BlogPost;
use crate::site::blog_post::parse_markdown_file;
use crate::site::helper::Helper;

pub struct BlogShared {
    state: Mutex<BlogState>,
}

impl BlogShared {
    pub fn new(base_dir: String, num_previews_per_page: usize) -> Self {
        Self {
            state: Mutex::new(BlogState::new(base_dir, num_previews_per_page)),
        }
    }
}

#[derive(Template)]
#[template(path = "blog_overview.stpl")]
pub struct BlogState {
    base_dir: String,
    num_previews_per_page: usize,
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

impl BlogState {
    pub fn new(base_dir: String, num_previews_per_page: usize) -> Self {
        Self {
            base_dir: base_dir,
            num_previews_per_page: num_previews_per_page,
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
}

pub async fn generate_blog(shared: Arc<BlogShared>, base_dir: String) {
    // create output dirs for topic, year and feeds:
    create_output_dirs(shared.clone());

    // parse all the markdown files in 'blog-posts' folder:
    parse_markdown_files(shared.clone(), base_dir).await;

    // generate all individual blog posts:
    generate_blog_posts(shared.clone()).await;

    // generate blog overview:
    generate_overview_posts(shared.clone()).await;

    // generate blog overview by topic:
    generate_overview_topic(shared.clone()).await;

    // generate blog overview by year:
    generate_overview_year(shared.clone()).await;

    // generate blog atom feed:
    generate_atom_feed(shared.clone()).await;
}

pub fn get_latest_blog_posts(shared: Arc<BlogShared>, num_posts: usize) -> Vec<BlogPost> {
    let lock = shared.state.lock().unwrap();
    let mut blog_posts = Vec::new();

    for i in 0..num_posts {
        if let Some(value) = lock.blog_posts.get(i) {
            blog_posts.push(value.clone());
        }
    }

    return blog_posts;
}

fn create_output_dirs(shared: Arc<BlogShared>) {
    let lock = shared.state.lock().unwrap();

    Helper::create_dir_all(&Helper::get_output_dir().join(&lock.base_dir).join("topic"));
    Helper::create_dir_all(&Helper::get_output_dir().join(&lock.base_dir).join("year"));
    Helper::create_dir_all(&Helper::get_output_dir().join("feeds"));
}

async fn parse_markdown_files(shared: Arc<BlogShared>, base_dir: String) {
    let mut reader = tokio::fs::read_dir("blog-posts").await.unwrap();
    let mut tasks = vec![];
    loop {
        if let Some(f) = reader.next_entry().await.unwrap() {
            tasks.push(tokio::spawn(parse_markdown_file(
                f.path(),
                base_dir.clone(),
            )));
        } else {
            break;
        }
    }

    // await all created blog_posts:
    let mut blog_posts = Vec::with_capacity(tasks.len());
    for task in tasks {
        blog_posts.push(task.await.unwrap());
    }

    // sort so that latest is first:
    blog_posts.sort_by(|a, b| b.get_published_date().cmp(a.get_published_date()));

    let mut topic_blog_indices: HashMap<String, Vec<usize>> = HashMap::new();
    let mut year_blog_indices: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, post) in blog_posts.iter().enumerate() {
        // add blog_id for each blog topic for later lookup:
        for topic in post.get_topics() {
            if let Some(indices) = topic_blog_indices.get_mut(topic) {
                indices.push(i);
            } else {
                topic_blog_indices.insert(String::from(topic), vec![i]);
            }
        }

        // add blog_id for each blog year for later lookup:
        let year_str = post.get_year();
        if let Some(indices) = year_blog_indices.get_mut(&year_str) {
            indices.push(i);
        } else {
            year_blog_indices.insert(String::from(year_str), vec![i]);
        }
    }

    // move into state:
    let mut lock = shared.state.lock().unwrap();
    lock.blog_posts = blog_posts;

    for (key, _value) in &topic_blog_indices {
        lock.topics.push(String::from(key));
        lock.topics_sanitized
            .push(String::from(Helper::sanitize_string(key)));
    }
    // sort by topic name:
    lock.topics
        .sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    lock.topics_sanitized.sort_by(|a, b| a.cmp(b));

    for (key, _value) in &year_blog_indices {
        lock.years.push(String::from(key));
    }
    // sort by year:
    lock.years.sort_by(|a, b| a.cmp(b));

    // move created hashmaps:
    lock.topic_blog_indices = topic_blog_indices;
    lock.year_blog_indices = year_blog_indices;
}

async fn generate_blog_posts(shared: Arc<BlogShared>) {
    let lock = shared.state.lock().unwrap();

    for post in &lock.blog_posts {
        post.create_output_dir();
        post.generate();
    }
}

async fn generate_overview_posts(shared: Arc<BlogShared>) {
    let mut lock = shared.state.lock().unwrap();

    lock.overview_offset = 0;
    let num = lock.blog_posts.len() as f32 / lock.num_previews_per_page as f32;
    lock.overview_num_pages = num.ceil() as usize;
    lock.overview_num_posts = lock.num_previews_per_page;
    lock.overview_keywords = String::from("overview");
    lock.overview_title = String::from("Blog Overview");
    lock.overview_type = 0;

    for i in 1..lock.overview_num_pages + 1 {
        lock.overview_current_page = i;
        lock.overview_page_url = format!("{}/page/{}", lock.base_dir, i);

        if i == lock.overview_num_pages {
            lock.overview_num_posts = cmp::min(
                lock.blog_posts.len() - lock.num_previews_per_page * (i - 1),
                lock.num_previews_per_page,
            );
        }

        if lock.overview_current_page == 1 {
            // write page to disk:
            Helper::write_file_sync(
                &Helper::get_output_dir()
                    .join(&lock.base_dir)
                    .join("index.html"),
                &lock.render().unwrap().as_bytes(),
            )
            .unwrap();
        }

        // create dir recursively:
        Helper::create_dir_all(&Helper::get_output_dir().join(&lock.overview_page_url));

        // write page to disk:
        Helper::write_file_sync(
            &Helper::get_output_dir()
                .join(&lock.overview_page_url)
                .join("index.html"),
            &lock.render().unwrap().as_bytes(),
        )
        .unwrap();

        lock.overview_offset += lock.overview_num_posts;
    }
}

async fn generate_overview_topic(shared: Arc<BlogShared>) {
    let mut lock = shared.state.lock().unwrap();

    // TODO: How to solve this without a clone??
    for (key, indices) in &lock.topic_blog_indices.clone() {
        lock.overview_offset = 0;
        let num = indices.len() as f32 / lock.num_previews_per_page as f32;
        lock.overview_num_pages = num.ceil() as usize;
        lock.overview_num_posts = lock.num_previews_per_page;
        lock.overview_keywords = format!("topic, {}", key);
        lock.overview_topic = String::from(key);
        lock.overview_topic_sanitized = Helper::sanitize_string(key);
        lock.overview_type = 1;

        for i in 1..lock.overview_num_pages + 1 {
            lock.overview_current_page = i;
            lock.overview_page_url = format!(
                "{}/topic/{}/page/{}",
                lock.base_dir,
                Helper::sanitize_string(key),
                i
            );
            lock.overview_title = format!("Blog posts by topic: {}", key);

            if i == lock.overview_num_pages {
                lock.overview_num_posts = cmp::min(
                    indices.len() - lock.num_previews_per_page * (i - 1),
                    lock.num_previews_per_page,
                );
            }

            if lock.overview_current_page == 1 {
                // create dir recursively:
                Helper::create_dir_all(
                    &Helper::get_output_dir()
                        .join(&lock.base_dir)
                        .join("topic")
                        .join(Helper::sanitize_string(key)),
                );

                // write page to disk:
                Helper::write_file_sync(
                    &Helper::get_output_dir()
                        .join(&lock.base_dir)
                        .join("topic")
                        .join(Helper::sanitize_string(key))
                        .join("index.html"),
                    &lock.render().unwrap().as_bytes(),
                )
                .unwrap();
            }

            // create dir recursively:
            Helper::create_dir_all(&Helper::get_output_dir().join(&lock.overview_page_url));

            // write page to disk:
            Helper::write_file_sync(
                &Helper::get_output_dir()
                    .join(&lock.overview_page_url)
                    .join("index.html"),
                &lock.render().unwrap().as_bytes(),
            )
            .unwrap();

            lock.overview_offset += lock.overview_num_posts;
        }
    }
}

async fn generate_overview_year(shared: Arc<BlogShared>) {
    let mut lock = shared.state.lock().unwrap();

    // TODO: How to solve this without a clone??
    for (key, indices) in &lock.year_blog_indices.clone() {
        lock.overview_offset = 0;
        let num = indices.len() as f32 / lock.num_previews_per_page as f32;
        lock.overview_num_pages = num.ceil() as usize;
        lock.overview_num_posts = lock.num_previews_per_page;
        lock.overview_keywords = format!("year, {}", key);
        lock.overview_year = String::from(key);
        lock.overview_type = 2;

        for i in 1..lock.overview_num_pages + 1 {
            lock.overview_current_page = i;
            lock.overview_page_url = format!(
                "{}/year/{}/page/{}",
                lock.base_dir,
                Helper::sanitize_string(key),
                i
            );
            lock.overview_title = format!("Blog posts by year: {}", key);

            if i == lock.overview_num_pages {
                lock.overview_num_posts = cmp::min(
                    indices.len() - lock.num_previews_per_page * (i - 1),
                    lock.num_previews_per_page,
                );
            }

            if lock.overview_current_page == 1 {
                // create dir recursively:
                Helper::create_dir_all(
                    &Helper::get_output_dir()
                        .join(&lock.base_dir)
                        .join("year")
                        .join(Helper::sanitize_string(key)),
                );

                // write page to disk:
                Helper::write_file_sync(
                    &Helper::get_output_dir()
                        .join(&lock.base_dir)
                        .join("year")
                        .join(Helper::sanitize_string(key))
                        .join("index.html"),
                    &lock.render().unwrap().as_bytes(),
                )
                .unwrap();
            }

            // create dir recursively:
            Helper::create_dir_all(&Helper::get_output_dir().join(&lock.overview_page_url));

            // write page to disk:
            Helper::write_file_sync(
                &Helper::get_output_dir()
                    .join(&lock.overview_page_url)
                    .join("index.html"),
                &lock.render().unwrap().as_bytes(),
            )
            .unwrap();

            lock.overview_offset += lock.overview_num_posts;
        }
    }
}

async fn generate_atom_feed(shared: Arc<BlogShared>) {
    let lock = shared.state.lock().unwrap();

    let mut _feed_data = String::new();
    let date_now = get_date_now_for_feed();

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
    _feed_data += "    <subtitle>This blog is dedicated to free software in general.</subtitle>\n";
    _feed_data += "    <logo>https:/www.luflow.net/static/img/icon.png</logo>\n";
    _feed_data += "    <icon>https:/www.luflow.net/favicon.ico</icon>\n";

    // each blog entry:
    for blog_post in &lock.blog_posts {
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
        _feed_data += &lock.base_dir;
        _feed_data += "/";
        _feed_data += &blog_post.url;
        _feed_data += "/\"/>\n";

        // id:
        _feed_data += "        <id>https:/www.luflow.net/";
        _feed_data += &lock.base_dir;
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

fn get_date_now_for_feed() -> String {
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
