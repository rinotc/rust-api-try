#!/bin/bash
set -euo pipefail

# --- Configuration ---
# スクリプトのディレクトリに移動
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

# 4. sea-orm-cli の存在確認 (generateには必須)
if ! command -v sea-orm-cli > /dev/null 2>&1; then
  echo "エラー: sea-orm-cli がインストールされていません。"
  echo "次のコマンドでインストールしてください: cargo install sea-orm-cli"
  exit 1
fi

# --- Execution ---
echo "データベースを初期化中 (migrate fresh)..."
sea-orm-cli migrate fresh -d .

echo "エンティティを生成中..."
OUTPUT_DIR="../infra-orm/src/orm"
mkdir -p "$OUTPUT_DIR"

# --with-serde both は、プロジェクトでserdeを利用している場合に有用です。
# 必要に応じてオプションを調整してください。
sea-orm-cli generate entity \
  -u "$DATABASE_URL" \
  -o "$OUTPUT_DIR" \
  --with-serde both \
  --expanded-format

echo "$OUTPUT_DIR にエンティティを正常に生成しました。"