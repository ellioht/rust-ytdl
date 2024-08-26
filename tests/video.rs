use rust_ytdl::innertube::{self};
use rust_ytdl::video;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_from_url_extraction_short() {
        let url = "https://www.youtube.com/watch?v=FGBhQbmPwH8";
        let id = video::get_video_id(url);

        assert!(id.is_some());
        let id_str = id.unwrap();
        println!("extracted video id: {}", id_str);
        assert_eq!(id_str, "FGBhQbmPwH8");
    }

    #[test]
    fn test_get_video_info() {
        let url = "https://www.youtube.com/watch?v=FGBhQbmPwH8";
        let info = video::get_video_info(url);

        if let Err(e) = &info {
            println!("Error: {:?}", e);
        }

        assert!(info.is_ok());
    }

    #[test]
    fn test_get_video_download_url() {
        let url = "https://www.youtube.com/watch?v=FGBhQbmPwH8";
        let video_info = video::get_video_info(url).unwrap();
        let url = video::get_video_download_url(&video_info);

        assert!(url.is_some());
    }

    #[test]
    fn test_get_innertube_session() {
        let session = innertube::get_innertube_session();
        assert!(session.is_ok());
        let session_details = session.unwrap();
        println!(
            "api_key: {}, api_version: {}, context: {:?}, player_url: {}, logged_in: {}, sts: {}",
            session_details.api_key,
            session_details.api_version,
            session_details.context,
            session_details.player_url,
            session_details.logged_in,
            session_details.sts
        );
    }

    #[test]
    fn test_get_file_name() {
        let url = "https://www.youtube.com/watch?v=FGBhQbmPwH8";
        let video_info = video::get_video_info(url).unwrap();
        let file_name = video::get_video_file_name(&video_info).expect("filename in video_info is present");

        println!("file_name: {}", file_name);
        assert_eq!(file_name, "Daft Punk - One More Time (Official Video).mp4");
    }
}