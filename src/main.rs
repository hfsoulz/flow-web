// luflow.net web site
// AGPL-3.0 License (see LICENSE)

mod site;

use crate::site::generate_site;

#[tokio::main]
async fn main() {
    // generate all site related files:
    generate_site().await;

    println!("\nDone! Output can be found in 'output' folder.");
    println!("(Serve locally: 'servez output')");
}
