// import 'package:chikin_airdrop_pool_client/src/config.dart';
// import 'package:solana/solana.dart' as solana;
// import 'package:solana/src/crypto/ed25519_hd_keypair.dart';
//
// class TestToken {
//   final Ed25519HDKeyPair mintAuthority;
//   final Ed25519HDKeyPair mint;
//
//   TestToken._({required this.mintAuthority, required this.mint});
//
//   static Future<TestToken> create(solana.RPCClient rpcClient, Config config) async {
//     final mintAuthority = await Ed25519HDKeyPair.random();
//     final mint = await Ed25519HDKeyPair.random();
//
//     final splTokenMintPackedSize = 82;
//     final minimumBalanceForRentExemption = await rpcClient.getMinimumBalanceForRentExemption(splTokenMintPackedSize)
//
//
//   }
// }