-- create a publish table to store the newsletter
CREATE TABLE publish(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    letter TEXT,
    published_at timestamptz NOT NULL
);