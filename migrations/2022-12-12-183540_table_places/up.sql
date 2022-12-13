-- Your SQL goes here
CREATE TABLE places (
  place_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name_place VARCHAR(50) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  updated_at TIMESTAMP NOT NULL DEFAULT now(),
  color_id UUID NOT NULL,
  user_id UUID NOT NULL,
  CONSTRAINT fk_color_pl 
    FOREIGN KEY(color_id)
      REFERENCES colors(color_id)
      ON DELETE CASCADE
      ON UPDATE CASCADE,
  CONSTRAINT fk_user_pl 
    FOREIGN KEY(user_id)
      REFERENCES users(user_id)
      ON DELETE CASCADE
      ON UPDATE CASCADE 
);
