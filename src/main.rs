use std::env;

use rust_ytdl::{decipher, video};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if let Some(link) = args.get(1) {
        let video_info = video::get_video_info(link).expect("video info is present");

        let url = video::get_video_download_url(&video_info).expect("download url is present");
        let url = video::extract_video_url(&url).expect("url is valid");

        let deciphered_url = video::decipher_url(&url);

        println!("Deciphered URL: {}", deciphered_url.unwrap());

        let file_name = video::get_video_file_name(&video_info).expect("filename in video_info is present");

        println!("Downloading video from {} with file name {}", url, file_name);
        // video::download_file(&url, &file_name)?;

        Ok(())
    } else {
        Err(anyhow::Error::msg(
            "video link must be provided".to_string(),
        ))
    }
}
