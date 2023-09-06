CREATE TABLE records (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  version TEXT NOT NULL,
  endpoint TEXT NOT NULL,
  base_url TEXT NOT NULL,
  platform_id TEXT NOT NULL,
  game_id TEXT NOT NULL,
  encryption_key TEXT NOT NULL,
  metadata TEXT NOT NULL,
  keyframes TEXT NOT NULL,
  game_data_chunks TEXT NOT NULL,
  storage TEXT NOT NULL,

  UNIQUE(platform_id, game_id)
);
