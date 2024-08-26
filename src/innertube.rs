use anyhow::{Result, anyhow};
use serde_json::{Value};

pub struct Innertube {
    pub api_key: String,
    pub api_version: String,
    pub context: Value,
    pub player_url: String,
    pub logged_in: bool,
    pub sts: u32,
}

pub fn get_innertube_session() -> Result<Innertube> {
    let url = "https://www.youtube.com/";
    let client = reqwest::blocking::Client::new();
    let res_body = client.get(url).send()?.text()?;

    let innertube_session_regex = get_string_between_strings(res_body.as_str(), "ytcfg.set({", "});");

    if let Some(innertube_session) = innertube_session_regex {
        let innertube_session_json: Value = serde_json::from_str(&format!("{{{}}}", innertube_session))?;
        let api_key = innertube_session_json["INNERTUBE_API_KEY"].as_str().unwrap();
        let api_version = innertube_session_json["INNERTUBE_API_VERSION"].as_str().unwrap();
        let context = innertube_session_json["INNERTUBE_CONTEXT"].clone();
        let player_url = innertube_session_json["PLAYER_JS_URL"].as_str().unwrap();
        let logged_in = innertube_session_json["LOGGED_IN"].as_bool().unwrap();
        let sts = innertube_session_json["STS"].as_u64().unwrap() as u32;

        Ok(Innertube {
            api_key: api_key.to_string(),
            api_version: api_version.to_string(),
            context: context,
            player_url: player_url.to_string(),
            logged_in: logged_in,
            sts: sts,
        })
    } else {
        Err(anyhow!("Failed to extract Innertube session from the response"))
    }
}

pub fn get_string_between_strings(data: &str, start_string: &str, end_string: &str) -> Option<String> {
    let pattern = format!(r"{}(.*?){}", regex::escape(start_string), regex::escape(end_string));
    let re = regex::Regex::new(&pattern).ok()?;
    re.captures(data).and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}
