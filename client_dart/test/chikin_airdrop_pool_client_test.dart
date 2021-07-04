// import 'package:chikin_airdrop_pool_client/src/command.dart';
// import 'package:chikin_airdrop_pool_client/src/config.dart';
// import 'package:chikin_airdrop_pool_client/src/utils.dart' as utils;
// import 'package:solana/solana.dart';
// import 'package:test/scaffolding.dart';
// import 'utils/test_utils.dart' as test_utils;
//
// void main() {
//   test('Test', () async {
//     final rpcUrl = 'http://localhost:8899';
//     final rpcClient = RPCClient(rpcUrl);
//     final config = Config.defaultValue;
//
//     final tokenMintId = '';
//     final feePayer = await test_utils.newAccountWithLamports(rpcClient: rpcClient);
//     final claimerWalletId = '';
//     final poolAccountNonce = [1, 0, 1, 0];
//
//     final poolAccountId = await utils.getPoolAccountId(
//         programId: config.programId,
//         tokenMintId: tokenMintId,
//         nonce: poolAccountNonce);
//
//     final initializeCommand = await Command.initialize(
//       config: config,
//       payerId: feePayer.address,
//       tokenMintId: tokenMintId,
//       poolAccountNonce: poolAccountNonce,
//     );
//
//     final claimCommand = await Command.claim(
//       rpcClient: rpcClient,
//       config: config,
//       tokenMintId: tokenMintId,
//       poolAccountId: poolAccountId,
//       claimerWalletId: claimerWalletId,
//     );
//
//     final message = Message(instructions: [initializeCommand, claimCommand]);
//     final transactionSignature = await rpcClient.signAndSendTransaction(
//       message,
//       [],
//       feePayer: feePayer.address,
//       commitment: Commitment.confirmed,
//     );
//   });
// }
