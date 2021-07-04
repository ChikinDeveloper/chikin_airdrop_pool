import 'package:chikin_airdrop_pool_client/src/command.dart';
import 'package:chikin_airdrop_pool_client/src/config.dart';
import 'package:chikin_airdrop_pool_client/src/utils.dart' as utils;
import 'package:solana/solana.dart' as solana;
import 'package:test/scaffolding.dart';

import 'utils/test_claimer.dart';

void main() {
  final defaultTimeoutDuration = Duration(seconds: 90);

  var _logIndex = 0;
  final debugPrint = () {
    print('test_claim.debugPrint : ${_logIndex++}');
  };

  test('test_claim', () async {
    final rpcUrl = 'http://localhost:8899';
    final rpcClient = solana.RPCClient(rpcUrl);
    final config = Config.defaultValue;

    final tokenMintId = 'G1ZHgFRiTvdwymXugMLPMWvTSsuxwewNVNJ9puq75eEu';
    final poolAccountNonce = [1, 0, 1, 0];

    debugPrint();
    final poolAccountId = await utils.getPoolAccountId(
        programId: config.programId,
        tokenMintId: tokenMintId,
        nonce: poolAccountNonce);

    debugPrint();
    final claimer1 = await TestClaimer.create(
        rpcClient: rpcClient, config: config, tokenMint: tokenMintId);

    debugPrint();
    var tmpTokenAccount = await rpcClient.getTokenAccountBalance(
      associatedTokenAccountAddress: claimer1.tokenAccount,
      commitment: solana.Commitment.finalized,
    );
    print('claimer1.balance.1 = ${await rpcClient.getBalance(claimer1.wallet.address)}');
    print('claimer1.tokenAccount.uiAmountString.1 = ${tmpTokenAccount.uiAmountString}');

    debugPrint();
    final message = solana.Message(
      instructions: [
        await Command.claim(
          rpcClient: rpcClient,
          config: config,
          tokenMintId: tokenMintId,
          poolAccountId: poolAccountId,
          claimerWalletId: claimer1.wallet.address,
        ),
      ],
    );
    debugPrint();
    final transactionSignature = await rpcClient.signAndSendTransaction(
      message,
      [claimer1.wallet],
      feePayer: claimer1.wallet.address,
      commitment: solana.Commitment.finalized,
    );

    debugPrint();
    await rpcClient.waitForSignatureStatus(
        transactionSignature, solana.Commitment.finalized,
        timeout: defaultTimeoutDuration);

    debugPrint();
    tmpTokenAccount = await rpcClient.getTokenAccountBalance(
      associatedTokenAccountAddress: claimer1.tokenAccount,
      commitment: solana.Commitment.finalized,
    );
    print('claimer1.balance.2 = ${await rpcClient.getBalance(claimer1.wallet.address)}');
    print('claimer1.tokenAccount.uiAmountString.2 = ${tmpTokenAccount.uiAmountString}');
  }, timeout: Timeout(defaultTimeoutDuration));
}
