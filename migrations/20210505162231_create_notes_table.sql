-- Add migration script here
CREATE TABLE notes(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   note TEXT NOT NULL,
   created timestamptz NOT NULL
);