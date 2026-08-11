// luflow.net web site
// AGPL-3.0 License (see LICENSE)

pub mod blog;
pub mod blog_post;
pub mod core;
pub mod helper;
pub mod screenshot;
pub mod screenshots;

use rich_rust::console::Console;
use rich_rust::interactive::Status;

use std::path::PathBuf;
use std::time::Instant;

use crate::site::blog::BlogShared;
use crate::site::blog::generate_blog;
use crate::site::blog::get_latest_blog_posts;
use crate::site::core::CoreShared;
use crate::site::core::generate_core;
use crate::site::core::generate_root_index;
use crate::site::helper::Helper;
use crate::site::screenshots::ScreenshotsShared;
use crate::site::screenshots::generate_screenshots;

fn init_logger() {
    fern::Dispatch::new()
        .format(|out, message, record| out.finish(format_args!("[{}] {}", record.level(), message)))
        .level(log::LevelFilter::Debug)
        .chain(
            std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .append(false)
                .open("flow-web.log")
                .unwrap(),
        )
        .apply()
        .unwrap();
}

pub async fn generate_site() {
    // make sure 'templates', 'screenshots' and 'blog-posts' dirs exists:
    // (this tool must be executed in root folder)
    if !Helper::exists_dir(&PathBuf::new().join("blog-posts"))
        || !Helper::exists_dir(&PathBuf::new().join("screenshots"))
        || !Helper::exists_dir(&PathBuf::new().join("templates"))
    {
        panic!(
            "flow-web must be executed in root folder where 'blog-posts', 'screenshots' and
'templates' folders reside. Use command 'cargo run --release' in root folder."
        );
    }

    let start_time = Instant::now();

    // init rich_rust:
    let console = Console::new().shared();
    if let Ok(_status) = Status::new(&console, "Generating site...") {
        // init logger:
        init_logger();

        // remove old output dir if exists:
        if Helper::exists_dir(&Helper::get_output_dir()) {
            Helper::remove_dir_all(&Helper::get_output_dir());
        }

        // generate core pages:
        let core_shared = CoreShared::new().shared();
        let core_handle = tokio::spawn(generate_core());

        // generate screenshot pages:
        let screenshots_shared = ScreenshotsShared::new().shared();
        let screenshots_handle = tokio::spawn(generate_screenshots(screenshots_shared.clone()));

        // generate blog pages:
        let blog_base_dir = "blog";
        let blog_shared = BlogShared::new(String::from(blog_base_dir), 20).shared();
        let blog_handle = tokio::spawn(generate_blog(
            blog_shared.clone(),
            String::from(blog_base_dir),
        ));

        // wait until blog and screenshots are generated:
        screenshots_handle.await.unwrap();
        blog_handle.await.unwrap();

        // generate core index page now when we've all data we need:
        core_shared.set_core_index_data(
            String::from(blog_base_dir),
            get_latest_blog_posts(blog_shared.clone(), 3),
            screenshots_shared.get_screenshots(6, "HFGE Screenshots"),
        );
        let core_index_handle = tokio::spawn(generate_root_index(core_shared.clone()));

        // wait until core is fully done:
        core_handle.await.unwrap();
        core_index_handle.await.unwrap();
    }

    console.print("[dim]See 'output' folder for generated files..[/]");
    console.print("[dim]Serve locally example: 'servez output'[/]");
    console.print("");
    console.print(&format!(
        "[dim]Completed in {:.1}s[/]",
        start_time.elapsed().as_secs_f64()
    ));
    console.print("[green]Generation completed successfully![/]");
}
