## 用語

#### iptables

iptablesとは一般的なLinuxに搭載されているパケットフィルタ

パケットとは、スマホのパケット通信費などでよく出てくるように、ネットワーク上を流れるデータの事だ。パケットフィルタとは、このパケットを選別して、通すものと通さないものを分けるということを意味している。

> iptables -L

```
Chain INPUT (policy DROP)
target     prot opt source      destination   
ACCEPT     all  --  anywhere    anywhere      
ACCEPT     all  --  anywhere    anywhere      state RELATED,ESTABLISHED
LOG        all  --  anywhere    anywhere      LOG level warning prefix "drop_packet: "
 
Chain FORWARD (policy DROP)
target     prot opt source      destination   
 
Chain OUTPUT (policy ACCEPT)
target     prot opt source      destination  

```



#### ルートキット（root kit）

ルートキットは、コンピューターに潜伏し不正アクセスを手助けするツールがパッケージ化されたもの

管理者権限を意味する「ルート」が攻撃対象として狙われるため、被害が大きくなるリスクが高い。直接的な被害を与えるよりも、バックドアとして機能するなど、ほかのマルウェアの活動を補佐するルートキットが多い。ルートキットには以下のようなツールが含まれる。

オペレーティングシステムの階層まで深く入り込み、自らを隠蔽する悪質なものが存在する。

ほかのマルウェアが侵入し、攻撃するのを補佐するようなツールが含まれているため、いったん感染すると被害が大きくなりやすい

ルートキットの機能


ログ改ざんツール

マルウェアの侵入を隠蔽し、マルウェアやルートキット自体の検知・駆除を遅らせる

バックドア生成ツール

ルートキットの侵入口が防がれても、ほかのマルウェアが再び侵入できるよう裏口（バックドア）を設ける

トラフィック監視ツール

ネットワークを介してやり取りされるデータを盗聴する

キーロガーツール

キーボード入力を盗み見てログを記録する。キーボード入力から個人情報や機密情報を抽出し、他のコンピューターへ送信する

また、ルートキットには二つのモードが存在し、それぞれ特徴が異なる。

ユーザーモード

感染すると、アプリケーションのプロセスを乗っ取られる、あるいはアプリケーションが使用するメモリーが上書きされてしまうといった被害につながる。

カーネルモード

OSの最下層で実行されるもので、感染するとコンピューターは完全に制御され、自由に操作されてしまうようになる。
