-- Your SQL goes here
CREATE TABLE emotions (
  emotion_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name_emotion VARCHAR(20) UNIQUE NOT NULL,
  color_id UUID NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  updated_at TIMESTAMP NOT NULL DEFAULT now(),
  CONSTRAINT fk_color_em 
    FOREIGN KEY(color_id)
      REFERENCES colors(color_id)
      ON DELETE CASCADE
      ON UPDATE CASCADE
)
