# /submit-exercise — 演習を提出する

現在の演習を完了し、PR を作成するスキル。

## トリガー

ユーザーが `/submit-exercise` と入力したとき。

## 手順

1. **現在のブランチを確認**
   - `git branch --show-current` でブランチ名から演習番号を特定
   - ブランチ名が `exercise/` で始まらない場合はエラー

2. **コードの状態を確認**
   - `cargo build` でコンパイルが通ることを確認
   - `cargo test` でテストが通ることを確認（テストがある場合）
   - `cargo clippy` で警告がないことを確認

3. **変更をコミット**
   - 未コミットの変更がある場合、ユーザーにコミットメッセージを確認
   - コミットメッセージの形式: `feat: <演習番号> <演習タイトル>`

4. **リモートにプッシュ**
   - `git push -u origin <ブランチ名>`

5. **PR を作成**
   - `gh pr create` で PR を作成
   - タイトル: Issue のタイトルと同じ
   - テンプレートが自動適用される
   - ユーザーに「学んだこと」「つまずいたポイント」「自己評価」の記入を促す

6. **ユーザーへの案内**
   - 「PR を作成しました。Claude Code が自動レビューを行います。」
   - 「レビューコメントを確認して、必要に応じて修正してください。」
   - PR の URL を表示

## 例

```
> /submit-exercise

🔍 ブランチ: exercise/1-1-fizzbuzz
✅ cargo build: 成功
✅ cargo test: 成功（テストなし）
⚠️ cargo clippy: 警告なし

📝 コミットメッセージ: feat: 1-1 FizzBuzz を実装
🚀 プッシュ完了

--- PR 作成 ---
PR テンプレートに以下を記入してください:

1. 学んだこと: （この演習で理解できたことを自分の言葉で）
2. つまずいたポイント: （詰まった箇所とどう解決したか）
3. 自己評価: （理解度チェック）

PR URL: https://github.com/sode0417/rust-learning/pull/XX
Claude Code が自動レビューを行います。レビューコメントを確認してください。
```

## PR マージ後

PR がマージされたら、以下を案内:
- `git checkout main && git pull`
- 対象 Issue のクローズ確認
- Phase 親 Issue のチェックリスト更新
- 「次の演習は /start-exercise <次の番号> で開始できます」
