CREATE TABLE IF NOT EXISTS songs (
  song_id VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  url VARCHAR NOT NULL,
  image VARCHAR NOT NULL,
  duration VARCHAR NOT NULL,
  artists VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS playlists (
  user_id VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  playlist_id VARCHAR NOT NULL,
  song_ids text[] NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
  username VARCHAR NOT NULL,
  user_id VARCHAR NOT NULL,
  password_hash VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
  user_id VARCHAR NOT NULL,
  session_id VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL,
  expired_at TIMESTAMP NOT NULL
);
