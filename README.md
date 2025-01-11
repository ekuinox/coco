# coco

.env を読んで、 TOML のテキストに変換するだけのツール

Nushell で .env から環境変数にセットするために使いたい

```shell
coco | from toml | load-env
```
