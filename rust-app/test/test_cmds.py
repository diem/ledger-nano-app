from ledgerblue.commTCP import getDongle as getDongleTCP
from ledgerblue.comm import getDongle

from random import getrandbits as rnd
from binascii import hexlify, unhexlify

rand_msg = hexlify(rnd(256).to_bytes(32, 'big')).decode()

CMDS = [
    "8002", # get pubkey
    # "8003" + "0000" + "20" + "00112233445566778899aabbccddeeff0123456789abcdeffedcba9876543210", # sign message
    # "80032000" + rand_msg,
    # "8004",
    # "80050008",
    # "80FE", # get private key
    # "80FF",
]
# --apdu-port
d = getDongleTCP(port=40000)     # Speculos
# d = getDongle()               # Nano

from time import sleep
for cmd in map(unhexlify,CMDS):
    r = None
    try:
        r = d.exchange(cmd, 20)
        sleep(1)
    except Exception as e:
        print(e)
    if r is not None:
        print("Response : ", hexlify(r))
