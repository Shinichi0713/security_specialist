## 暗号化の種類
1. 共通鍵暗号方式
暗号化に使用する鍵と復号に使用する鍵が同じものを使用する暗号化方式。
送信側と受信側で同じ鍵を保管し、暗号化するときも復号するときも同一の鍵を使う。

__代表例__
AES（Advanced Encryption Standard）
ブロック暗号で、128、192、256ビットの鍵長を使用します。
DES（Data Encryption Standard）
古いブロック暗号で、56ビットの鍵長を使用しますが、現在は安全性の観点から使用が推奨されていません。
3DES（Triple DES）
DESを3回適用して安全性を向上させた方式ですが、近年ではAESに取って代わられつつあります。



2. 公開鍵暗号方式
暗号化に使用する鍵と、復号に使用する鍵が異なる暗号方式。
２つの鍵はペアとなっている。
片方で暗号化したものを、もう片方が復号する。
片方の鍵を秘密鍵、もう片方の鍵は誰もが自由に使える公開鍵とする。
暗号化通信する場合は、相手の公開鍵で暗号化して送信する

__代表例__

RSA（Rivest-Shamir-Adleman）
非対称鍵暗号方式の代表格で、大きな素数の積に基づく安全性を持ちます。
DSA（Digital Signature Algorithm）
デジタル署名に特化したアルゴリズムで、非対称鍵を使用します。
ECC（Elliptic Curve Cryptography）
楕円曲線に基づく暗号方式で、同等の安全性を提供しながら鍵長を短く保つことができます。

3. ハイブリッド暗号方式
通信は共通鍵で行い、共通鍵の鍵交換を公開鍵暗号方式を用いて実施する方式。

__代表例__

TLS（Transport Layer Security）

インターネット上での安全な通信を確保するために広く利用されています。TLSは、セッションの開始時に非対称鍵暗号（通常はRSAやECDHE）を用いて対称鍵を交換し、その後の通信にはAESなどの対称鍵暗号を使用します。
PGP（Pretty Good Privacy）

主に電子メールの暗号化に使用される方式で、データ自体を対称鍵暗号（通常はAES）で暗号化し、使用する鍵を非対称鍵暗号（RSAなど）で暗号化します。
S/MIME（Secure/Multipurpose Internet Mail Extensions）

こちらも電子メールのセキュリティを目的とした方式で、PCKS#7形式を用いてメッセージを非対称鍵暗号で暗号化し、その中に対称鍵を格納します。


## CCMP
CCMP(Counter Mode with Cipher Block Chaining Message  Authentication Code Protocol)は、Wi-Fi セキュリティにおけるAESベースの暗号化プロトコル。
WPA2で標準的に採用され、高いデータ保護と改ざん防止機能を提供します。
WPA2(Wi-Fi Protected Access II)およびWPA3のセキュリティ規格において、標準的な暗号化手段として採用されています。

CCMPは、従来のTKIP(Temporal Key Integrity Protocol)に比べて、より強固なセキュリティを提供します。

具体的には、CTR(Counter)モードを使用した暗号化と、CBC-MAC(Cipher Block Chaining Message  Authentication Code)を使用したメッセージ 認証を組み合わせることで、高速かつ安全なデータ通信を実現しています。

この組み合わせにより、データの不正アクセスや改ざんを防ぎ、ネットワーク全体のセキュリティを向上させています。
