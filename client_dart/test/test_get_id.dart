import 'package:chikin_airdrop_pool_client/src/config.dart';
import 'package:chikin_airdrop_pool_client/src/utils.dart' as utils;
import 'package:test/scaffolding.dart';

void main() {
  final config = Config.defaultValue;
  final programId = '3K1Td3DmxWt2rxT1H4furqWJyZu3nuc7QQs6W5rtHY3P';
  final tokenMintId = '8s9FCz99Wcr3dHpiauFRi6bLXzshXfcGTfgQE7UEopVx';
  final poolAccountNonce = [1, 0, 1, 0];
  final claimerWalletId = 'DkmfiWSC4mnPvfMXZY2CkT4skvFkGr4u5DwRX2htRvJ2';

  final poolAccountId = '25sXXVsBY5Qx5QQ5w8563BmqibgkjwHBvKDBVFP52dCQ';
  final poolTokenAccountId = '7NbJf1oXinHBYq3BF528xcUUmQ9786G8xZZFAB5jGe58';
  final claimerAccountId = '3NxLy8h8CwZYYt7K8ZnqhZehrirLPKuZdyvBD1vPhS1A';
  final claimerTokenAccountId = 'Esi6Z7reZt9NjZ2TeTFRXcTez1XA7764dE9bZoKCdjTb';

  test('test getPoolAccountId', () async {
    final result = await utils.getPoolAccountId(
      programId: programId,
      tokenMintId: tokenMintId,
      nonce: poolAccountNonce,
    );
    assert(result == poolAccountId, '$result != $poolAccountId');
  });


  test('test getPoolTokenAccountId', () async {
    final result = await utils.getPoolTokenAccountId(
      programId: programId,
      poolAccountId: poolAccountId,
    );
    assert(result == poolTokenAccountId, '$result != $poolTokenAccountId');
  });

  test('test getClaimerAccountId', () async {
    final result = await utils.getClaimerAccountId(
      programId: programId,
      poolAccountId: poolAccountId,
      claimerWalletId: claimerWalletId,
    );
    assert(result == claimerAccountId, '$result != $claimerAccountId');
  });

  test('test getClaimerTokenAccountId', () async {
    final result = await utils.getClaimerTokenAccountId(
      config: config,
      tokenMintId: tokenMintId,
      claimerWalletId: claimerWalletId,
    );
    assert(result == claimerTokenAccountId, '$result != $claimerTokenAccountId');
  });
}
