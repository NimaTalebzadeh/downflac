use reqwest::{Client as httpclient, dns::Name};
pub struct Music {
    pub title: String,
    pub artist: String,
}
fn strfield(v: &serde_json::Value, key: &str) -> Result<String, Box<dyn std::error::Error>> {
    match &v[key] {
        serde_json::Value::String(s) => Ok(s.clone()),
        serde_json::Value::Number(n) => Ok(n.to_string()),
        other => Err(format!("field `{key}` not a string/number: {other}").into()),
    }
}
pub async fn dsearch(
    clnt: httpclient,
    Name: String,
) -> Result<Vec<Music>, Box<dyn std::error::Error>> {
    let result: serde_json::Value = clnt
        .get("https://api.deezer.com/search")
        .query(&[("q", Name), ("limit", "5".to_string())])
        .send()
        .await
        .expect("There was an error searching your query in deezer")
        .json()
        .await
        .expect("there was an error parsing search JSON");
    let musics = result["data"].as_array().expect("error");
    let mut Mames: Vec<Music> = Vec::new();
    for music in musics {
        let title = music["title"].to_string().trim_matches('"').to_string();
        let artist = music["artist"]["name"]
            .to_string()
            .trim_matches('"')
            .to_string();
        Mames.push(Music { title, artist });
    }
    Ok(Mames)
}
