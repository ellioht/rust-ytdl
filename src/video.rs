use anyhow::Ok;
use anyhow::Result;
use reqwest::Url;
use serde_json::{json, Value};

use crate::innertube::{self};
use crate::decipher::SigDecipher;

pub fn get_video_id(url: &str) -> Option<&str> {
    if let Some(id) = regex::Regex::new(r"https://www\.youtube\.com/watch\?v=(.*)").expect("correct regex").captures(url).unwrap().get(1) {
        Some(id.as_str())
    } else {
        None
    }
}

pub fn get_video_info(url: &str) -> Result<Value> {
    let session = innertube::get_innertube_session()?;
    let id = get_video_id(url).ok_or(anyhow::anyhow!("Failed to extract video id"))?;

    let request_url = "https://www.youtube.com/youtubei/v1/player";
    let request_body = json!({
        "playbackContext": {
        "contentPlaybackContext": {
            "currentUrl": format!("/watch?v={}", id),
            "vis": 0,
            "splay": false,
            "autoCaptionsDefaultOn": false,
            "autonavState": "STATE_OFF",
            "html5Preference": "HTML5_PREF_WANTS",
            "signatureTimestamp": session.sts,
            "referer": "https://www.youtube.com",
            "lactMilliseconds": "-1"
            }
        },
        "context": session.context,
        "videoId": id
    });

    let client = reqwest::blocking::Client::new();
    let res_body = client.post(request_url).json(&request_body).send()?.text()?;

    let res_json: serde_json::Value = serde_json::from_str(&res_body)?;

    Ok(res_json)
}

pub fn get_video_download_url(video_info: &serde_json::Value) -> Option<&str> {
    let mp4_codec_regex = regex::Regex::new(r"codecs=(.*mp4.*)").expect("correct codecs regexp");

    let url = ["url", "signatureCipher", "cipher"];

    for t in video_info["streamingData"]["formats"].as_array().unwrap() {
        if let Some("360p") = t["qualityLabel"].as_str() {
            if mp4_codec_regex.find(t["mimeType"].as_str().unwrap()).is_some() {
                for &key in &url {
                    if let Some(url) = t[key].as_str() {
                        return Some(url);
                    }
                }
            }
        }
    }

    None
}

pub fn extract_video_url(sig: &str) -> Option<String> {
    if let Some(start) = sig.find("url=") {
        let end = sig[start..].find('&').map(|i| start + i).unwrap_or(sig.len());
        let url_encoded = &sig[start + 4..end];
        let url = url_encoded.replace("%3D", "=").replace("%26", "&").replace("%3F", "?");
        return Some(url);
    }
    None
}

pub fn decipher_url(sig: &str) -> Option<String> {
    let session = innertube::get_innertube_session().unwrap();
    let player_name = innertube::get_string_between_strings(&session.player_url, "/player/", "/");

    let player_name_str = player_name.unwrap();

    let decipher = SigDecipher::new(sig.to_string(), player_name_str);

    decipher.decipher().ok()
}

pub fn get_video_file_name(video_info: &Value) -> Option<String> {
    if let Some(name) = video_info["videoDetails"]["title"].as_str() {
        Some(format!("{}.mp4", name))
    } else {
        None
    }
}

// pub fn download_file(url: &str, file_name: &str) -> anyhow::Result<()> {
//     let url = Url::parse(url)?;
//     let mut resp = reqwest::blocking::Client::new().get(url.as_str()).send()?;
//     let mut out = std::fs::File::create(file_name)?;
//     std::io::copy(&mut resp, &mut out)?;

//     Ok(())
// }

pub fn download_file(url: &str, file_name: &str) -> anyhow::Result<()> {
    let url = Url::parse(url).map_err(|e| anyhow::anyhow!("Invalid URL: {}", e))?;
    
    let mut resp = reqwest::blocking::Client::new()
        .get(url.as_str())
        .send()
        .map_err(|e| anyhow::anyhow!("Failed to send request: {}", e))?;
    
    let mut out = std::fs::File::create(file_name)
        .map_err(|e| anyhow::anyhow!("Failed to create file: {}", e))?;
    
    std::io::copy(&mut resp, &mut out)
        .map_err(|e| anyhow::anyhow!("Failed to copy content: {}", e))?;
    
    Ok(())
}
