use crate::graph_api::GraphQLClient;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Poster {
    url: String,
    width: i32,
    height: i32,
}

#[derive(Deserialize)]
struct Posters {
    posters: Vec<Poster>,
}

fn generate_query(id: &str) -> String {
    const BASE_QUERY: &str = r#"
query titleById {
  title(id: ""#;

    let mut query = String::from(BASE_QUERY);
    query.push_str(id);
    query.push_str(
        r#"") {
    posters {
      url
      width
      height
    }
  }
}
"#,
    );
    return query;
}
pub async fn get_image_path(id_opt: Option<String>) -> Vec<Poster> {
    if let Some(id) = id_opt {
        let url = "https://graph.imdbapi.dev/v1";
        let client = GraphQLClient::new(url);

        let query: String = generate_query(id.as_str());
        let result = client.send_query(query.as_str()).await.unwrap();

        if result["data"] != Value::Null && result["data"]["title"] != Value::Null {
            let posters = Posters::deserialize(&result["data"]["title"]).unwrap();

            for item in &posters.posters {
                println!("Image urls:{}", item.url);
                println!("===========================");
            }

            return posters.posters;
        }
    }

    return vec![];
}
