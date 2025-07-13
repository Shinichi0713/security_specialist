# 重点対策1章

## 1

### 1-(1)

#### 解答

<textarea rows="4" id="answer" cols="50" oninput="countCharacters()">
多要素認証をSサービス側に用意しなくても良い
</textarea>

正誤

- **得点:** [6]
- **配点:** [6]

### 問題番号

### 1-(2)

#### 解答

<textarea rows="4" cols="50">
Tサービスに障害が起こると、Sサービスで多要素認証を行うことが出来なくなる。
</textarea>

正誤

- **得点:** [3 ]
- **配点:** [6 ]

### 1-(3)

#### 解答

<textarea rows="4" cols="50">
a:Sサービス
b:Tサービス
c:利用者
</textarea>

#### 正誤

- **得点:** [ 6]
- **配点:** [6 ]

## 1-(4)

### 解答

`<textarea id="answer" rows="4" cols="50" oninput="countCharacters()">`

`え`

`</textarea>`

<p>文字数: <span id="charCount">0</span></p>

### 正誤

- **得点:** [2 ]
- **配点:** [2 ]

<script>
function countCharacters() {
    var text = document.getElementById('answer').value;
    var charCount = text.length;
    document.getElementById('charCount').innerText = charCount;
}
</script>


### 2-(1)

#### 解答

`<textarea rows="4" cols="50">d:ウ e:ア</textarea>`

#### 正誤

- **得点:** [4 ]
- **配点:** [ 4]


### 2-(2)

#### 解答

<textarea rows="4" cols="50">
攻撃者
</textarea>

#### 正誤

- **得点:** [ ]
- **配点:** [ ]


### 2-(3)

#### 解答

<textarea rows="4" cols="50">
β:い
γ:か
</textarea>

#### 正誤

- **得点:** [4 ]
- **配点:** [ 4]



### 3-(1)

#### 解答

<textarea rows="4" cols="50">
ア
</textarea>

#### 正誤

- **得点:** [ 0]
- **配点:** [2 ]


### 3-(2)

#### 解答

<textarea rows="4" cols="50">
SサービスにIDの登録がないS会員
</textarea>

#### 正誤

- **得点:** [ 3]
- **配点:** [ 6]


### 4

#### 解答

<textarea rows="4" cols="50">
Tサービスに利用者IDとパスワードの認証がされた利用者であるかを認証している
</textarea>

#### 正誤

- **得点:** [ 0]
- **配点:** [ 8]

## SSO
個別に導入されたシステムが独立して稼働している状況では利用者は数多くのIDとパスワードを記憶・管理しなければならない。
それを一度ログインするだけで、複数のサーバーに自由にアクセスできるようにすることをシングルサインオンという。



