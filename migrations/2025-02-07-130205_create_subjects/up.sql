CREATE TABLE subjects (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

ALTER TABLE homeworks
ADD subject_id INTEGER REFERENCES subjects(id);
