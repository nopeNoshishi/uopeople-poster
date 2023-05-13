# Uopeople-Poster

Uopeopleに関わる内容のURLであればなんでも登録できる。
登録されると、ジャンル分けされて表示される。

タブ構成
- 新着
- 日常
- お得
- 講義
    - CS
        - CS 1101
        - CS 1102
    - BA    
- マイページ
- 投稿

ジャンルごとの投稿を返す、登録する
自分が登録した内容であれば、更新、削除できるし、他人のものであれば削除申請ができる。

## （予定）使用スタック

バックエンド
Rust (Axum)

フロントエンド
Elm

データベース
Mysql

認証
Keycloak

## database
- ブログ情報(articles)
    - id
    - url
    - tag
- ブログ評価情報(informations_evaluation)
    - id
    - information_id
    - user_id
    - eval
- ユーザー情報(users)
    - id
    - name
    - email
