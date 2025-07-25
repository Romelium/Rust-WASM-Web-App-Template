use anyhow::{bail, Context, Result};
use local_ip_address::local_ip;
use std::path::Path;
use warp::Filter;

#[tokio::main]
pub async fn serve_static(static_dir_path: &Path, port: u16, host: &str) -> Result<()> {
    if !static_dir_path.is_dir() {
        bail!(
            "Static directory '{}' does not exist or is not a directory.",
            static_dir_path.display()
        );
    }

    let routes = warp::fs::dir(static_dir_path.to_path_buf())
        .with(warp::reply::with::header(
            "Cache-Control",
            "no-cache, no-store, must-revalidate",
        ))
        .with(warp::reply::with::header("Pragma", "no-cache"))
        .with(warp::reply::with::header("Expires", "0"));

    println!("Serving static files from '{}'", static_dir_path.display());
    println!("Listening on: {}:{}", host, port);

    if host == "0.0.0.0" {
        println!("Accessible at:");
        println!("  - http://localhost:{}", port);
        if let Ok(my_local_ip) = local_ip() {
            println!("  - http://{}:{}", my_local_ip, port);
        }
    } else {
        println!("Accessible at: http://{}:{}", host, port);
    }

    let host_addr: std::net::IpAddr = host.parse().context("Invalid host address")?;
    warp::serve(routes).run((host_addr, port)).await;
    Ok(())
}
