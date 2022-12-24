-- Your SQL goes here
CREATE TABLE think_emotions (
  think_emotion_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  think_id UUID NOT NULL,
  emotion_id UUID NOT NULL,
  CONSTRAINT fk_think_thiemo 
    FOREIGN KEY(think_id)
      REFERENCES thinks(think_id)
      ON DELETE CASCADE
      ON UPDATE CASCADE,
  CONSTRAINT fk_emotion_thiemo 
    FOREIGN KEY(emotion_id)
      REFERENCES emotions(emotion_id)
      ON DELETE CASCADE
      ON UPDATE CASCADE
);

CREATE TABLE think_trash_emotions (
  think_trash_emotion_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  trash_th_id UUID NOT NULL,
  emotion_id UUID NOT NULL,
  CONSTRAINT fk_think_trash_thiemo
    FOREIGN KEY(trash_th_id)
      REFERENCES trash_thinks(trash_th_id)
      ON DELETE CASCADE
      ON UPDATE CASCADE,
  CONSTRAINT fk_emotion_thiemo_trash
    FOREIGN KEY(emotion_id)
      REFERENCES emotions(emotion_id)
      ON DELETE CASCADE
      ON UPDATE CASCADE
);