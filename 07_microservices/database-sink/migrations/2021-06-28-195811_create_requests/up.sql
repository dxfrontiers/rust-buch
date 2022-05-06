CREATE TABLE requests (
                          id          SERIAL PRIMARY KEY,
                          email       VARCHAR NOT NULL,
                          message     VARCHAR NOT NULL
);