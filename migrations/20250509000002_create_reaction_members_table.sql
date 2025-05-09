CREATE TABLE IF NOT EXISTS reaction_members (
  reaction_id binary(16) NOT NULL,
  FOREIGN KEY (reaction_id) REFERENCES reactions (id),
  member_id binary(16) NOT NULL,
  FOREIGN KEY (member_id) REFERENCES members (id)
);
