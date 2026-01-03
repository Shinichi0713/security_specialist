/* 目次全体のコンテナ */
nav {
    background-color: #f8f9fa;
    border: 1px solid #e1e4e8;
    border-radius: 12px;
    padding: 24px;
    margin: 20px 0 40px 0;
    max-width: 600px;
    box-shadow: 0 4px 6px rgba(0,0,0,0.05);
}

/* 「目次」というタイトル */
nav h3 {
    margin-top: 0;
    margin-bottom: 16px;
    padding-bottom: 8px;
    border-bottom: 2px solid #0969da;
    color: #24292f;
    font-size: 1.2rem;
    display: flex;
    align-items: center;
}

/* タイトルの前にアイコン（絵文字）を追加 */
nav h3::before {
    content: "📖";
    margin-right: 8px;
}

/* リストのスタイル調整 */
#toc {
    list-style: none;
    padding-left: 0;
    margin: 0;
}

#toc li {
    margin-bottom: 8px;
    line-height: 1.4;
}

/* リンクのスタイル */
#toc a {
    color: #0969da;
    text-decoration: none;
    font-weight: 500;
    transition: all 0.2s ease;
    display: inline-block;
}

#toc a:hover {
    color: #cf222e;
    transform: translateX(5px); /* ホバー時に少し右に動く */
}

/* h3（小見出し）がある場合のネスト表現（JSの修正も必要） */
.toc-h3 {
    padding-left: 20px;
    font-size: 0.9em;
    opacity: 0.8;
}
/* 記事全体のベース（必要に応じて調整） */
article, .content {
    line-height: 1.8;
    color: #333;
}

/* H1: 記事の主タイトル */
h1 {
    padding: 0.5em 0;
    margin-bottom: 1em;
    border-bottom: 3px solid #6c5ce7; /* 下線でアクセント */
    color: #2d3436;
    font-size: 1.8em;
}

/* H2: 大見出し（背景色あり） */
h2 {
    position: relative;
    padding: 0.6em 1em;
    margin: 2em 0 1em;
    background-color: #f1f2f6; /* 薄いグレーの背景 */
    border-left: 6px solid #6c5ce7; /* 左側に濃い色のアクセント線 */
    border-radius: 0 4px 4px 0;
    color: #2d3436;
    font-size: 1.5em;
}

/* H3: 中見出し（下線デザイン） */
h3 {
    padding: 0.4em 0.5em;
    margin: 1.5em 0 0.8em;
    border-bottom: 2px solid #dfe4ea; /* 薄い下線 */
    color: #4b4b4b;
    font-size: 1.25em;
}

/* 目次（TOC）内の h3 の字下げ設定 */
.toc-h3 {
    margin-left: 1.5em;
    font-size: 0.9em;
    list-style-type: circle; /* 子要素っぽく白丸にする */
}

/* 目次全体のスタイル（おまけ） */
#toc {
    background: #ffffff;
    border: 1px solid #dfe4ea;
    padding: 1.5em;
    border-radius: 8px;
    margin-bottom: 2em;
}



<nav>
        <h3>目次</h3>
        <ul id="toc"></ul> 
</nav>

<script>
const toc = document.getElementById('toc');
// h2 と h3 の両方を取得
const headings = document.querySelectorAll('h2, h3');

headings.forEach((heading, i) => {
    if (!heading.id) heading.id = `heading-${i}`;
    
    const li = document.createElement('li');
    const link = document.createElement('a');
    link.href = `#${heading.id}`;
    link.textContent = heading.textContent;
    
    // h3 の場合はクラスを付与して字下げする
    if (heading.tagName === 'H3') {
        li.classList.add('toc-h3');
    }
    
    li.appendChild(link);
    toc.appendChild(li);
});

// スムーズスクロールを有効にする
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
        e.preventDefault();
        document.querySelector(this.getAttribute('href')).scrollIntoView({
            behavior: 'smooth'
        });
    });
});
</script>