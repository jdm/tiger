if (-not (Test-Path -Path env:\RELEASE_ASSETS)) {
	throw 'RELEASE_ASSETS environment variable is not defined'
}
$release_assets = $env:RELEASE_ASSETS

if (-not (Test-Path -Path env:\TIGER_VERSION)) {
	throw 'TIGER_VERSION environment variable is not defined'
}
$tiger_version = $env:TIGER_VERSION

$signature_url = $release_assets
| ConvertFrom-Json
| Where-Object { $_.browser_download_url -match '\.msi\.zip\.sig$' }
| Select-Object -ExpandProperty browser_download_url
$signature = (Invoke-webrequest -URI $signature_url).Content

$updater_url = $release_assets
| ConvertFrom-Json `
| Where-Object { $_.browser_download_url -match '\.msi\.zip$' }
| Select-Object -ExpandProperty browser_download_url

$manifest = Get-Content -Path 'src-tauri/distribution/update-manifest-template.json'
$manifest = $manifest -replace '{TIGER_VERSION}', $tiger_version
$manifest = $manifest -replace '{MSI_SIGNATURE}', $signature
$manifest = $manifest -replace '{MSI_URL}', $updater_url
$manifest | Set-Content -Path 'update-manifest.json'
