CREATE VIEW members_with_profile_fallback AS
SELECT
    m.id,
    m.server_id,
    m.user_id,
    COALESCE(m.name, p.name) AS name,
    COALESCE(m.image_url, p.image_url) AS image_url,
    m.status,
    (SELECT r.id
     FROM member_roles mr
     JOIN roles r ON mr.role_id = r.id
     WHERE mr.member_id = m.id
     ORDER BY r.priority DESC
     LIMIT 1) AS role_id
FROM
    members m
JOIN
    users u ON m.user_id = u.id
LEFT JOIN
    profiles p ON u.id = p.user_id;
