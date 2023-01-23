if (-not (Test-Path -Path env:\WINDOWS_ASSETS)) {
	throw 'RELEASE_ASSETS environment variable is not defined'
}
$windows_assets = $env:WINDOWS_ASSETS

if (-not (Test-Path -Path env:\TIGER_VERSION)) {
	throw 'TIGER_VERSION environment variable is not defined'
}
$tiger_version = $env:TIGER_VERSION

$msi_signature_url = $windows_assets
| ConvertFrom-Json
| Where-Object { $_.browser_download_url -match '\.msi\.zip\.sig$' }
| Select-Object -ExpandProperty browser_download_url
$msi_signature = (Invoke-webrequest -URI $msi_signature_url).Content
if ($msi_signature.GetType().IsArray) {
	$msi_signature = [System.Text.Encoding]::ASCII.GetString($msi_signature);
}

$msi_updater_url = $windows_assets
| ConvertFrom-Json `
| Where-Object { $_.browser_download_url -match '\.msi\.zip$' }
| Select-Object -ExpandProperty browser_download_url

$manifest = Get-Content -Path 'src-tauri/distribution/update-manifest-template.json'
$manifest = $manifest -replace '{TIGER_VERSION}', $tiger_version
$manifest = $manifest -replace '{MSI_SIGNATURE}', $msi_signature
$manifest = $manifest -replace '{MSI_URL}', $msi_updater_url
$manifest | Set-Content -Path 'update-manifest.json'
