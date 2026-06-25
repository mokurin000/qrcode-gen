-version = Versio
-error = Virhe
-ec-level = EC-taso
-qr-code = QR-koodi

version-normal = { -version }: { $v }, { -ec-level }: { $ec_level }
version-micro-simple = { -version }: M{ $v }
version-micro = { -version }: M{ $v }, { -ec-level }: { $ec_level }
error-ec-level-not-supported = { -error }: { -ec-level } { $ec_level } ei tueta M{ $v }:ssä
error-data-too-long = { -error }: data liian pitkä
error-unsupported-charset = { -error }: tuematon merkistö
error-unknown = { -error }: tuntematon virhe
