CREATE TABLE boards (
    id          SMALLINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    slug        TEXT NOT NULL UNIQUE,
    name        TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    bump_limit  SMALLINT NOT NULL DEFAULT 300,
    max_threads SMALLINT NOT NULL DEFAULT 200,
    is_nsfw     BOOLEAN NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE images (
    id         BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    disk_name  TEXT NOT NULL,
    thumb_name TEXT,
    mime_type  TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    width      INTEGER,
    height     INTEGER,
    sha256     BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_images_sha256 ON images (sha256);

CREATE TABLE threads (
    id          BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    board_id    SMALLINT NOT NULL REFERENCES boards (id) ON DELETE CASCADE,
    subject     TEXT,
    is_pinned   BOOLEAN NOT NULL DEFAULT FALSE,
    is_locked   BOOLEAN NOT NULL DEFAULT FALSE,
    is_archived BOOLEAN NOT NULL DEFAULT FALSE,
    reply_count INTEGER NOT NULL DEFAULT 0,
    image_count INTEGER NOT NULL DEFAULT 0,
    bumped_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_threads_board_bump ON threads (board_id, is_pinned DESC, bumped_at DESC);

CREATE TABLE posts (
    id           BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    thread_id    BIGINT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
    local_number INTEGER NOT NULL,
    is_op        BOOLEAN NOT NULL DEFAULT FALSE,
    body         TEXT NOT NULL DEFAULT '',
    image_id     BIGINT REFERENCES images (id) ON DELETE SET NULL,
    poster_id    TEXT,
    ip_hash      BYTEA NOT NULL,
    is_sage      BOOLEAN NOT NULL DEFAULT FALSE,
    is_deleted   BOOLEAN NOT NULL DEFAULT FALSE,
    deleted_at   TIMESTAMPTZ,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (thread_id, local_number)
);

CREATE INDEX idx_posts_thread ON posts (thread_id, id);
CREATE INDEX idx_posts_ip_created ON posts (ip_hash, created_at);

CREATE UNIQUE INDEX idx_posts_one_op ON posts (thread_id) WHERE is_op;

CREATE TABLE post_references (
    post_id       BIGINT NOT NULL REFERENCES posts (id) ON DELETE CASCADE,
    referenced_id BIGINT NOT NULL REFERENCES posts (id) ON DELETE CASCADE,
    thread_id     BIGINT NOT NULL REFERENCES threads (id) ON DELETE CASCADE,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (post_id, referenced_id)
);

CREATE INDEX idx_post_refs_referenced ON post_references (referenced_id);
CREATE INDEX idx_post_refs_thread ON post_references (thread_id);