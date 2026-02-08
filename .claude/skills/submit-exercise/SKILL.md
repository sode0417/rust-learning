---
name: submit-exercise
description: >
  現在の Rust 学習演習を完了し、PR を作成する。ユーザーが `/submit-exercise` と入力
  したときにトリガーする。コードの品質チェック（build/test/clippy）を行い、コミット・
  プッシュ・PR 作成まで自動化する。PR 作成後は Claude Code が自動レビューを行う。
---

# submit-exercise

## 手順

1. **現在のブランチを確認**
   ```bash
   git branch --show-current
   ```
   - `exercise/` で始まらない場合はエラー
   - ブランチ名から演習番号を特定

2. **対応 Issue を取得**
   ```bash
   gh issue list --repo sode0417/rust-learning --label "phase:<Phase番号>" --json number,title,state
   ```
   - 演習番号に対応する Issue 番号とタイトルを特定

3. **コードの品質チェック**
   ```bash
   cargo build
   cargo test    # テストがある場合
   cargo clippy
   ```
   - いずれかが失敗した場合はユーザーに修正を促し、中断

4. **変更をコミット**
   - 未コミット変更がある場合、ユーザーにコミットメッセージを確認
   - コミットメッセージ形式: `feat: <演習番号> <演習タイトル>`

5. **リモートにプッシュ**
   ```bash
   git push -u origin <ブランチ名>
   ```

6. **PR を作成**
   ```bash
   gh pr create --repo sode0417/rust-learning --title "<Issueタイトル>" --body "..."
   ```
   - PR テンプレートが自動適用される
   - ユーザーに「学んだこと」「つまずいたポイント」「自己評価」の記入を促す
   - body に `Closes #<Issue番号>` を含める

7. **案内を表示**
   - PR の URL を表示
   - 「Claude Code が自動レビューを行います。レビューコメントを確認してください。」

## PR マージ後の案内

PR がマージされたら以下を案内:
- `git checkout main && git pull`
- 対象 Issue のクローズ確認
- 「次の演習は `/start-exercise <次の番号>` で開始できます」
