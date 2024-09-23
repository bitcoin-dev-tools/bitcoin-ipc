# Copyright (c) 2024 The Bitcoin Core developers
# Distributed under the MIT software license, see the accompanying
# file COPYING or http://www.opensource.org/licenses/mit-license.php.

@0xc77d03df6a41b505;

# using Cxx = import "/capnp/c++.capnp";
# $Cxx.namespace("ipc::capnp::messages");

using Proxy = import "proxy.capnp";
# $Proxy.include("interfaces/mining.h");
# $Proxy.includeTypes("ipc/capnp/mining-types.h");

interface Mining $Proxy.wrap("interfaces::Mining") {
    isTestChain @0 (context :Proxy.Context) -> (result: Bool);
    isInitialBlockDownload @1 (context :Proxy.Context) -> (result: Bool);
    getTipHash @2 (context :Proxy.Context) -> (result: Data);
    createNewBlock @3 (scriptPubKey: Data, options: BlockCreateOptions) -> (result: BlockTemplate);
    processNewBlock @4 (context :Proxy.Context, block: Data) -> (newBlock: Bool, result: Bool);
    getTransactionsUpdated @5 (context :Proxy.Context) -> (result: UInt32);
    testBlockValidity @6 (context :Proxy.Context, block: Data, checkMerkleRoot: Bool) -> (state: BlockValidationState, result: Bool);
}

interface BlockTemplate $Proxy.wrap("interfaces::BlockTemplate") {
    getBlockHeader @0 (context: Proxy.Context) -> (result: Data);
    getBlock @1 (context: Proxy.Context) -> (result: Data);
    getTxFees @2 (context: Proxy.Context) -> (result: List(Int64));
    getTxSigops @3 (context: Proxy.Context) -> (result: List(Int64));
    getCoinbaseTx @4 (context: Proxy.Context) -> (result: Data);
    getCoinbaseCommitment @5 (context: Proxy.Context) -> (result: Data);
    getWitnessCommitmentIndex @6 (context: Proxy.Context) -> (result: Int32);
}

struct BlockCreateOptions $Proxy.wrap("node::BlockCreateOptions") {
    useMempool @0 :Bool $Proxy.name("use_mempool");
    coinbaseMaxAdditionalWeight @1 :UInt64 $Proxy.name("coinbase_max_additional_weight");
    coinbaseOutputMaxAdditionalSigops @2 :UInt64 $Proxy.name("coinbase_output_max_additional_sigops");
}

struct BlockValidationState {
    mode @0 :Int32;
    result @1 :Int32;
    rejectReason @2 :Text;
    debugMessage @3 :Text;
}
