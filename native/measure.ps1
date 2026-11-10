# K3C Bevy-Prototyp messen (B-776/AC-04): startet k3c-proto für die Lastfälle `level` und `alle`, liest dabei den
# GPU-Speicher des Prozesses (WMI-Klasse der GPU-Zähler, sprachunabhängig) und den Arbeitsspeicher und schreibt einen
# gemeinsamen Bericht nach reports/native-proto-<rechner>-<zeit>.json. Läuft mit Windows PowerShell 5.1 und pwsh 7.
# Aufruf im Repo: `task native:measure`; im Paket auf dem Mini-PC: `messen.cmd` (gleicher Ordner wie die EXE).
param(
    [string]$Exe = '',
    [string]$Root = '',
    [int]$Seconds = 20,
    [int]$Warmup = 5,
    [string]$Backend = 'vulkan',
    [string[]]$Modes = @('level', 'alle')
)
$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = [Text.Encoding]::UTF8
$here = $PSScriptRoot

# Paket (EXE und public/grafik neben dem Skript) oder Repo (EXE unter native/k3c-proto/target/release)
if (-not $Exe) {
    $Exe = Join-Path $here 'k3c-proto.exe'
    if (-not (Test-Path $Exe)) { $Exe = Join-Path $here 'k3c-proto\target\release\k3c-proto.exe' }
}
if (-not $Root) {
    $Root = $here
    if (-not (Test-Path (Join-Path $Root 'public\grafik'))) { $Root = Split-Path $here -Parent }
}
if (-not (Test-Path $Exe)) { throw "EXE fehlt: $Exe (erst task native:proto:build)" }
if (-not (Test-Path (Join-Path $Root 'public\grafik'))) { throw "public\grafik fehlt unter $Root (erst task gen:paper)" }

$reports = Join-Path $Root 'reports'
New-Item -ItemType Directory -Force $reports | Out-Null
$stamp = Get-Date -Format 'yyyyMMdd-HHmmss'

function Get-Machine {
    $cs = Get-CimInstance Win32_ComputerSystem
    $gpus = @(Get-CimInstance Win32_VideoController | ForEach-Object { "$($_.Name) (Treiber $($_.DriverVersion))" })
    [ordered]@{
        name   = $env:COMPUTERNAME
        modell = "$($cs.Manufacturer) $($cs.Model)"
        cpu    = (Get-CimInstance Win32_Processor | Select-Object -First 1).Name.Trim()
        ram_gb = [math]::Round($cs.TotalPhysicalMemory / 1GB)
        gpus   = $gpus
    }
}

# Höchstwerte von GPU-Speicher (dediziert, geteilt) und Arbeitsspeicher, solange der Prozess läuft
function Watch-Process($proc) {
    $max = @{ ded = 0; shr = 0; ws = 0 }
    while (-not $proc.HasExited) {
        try {
            $rows = @(Get-CimInstance Win32_PerfFormattedData_GPUPerformanceCounters_GPUProcessMemory |
                    Where-Object { $_.Name -like "pid_$($proc.Id)_*" })
            $ded = ($rows | Measure-Object -Property DedicatedUsage -Sum).Sum
            $shr = ($rows | Measure-Object -Property SharedUsage -Sum).Sum
            if ($ded -gt $max.ded) { $max.ded = $ded }
            if ($shr -gt $max.shr) { $max.shr = $shr }
            $proc.Refresh()
            if ($proc.WorkingSet64 -gt $max.ws) { $max.ws = $proc.WorkingSet64 }
        } catch { }
        Start-Sleep -Milliseconds 500
    }
    $max
}

$runs = @()
foreach ($mode in $Modes) {
    $tmp = Join-Path $reports "native-proto-$stamp-$mode.tmp.json"
    Write-Host "🚀 Messe $mode ($Seconds s nach $Warmup s Aufwärmen, Backend $Backend)"
    $env:WGPU_BACKEND = $Backend
    $argList = @('--root', "`"$Root`"", '--mode', $mode, '--measure', $Seconds, '--warmup', $Warmup, '--report', "`"$tmp`"", '--novsync', '--auto')
    $proc = Start-Process -FilePath $Exe -ArgumentList $argList -PassThru
    $max = Watch-Process $proc
    if (-not (Test-Path $tmp)) { Write-Host "❌ Kein Bericht für $mode (Exit $($proc.ExitCode))"; continue }
    $r = Get-Content $tmp -Raw | ConvertFrom-Json
    Remove-Item $tmp
    $r | Add-Member gpu_dediziert_mib_max ([math]::Round($max.ded / 1MB))
    $r | Add-Member gpu_geteilt_mib_max ([math]::Round($max.shr / 1MB))
    $r | Add-Member ram_mib_max ([math]::Round($max.ws / 1MB))
    $runs += $r
    Write-Host ("✅ {0}: {1:N0} FPS Mittel, {2:N0} FPS 1-%-Low, p99 {3:N1} ms, GPU {4} MiB, RAM {5} MiB" -f $mode, $r.fps_mittel, $r.fps_1pct_low, $r.frametime_ms.p99, $r.gpu_dediziert_mib_max, $r.ram_mib_max)
}

$out = Join-Path $reports "native-proto-$($env:COMPUTERNAME)-$stamp.json"
$json = [ordered]@{ zeit = (Get-Date -Format 'o'); rechner = (Get-Machine); laeufe = $runs } | ConvertTo-Json -Depth 6
# UTF-8 ohne BOM (Set-Content -Encoding UTF8 schreibt in PowerShell 5.1 eine BOM, an der JSON-Leser scheitern)
[IO.File]::WriteAllText($out, $json, (New-Object Text.UTF8Encoding $false))
Write-Host "💾 Bericht: $out"
