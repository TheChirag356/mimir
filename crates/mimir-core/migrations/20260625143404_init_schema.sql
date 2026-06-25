-- migrations/<timestamp>_init_schema.sql

-- SQLite has no native UUID type, so we store IDs as TEXT.
-- Timestamps are stored as INTEGER (unix milliseconds) — matches how
-- ABS represents timestamps in its own API, which keeps you compatible.

CREATE TABLE users (
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE TABLE libraries (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    media_type TEXT NOT NULL CHECK (media_type IN ('book', 'podcast')),
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE TABLE library_folders (
    id TEXT PRIMARY KEY NOT NULL,
    library_id TEXT NOT NULL REFERENCES libraries(id) ON DELETE CASCADE,
    path TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

-- A LibraryItem is the generic "thing in a library" — a Book row hangs
-- off it via item_id. Splitting it this way mirrors ABS's own model and
-- leaves room for a future `podcasts` table without reshaping this one.
CREATE TABLE library_items (
    id TEXT PRIMARY KEY NOT NULL,
    library_id TEXT NOT NULL REFERENCES libraries(id) ON DELETE CASCADE,
    folder_id TEXT NOT NULL REFERENCES library_folders(id) ON DELETE CASCADE,
    path TEXT NOT NULL,
    media_type TEXT NOT NULL CHECK (media_type IN ('book', 'podcast')),
    is_missing BOOLEAN NOT NULL DEFAULT FALSE,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE TABLE authors (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    asin TEXT,
    created_at INTEGER NOT NULL
);

CREATE TABLE book_metadata (
    id TEXT PRIMARY KEY NOT NULL,
    library_item_id TEXT NOT NULL UNIQUE REFERENCES library_items(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    subtitle TEXT,
    description TEXT,
    publisher TEXT,
    published_year INTEGER,
    language TEXT,
    isbn TEXT,
    asin TEXT,
    explicit BOOLEAN NOT NULL DEFAULT FALSE
);

-- Many-to-many: a book can have multiple authors, an author can have
-- multiple books.
CREATE TABLE book_authors (
    book_metadata_id TEXT NOT NULL REFERENCES book_metadata(id) ON DELETE CASCADE,
    author_id TEXT NOT NULL REFERENCES authors(id) ON DELETE CASCADE,
    PRIMARY KEY (book_metadata_id, author_id)
);

CREATE TABLE audio_files (
    id TEXT PRIMARY KEY NOT NULL,
    library_item_id TEXT NOT NULL REFERENCES library_items(id) ON DELETE CASCADE,
    file_path TEXT NOT NULL,
    track_index INTEGER NOT NULL,
    duration_seconds REAL NOT NULL,
    codec TEXT,
    bitrate INTEGER
);

CREATE TABLE ebook_files (
    id TEXT PRIMARY KEY NOT NULL,
    library_item_id TEXT NOT NULL UNIQUE REFERENCES library_items(id) ON DELETE CASCADE,
    file_path TEXT NOT NULL,
    format TEXT NOT NULL CHECK (format IN ('epub', 'pdf', 'mobi', 'cbz', 'cbr'))
);

CREATE INDEX idx_library_items_library_id ON library_items(library_id);
CREATE INDEX idx_audio_files_item_id ON audio_files(library_item_id);
