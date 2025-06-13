from wahyucoin import Blockchain
from wallet import gen_wallet

if __name__ == "__main__":
    bc = Blockchain()
    sk, pk, addr = gen_wallet()
    print("Miner address:", addr)

    while True:
        blk = bc.mine_block([], addr)
        print(f"Mined block #{blk.index} with hash: {blk.hash}  Reward: {blk.txs[-1]['amount']}")
