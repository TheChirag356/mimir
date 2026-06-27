pub const AUDIO_EXTENSIONS: &[&str] = &["mp3", "m4a", "m4b", "flac", "ogg", "opus", "wav"];
pub const EBOOK_EXTENSIONS: &[&str] = &["epub", "pdf", "mobi", "cbz", "cbr"];

pub fn is_audio_file(ext: &str) -> bool {
    AUDIO_EXTENSIONS.contains(&ext.to_lowercase().as_str())
}

pub fn is_ebook_file(ext: &str) -> bool {
    EBOOK_EXTENSIONS.contains(&ext.to_lowercase().as_str())
}
