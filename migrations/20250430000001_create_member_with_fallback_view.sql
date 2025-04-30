CREATE VIEW members_with_profile_fallback AS
SELECT
    m.id,
    m.server_id,
    m.user_id,
    COALESCE(m.name, p.name) AS name,
    COALESCE(m.image_url, p.image_url) AS image_url,
    m.status,
    m.role_id
FROM
    members m
JOIN
    users u ON m.user_id = u.id
LEFT JOIN
    profiles p ON u.id = p.user_id;
