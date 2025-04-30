ALTER TABLE members ADD role_id binary(16),
ADD CONSTRAINT FOREIGN KEY (role_id) REFERENCES roles (id);
