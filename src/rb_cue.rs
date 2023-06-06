use regex::Regex;
use std::error;
use std::fs;

#[derive(Clone, Debug)]
pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    pub time: String,
}

impl TrackInfo {
    pub fn format(&self, format: &str) -> String {
        format!(
            "{}",
            format
                .replace("%T", &self.time)
                .replace("%A", &self.artist)
                .replace("%N", &self.title)
        )
    }
}

pub struct CueSheet {
    pub playlist: Vec<TrackInfo>,
}

impl CueSheet {
    pub fn new(cue_file_path: &str) -> Result<Self, Box<dyn error::Error>> {
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

        let playlist: Vec<TrackInfo> = titles
            .iter()
            .zip(artists.iter().zip(timestamps.iter()))
            .map(|(&title, (&artist, &time))| TrackInfo {
                title: title.to_string(),
                artist: artist.to_string(),
                time: time.to_string(),
            })
            .collect();

        Ok(Self { playlist })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cue_sheet_new() {
        let cue_file_path = "./test/test.cue";

        let cue_sheet = CueSheet::new(cue_file_path);
        assert!(cue_sheet.is_ok());

        let playlist = cue_sheet.unwrap().playlist;

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

    #[test]
    fn test_track_info_format() {
        let t = TrackInfo {
            title: "Hey Baby".to_string(),
            artist: "Los Pinos".to_string(),
            time: "00:04:32".to_string(),
        };

        let formatted = t.format("[%T]: %A - %N");
        assert_eq!(formatted, "[00:04:32]: Los Pinos - Hey Baby");

        let formatted = t.format("[%A] %N (%T)");
        assert_eq!(formatted, "[Los Pinos] Hey Baby (00:04:32)");

        let formatted = t.format("%A - %N");
        assert_eq!(formatted, "Los Pinos - Hey Baby");

        let formatted = t.format("%N [%A]");
        assert_eq!(formatted, "Hey Baby [Los Pinos]");
    }
}
