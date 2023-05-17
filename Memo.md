# MEMO

## Backend API仕様
```bash
# GET /healthcheck
# ヘルスチェック用のエンドポイントです。
GET http://localhost:8080/healthcheck HTTP/1.1

# GET /prime?base={base}
# baseの数値までの素数を見つけ出し、結果を返す。
GET http://localhost:8080/prime?base=100 HTTP/1.1


### Information API version 1 
# POST /api/v1/info/create
# Uopeopleに関係する記事を投稿します。
POST http://localhost:8080/api/v1/info/create HTTP/1.1
Content-Type: application/json

{
    "user_id": 1,
    "url": "https;//lecture.com",
    "tag": "Lecture",
    "title": "CS 4403 Machine Learnong"
}

# GET /api/v1/info/all
# 登録済みの記事の一覧を返します。
GET http://localhost:8080/api/v1/info/all HTTP/1.1

# /api/v1/info/update
# 登録済みの記事の内容を更新します。
POST http://localhost:8080/api/v1/info/update HTTP/1.1
Content-Type: application/json

{
    "id": 1,
    "url": "https;//example.com",
    "tag": "Example",
    "title": "Test1"
}

# /api/v1/info/delete
# 登録済みの記事を削除します。
POST http://localhost:8080/api/v1/info/delete HTTP/1.1
Content-Type: application/json

{
    "id": 2
}
```

## database
- ブログ情報(informations)
    - id
    - user_id
    - url
    - tag
    - title
    - created_at

- ブログいいね(likes)
    - id
    - user_id
    - information_id
    - created_at

- ユーザー情報(users)
    - id
    - name
    - email
    - passward
    - created_at

## 開発環境
docker composeによりAPIサーバーとDBサーバーが起動する。  

# 参考記事一覧
[Layered Architecture Style](https://overcoded.dev/posts/Arch-14)
[Rust でレイヤードアーキテクチャ](https://github.com/foresta/rust-api-architecture-sample/blob/master/src/domains/documents.rs)