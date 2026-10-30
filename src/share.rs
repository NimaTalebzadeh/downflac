use crate::api;
use reqwest::Client as httpclient;
use serde_json::json;

pub const BASE: &str = "https://downflac.nimadev.online";

fn share_id(s: &str) -> Option<&str> {
    let t = s.trim();
    let t = t.rsplit_once("#s/").map(|(_, id)| id).unwrap_or(t);
    let t = t.split(['?', '/']).next().unwrap_or(t);
    let t = t.trim();
    if t.len() == 16 && t.bytes().all(|b| b.is_ascii_hexdigit()) {
        Some(t)
    } else {
        None
    }
}

pub fn parse(s: &str) -> Option<String> {
    share_id(s).map(str::to_string)
}

fn hours() -> Result<u32, Box<dyn std::error::Error>> {
    let opts: Vec<String> = vec![
        "1 hour".to_string(),
        "6 hours".to_string(),
        "24 hours".to_string(),
        "72 hours".to_string(),
        "7 days".to_string(),
    ];
    let hours: Vec<u32> = vec![1, 6, 24, 72, 168];
    let pick = inquire::Select::new("how long should the link work?", opts.clone()).prompt()?;
    let idx = opts.iter().position(|o| *o == pick).unwrap_or(2);
    Ok(hours[idx])
}

pub async fn create(
    clnt: httpclient,
    label: &str,
    dt: &api::Mdata,
) -> Result<String, Box<dyn std::error::Error>> {
    let exp = hours()?;
    let body = json!({
        "id": dt.id,
        "time": dt.time,
        "sign": dt.sign,
        "format": dt.minfo.format,
        "bitrate": dt.minfo.bitrate,
        "label": label,
        "pic": dt.pic,
        "expiry_hours": exp,
    });
    let resp: serde_json::Value = clnt
        .post(format!("{BASE}/api/share"))
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    Ok(resp["url"]
        .as_str()
        .ok_or("share server returned no url")?
        .to_string())
}

pub async fn resolve(
    clnt: httpclient,
    id: &str,
) -> Result<api::Nusic, Box<dyn std::error::Error>> {
    let resp: serde_json::Value = clnt
        .post(format!("{BASE}/api/resolve"))
        .json(&json!({ "share": id }))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let s = |k: &str| resp[k].as_str().unwrap_or("").to_string();
    let minfo = api::Minfobj {
        format: s("format"),
        bitrate: s("bitrate"),
        level: String::new(),
        size: String::new(),
    };
    let title = s("label");
    let (title, artist, album) = match title
        .split_once(" by ")
        .and_then(|(t, rest)| rest.rsplit_once(" (").map(|(a, al)| (t, a, al)))
    {
        Some((t, a, al)) => (
            t.to_string(),
            a.to_string(),
            al.trim_end_matches(')').to_string(),
        ),
        None => (title.clone(), String::new(), String::new()),
    };
    Ok(api::Nusic {
        title,
        artist,
        album,
        data: api::Mdata {
            id: s("id"),
            time: s("time"),
            sign: s("sign"),
            minfo,
            pic: s("pic"),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_full_link_or_bare_id() {
        let id = "0123456789abcdef";
        assert_eq!(
            parse(&format!("{BASE}/#s/{id}")).as_deref(),
            Some(id),
            "full link"
        );
        assert_eq!(parse(id).as_deref(), Some(id), "bare id");
        assert_eq!(parse(&format!(" {id} ")).as_deref(), Some(id), "padded");
    }

    #[test]
    fn rejects_non_share_search_text() {
        assert_eq!(parse("never gonna give you up"), None);
        assert_eq!(parse("0123456789abcde"), None, "too short");
        assert_eq!(parse("0123456789abcdeZ"), None, "non hex");
    }
}