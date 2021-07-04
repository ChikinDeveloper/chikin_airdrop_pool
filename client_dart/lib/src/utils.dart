import 'dart:convert';
import 'dart:math' as math;
import 'dart:typed_data';
import 'package:solana/src/utils.dart' as solana_utils;
import 'package:solana/solana.dart' as solana;

import 'config.dart';

Future<String> getPoolAccountId({
  required String programId,
  required String tokenMintId,
  required List<int> nonce,
}) {
  return solana_utils.findProgramAddress(
    seeds: [
      solana.base58decode(programId),
      solana.base58decode(tokenMintId),
      utf8.encode('pool_account'),
      nonce,
    ],
    programId: programId,
  );
}

Future<String> getPoolTokenAccountId({
  required String programId,
  required String poolAccountId,
}) {
  return solana_utils.findProgramAddress(
    seeds: [
      solana.base58decode(programId),
      solana.base58decode(poolAccountId),
      utf8.encode('pool_token_account'),
    ],
    programId: programId,
  );
}

Future<String> getClaimerAccountId({
  required String programId,
  required String poolAccountId,
  required String claimerWalletId,
}) {
  return solana_utils.findProgramAddress(
    seeds: [
      solana.base58decode(programId),
      solana.base58decode(poolAccountId),
      solana.base58decode(claimerWalletId),
      utf8.encode('claimer_account'),
    ],
    programId: programId,
  );
}

/*
pub(crate) fn get_associated_token_address_and_bump_seed(
    wallet_address: &Pubkey,
    spl_token_mint_address: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            &wallet_address.to_bytes(),
            &spl_token::id().to_bytes(),
            &spl_token_mint_address.to_bytes(),
        ],
        program_id,
    )
}
 */

Future<String> getClaimerTokenAccountId({
  required Config config,
  required String tokenMintId,
  required String claimerWalletId,
}) async {
  return solana_utils.findProgramAddress(
    seeds: [
      solana.base58decode(claimerWalletId),
      solana.base58decode(config.tokenProgramId),
      solana.base58decode(tokenMintId),
    ],
    programId: config.associatedTokenProgramId,
  );
}

int unpackUInt(List<int> data, {Endian endian = Endian.big}) {
  var slice = List.of(data);
  if (endian == Endian.big) {
    slice = slice.reversed.toList();
  }
  var result = 0;
  var pow = 0;
  for (final e in slice) {
    result += e * math.pow(2, pow).toInt();
    pow += 8;
  }
  return result;
}

List<int> packUInt32(int data, {Endian endian = Endian.big}) {
  return Uint8List(4)..buffer.asByteData().setUint32(0, data, endian);
}

List<int> packUInt64(int data, {Endian endian = Endian.big}) {
  return Uint8List(8)..buffer.asByteData().setUint64(0, data, endian);
}

