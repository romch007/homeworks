CREATE TABLE homeworks (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  due_date TIMESTAMPTZ,
  title VARCHAR NOT NULL,
  description VARCHAR NOT NULL DEFAULT '',
  done BOOLEAN NOT NULL DEFAULT FALSE
);

ALTER TABLE homeworks ADD COLUMN textsearchable_index_col tsvector
  NOT NULL
  GENERATED ALWAYS AS (
    setweight(to_tsvector('english', title), 'A') || ' ' ||
    setweight(to_tsvector('english', description), 'B') :: tsvector
) STORED;

CREATE INDEX textsearch_idx ON homeworks USING gin (textsearchable_index_col);
