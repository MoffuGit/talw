CREATE TABLE IF NOT EXISTS threads_members (
  thread_id binary(16) NOT NULL,
  member_id binary(16) NOT NULL,
  FOREIGN KEY (thread_id) REFERENCES threads (id),
  FOREIGN KEY (member_id) REFERENCES members (id)
);
