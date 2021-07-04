class Config {
  const Config({
    required this.programId,
    required this.rentSysvarId,
    required this.systemProgramId,
    required this.tokenProgramId,
    required this.associatedTokenProgramId,
  });

  final String programId;
  final String rentSysvarId;
  final String systemProgramId;
  final String tokenProgramId;
  final String associatedTokenProgramId;

  static const defaultValue = Config(
    programId: '3K1Td3DmxWt2rxT1H4furqWJyZu3nuc7QQs6W5rtHY3P',
    rentSysvarId: 'SysvarRent111111111111111111111111111111111',
    systemProgramId: '11111111111111111111111111111111',
    tokenProgramId: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
    associatedTokenProgramId: 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL',
  );
}
