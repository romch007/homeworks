CREATE TABLE homeworks (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  due_date TIMESTAMPTZ,
  title VARCHAR NOT NULL,
  description VARCHAR NOT NULL DEFAULT '',
  done BOOLEAN NOT NULL DEFAULT FALSE
);
