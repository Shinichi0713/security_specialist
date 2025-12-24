GitHubのレポジトリ（特にGitHub Pagesで公開しているサイト）にGoogle Analytics（GA4）を導入するのは、アクセス統計を知る上で非常に有効なステップです。

GitHubレポジトリの「どの画面」を解析したいかによって、2つの方法があります。

---

## 1. GitHub Pages（公開サイト）を解析する場合

もっとも一般的な方法です。自分のレポジトリをWebサイトとして公開している場合、通常のWebサイトと同じ手順で導入できます。

### 手順

1. **Google Analyticsで「測定ID」を取得する**
   * [Google Analytics](https://www.google.com/search?q=https://analytics.google.com/)にログインし、「管理」→「データストリーム」→「ウェブ」を選択。
   * 自分のサイトのURL（`https://ユーザー名.github.io/レポジトリ名/`）を入力して作成。
   * **「G-XXXXXXXXXX」** という形式の測定IDをコピーします。
2. **HTMLにタグを埋め込む**
   * HTMLファイルの `<head>` タグの直後に、Googleから提供された「Google タグ (gtag.js)」を貼り付けます。
   * 先ほど作成したJavaScriptによる目次自動生成機能などがある場合は、その近くに配置しても問題ありません。
3. **GitHubにプッシュする**
   * 変更をコミットしてGitHubへプッシュすれば、数時間以内にデータの収集が始まります。

---

## 2. GitHubの「レポジトリ画面そのもの」を解析したい場合

GitHubのレポジトリページ（コードやIssueが並んでいる画面）には、セキュリティ上の理由からGoogle AnalyticsのJavaScriptを直接埋め込むことはできません。

しかし、**「誰がどれくらい見に来ているか」**を知る代替案が2つあります。

### A. GitHub標準の「Insights」を使う（一番手軽）

GitHubには標準で解析機能が備わっています。

* レポジトリ上部の **「Insights」** タブをクリック ＞ 左メニューの **「Traffic」** を選択。
* 過去14日間の訪問者数（Unique visitors）や閲覧数（Page views）を確認できます。

### B. 「README」に解析用バッジを貼る

どうしてもGoogle Analyticsで管理したい場合、画像（ビーコン）を介して計測する「GitHub Profile Views Counter」などの外部サービスを利用し、そのリンクをREADME.mdに貼り付ける手法があります。

* **GitHub用GAバッジの例:** `GA Beacon` などのツールを使うと、READMEが表示されるたびにGoogle Analyticsにイベントが飛ぶようになります。

---

## 💡 注意点：GitHub Pages + Jekyll を使っている場合

もしGitHub Pagesで  **Jekyll** （デフォルトのテーマ機能）を使っているなら、HTMLを直接編集しなくても設定ファイル（`_config.yml`）に1行追加するだけで導入できるテーマが多いです。

**YAML**

```
# _config.yml の例
google_analytics: G-XXXXXXXXXX
```

お使いのレポジトリは、**「GitHub Pagesで作成した独自のHTMLサイト」**ですか？ それとも **「GitHub標準のテーマ」** を使っていますか？

状況に合わせて、最適な設定ファイルの書き方をご案内することも可能です。

次は、**「特定のボタンがクリックされた回数」をGoogle Analyticsで計測するためのJavaScriptコード**の実装についても紹介しましょうか？


# GitHub Pages

GitHub Pagesで公開しているHTMLにGoogle Analytics（GA4）を導入するには、Googleから発行された「タグ（トラッキングコード）」をHTMLの `<head>` 内に貼り付けるだけです。

具体的な手順を3つのステップで解説します。

---

### ステップ1：Google Analyticsでタグを取得する

1. [Google Analytics](https://www.google.com/search?q=https://analytics.google.com/) にログインし、管理画面から「データストリーム」→「ウェブ」を選択します。
2. 自分のGitHub PagesのURL（`https://ユーザー名.github.io/レポジトリ名/`）を登録します。
3. **「G-XXXXXXXXXX」** という測定IDとともに、**「Googleタグ（gtag.js）」** という数行のJavaScriptコードが表示されるので、それをコピーします。

---

### ステップ2：HTMLファイルに貼り付ける

GitHubレポジトリ内のHTMLファイル（`index.html`など）を開き、コピーしたコードを **`<head>` タグの開始直後** に貼り付けます。

**HTML**

```
<!DOCTYPE html>
<html lang="ja">
<head>
    <script async src="https://www.googletagmanager.com/gtag/js?id=G-XXXXXXXXXX"></script>
    <script>
      window.dataLayer = window.dataLayer || [];
      function gtag(){dataLayer.push(arguments);}
      gtag('js', new Date());
      gtag('config', 'G-XXXXXXXXXX');
    </script>
    <meta charset="UTF-8">
    <title>マイゲーム</title>
</head>
<body>
    ...
</body>
</html>
```

---

### ステップ3：GitHubにプッシュして反映

1. 修正したファイルを保存し、GitHubに `commit` & `push` します。
2. GitHub Actionsが走り、デプロイ（反映）が完了するまで数分待ちます。
3. 自分のサイトを開いた状態で、Google Analyticsの「リアルタイム」レポートを確認し、アクセスがカウントされていれば成功です！

---

### 💡 ヒント：複数のページがある場合

もしHTMLファイルが複数（`stage1.html`, `stage2.html`など）ある場合は、**すべてのファイルの `<head>` 内に同じコードを貼る** 必要があります。

Jekyll（GitHub標準テーマ）を使っている場合：

もし自分でHTMLを書かず、GitHubの「Theme Chooser」を使っている場合は、_config.yml というファイルに以下を追記するだけで済むことがあります。

**YAML**

```
google_analytics: G-XXXXXXXXXX
```

※テーマによって設定項目名が異なる場合があるため、うまくいかないときはHTMLに直接貼るのが一番確実です。

これでマリオのゲームに誰が遊びに来てくれたか、どのステージが人気かなどを分析できるようになりますね！次は、**「特定のボタンを押した（ジャンプした）回数を計測する」** ようなカスタムイベントの設定についても興味がありますか？
