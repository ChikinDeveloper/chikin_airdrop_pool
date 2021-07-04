import 'utils.dart' as utils;

class AirdropPool {
  static const packedSize = 153;

  final List<int> tokenProgramId;
  final List<int> tokenMintId;
  final List<int> accountId;
  final List<int> tokenAccountId;
  final List<int> poolAccountNonce;
  final int rewardPerAccount;
  final int rewardPerReferral;
  final int maxReferralDepth;
  final bool isInitialized;

  AirdropPool({
    required this.tokenProgramId,
    required this.tokenMintId,
    required this.accountId,
    required this.tokenAccountId,
    required this.poolAccountNonce,
    required this.rewardPerAccount,
    required this.rewardPerReferral,
    required this.maxReferralDepth,
    required this.isInitialized,
  });

  static AirdropPool unpack(List<int> data) {
    assert(data.length == packedSize);
    return AirdropPool(
      tokenProgramId: data.sublist(0, 32),
      tokenMintId: data.sublist(32, 64),
      accountId: data.sublist(64, 96),
      tokenAccountId: data.sublist(96, 128),
      poolAccountNonce: data.sublist(128, 132),
      rewardPerAccount: utils.unpackUInt(data.sublist(132, 140)),
      rewardPerReferral: utils.unpackUInt(data.sublist(140, 148)),
      maxReferralDepth: utils.unpackUInt(data.sublist(148, 152)),
      isInitialized: data[152] != 0,
    );
  }
}

class AirdropClaimer {
  static const packedSize = 34;

  final List<int>? referrerWallet;
  final bool claimed;

  AirdropClaimer({this.referrerWallet, required this.claimed});

  static AirdropClaimer unpack(List<int> data) {
    assert(data.length == packedSize);
    return AirdropClaimer(
      referrerWallet: (data[0] == 0) ? null : data.sublist(1, 33),
      claimed: data[33] != 0,
    );
  }
}

abstract class AirdropPoolInstruction {
  AirdropPoolInstruction._();

  List<int> pack();
}

class AirdropPoolInstructionInitialize extends AirdropPoolInstruction {
  final List<int> poolAccountNonce;
  final int rewardPerAccount;
  final int rewardPerReferral;
  final int maxReferralDepth;

  AirdropPoolInstructionInitialize({
    required this.poolAccountNonce,
    required this.rewardPerAccount,
    required this.rewardPerReferral,
    required this.maxReferralDepth,
  }) : super._();

  @override
  List<int> pack() {
    return [
      0,
      ...poolAccountNonce,
      ...utils.packUInt64(rewardPerAccount),
      ...utils.packUInt64(rewardPerReferral),
      ...utils.packUInt32(maxReferralDepth),
    ];
  }
}

class AirdropPoolInstructionClaim extends AirdropPoolInstruction {
  final List<int>? referrerWallet;

  AirdropPoolInstructionClaim({required this.referrerWallet}) : super._();

  @override
  List<int> pack() {
    return [1, ...(referrerWallet ?? List.generate(32, (e) => 0))];
  }
}
