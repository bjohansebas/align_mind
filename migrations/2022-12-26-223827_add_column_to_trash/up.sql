-- Your SQL goes here
ALTER TABLE trash_thinks ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT now(); 
ALTER TABLE trash_thinks ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT now(); 