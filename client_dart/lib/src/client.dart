import 'package:chikin_airdrop_pool_client/chikin_airdrop_pool_client.dart';
import 'package:solana/solana.dart';

import 'utils.dart' as utils;

Future<AirdropPool> getAirdropPool({
  required RPCClient rpcClient,
  required Config config,
  required String poolAccountId,
}) async {
  final poolAccount = await rpcClient.getAccountInfo(poolAccountId);
  return AirdropPool.unpack(poolAccount.data);
}

Future<AirdropClaimer> getAirdropClaimer({
  required RPCClient rpcClient,
  required Config config,
  required String poolAccountId,
  required String claimerWalletId,
}) async {
  final claimerAccountId = await utils.getClaimerAccountId(
    programId: config.programId,
    poolAccountId: poolAccountId,
    claimerWalletId: claimerWalletId,
  );
  final claimerAccount = await rpcClient.getAccountInfo(claimerAccountId);
  return AirdropClaimer.unpack(claimerAccount.data);
}
