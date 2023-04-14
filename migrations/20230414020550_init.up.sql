-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


CREATE TABLE
    IF NOT EXISTS tasks (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        title VARCHAR(255) NOT NULL UNIQUE,
        content VARCHAR(255),
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
      );
