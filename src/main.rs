mod graph_api;
mod imdb_api;
mod vidsrc_api;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let items = vidsrc_api::get_new_movies().await;

    for item in items {
        imdb_api::get_image_path(Some(item.imdb_id)).await;
    }

    Ok(())
}
