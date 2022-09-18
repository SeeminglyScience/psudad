cargo build --release

if ($LASTEXITCODE -ne 0) {
    throw [System.ComponentModel.Win32Exception]::new($LASTEXITCODE)
}

$psudad = "$PSScriptRoot/target/release/psudad.exe"
$version = & $psudad --version
$null, $version = $version -split ' '

New-Item -ItemType Directory -ErrorAction Ignore "$PSScriptRoot/out" | Out-Null

$archiveName = "psudad-$version-win-x64.zip"
Compress-Archive -LiteralPath $psudad -DestinationPath "$PSScriptRoot/out/$archiveName" -Force -ErrorAction Stop
