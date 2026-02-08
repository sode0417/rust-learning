# /start-exercise — 演習を開始する

演習番号を指定して、学習演習の開始準備を自動化するスキル。

## トリガー

ユーザーが `/start-exercise <番号>` と入力したとき。例: `/start-exercise 1-1`

## 手順

1. **Issue 情報を取得**
   - `gh issue list --repo sode0417/rust-learning --label "phase:<Phase番号>" --json number,title,body` で対象 Issue を検索
   - 該当する演習 Issue の内容（課題、学ぶこと、完了条件、ブランチ名）を表示

2. **ブランチを作成**
   - `git checkout main && git pull`
   - Issue に記載されたブランチ名で `git checkout -b exercise/<番号>-<名前>` を実行

3. **プロジェクトを scaffold**
   - `cargo new p<番号>-<名前>` でプロジェクトを作成（ルートディレクトリ直下）
   - ディレクトリ名の規則: `p1-1-fizzbuzz`, `p2-3-iterators` 等

4. **Issue にステータスラベルを付与**
   - `gh issue edit <番号> --add-label "status:in-progress" --repo sode0417/rust-learning`

5. **学習ガイドを表示**
   - Issue の「学ぶこと」セクションを表示
   - 参考リンクを表示
   - 「まずは自分で考えて書いてみてください。詰まったらヒントを出します。」と伝える

## 例

```
> /start-exercise 1-1

✅ 演習 1-1「変数・型・関数 — FizzBuzz」を開始します

📋 課題: FizzBuzz を書く（1〜100）
📚 学ぶこと: let/let mut、基本型、関数定義、制御構文
🌿 ブランチ: exercise/1-1-fizzbuzz
📁 プロジェクト: p1-1-fizzbuzz/

まずは自分で考えて書いてみてください。詰まったらヒントを出します。
```

## 注意

- CLAUDE.md の「制約ルール」に従い、正解コードは直接提示しない
- 段階的ヒントシステム（Level 1〜4）で伴走する
- 環境構築（Cargo.toml の依存関係追加など）は直接サポートしてよい
