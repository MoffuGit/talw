CREATE TABLE IF NOT EXISTS member_roles (
    member_id binary(16) NOT NULL,
    role_id binary(16) NOT NULL,
    FOREIGN KEY (member_id) REFERENCES members (id),
    FOREIGN KEY (role_id) REFERENCES roles (id)
);
