-- Add migration script here

CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE chat_rooms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT,
    is_group BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sender_id UUID NOT NULL REFERENCES users(id),
    room_id UUID NOT NULL REFERENCES chat_rooms(id),
    content TEXT NOT NULL,
    sent_at TIMESTAMPTZ DEFAULT now()
);