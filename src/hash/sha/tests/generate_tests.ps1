$TestInputs = Get-ChildItem -Path .\ -Filter *.in

function CreateFileHash {
    param (
        $InputPath,
        $AlgorithmLength
    )

    switch ($AlgorithmLength) {
        256 { $Algorithm = "SHA256" }
        384 { $Algorithm = "SHA384" }
        512 { $Algorithm = "SHA512" }
        default { throw "Can only hash using 256, 384, or 512" }
    }

    $Hash = Get-FileHash -Path $InputPath -Algorithm $Algorithm

    Write-Host " -" $Algorithm "Hash:" $Hash.Hash

    $OutputPath = $InputPath -replace "in$", $AlgorithmLength

    Out-File -FilePath $OutputPath -InputObject $Hash.Hash -NoNewline
}

$first = $true
foreach ($TestInput in $TestInputs) {
    if ($first) { 
        $first = $false
    } else {
        Write-Host
    }
    
    Write-Host "Generating Test: " $TestInput

    CreateFileHash -InputPath $TestInput -AlgorithmLength 256
    CreateFileHash -InputPath $TestInput -AlgorithmLength 384
    CreateFileHash -InputPath $TestInput -AlgorithmLength 512
}