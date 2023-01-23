if (-not (Test-Path -Path env:\GITHUB_TOKEN)) {
	throw 'GITHUB_TOKEN environment variable is not defined'
}
$github_token = $env:GITHUB_TOKEN

if (-not (Test-Path -Path env:\WINDOWS_ASSETS)) {
	throw 'WINDOWS_ASSETS environment variable is not defined'
}
$windows_assets = $env:WINDOWS_ASSETS

if (-not (Test-Path -Path env:\TIGER_VERSION)) {
	throw 'TIGER_VERSION environment variable is not defined'
}
$tiger_version = $env:TIGER_VERSION


$msi_signature_asset_id = $windows_assets
| ConvertFrom-Json
| Where-Object { $_.name -match '\.msi\.zip\.sig$' }
| Select-Object -ExpandProperty id
$msi_signature_draft_url = 'https://api.github.com/repos/agersant/tiger/releases/assets/' + $msi_signature_asset_id
$msi_signature = (Invoke-webrequest -Headers @{'Accept' = 'application/octet-stream'; 'Authorization' = $github_token } -URI $msi_signature_draft_url).Content
if ($msi_signature.GetType().IsArray) {
	$msi_signature = [System.Text.Encoding]::ASCII.GetString($msi_signature);
}

$msi_updater_name = $windows_assets
| ConvertFrom-Json `
| Where-Object { $_.name -match '\.msi\.zip$' }
| Select-Object -ExpandProperty name
# Cannot use the browser_download_url property for this asset as it changes when publishing the draft
$msi_updater_url = 'https://github.com/agersant/tiger/releases/download/' + $tiger_version + '/' + $msi_updater_name

$manifest = Get-Content -Path 'src-tauri/distribution/update-manifest-template.json'
$manifest = $manifest -replace '{TIGER_VERSION}', $tiger_version
$manifest = $manifest -replace '{MSI_SIGNATURE}', $msi_signature
$manifest = $manifest -replace '{MSI_URL}', $msi_updater_url
$manifest | Set-Content -Path 'update-manifest.json'
