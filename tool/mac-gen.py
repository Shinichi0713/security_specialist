import hmac
import hashlib

# 共通鍵
key = b'secretkey123'

# 送信するメッセージ
message = b'Hello Bob!'

# 送信者がMACを作る
mac = hmac.new(key, message, hashlib.sha256).hexdigest()
print("送信するMAC:", mac)

# 受信者が検証
verify_mac = hmac.new(key, message, hashlib.sha256).hexdigest()
if mac == verify_mac:
    print("改ざんなし！")
else:
    print("改ざんの可能性あり！")
