import 'utils.dart' as utils;

class AirdropPool {
  static const packedSize = 85;

  final List<int> tokenProgramId;
  final List<int> tokenMintId;
  final List<int> poolAccountNonce;
  final int rewardPerAccount;
  final int rewardPerReferral;
  final int maxReferralDepth;

  AirdropPool({
    required this.tokenProgramId,
    required this.tokenMintId,
    required this.poolAccountNonce,
    required this.rewardPerAccount,
    required this.rewardPerReferral,
    required this.maxReferralDepth,
  });

  static AirdropPool unpack(List<int> data) {
    assert(data.length == packedSize);
    return AirdropPool(
      tokenProgramId: data.sublist(0, 32),
      tokenMintId: data.sublist(32, 64),
      poolAccountNonce: data.sublist(64, 68),
      rewardPerAccount: utils.unpackUInt(data.sublist(68, 76)),
      rewardPerReferral: utils.unpackUInt(data.sublist(76, 84)),
      maxReferralDepth: data[84],
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
  static const packedSize = 34;

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
    final result = [
      0,
      ...poolAccountNonce,
      ...utils.packUInt64(rewardPerAccount),
      ...utils.packUInt64(rewardPerReferral),
      ...utils.packUInt32(maxReferralDepth),
    ];
    assert(result.length <= AirdropPoolInstruction.packedSize);
    result.addAll(List.filled(AirdropPoolInstruction.packedSize - result.length, 0));
    return result;
  }
}

class AirdropPoolInstructionClaim extends AirdropPoolInstruction {
  final List<int>? referrerWallet;

  AirdropPoolInstructionClaim({required this.referrerWallet}) : super._();

  @override
  List<int> pack() {
    final result = [1, ...(referrerWallet ?? List.generate(32, (e) => 0))];
    assert(result.length <= AirdropPoolInstruction.packedSize);
    result.addAll(List.filled(AirdropPoolInstruction.packedSize - result.length, 0));
    return result;
  }
}
