from ledgerblue.commTCP import getDongle as getDongleTCP
from ledgerblue.comm import getDongle

from random import getrandbits as rnd
from binascii import hexlify, unhexlify
from time import sleep

MAX_CHUNK_SIZE = 255

# --apdu-port
d = getDongleTCP(port=40000)     # Speculos
# d = getDongle()               # Nano

def chunks(lst, n):
    """Yield successive n-sized chunks from lst."""
    for i in range(0, len(lst), n):
        yield lst[i:i + n]

def serialize(buffer):
    return bytes([len(buffer)]) + buffer

def generate_apdu_cmd(data=b"", p1=0, p2=0):
    apdu = bytes([0x80, 0x03, p1, p2])
    apdu += serialize(data)
    return apdu

# 32 byte CryptoHasher seed prefix for the RawTxn struct. Comes from here https://github.com/diem/diem/blob/6c36ab4b32b20069147caf1b671493e76d8097f0/crypto/crypto/src/ed25519.rs#L238
# sha3_256("DIEM::RawTransaction")
raw_txn_crypto_hash_prefix = "e74c3978c4493b06fec031b3b5b97fee945b2d7628528d85d19509dab9f4189c"
msg = raw_txn_crypto_hash_prefix + "e1b3d22871989e9fd9dc6814b2f4fc412a0000000000000001e101a11ceb0b010000000701000202020403061004160205181d0735610896011000000001010000020001000003020301010004010300010501060c0108000506080005030a020a020005060c05030a020a020109000c4c696272614163636f756e741257697468647261774361706162696c6974791b657874726163745f77697468647261775f6361706162696c697479087061795f66726f6d1b726573746f72655f77697468647261775f6361706162696c69747900000000000000000000000000000001010104010c0b0011000c050e050a010a020b030b0438000b05110202010700000000000000000000000000000001034c4252034c425200040371e931795d23e9634fd24a5992065f6b0164000000000000000400040040420f00000000000000000000000000034c4252fc24f65e0000000004"

msg_bytes = unhexlify(msg)

chunked_msg = list(chunks(msg_bytes, MAX_CHUNK_SIZE))

for (i, message) in enumerate(chunked_msg):
    last_chunk = (i == (len(chunked_msg) - 1))
    p2 = 0
    if last_chunk:
        p2 = 1
    apdu_cmd = generate_apdu_cmd(message, i, p2)
    # print(hexlify(apdu_cmd))
    r = None
    try:
        r = d.exchange(apdu_cmd, 20)
        sleep(1)
    except Exception as e:
        print(e)
    if r is not None:
        if len(r) != 0:
            print("Txn Signature : ", hexlify(r))
