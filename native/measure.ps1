# K3C-Prototypen messen (B-776/AC-04): startet den Ebitengine-Prototyp (k3c-ebiten) für die Lastfälle `level` und
# `alle`, liest dabei den GPU-Speicher des Prozesses (WMI-Klasse der GPU-Zähler, sprachunabhängig) und den
# Arbeitsspeicher und schreibt einen Bericht nach reports/native-proto-<rechner>-<zeit>.json (je Lauf ein Feld `engine`).
# Läuft mit Windows PowerShell 5.1 und pwsh 7. Der Rust-Vergleichs-Prototyp ist entfallen (Entscheidung 008, B-779).
# Aufruf im Repo: `task native:measure:ebiten`; im Paket auf dem Mini-PC: `messen.cmd` (gleicher Ordner wie die EXE).
# Ohne -Engine werden alle vorhandenen EXE gemessen.
# Rig-Prototyp (B-778/AC-05, `task native:rig:measure`, -Engine rig): k3c-rig mit 2, 50 und 200 Reitern, je als Rig und
# aus dem Sheet; Bericht nach reports/native-rig-<rechner>-<zeit>.json (Felder `art` und `figuren` je Lauf).
# Natives Spiel (B-802/AC-03, `task native:game:measure`, -Engine game): k3c-game im Messmodus, Fenster 1920x1080, VSync
# aus, 2 Spieler, je einmal die Szenen `tag` und `nacht` (Gegner an der Burg); Arbeitsordner ist -Root (public\grafik,
# native\k3c-rig\assets); Bericht nach reports/native-game-<rechner>-<zeit>.json (Feld `szene` je Lauf).
param(
    [string]$Exe = '',
    [string]$Root = '',
    [int]$Seconds = 20,
    [int]$Warmup = 5,
    [string[]]$Modes = @('level', 'alle'),
    [ValidateSet('', 'ebiten', 'rig', 'game')]
    [string]$Engine = '',
    [string[]]$EbitenArgs = @(),
    [int[]]$Figuren = @(2, 50, 200),
    [string[]]$Arten = @('rig', 'sheet'),
    [string[]]$Szenen = @('tag', 'nacht'),
    [string]$Size = '1920x1080'
)
$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = [Text.Encoding]::UTF8
$here = $PSScriptRoot

# Paket (EXE und public/grafik neben dem Skript) oder Repo (EXE im Bau-Ordner des Prototyps)
function Find-Exe($name, $repoPath) {
    $p = Join-Path $here "$name.exe"
    if (-not (Test-Path $p)) { $p = Join-Path $here $repoPath }
    $p
}
$exes = [ordered]@{
    ebiten = Find-Exe 'k3c-ebiten' 'k3c-ebiten\bin\k3c-ebiten.exe'
    rig    = Find-Exe 'k3c-rig' 'k3c-rig\bin\k3c-rig.exe'
    game   = Find-Exe 'k3c-game' 'k3c-game\bin\k3c-game.exe'
}
$buildTask = @{ ebiten = 'ebiten'; rig = 'rig'; game = 'game' }
if ($Exe) {
    # Eigene EXE: gilt für die genannte Engine, sonst entscheidet der Dateiname
    if (-not $Engine) {
        $leaf = Split-Path $Exe -Leaf
        $Engine = if ($leaf -like '*rig*') { 'rig' } elseif ($leaf -like '*game*') { 'game' } else { 'ebiten' }
    }
    $exes[$Engine] = $Exe
}
$engines = @(switch ($Engine) {
        'ebiten' { 'ebiten' }
        'rig' { 'rig' }
        'game' { 'game' }
        default { $exes.Keys | Where-Object { Test-Path $exes[$_] } }
    })
if ($engines.Count -eq 0) { throw "Keine EXE gefunden (erst task native:ebiten:build, native:rig:build oder native:game:build)" }
foreach ($e in $engines) {
    if (-not (Test-Path $exes[$e])) { throw "EXE fehlt: $($exes[$e]) (erst task native:$($buildTask[$e]):build)" }
}
if (-not $Root) {
    $Root = $here
    if (-not (Test-Path (Join-Path $Root 'public\grafik'))) { $Root = Split-Path $here -Parent }
}
if (-not (Test-Path (Join-Path $Root 'public\grafik'))) { throw "public\grafik fehlt unter $Root (erst task gen:paper)" }

$reports = Join-Path $Root 'reports'
New-Item -ItemType Directory -Force $reports | Out-Null
$stamp = Get-Date -Format 'yyyyMMdd-HHmmss'
# Zeitlimit je Lauf: Aufwärmen + Messen + 120 s für Start und Laden; danach wird der Prozess beendet
$limit = $Warmup + $Seconds + 120

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

# Höchstwerte von GPU-Speicher (dediziert, geteilt) und Arbeitsspeicher, solange der Prozess läuft; nach $limit Sekunden
# wird er beendet (timeout = $true)
function Watch-Process($proc, $limit) {
    $max = @{ ded = 0; shr = 0; ws = 0; timeout = $false }
    $watch = [Diagnostics.Stopwatch]::StartNew()
    while (-not $proc.HasExited) {
        if ($watch.Elapsed.TotalSeconds -gt $limit) {
            Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue
            $proc.WaitForExit(5000) | Out-Null
            $max.timeout = $true
            break
        }
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

# Szenen des nativen Spiels: fester Seed, 2 Spieler, Vorlauf über --tick (ohne Fenster), feste Eingabe-Folge.
# tag: Spieler 1 läuft rechts, Spieler 2 links. nacht: erste Nacht, beide stehen an der Burg, die Welle kommt dort an.
$gameScenes = @{
    tag   = @('--seed', 'k3c', '--players', '2', '--tick', '300', '--input', '0@0=R,1@0=L')
    nacht = @('--seed', 'k3c', '--players', '2', '--tick', '15600', '--input', '0@0=-,1@0=-')
}

# Läufe je Engine: Lastfälle `level` und `alle` (Ebitengine), Art × Figurenzahl (Rig-Prototyp) oder Szenen (Spiel)
function Get-Jobs($eng) {
    if ($eng -eq 'game') { return @($Szenen | ForEach-Object { @{ name = $_; args = $gameScenes[$_] } }) }
    if ($eng -ne 'rig') { return @($Modes | ForEach-Object { @{ name = $_; args = @('--mode', $_, '--auto') } }) }
    $jobs = @()
    foreach ($n in $Figuren) { foreach ($art in $Arten) { $jobs += @{ name = "$art-$n"; args = @('--art', $art, '--figuren', $n) } } }
    $jobs
}

$runs = @()
foreach ($eng in $engines) {
    foreach ($job in (Get-Jobs $eng)) {
        $mode = $job.name
        $tmp = Join-Path $reports "native-proto-$stamp-$eng-$mode.tmp.json"
        $argList = @('--measure', $Seconds, '--warmup', $Warmup, '--report', "`"$tmp`"", '--novsync') + $job.args
        # Das Spiel liest seine Dateien relativ zum Arbeitsordner, die Prototypen über --root
        if ($eng -eq 'game') { $argList += @('--size', $Size) } else { $argList = @('--root', "`"$Root`"") + $argList }
        # Ebitengine wählt seine Grafik-Bibliothek selbst (Windows: DirectX 11)
        if ($eng -eq 'ebiten') { $argList += $EbitenArgs }
        Write-Host "🚀 Messe $eng $mode ($Seconds s nach $Warmup s Aufwärmen)"
        $proc = Start-Process -FilePath $exes[$eng] -ArgumentList $argList -WorkingDirectory $Root -PassThru
        $max = Watch-Process $proc $limit
        if ($max.timeout) { Write-Host "❌ $eng $mode nach $limit s nicht fertig, Prozess beendet"; Remove-Item $tmp -ErrorAction SilentlyContinue; continue }
        if (-not (Test-Path $tmp)) { Write-Host "❌ Kein Bericht für $eng $mode (Exit $($proc.ExitCode))"; continue }
        $r = Get-Content $tmp -Raw -Encoding UTF8 | ConvertFrom-Json
        Remove-Item $tmp
        $r = $r | Select-Object @{ n = 'engine'; e = { $eng } }, @{ n = 'szene'; e = { $mode } }, *
        $r | Add-Member gpu_dediziert_mib_max ([math]::Round($max.ded / 1MB))
        $r | Add-Member gpu_geteilt_mib_max ([math]::Round($max.shr / 1MB))
        $r | Add-Member ram_mib_max ([math]::Round($max.ws / 1MB))
        $runs += $r
        Write-Host ("✅ {0} {1}: {2:N0} FPS Mittel, {3:N0} FPS 1-%-Low, p99 {4:N1} ms, GPU {5} MiB, RAM {6} MiB" -f $eng, $mode, $r.fps_mittel, $r.fps_1pct_low, $r.frametime_ms.p99, $r.gpu_dediziert_mib_max, $r.ram_mib_max)
    }
}

$kind = if ($engines.Count -eq 1 -and $engines[0] -in @('rig', 'game')) { $engines[0] } else { 'proto' }
$out = Join-Path $reports "native-$kind-$($env:COMPUTERNAME)-$stamp.json"
$json = [ordered]@{ zeit = (Get-Date -Format 'o'); rechner = (Get-Machine); laeufe = $runs } | ConvertTo-Json -Depth 6
# UTF-8 ohne BOM (Set-Content -Encoding UTF8 schreibt in PowerShell 5.1 eine BOM, an der JSON-Leser scheitern)
[IO.File]::WriteAllText($out, $json, (New-Object Text.UTF8Encoding $false))
Write-Host "💾 Bericht: $out"
