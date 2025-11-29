# todo_app_actix_web
Todoアプリ
バックエンドを Rust のフレームワーク Actix Web で実装
フロントエンドを Bulma で実装
DB は　PostgreSQL

# コマンド


# docker
1. docker 起動 DB設定・作成
`docker run --name <container name> -e POSTGRES_PASSWORD=<password> -p 5432:5432 -d <database name>`
2. DB コンテナ起動
`docker start <name で設定した container name>`
3. cargo 起動
`cargo run`

