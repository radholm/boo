use serde_json::Value;
use reqwest;

struct ProviderMeta {
    url: String,
}

async fn get_response_obj(provider: &str) -> Result<Value, reqwest::Error> {
    let client = reqwest::Client::new();

    let provider_meta = match provider {
        "boneo" => ProviderMeta {url: "https://objectsapi.boneo.se/v1/search/es/properties".to_owned()},
        _ => todo!()
    };

    let res = client
        .post(provider_meta.url)
        .header("method", "POST")
        .header("content-type", "application/x-www-form-urlencoded;charset=UTF-8")
        .body(
            "sort=field_property_publish_date&order=desc&start=0&field_property_type=Bostadsr%C3%A4tt&field_property_rum_filter=2%2C*&locs_field_name=%5B%7B%22locs_field_name%22%3A%22Sk%C3%B6ndal%2C%20Stockholm%22%2C%22locs_field_combination%22%3A%22field_area_province%22%7D%5D&type=bostad&limit=25"
        )
        .send()
        .await?;

    let res_obj: Value = res.json().await?;

    Ok(res_obj)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let boneo = get_response_obj("boneo").await?;

    println!("{:#?}", boneo["data"]["docs"]);

    Ok(())
}
