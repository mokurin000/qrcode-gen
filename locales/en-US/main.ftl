-version = Version
-error = Error
-ec-level = EcLevel
-qr-code = QR Code

version-normal = { -version }: { $v }, { -ec-level }: { $ec_level }
version-micro-simple = { -version }: M{ $v }
version-micro = { -version }: M{ $v }, { -ec-level }: { $ec_level }
error-ec-level-not-supported = { -error }: EC level { $ec_level } not supported in M{ $v }
error-data-too-long = { -error }: data too long
error-unsupported-charset = { -error }: unsupported character set
error-unknown = { -error }: unknown error
