// luflow.net web site
// AGPL-3.0 License (see LICENSE)

mod site;

use std::path::PathBuf;

use crate::site::blog::Blog;
use crate::site::core::Core;
use crate::site::helper::Helper;
use crate::site::screenshots::Screenshots;

#[tokio::main]
async fn main() {
    // make sure 'templates', 'screenshots' and 'blog-posts' dirs exists:
    // (this tool must be executed in root folder)
    if !Helper::exists_dir(&PathBuf::new().join("blog-posts"))
        || !Helper::exists_dir(&PathBuf::new().join("screenshots"))
        || !Helper::exists_dir(&PathBuf::new().join("templates"))
    {
        panic!(
            "flow-web must be executed in root folder where 'blog-posts', 'screenshots' and 'templates' folders reside."
        );
    }

    // remove old output dir if exists:
    if Helper::exists_dir(&Helper::get_output_dir()) {
        Helper::remove_dir_all(&Helper::get_output_dir());
    }

    // generate screenshots pages:
    let mut screenshots = Screenshots::new(String::from("screenshots"));
    screenshots.generate().await;

    // generate blog pages:
    let blog_base_dir = "blog";
    let mut blog = Blog::new(String::from(blog_base_dir), 20);
    blog.generate().await;

    // generate core pages:
    let mut core = Core::new();
    core.blog_posts = blog.get_latest_blog_posts(3);
    core.screenshots = screenshots.get_screenshots(6, "HFGE Screenshots");
    core.blog_base_dir = String::from(blog_base_dir);
    core.generate().await;

    println!("\nDone! Output can be found in 'output' folder.");
    println!("(Serve locally: 'servez output')");
}
