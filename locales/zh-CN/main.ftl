-version = 版本
-error = 错误
-ec-level = 纠错等级
-qr-code = 二维码

version-normal = { -version }: { $v }, { -ec-level }: { $ec_level }
version-micro-simple = { -version }: M{ $v }
version-micro = { -version }: M{ $v }, { -ec-level }: { $ec_level }
error-ec-level-not-supported = { -error }: { -ec-level } { $ec_level } 在 M{ $v } 中不受支持
error-data-too-long = { -error }: 数据过长
error-unsupported-charset = { -error }: 不支持的字符集
error-unknown = { -error }: 未知错误
