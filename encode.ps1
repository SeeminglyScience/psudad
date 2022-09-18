# Use this to test psudad
# Usage: ./encode.ps1 test | ./target/debug/psudad -
#        ./target/debug/psudad (./encode.ps1 test)
param([string] $value)
end {
    $ms = [System.IO.MemoryStream]::new()
    $zip = [System.IO.Compression.GZipStream]::new($ms, [System.IO.Compression.CompressionMode]::Compress)
    $zip.Write([System.Text.UTF8Encoding]::new().GetBytes($value))
    $zip.Close()
    return [convert]::ToBase64String($ms.ToArray())
}
