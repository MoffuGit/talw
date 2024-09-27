ALTER TABLE channels ADD category_id binary(16),
ADD CONSTRAINT FOREIGN KEY (category_id) REFERENCES categories (id);
