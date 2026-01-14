CREATE DATABASE account;
CREATE USER account_user WITH PASSWORD 'account_password';
GRANT ALL PRIVILEGES ON DATABASE account TO account_user;
ALTER DATABASE account OWNER TO account_user;
