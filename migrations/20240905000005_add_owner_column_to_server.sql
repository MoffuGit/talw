ALTER TABLE servers ADD owner_id binary(16) NOT NULL,
ADD CONSTRAINT FOREIGN KEY (owner_id) REFERENCES users (id);
