use crate::scanner::file_types::{is_audio_file, is_ebook_file};
use lofty::file::AudioFile;
use lofty::probe::read_from_path;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// Plain data — no DB types here at all. This module's only job is
// "look at the filesystem, tell me what's there." Keeping it free of
// sqlx means you could unit-test it against a temp folder with no
// database involved.
pub struct DiscoveredAudioFile {
    pub file_path: String,
    pub track_index: i32,
    pub duration_seconds: f64,
    pub codec: Option<String>,
    pub bitrate: Option<i32>,
}

pub struct DiscoveredEbookFile {
    pub file_path: String,
    pub format: String,
}

pub struct DiscoveredItem {
    pub path: String,
    pub title: String,
    pub audio_files: Vec<DiscoveredAudioFile>,
    pub ebook_files: Vec<DiscoveredEbookFile>,
}

// Probes one audio file for duration/codec/bitrate. Returns `None`
// rather than erroring if lofty can't read it — a single corrupt file
// shouldn't abort the whole scan.
fn probe_audio(path: &Path, track_index: i32) -> Option<DiscoveredAudioFile> {
    let tagged_file = read_from_path(path).ok()?;
    let properties = tagged_file.properties();

    Some(DiscoveredAudioFile {
        file_path: path.to_string_lossy().to_string(),
        track_index,
        duration_seconds: properties.duration().as_secs_f64(),
        codec: path.extension().map(|e| e.to_string_lossy().to_lowercase()),
        bitrate: properties.audio_bitrate().map(|b| b as i32),
    })
}

// Scans a single item folder (one book's worth of files) and returns
// what it found, or `None` if there's nothing usable inside.
fn scan_item_folder(folder: &Path) -> Option<DiscoveredItem> {
    let mut audio_paths: Vec<_> = WalkDir::new(folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(is_audio_file)
                .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    // Sort alphabetically so multi-file audiobooks get a stable,
    // predictable track order. Known limitation: this only works
    // reliably if filenames are zero-padded (e.g. "01.mp3", "02.mp3"),
    // not "1.mp3", "2.mp3", "10.mp3" — worth fixing later by parsing
    // leading numbers instead of doing a pure string sort.
    audio_paths.sort();

    let audio_files: Vec<DiscoveredAudioFile> = audio_paths
        .iter()
        .enumerate()
        .filter_map(|(i, path)| probe_audio(path, i as i32))
        .collect();

    let ebook_files: Vec<DiscoveredEbookFile> = WalkDir::new(folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let ext = e.path().extension()?.to_str()?.to_lowercase();
            is_ebook_file(&ext).then(|| DiscoveredEbookFile {
                file_path: e.path().to_string_lossy().to_string(),
                format: ext,
            })
        })
        .collect();

    if audio_files.is_empty() && ebook_files.is_empty() {
        return None;
    }

    let title = folder
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    Some(DiscoveredItem {
        path: folder.to_string_lossy().to_string(),
        title,
        audio_files,
        ebook_files,
    })
}

// Checks only the *immediate* children of a directory for audio/ebook
// files — deliberately not recursive. This is the signal we use to
// decide "is this folder itself a book" vs "is this folder just a
// container (Author, Series) that I should keep descending into."
fn has_direct_media(dir: &Path) -> bool {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return false;
    };

    entries.filter_map(|e| e.ok()).any(|e| {
        e.path()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| is_audio_file(ext) || is_ebook_file(ext))
            .unwrap_or(false)
    })
}

fn subdirectories(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return vec![];
    };

    entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect()
}

// Walks downward from `dir`, treating the *first* folder it finds with
// direct media in it as a book boundary — everything beneath that point
// (multi-disc subfolders, etc.) belongs to that one item, but everything
// above it (Author, Series folders) is just a container we keep
// descending through.
//
// `depth` guards against runaway recursion on deeply nested or
// symlink-looped folder structures — 6 levels comfortably covers
// Library/Author/Series/Book and then some, without risking a hang on
// a pathological filesystem.
fn discover_recursive(dir: &Path, depth: u32, items: &mut Vec<DiscoveredItem>) {
    const MAX_DEPTH: u32 = 6;
    if depth > MAX_DEPTH {
        return;
    }

    if has_direct_media(dir) {
        if let Some(item) = scan_item_folder(dir) {
            items.push(item);
        }
        // Deliberately don't recurse further once we've found a book
        // boundary — subfolders from here (e.g. "Disc 1", "Disc 2")
        // are part of *this* item, already picked up by
        // `scan_item_folder`'s own recursive file walk, not separate
        // items in their own right.
        return;
    }

    // No media directly here — this is a container folder (Author,
    // Series, or an empty/irrelevant directory). Keep descending.
    for sub in subdirectories(dir) {
        discover_recursive(&sub, depth + 1, items);
    }
}

// Entry point: now supports arbitrary Author/Series/Book nesting, a
// flat Book-folder-only layout, or anything in between — the boundary
// is detected by content (where media files actually live), not by a
// fixed folder-depth assumption.
pub fn discover_items(library_folder: &Path) -> Vec<DiscoveredItem> {
    let mut items = Vec::new();
    discover_recursive(library_folder, 0, &mut items);
    items
}
