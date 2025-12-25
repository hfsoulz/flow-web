// luflow.net web site
// AGPL-3.0 License (see LICENSE)

use sailfish::Template;

use crate::site::helper::Helper;

#[derive(Template)]
#[template(path = "screenshot.stpl")]
#[derive(Clone, Debug)]
pub struct Screenshot {
    pub screenshots_title: String,
    pub screenshots_url: String,
    pub title: String,
    pub image_min: String,
    pub image_big: String,
    pub url: String,
    pub screenshots: Vec<Screenshot>,
}

impl Screenshot {
    pub fn new() -> Screenshot {
        Screenshot {
            screenshots_title: String::new(),
            screenshots_url: String::new(),
            title: String::new(),
            image_min: String::new(),
            image_big: String::new(),
            url: String::new(),
            screenshots: Vec::new(),
        }
    }

    pub async fn generate(&self) {
        // create output dir needed:
        self.create_output_dir();

        // write page to disk:
        Helper::write_file_sync(
            &Helper::get_output_dir().join(&self.url).join("index.html"),
            &self.render().unwrap().as_bytes(),
        )
        .unwrap();
    }

    fn create_output_dir(&self) {
        Helper::create_dir_all(&Helper::get_output_dir().join(&self.url));
    }
}
