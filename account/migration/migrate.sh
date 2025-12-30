#!/bin/bash
set -euo pipefail

# --- Configuration ---
# スクリプトのディレクトリに移動（どこから実行しても同じ挙動にする）
SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
cd "$SCRIPT_DIR"

# 1. 引数がある場合はそれをDATABASE_URLとして優先
if [ $# -ge 1 ]; then
  export DATABASE_URL="$1"
fi

# 2. DATABASE_URLが設定されていない場合はデフォルト値（ローカルDocker）を使用
if [ -z "${DATABASE_URL:-}" ]; then
  export DATABASE_URL="postgres://user:password@localhost:5432/myapp"
  echo "DATABASE_URL が設定されていません。デフォルト値を使用します: $DATABASE_URL"
fi

# 3. 最終確認
if [ -z "${DATABASE_URL:-}" ]; then
  echo "エラー: DATABASE_URL が設定されていません。"
  echo "使用法: $0 [DATABASE_URL]"
  exit 1
fi

echo "データベースに接続中..."

# --- Execution ---
echo "マイグレーションを実行中 (migrate up)..."

# sea-orm-cliがあれば使い、なければ cargo run で代用
if command -v sea-orm-cli > /dev/null 2>&1; then
  sea-orm-cli migrate up -d .
else
  echo "sea-orm-cli が見つかりません。'cargo run' で代用します..."
  cargo run -- migrate up
fi

echo "完了しました。"