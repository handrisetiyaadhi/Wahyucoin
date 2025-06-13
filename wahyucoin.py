import hashlib, time, json
from config import *

class Block:
    def __init__(self, index, prev_hash, timestamp, txs, nonce, miner):
        self.index = index
        self.prev_hash = prev_hash
        self.timestamp = timestamp
        self.txs = txs
        self.nonce = nonce
        self.miner = miner
        self.hash = self.calc_hash()

    def calc_hash(self):
        data = f"{self.index}{self.prev_hash}{self.timestamp}{self.txs}{self.nonce}{self.miner}"
        return hashlib.sha256(data.encode()).hexdigest()

class Blockchain:
    def __init__(self):
        self.chain = []
        self.difficulty = DIFFICULTY
        self.reward = INITIAL_REWARD
        self.total_minted = 0
        self.create_genesis()

    def create_genesis(self):
        tx = {'to': 'GENESIS', 'amount': PREMINE}
        blk = Block(0, '0'*64, time.time(), [tx], 0, 'GENESIS')
        self.chain.append(blk)
        self.total_minted += PREMINE

    def mine_block(self, txs, miner_addr):
        if self.total_minted < TOTAL_SUPPLY:
            reward = self.reward
        else:
            reward = 0
        idx = len(self.chain)
        prev = self.chain[-1].hash
        ts = time.time()
        nonce = 0
        while True:
            blk = Block(idx, prev, ts, txs + [{'to': miner_addr,'amount': reward}], nonce, miner_addr)
            if blk.hash.startswith('0'*self.difficulty):
                break
            nonce += 1

        self.chain.append(blk)
        self.total_minted += reward
        if len(self.chain) % HALVING_INTERVAL == 0:
            self.reward = max(1, self.reward // 2)
        return blk
