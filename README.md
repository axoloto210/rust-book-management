## Rustによる蔵書管理システム
「RustによるWebアプリケーション開発」を参考に実装する。

https://github.com/rust-web-app-book

### cargo make
`cargo make compose-up-db`でpostgre Dockerが起動できます。
各taskは`Makefile.toml`に記載されています。


### レイヤードアーキテクチャ
api → kernel → adapter によるレイヤードアーキテクチャが採用されています。

api: Next.js製のWebフロントエンドアプリケーションとやり取りを行います。この3層のうち、入出力に最も近いため最上位のレイヤーとなります。プレゼンテーション層やインターフェース層にあたります。

kernel: 入力情報をアプリケーションの扱いやすい形に処理したりといったことを行います。
ドメインやビジネスルールを扱う層で、ビジネスロジック層にあたります。

adapter: データベースなど外部依存への接続を行う最下位の層です。
データアクセス層にあたります。


####　modelとrepository
modelはデータの構造とビジネスルールを表現する。
repositoryではデータの取得と永続化を抽象化します。特定のModelに対するデータアクセス機能を提供します。

#### registry
registryでは依存関係の生成・管理を行います。
各層で使うrepositoryのインスタンスをまとめて生成し、提供します。
テスト容易性・拡張性を向上させます。