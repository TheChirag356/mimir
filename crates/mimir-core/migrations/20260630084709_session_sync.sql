-- Tracks a user's overall progress on an item — this is what answers
-- "where did I leave off," independent of any single session.
CREATE TABLE media_progress(
  id TEXT PRIMARY KEY NOT NULL,
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  library_item_id TEXT NOT NULL REFERENCES library_items(id) ON DELETE CASCADE,
  current_time_seconds REAL NOT NULL DEFAULT (0),
  duration_seconds REAL NOT NULL DEFAULT (0),
  progress_percent REAL NOT NULL DEFAULT (0),
  is_finished BOOLEAN NOT NULL DEFAULT (FALSE),
  time_listened_seconds REAL NOT NULL DEFAULT (0),
  last_update INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  UNIQUE(user_id, library_item_id)
);

-- One row per "listening session" — opened when playback starts,
-- updated periodically while it continues. Useful for stats/history
-- later; media_progress above is what actually drives resume-playback.
CREATE TABLE playback_sessions(
  id TEXT PRIMARY KEY NOT NULL,
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  library_item_id TEXT NOT NULL REFERENCES library_items(id) ON DELETE CASCADE,
  audio_file_id TEXT REFERENCES audio_files(id) ON DELETE SET NULL,
  current_time_seconds REAL NOT NULL DEFAULT (0),
  duration_seconds REAL NOT NULL DEFAULT (0),
  time_listened_seconds REAL NOT NULL DEFAULT (0),
  started_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

CREATE INDEX idx_media_progress_user_item ON media_progress (
  user_id,
  library_item_id
);

CREATE INDEX idx_playback_sessions_user ON playback_sessions (user_id);
