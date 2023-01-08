ALTER TABLE emotions drop CONSTRAINT fk_color_em;

ALTER TABLE emotions
ADD CONSTRAINT fk_color_em
    FOREIGN KEY (color_id)
    REFERENCES colors(color_id)
    ON DELETE RESTRICT 
    ON UPDATE CASCADE;

ALTER TABLE places drop CONSTRAINT fk_color_pl;

ALTER TABLE places
ADD CONSTRAINT fk_color_pl
    FOREIGN KEY(color_id)
    REFERENCES colors(color_id)
    ON DELETE RESTRICT 
    ON UPDATE CASCADE;