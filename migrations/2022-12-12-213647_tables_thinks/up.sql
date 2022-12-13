-- Your SQL goes here
CREATE TABLE thinks (
  think_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id UUID NOT NULL,
  place_id UUID NOT NULL,
  is_archive BOOLEAN NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  updated_at TIMESTAMP NOT NULL DEFAULT now(),
  CONSTRAINT fk_user_tk 
    FOREIGN KEY(user_id)
      REFERENCES users(user_id)
      ON DELETE CASCADE
      ON UPDATE CASCADE,
  CONSTRAINT fk_place_tk 
    FOREIGN KEY(place_id)
      REFERENCES places(place_id)
      ON DELETE CASCADE
      ON UPDATE CASCADE
);

CREATE TABLE trash_thinks (
  trash_th_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  think_id UUID NOT NULL,
  date_start DATE NOT NUll DEFAULT now(),
  date_end DATE NOT NULL,
  CONSTRAINT fk_think_ts 
    FOREIGN KEY(think_id)
      REFERENCES thinks(think_id)
      ON DELETE CASCADE
      ON UPDATE CASCADE
);
