// luflow.net web site
// AGPL-3.0 License (see LICENSE)

use sailfish::Template;
use sailfish::TemplateSimple;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::site::blog_post::BlogPost;
use crate::site::helper::Helper;
use crate::site::screenshot::Screenshot;

pub struct CoreShared {
    state: Mutex<CoreState>,
}

impl CoreShared {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(CoreState::new()),
        }
    }

    pub fn set_core_index_data(
        &self,
        blog_base_dir: String,
        blog_posts: Vec<BlogPost>,
        screenshots: Vec<Screenshot>,
    ) {
        let mut lock = self.state.lock().unwrap();
        lock.blog_base_dir = blog_base_dir;
        lock.blog_posts = blog_posts;
        lock.screenshots = screenshots;
    }
}

#[derive(Template)]
#[template(path = "index.stpl")]
struct CoreState {
    pub blog_posts: Vec<BlogPost>,
    pub screenshots: Vec<Screenshot>,
    pub hfge_url: String,
    pub blog_base_dir: String,
}

impl CoreState {
    pub fn new() -> Self {
        Self {
            blog_posts: Vec::new(),
            screenshots: Vec::new(),
            hfge_url: String::from("projects/hfge"),
            blog_base_dir: String::new(),
        }
    }
}

pub async fn generate_core() {
    // create output dirs needed:
    create_output_dirs();

    let mut tasks = Vec::with_capacity(4);

    // copy all static related files:
    tasks.push(tokio::spawn(copy_static_dirs()));

    // generate all core pages (core index will be done as the very
    // last thing as screenshot and blog generation must be done first):
    tasks.push(tokio::spawn(generate_error_pages()));
    tasks.push(tokio::spawn(generate_project_pages()));
    tasks.push(tokio::spawn(generate_contact_page()));

    // wait until all taks are done:
    for task in tasks {
        task.await.unwrap();
    }
}

fn create_output_dirs() {
    Helper::create_dir_all(&Helper::get_output_dir().join("contact"));
    Helper::create_dir_all(&Helper::get_output_dir().join("projects/hfge"));
}

async fn copy_static_dirs() {
    // copy static and static_root to output folder:
    Helper::copy_dir_all(Path::new("static"), Helper::get_output_dir().join("static"))
        .await
        .unwrap();
    println!(
        "Copied dir 'static' recursively to '{}'",
        Helper::get_output_dir().join("static").display()
    );

    Helper::copy_dir_all(Path::new("static_root"), Helper::get_output_dir())
        .await
        .unwrap();
    println!(
        "Copied dir 'static_root' recursively to '{}'",
        Helper::get_output_dir().display()
    );
}

async fn generate_error_pages() {
    // 404:
    #[derive(TemplateSimple)]
    #[template(path = "404.stpl")]
    struct Err404Template {}

    let ctx = Err404Template {};
    Helper::write_file(
        &Helper::get_output_dir().join("404.html"),
        &ctx.render_once().unwrap().as_bytes(),
    )
    .await
    .unwrap();

    // 500:
    #[derive(TemplateSimple)]
    #[template(path = "500.stpl")]
    struct Err500Template {}

    let ctx = Err500Template {};
    Helper::write_file(
        &Helper::get_output_dir().join("500.html"),
        &ctx.render_once().unwrap().as_bytes(),
    )
    .await
    .unwrap();
}

async fn generate_project_pages() {
    // projects/hfge:
    #[derive(TemplateSimple)]
    #[template(path = "hfge.stpl")]
    struct HFGETemplate {}

    let ctx = HFGETemplate {};
    Helper::write_file(
        &Helper::get_output_dir().join("projects/hfge/index.html"),
        &ctx.render_once().unwrap().as_bytes(),
    )
    .await
    .unwrap();
}

async fn generate_contact_page() {
    // contact:
    #[derive(TemplateSimple)]
    #[template(path = "contact.stpl")]
    struct ContactTemplate {}

    let ctx = ContactTemplate {};
    Helper::write_file(
        &Helper::get_output_dir().join("contact/index.html"),
        &ctx.render_once().unwrap().as_bytes(),
    )
    .await
    .unwrap();
}

pub async fn generate_root_index(shared: Arc<CoreShared>) {
    let lock = shared.state.lock().unwrap();

    // write page to disk:
    Helper::write_file_sync(
        &Helper::get_output_dir().join("index.html"),
        &lock.render().unwrap().as_bytes(),
    )
    .unwrap();
}
