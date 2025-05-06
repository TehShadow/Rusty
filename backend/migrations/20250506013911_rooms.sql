-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE room_members (
    user_id UUID REFERENCES users(id),
    room_id UUID REFERENCES chat_rooms(id),
    PRIMARY KEY (user_id, room_id)
);