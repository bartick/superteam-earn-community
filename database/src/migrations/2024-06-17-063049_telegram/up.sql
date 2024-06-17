-- Your SQL goes here

CREATE TABLE telegram (
    id BIGINT PRIMARY KEY,
    thread_id INT,
    can_send_messages BOOLEAN NOT NULL
);