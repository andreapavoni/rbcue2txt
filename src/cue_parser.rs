use regex::Regex;
use std::error;
use std::fs;

#[derive(Clone, Debug)]
pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    pub time: String,
}

pub fn run(cue_file_path: &str) -> Result<Vec<TrackInfo>, Box<dyn error::Error>> {
    let file_content = match fs::read_to_string(cue_file_path) {
        Ok(content) => content,
        Err(err) => {
            println!("Failed to read cue file: {err}");
            return Err(Box::new(err));
        }
    };

    let artist_re = Regex::new(r#"[\t\t| +]PERFORMER "(.*)""#).unwrap();
    let title_re = Regex::new(r#"[\t\t| +]TITLE "(.*)""#).unwrap();
    let index_re = Regex::new(r#"[\t\t| +]INDEX \d+ (\d+:\d+:\d+)"#).unwrap();

    let artists: Vec<_> = artist_re
        .captures_iter(&file_content)
        .map(|captures| captures.get(1).unwrap().as_str())
        .collect();
    let titles: Vec<_> = title_re
        .captures_iter(&file_content)
        .map(|captures| captures.get(1).unwrap().as_str())
        .collect();
    let timestamps: Vec<_> = index_re
        .captures_iter(&file_content)
        .map(|captures| captures.get(1).unwrap().as_str())
        .collect();

    let mut outputs: Vec<TrackInfo> = Vec::new();

    for i in 0..titles.len() {
        outputs.push(TrackInfo {
            title: titles[i].to_string(),
            artist: artists[i].to_string(),
            time: timestamps[i].to_string(),
        });
    }

    Ok(outputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let cue_file_path = "./test/test.cue";

        let result = run(cue_file_path);
        assert!(result.is_ok());

        let playlist = result.unwrap();

        assert_eq!(playlist.len(), 17);

        // first track
        assert_eq!(playlist[0].title, "Shot You Down (Original Mix)");
        assert_eq!(playlist[0].artist, "Giuseppe Surace");
        assert_eq!(playlist[0].time, "00:00:02");

        // second track
        assert_eq!(playlist[1].title, "Tzu-Mani");
        assert_eq!(playlist[1].artist, "Aldo Cadiz");
        assert_eq!(playlist[1].time, "00:03:06");

        // last track
        assert_eq!(playlist[16].title, "Super Flu 3 < Isaac");
        assert_eq!(playlist[16].artist, "Super Flu");
        assert_eq!(playlist[16].time, "00:57:02");
    }
}
