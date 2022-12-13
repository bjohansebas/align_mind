-- Your SQL goes here
CREATE TABLE colors (
  color_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name_color VARCHAR(30) NOT NULL UNIQUE,
  code_color VARCHAR(6) NOT NULL UNIQUE,
  created_at TIMESTAMP DEFAULT now()
)
