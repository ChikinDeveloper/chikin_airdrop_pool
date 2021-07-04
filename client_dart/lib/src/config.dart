class Config {
  const Config({
    required this.programId,
    required this.rentSysvarId,
    required this.systemProgramId,
    required this.tokenProgramId,
  });

  final String programId;
  final String rentSysvarId;
  final String systemProgramId;
  final String tokenProgramId;

  static const defaultValue = Config(
    programId: '',
    rentSysvarId: '',
    systemProgramId: '',
    tokenProgramId: '',
  );
}
