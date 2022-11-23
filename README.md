# Blink
Partake in a cutting-edge crypto/blockchain startup with the goal of revolutionizing finance.
Maximum Extractable Value (MEV), formerly named Miner Extractable Value, is a measure of the total value that can be extracted permissionlessly (i.e. without any special rights) from transaction ordering.
Blink is a blockchain crawler which scans Ethereum and identifies MEV extraction activity.
We do not cover CEX-DEX arbitrage since the CEX bit of the trade does not have publicly available data for us to crawl.

Blink is a FAAS (Front Running As A Service or MEV) for Market Maker Firms, its purpose is to discover MEV opportunities.
We check/analyse transactions in the Mempool (unconfirmed transactions) and Oracle updates.
Validators should give priority not to bigger fees but better package. (Searchers, Builders)

TODO!
- Liquidity
- Gaz fees (priority)
- Nonce for next transaction
- Coingecko's price for the asset. (We normalize all our Extracted MEV calculations first to USD)
- Nonce, block template, latency, PGAs
- Strategies : Reactive Blind, Grim trigger, Network connectivity, Time-bandit

TOCHECK!
- Chainlink FSS (ordering policies)
- Mev-boost
- Read Flash Boys 2.0

Plusieurs pairs 
Arbitrage, change collatéral 
Self liquidation??
Front-Run (pays more gaz)
Slippage!!

DATA!
No data gaps, market open market close. (24/7)
Erigon archive node with tracing enabled, PostgreSQL database for further querying and analysis.

Diesel : Rust & SQL

Ethereum archive node, which we setup using go-ethereum.
(https://github.com/ethereum/go-ethereum)
An archive node provides a complete history of all state changes on the Ethereum blockchain, which allowed us to query data on any published block.
We store our collected data inside a MongoDB database.

Tokio:
fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // ...
        })
}

IDEAS:
Never lose lottery.
Projet, ajouter à chaque fois bitcoin ou eth ou.. (verification?)
Quicknode : 2 nodes, load balancer.
All SC will be writen.
Multi sig

SECURITY:
Solid experience in threat analysis, advanced persistent threat (APT) or response.
Experienced in threat and vulnerability management, penetration testing, and SecOps (intrusion detection, security logging, malware analysis, and forensics)

Expertise in networking, consensus protocols, parallel execution, storage, authenticated data structures, and/or caching

Experience with key-value store and/or relational databases, file systems, caches.
Familiarity with data structures like search trees, prefix trees, Merkle Trees, LSM Trees, BTrees, etc.
Experience with system performance optimization.
Implement such data structures using Rust on top of key-value or relational databases, with support for multiversion concurrency control (MVCC).
Design efficient data structures to represent the blockchain.
Design and implement mechanisms that incentivizes smart contract developers and blockchain users to use on-chain storage space efficiently and responsibly, and keeps the cost low as well.
Optimize the data flow inside of Aptos nodes to maximize the transaction throughput and minimize the transaction finality latency.

You can analyze and optimize the platform’s performance.
You have experience with security audits of third-party and internal solutions.

Experience with alerting and monitoring tools is plus

Know your way around Linux CLI
Rust
Node.js
C++
MongoDB
Redis
Docker
Deep understanding of the full web technology stack (e.g. HTTP, cookies, asset loading, caching, REST, websockets, gRPC, etc)
Interest in Blockchain/DLT and digital assets

Background or interest in financial/capital markets

Stream-based processing

Kubernetes

Proactive challenger and proposer

Statistical analysis and/or ML

ow-latency trading systems Keyrock

I have 7 years of experience as a C++ software engineer, I just discovered Rust and I love it and can't wait to master it!
Best way to learn is to practice, was hoping to find a job to do so and not only work on my own.
I'm really curious to know what your software is about and how it can be deployed in any type of asset, so I hope I'll get the chance to  talk you. I'm also really passionate about trading, that's why I really like DeFi, I even work in a well known Fintech.
With several years of experience as a computer science engineer and after having recently obtained my MBA, having both technical and business background I can be a great asset to Keyrock.

Algorithm Developer with Rust (Remote possible)

account abstraction : multicall
session based wallet : authorise all transactions
sign to have a session wallet : multicall (login, logout)
ARGENT : Zksync & Starknet

Liquid staking 