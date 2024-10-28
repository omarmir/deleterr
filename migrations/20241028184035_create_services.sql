CREATE TABLE IF NOT EXISTS services
(
    host            TEXT        NOT NULL,
    api_key         TEXT        NOT NULL,
    use_ssl         BOOLEAN,
    service         VARCHAR(10) NOT NULL
);