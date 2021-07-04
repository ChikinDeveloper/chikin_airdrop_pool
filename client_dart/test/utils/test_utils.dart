import 'dart:io';

import 'package:solana/solana.dart' as solana;
import 'package:solana/src/crypto/ed25519_hd_keypair.dart';

Future<Ed25519HDKeyPair> newAccountWithLamports({
  required solana.RPCClient rpcClient,
  int lamports = 10000000,
}) async {
  final keypair = await Ed25519HDKeyPair.random();

  for (var i = 0; i < 30; i++) {
    await rpcClient.requestAirdrop(
      address: keypair.address,
      lamports: lamports,
    );
    final balance = await rpcClient.getBalance(keypair.address);
    if (balance == lamports) {
      break;
    }
    sleep(Duration(milliseconds: 500));
  }

  final balance = await rpcClient.getBalance(keypair.address);
  assert(balance == lamports);
  return keypair;
}
