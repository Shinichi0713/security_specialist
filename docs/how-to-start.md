# Github Pagesの作り方

「GitHub Pages で Docs を作ると検索に強くなる」という話を、**やさしく説明**します。

---

## ✅ まず結論

GitHub Pages = **GitHub上に無料で作れるホームページ**

Docs（ドキュメントサイト）を作ると…

* Google にインデックスされやすくなる
* 外部リンク扱いになるので SEO が強くなる
* READMEだけよりも情報が多くなり、検索キーワードが増える
* 「このリポジトリはしっかりしたプロジェクトだ」と Google に思われる

👉 **検索結果に出やすくなる**

---

## ✅ なぜ検索に強いの？

検索エンジン（GoogleやBing）は

* 文章量が多いサイト
* 更新頻度があるサイト
* 専用ドメインを持つページ
* 構造化されたドキュメント

を高く評価します。

GitHub Pages で Docs を作ると、プロジェクトが…

* README だけより内容量が多い
* `/docs` やブログ形式で説明が整理される
* Webサイトとして Google がクロールしやすい

つまり

> **GitHub のコード + WebサイトのSEO**
>
> → ダブルで有利

---

## ✅ 具体例

リポジトリが `yourname/robot-servo-control` だとします。

README だけより

以下があるとめちゃ強い：

```
https://yourname.github.io/robot-servo-control/
```

このサイトに：

* 使い方
* サーボ理論
* シミュレーション手順
* コードチュートリアル
* 参考資料

などを載せると、

Google が「価値のある技術情報だ」と判断します。

---

## ✅ 作り方（超簡単）

### 1) GitHub → Repository → Settings → Pages

* Source → **Deploy from branch**
* Branch → `docs/` or `main`

### 2) `docs/`フォルダを作成して `index.md` を置く

#### `docs/index.md`

```markdown
# Servo Control Simulation with Webots

This project demonstrates PID-based servo control for robot joints in Webots simulator.

## Features
- Webots robot simulation
- Servo joint control
- PID controller implementation in Python
- ROS2 optional integration
```

### 3) Push すると自動でサイト生成

---

## ✅ GitHub Pages のテンプレート例

この3つが簡単＆人気：

| ジェネレーター       | 特徴                                   |
| -------------------- | -------------------------------------- |
| **Jekyll**     | GitHub標準、Markdownで簡単             |
| **MkDocs**     | Python系技術者に人気、ドキュメント向け |
| **Docusaurus** | Meta製、技術ブログ＋ドキュメント向け   |

---

## ✅ まとめ

| 方法                        | 効果                    |
| --------------------------- | ----------------------- |
| READMEのみ                  | ✋ 最低限               |
| GitHub Topics               | 👍 GitHub検索に有利     |
| GitHub Pages Docs           | 🚀 Google検索に強くなる |
| 外部記事(Qiita, Zenn)リンク | 🎯 SEO最強              |

---

## 📩 次のステップ

もしよければ、あなたのリポジトリ URL を教えてください。

* GitHub Pages テンプレ作ります
* Docs構成案出します
* SEOキーワード案も出します
* 技術紹介文も作れます

「**研究・就活・技術PR全部に効く** GitHubの仕上げ方」まで一緒にやりましょう💪😎
