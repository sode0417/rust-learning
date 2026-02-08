---
name: start-exercise
description: >
  Rust 学習演習の開始準備を自動化する。ユーザーが `/start-exercise <番号>` と入力したとき
  （例: `/start-exercise 1-1`）にトリガーする。GitHub Issue から演習情報を取得し、
  ブランチ作成、Cargo プロジェクトの scaffold、ラベル付与を行い、学習ガイドを表示する。
---

# start-exercise

## 手順

1. **main ブランチに移動して最新化**
   ```bash
   git checkout main && git pull origin main
   ```
   - 未コミットの変更がある場合はユーザーに確認してから stash or commit

2. **Issue 情報を取得**
   ```bash
   gh issue list --repo sode0417/rust-learning --label "phase:<Phase番号>" --json number,title,body,state
   ```
   - 演習番号（例: `1-1`）から対象 Issue を特定
   - Issue の body から課題内容・学ぶこと・完了条件・ブランチ名を抽出

3. **ブランチを作成**
   ```bash
   git checkout -b exercise/<番号>-<名前>
   ```
   - ブランチ名は Issue 記載に従う（例: `exercise/1-1-fizzbuzz`）

4. **プロジェクトを scaffold**
   ```bash
   cargo new p<番号>-<名前>
   ```
   - ルートディレクトリ直下に作成（例: `p1-1-fizzbuzz`）

5. **Issue にステータスラベルを付与**
   ```bash
   gh issue edit <Issue番号> --add-label "status:in-progress" --repo sode0417/rust-learning
   ```

6. **学習ガイドを表示**
   - Issue の「学ぶこと」「参考リンク」セクションを表示
   - 「まずは自分で考えて書いてみてください。詰まったらヒントを出します。」と伝える

## 出力例

```
演習 1-1「変数・型・関数 - FizzBuzz」を開始します

課題: FizzBuzz を書く（1〜100）
学ぶこと: let/let mut、基本型、関数定義、制御構文
ブランチ: exercise/1-1-fizzbuzz
プロジェクト: p1-1-fizzbuzz/

まずは自分で考えて書いてみてください。詰まったらヒントを出します。
```

## 注意

- CLAUDE.md の「制約ルール」に従い、正解コードは直接提示しない
- 段階的ヒントシステム（Level 1〜4）で伴走する
- 環境構築（Cargo.toml の依存関係追加など）は直接サポートしてよい
