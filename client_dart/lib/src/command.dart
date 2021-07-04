import 'package:chikin_airdrop_pool_client/src/config.dart';
import 'package:chikin_airdrop_pool_client/src/model.dart';
import 'package:solana/solana.dart';
import 'utils.dart' as utils;

class Command {
  static Future<Instruction> initialize({
    required Config config,
    required String payerId,
    required String tokenMintId,
    required List<int> poolAccountNonce,
  }) async {
    final poolAccountId = await utils.getPoolAccountId(programId: config.programId, tokenMintId: tokenMintId, nonce: poolAccountNonce);
    final poolTokenAccountId = await utils.getPoolTokenAccountId(programId: config.programId, poolAccountId: poolAccountId);
    
    return Instruction(
      programId: config.programId,
      accounts: [
        AccountMeta.writeable(pubKey: payerId, isSigner: true),
        AccountMeta.readonly(pubKey: config.programId, isSigner: false),
        AccountMeta.readonly(pubKey: config.rentSysvarId, isSigner: false),
        AccountMeta.readonly(pubKey: config.systemProgramId, isSigner: false),
        AccountMeta.readonly(pubKey: config.tokenProgramId, isSigner: false),
        AccountMeta.readonly(pubKey: tokenMintId, isSigner: false),
        AccountMeta.readonly(pubKey: poolAccountId, isSigner: false),
        AccountMeta.readonly(pubKey: poolTokenAccountId, isSigner: false),
      ],
      data: AirdropPoolInstructionInitialize(
        poolAccountNonce: poolAccountNonce,
        rewardPerAccount: 500,
        rewardPerReferral: 100,
        maxReferralDepth: 2,
      ).pack(),
    );
  }

  static Future<Instruction> claim({
    required RPCClient rpcClient,
    required Config config,
    required String tokenMintId,
    required String poolAccountId,
    required String claimerWalletId,
  }) async {
    final poolTokenAccountId = await utils.getPoolTokenAccountId(programId: config.programId, poolAccountId: poolAccountId);
    final claimerAccountId = await utils.getClaimerAccountId(programId: config.programId, poolAccountId: poolAccountId, claimerWalletId: claimerWalletId);
    final claimerTokenAccountId = await utils.getClaimerTokenAccountId(rpcClient: rpcClient, tokenMintId: tokenMintId, claimerWalletId: claimerWalletId);
    
    return Instruction(
      programId: config.programId,
      accounts: [
        AccountMeta.readonly(pubKey: config.programId, isSigner: false),
        AccountMeta.readonly(pubKey: config.rentSysvarId, isSigner: false),
        AccountMeta.readonly(pubKey: config.systemProgramId, isSigner: false),
        AccountMeta.readonly(pubKey: config.tokenProgramId, isSigner: false),
        AccountMeta.readonly(pubKey: tokenMintId, isSigner: false),
        AccountMeta.readonly(pubKey: poolAccountId, isSigner: false),
        AccountMeta.readonly(pubKey: poolTokenAccountId, isSigner: false),
        AccountMeta.readonly(pubKey: claimerWalletId, isSigner: false),
        AccountMeta.readonly(pubKey: claimerAccountId, isSigner: false),
        AccountMeta.readonly(pubKey: claimerTokenAccountId, isSigner: false),
      ],
      data: AirdropPoolInstructionClaim(
        referrerWallet: null,
      ).pack(),
    );
  }
}