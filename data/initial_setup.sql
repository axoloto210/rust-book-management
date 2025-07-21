INSERT INTO
    roles (name)
VALUES
    ('Admin'),
    ('User') ON CONFLICT DO NOTHING;

INSERT INTO
    users (name, email, password_hash, role_id)
SELECT
    'うぱ',
    'example@example.com',
    '$2b$12$xNzNvRMX9v1AMW6ohVFe/.R/OkH5YIudR3fW8PALYktR7TjnWolKO',-- password という文字列をbcryptでハッシュ化
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';