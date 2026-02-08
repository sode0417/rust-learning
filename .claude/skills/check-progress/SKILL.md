---
name: check-progress
description: >
  Rust 学習の進捗を GitHub Issue の状態から一覧表示する。ユーザーが `/check-progress`
  と入力したときにトリガーする。Phase ごとの完了状況を集計し、次のアクションを提案する。
---

# check-progress

## 手順

1. **Issue の状態を取得**
   ```bash
   gh issue list --repo sode0417/rust-learning --state all --json number,title,state,labels --limit 50
   ```

2. **Phase ごとに進捗を集計・表示**

   状態の判定ルール:
   - Issue が CLOSED → 完了
   - `status:in-progress` ラベルあり → 進行中
   - `status:blocked` ラベルあり → つまずき中
   - それ以外 → 未着手

   表示形式:
   ```
   Rust 学習進捗

   Phase 1: Rust の基礎文法 [2/6 完了]
   [完了] 1-1: 変数・型・関数 - FizzBuzz
   [進行中] 1-2: 所有権・借用・ライフタイム
   [ ] 1-3: 構造体・列挙型
   [ ] 1-4: パターンマッチ・Option・Result
   [ ] 1-5: コレクション（Vec, HashMap）
   [ ] 1-6: トレイト基礎

   Phase 2: 実践的な Rust [0/6 完了]
   ...

   全体: 2/24 完了 (8%)
   ```

3. **次のアクションを提案**
   - 進行中の演習があれば続行を案内
   - なければ次の未着手演習を `/start-exercise <番号>` で提案
