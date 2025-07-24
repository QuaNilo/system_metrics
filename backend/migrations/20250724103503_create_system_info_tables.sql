-- Add migration script here
-- +migrate Up
CREATE TABLE metrics (
  id SERIAL PRIMARY KEY,
  timestamp TIMESTAMPZ NOT NULL DEFAULT NOW()
);

CREATE TABLE cpu_info (
  id SERIAL PRIMARY KEY,
  metrics_id INTEGER NOT NULL REFERENCES metrics(id) ON DELETE CASCADE,
  usage REAL NOT NULL,
  name TEXT NOT NULL,
  frequency BIGINT NOT NULL,
  vendor_id TEXT NOT NULL
);

CREATE TABLE disk_info (
    id SERIAL PRIMARY KEY,
    metrics_id INTEGER NOT NULL REFERENCES metrics(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    total_space BIGINT NOT NULL,
    available_space BIGINT NOT NULL,
    used_space BIGINT NOT NULL
);

CREATE TABLE memory_info (
    id SERIAL PRIMARY KEY,
    metrics_id INTEGER NOT NULL UNIQUE REFERENCES metrics(id) ON DELETE CASCADE,
    total_memory_mb BIGINT NOT NULL,
    used_memory_mb BIGINT NOT NULL
);

CREATE TABLE swap_info (
    id SERIAL PRIMARY KEY,
    metrics_id INTEGER NOT NULL UNIQUE REFERENCES metrics(id) ON DELETE CASCADE,
    free_swap BIGINT NOT NULL,
    used_swap BIGINT NOT NULL
);

CREATE TABLE component_temperatures (
    id SERIAL PRIMARY KEY,
    metrics_id INTEGER NOT NULL REFERENCES metrics(id) ON DELETE CASCADE,
    name TEXT,
    temperature REAL,
    max_temperature REAL,
    threshold_critical REAL
);

CREATE TABLE system_uptime (
    id SERIAL PRIMARY KEY,
    metrics_id INTEGER NOT NULL UNIQUE REFERENCES metrics(id) ON DELETE CASCADE,
    seconds BIGINT NOT NULL,
    minutes BIGINT NOT NULL,
    hours BIGINT NOT NULL
);


