from ecdsa import SigningKey, SECP256k1
import hashlib, base58

def gen_wallet():
    sk = SigningKey.generate(curve=SECP256k1)
    vk = sk.verifying_key
    sha = hashlib.sha256(vk.to_string()).digest()
    rip = hashlib.new('ripemd160', sha).digest()
    addr = base58.b58encode_check(b'\x00' + rip).decode()
    return sk.to_string().hex(), vk.to_string().hex(), addr
