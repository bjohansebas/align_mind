-- This file should undo anything in `up.sql`
ALTER TABLE users ADD COLUMN username VARCHAR(30) NOT NULL; 
