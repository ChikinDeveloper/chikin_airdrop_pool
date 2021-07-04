import 'package:chikin_airdrop_pool_client/src/config.dart';
import 'package:chikin_airdrop_pool_client/src/utils.dart' as utils;
import 'package:solana/solana.dart' as solana;
import 'package:solana/src/associated_token_account_program/associated_token_account_program.dart'
    as associated_token_account_program;
import 'package:solana/src/crypto/ed25519_hd_keypair.dart';

import 'test_utils.dart' as test_utils;

class TestClaimer {
  final Ed25519HDKeyPair wallet;
  final String tokenAccount;

  TestClaimer._({
    required this.wallet,
    required this.tokenAccount,
  });

  static Future<TestClaimer> create({
    required solana.RPCClient rpcClient,
    required Config config,
    required String tokenMint,
    int lamports = 10000000,
  }) async {
    var _logIndex = 0;
    final debugPrint = () {
      print('TestClaimer.create.debugPrint : ${_logIndex++}');
    };

    debugPrint();
    final wallet =
        await test_utils.newAccountWithLamports(rpcClient: rpcClient);
    debugPrint();
    final tokenAccountId = await utils.getClaimerTokenAccountId(
      config: config,
      tokenMintId: tokenMint,
      claimerWalletId: wallet.address,
    );

    // Create token account
    debugPrint();
    final message = solana.Message(instructions: [
      associated_token_account_program.AssociatedTokenAccountInstruction(
        funder: wallet.address,
        address: tokenAccountId,
        owner: wallet.address,
        mint: tokenMint,
      ),
    ]);
    debugPrint();
    await rpcClient.signAndSendTransaction(message, [
      wallet,
    ]);

    debugPrint();
    return TestClaimer._(
      wallet: wallet,
      tokenAccount: tokenAccountId,
    );
  }
}
