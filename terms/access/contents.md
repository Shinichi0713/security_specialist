### 検疫ネットワーク

IEEE802.1Xを応用することで検疫NWを構築できる

一定の基準を満たしていないと社内LANには接続できないようにできる


## IEEE802.1X

IEEE802.1Xとは、有線LANや無線LANにおけるユーザ認証の規格

当初は有線LANでクライアントPCをネットワーク接続する際にユーザ認証を行う目的で策定された規格

無線LANの初期においてはWEPによるセキュリティしかなかったため、WEPにはないユーザ認証などの仕組みが無線LAN環境においてセキュリティ的に最適であったことから有線LANよりも無線LAN環境で先に普及

#### IEEE802.1Xの構成要素

IEEE802.1X認証を行うためにはサプリカント、認証装置、認証サーバの3つの構成要素が必要

- サプリカント（Supplicant)

IEEE802.1Xにおけるクライアント。認証を受けるクライアントはPCにインストールする必要があるが、最近のPCには標準搭載されている。

- 認証装置(Authenticator)

サプリカントと認証サーバの仲介役となるネットワーク機器。IEEE802.1X対応のLANスイッチまたは無線LANアクセスポイントのこと。

- 認証サーバ(Authentication Server)

ユーザ認証を行うサーバのこと。IEEE802.1X/EAPに対応したRadiusサーバを使用する。


#### EAPOL **【EAP over LAN】**

[認証](https://e-words.jp/w/%E8%AA%8D%E8%A8%BC.html)[プロトコル](https://e-words.jp/w/%E3%83%97%E3%83%AD%E3%83%88%E3%82%B3%E3%83%AB.html)の[EAP](https://e-words.jp/w/EAP.html)を[イーサネット](https://e-words.jp/w/%E3%82%A4%E3%83%BC%E3%82%B5%E3%83%8D%E3%83%83%E3%83%88.html)([Ethernet](https://e-words.jp/w/%E3%82%A4%E3%83%BC%E3%82%B5%E3%83%8D%E3%83%83%E3%83%88.html))や[Wi-Fi](https://e-words.jp/w/Wi-Fi.html)などの[LAN](https://e-words.jp/w/LAN.html)を通じて利用するための標準仕様

企業などの組織内[ネットワーク](https://e-words.jp/w/%E3%83%8D%E3%83%83%E3%83%88%E3%83%AF%E3%83%BC%E3%82%AF.html)で[IEEE 802.1X認証](https://e-words.jp/w/IEEE_802.1X.html)を利用する際などに標準的に用いられる。

IEEE802.1X以外の認証規格


* **EAP (Extensible Authentication Protocol)** : これは、様々な認証方法をサポートするプロトコルで、PPP（Point-to-Point Protocol）を拡張したものです。EAPは、IEEE802.1X以外の規格でも利用されています。
* **PAP (Password Authentication Protocol)** : これは、IDとパスワードを平文で送信するシンプルな認証プロトコルです。
* **CHAP (Challenge-Handshake Authentication Protocol)** : これは、パスワードをハッシュ化して送信することでセキュリティを強化した認証プロトコルです。
* **RADIUS (Remote Authentication Dial-In User Service)** : これは、ネットワークアクセスの認証、承認、およびアカウンティングを提供するプロトコルです。





## ICカード

#### PIN

盗難防止のためにPINによる認証

ロックを解消するのは新たな鍵ペア生成、PINの再設定、
