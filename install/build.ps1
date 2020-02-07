$CurrentVersion = (git describe --tags --abbrev=0)

if ($CurrentVersion -eq "") {
    throw "Current git tag is invalid"
}

Write-Output "Current git tag version: $CurrentVersion"

$Re = "[0-9]+\.[0-9]+(\.[0-9]+)?"
$ApplicationVersion = ""
$match = $CurrentVersion -match $Re
if ($match) {
    $ApplicationVersion = $matches[0]
}

if (!$ApplicationVersion) {
    throw "Application version is invalid"
}

Write-Output "Application version: $ApplicationVersion"

go build -o ../cyak.exe ..
iscc /dApplicationVersion=$ApplicationVersion win-install.iss