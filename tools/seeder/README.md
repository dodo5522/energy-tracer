# seeder

`energy-tracer` の初期データを backend に投入するためのツールです。

## 事前準備

```bash
cd tools/seeder
uv sync
```

backend が起動している状態で実行してください。

## 使い方

CSV ファイルは 1 つ以上の位置引数として指定します。

```bash
# measurement のみ投入する
uv run seeder data/grid-20191001-20191015.csv

# 複数の CSV をまとめて投入し、マスタデータも登録する
uv run seeder -m \
  data/grid-20191001-20191015.csv \
  data/grid-20191016-20191031.csv
```

API のベース URL を変更する場合は `-b` / `--base-url` を指定します。

```bash
uv run seeder \
  --base-url http://localhost:8000/generation \
  data/grid-20191001-20191015.csv
```

## 引数

- `-b` / `--base-url URL`
    - マスタデータと measurement の投稿先となるベース URL
    - デフォルト: `http://localhost:8000/generation`
- `-m` / `--post-master`
    - measurement の投入前に `label` / `system` / `unit` のマスタデータを登録する
    - 省略時はマスタデータを登録しない
- `csv_files`
    - `measurement` として投入する CSV ファイルを 1 つ以上指定する必須の位置引数
    - 複数指定した場合はまとめて読み込み、timestamp 順に投入する

利用可能な引数は次のコマンドでも確認できます。

```bash
uv run seeder --help
```

### `Failed to spawn: seeder` になった場合

仮想環境にプロジェクトの実行ファイルが正しく作成されていない可能性があります。次のコマンドでプロジェクトを再インストールしてください。

```bash
uv sync --reinstall-package et-seeder
uv run seeder --help
```
