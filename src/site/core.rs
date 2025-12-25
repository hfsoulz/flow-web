// luflow.net web site
// AGPL-3.0 License (see LICENSE)

use sailfish::Template;
use sailfish::TemplateSimple;
use std::path::Path;

use crate::site::blog_post::BlogPost;
use crate::site::helper::Helper;
use crate::site::screenshot::Screenshot;

#[derive(Template)]
#[template(path = "index.stpl")]
pub struct Core {
    pub blog_posts: Vec<BlogPost>,
    pub screenshots: Vec<Screenshot>,
    pub hfge_url: String,
    pub blog_base_dir: String,
}

impl Core {
    pub fn new() -> Core {
        Core {
            blog_posts: Vec::new(),
            screenshots: Vec::new(),
            hfge_url: String::from("projects/hfge"),
            blog_base_dir: String::new(),
        }
    }

    pub async fn generate(&self) {
        // create output dirs needed:
        self.create_output_dirs();

        // copy all static related files:
        self.copy_static_dirs().await;

        // generate all core pages:
        self.generate_error_pages().await;
        self.generate_project_pages().await;
        self.generate_contact_page().await;
        self.generate_root_index().await;
    }

    fn create_output_dirs(&self) {
        Helper::create_dir_all(&Helper::get_output_dir().join("contact"));
        Helper::create_dir_all(&Helper::get_output_dir().join("projects/hfge"));
    }

    async fn copy_static_dirs(&self) {
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

    async fn generate_error_pages(&self) {
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

    async fn generate_project_pages(&self) {
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

    async fn generate_contact_page(&self) {
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

    async fn generate_root_index(&self) {
        // write page to disk:
        Helper::write_file_sync(
            &Helper::get_output_dir().join("index.html"),
            &self.render().unwrap().as_bytes(),
        )
        .unwrap();
    }
}
