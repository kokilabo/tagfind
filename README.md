# tagfind

タグベースのファイル検索CLIツール

## 機能

- ファイルにタグを付与
- タグの削除
- タグによるファイルの検索
- JSONベースの設定ファイル管理
- 詳細なエラーメッセージ

## インストール

### Cargo (Rustツールチェーン)

```bash
cargo install tagfind
```

### プリビルドバイナリ

プラットフォーム別のプリビルドバイナリは[リリースページ](https://github.com/yourusername/tagfind/releases/latest)からダウンロードできます。

対応プラットフォーム:
- Linux (x86_64)
- macOS (Intel/Apple Silicon)
- Windows (x86_64)

## 使用方法

### 基本コマンド

```bash
# タグの追加
tagfind add <filename> <tag>

# タグの削除
tagfind remove <filename> <tag>

# タグによる検索
tagfind <tag>
```

### 使用例

```bash
# ソースファイルにタグを追加
tagfind add main.rs rust
tagfind add main.rs source

# タグの削除
tagfind remove main.rs source

# タグで検索
tagfind rust    # rustタグが付いているファイルを表示

# エラー処理の例
tagfind add nonexistent.txt tag     # ファイルが存在しない場合エラー
tagfind add "" tag                  # 空のファイル名はエラー
tagfind add file.txt ""            # 空のタグ名はエラー
```

### エラーメッセージ

プログラムは以下のような状況で分かりやすいエラーメッセージを表示します：

- ファイルが存在しない場合
- 空のファイル名やタグ名が指定された場合
- 既に同じタグが付与されている場合
- 削除しようとしたタグが存在しない場合

## ライセンス

MIT License - 詳細は[LICENSE](LICENSE)ファイルを参照してください。