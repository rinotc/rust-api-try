# 1. 接続先環境変数をセット（Dockerが動いている前提）
export DATABASE_URL=postgres://user:password@localhost:5432/myapp

# 2. マイグレーション実行
# account/migration ディレクトリで実行するか、-d でパスを指定します
sea-orm-cli migrate up -d .