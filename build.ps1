$ErrorActionPreference = "Stop"
cargo build --release

#$name = (Select-String -Path ".\Cargo.toml" -Pattern '^name\s*=\s*"(.*)"').Matches[0].Groups[1].Value
$package = cargo metadata --no-deps --format-version 1 | ConvertFrom-Json
$name = $package.packages[0].name

upx -9 ".\target\release\$name.exe"
