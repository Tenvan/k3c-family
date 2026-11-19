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
# Rust-Wiedergabe (RU39.1, -Engine replay): dieselbe Rust-EXE wie -Engine rustgame (dist\k3c-game im Repo, ..\k3c-game im Paket) spielt die
# Aufzeichnung -Trace (Szene `nacht` als `messung-nacht`) im Fenster; --measure in Takten (Sekunden x 30), 60 Takte
# Aufwärmen; ohne -Vsync mit --novsync (wie Go), mit -Vsync ohne den Schalter. Nur auf ausdrücklichen Wunsch, ohne -Engine läuft sie nicht mit.
# Rust-Spiel (RU36.1, -Engine rustgame): dieselben Szenen `tag` und `nacht` mit denselben Schaltern wie -Engine game, EXE aus
# dist\k3c-game (Repo: task build bzw. task rust:game:build; Paket: ..\k3c-game neben dem Go-Paket), Arbeitsordner ist der
# Ordner der EXE (data, public\grafik). Das Go-Spiel liegt seit RU35 in dist\k3c-game-go (task native:game:build).
# Neu für game und rustgame: Feld `start_ms` (Prozessstart bis sichtbares Fenster, MainWindowHandle ungleich 0, Abfrage alle
# 50 ms; bei `nacht` zählt der Vorlauf --tick mit, für die Gate-Startzeit gilt die Szene `tag`), `exe_mib` (Größe der EXE),
# -Runs (Läufe je Szene, Feld `lauf`).
# -Vsync: ohne --novsync messen (G2: Ruckler bei 60 Hz); -Frames: je Lauf eine Zeilen-Datei je Bild nach reports\ und
# deren Auswertung (Get-FrameStats, gleiche Formel für Go und Rust) im Feld `bilder` des Laufs. -StatsOnly <datei>: nur
# eine Zeilen-Datei auswerten und ausgeben.
param(
    [string]$Exe = '',
    [string]$Root = '',
    [int]$Seconds = 20,
    [int]$Warmup = 5,
    [string[]]$Modes = @('level', 'alle'),
    [ValidateSet('', 'ebiten', 'rig', 'game', 'rustgame', 'replay')]
    [string]$Engine = '',
    [string[]]$EbitenArgs = @(),
    [int[]]$Figuren = @(2, 50, 200),
    [string[]]$Arten = @('rig', 'sheet'),
    [string[]]$Szenen = @('tag', 'nacht'),
    [string]$Size = '1920x1080',
    [int]$Runs = 1,
    [string]$Trace = '',
    [switch]$Vsync,
    [switch]$Frames,
    [string]$StatsOnly = ''
)
$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = [Text.Encoding]::UTF8
$here = $PSScriptRoot

# Kennzahlen einer Zeilen-Datei je Bild (Format von measure_frames.go, Rust: replay/frames.rs), für Go und Rust gleich:
# Median, p99, p99,9 und längstes Bild von dt_ms (Perzentil nach nächstem Rang wie `percentile` in measure.go:
# sortiert[round((n-1)*p)]), lang = Bilder über dem 1,5-Fachen des Medians, lang_programm = davon mit draw_ms über dem
# halben Median oder GC im Bild, max_zu_median, Median und p99 von draw_ms (Rechenzeit je Bild).
function Get-Rank($sorted, [double]$p) {
    if ($sorted.Count -eq 0) { return 0.0 }
    $sorted[[int][math]::Round(($sorted.Count - 1) * $p, [MidpointRounding]::AwayFromZero)]
}
function Get-FrameStats([string]$path) {
    $rows = @(Get-Content $path -Encoding UTF8 | Where-Object { $_ } | ForEach-Object { $_ | ConvertFrom-Json })
    $dt = [double[]]@($rows | ForEach-Object { [double]$_.dt_ms }); [Array]::Sort($dt)
    $dr = [double[]]@($rows | ForEach-Object { [double]$_.draw_ms }); [Array]::Sort($dr)
    $median = Get-Rank $dt 0.5
    $max = if ($dt.Count) { $dt[$dt.Count - 1] } else { 0.0 }
    $long = @($rows | Where-Object { [double]$_.dt_ms -gt $median * 1.5 })
    $prog = @($long | Where-Object { [double]$_.draw_ms -gt $median * 0.5 -or [int]$_.gc -gt 0 })
    [ordered]@{
        datei          = $path
        bilder         = $rows.Count
        median_ms      = [math]::Round($median, 4)
        p99_ms         = [math]::Round((Get-Rank $dt 0.99), 4)
        p999_ms        = [math]::Round((Get-Rank $dt 0.999), 4)
        max_ms         = [math]::Round($max, 4)
        max_zu_median  = if ($median -gt 0) { [math]::Round($max / $median, 3) } else { 0 }
        lang           = $long.Count
        lang_programm  = $prog.Count
        draw_median_ms = [math]::Round((Get-Rank $dr 0.5), 4)
        draw_p99_ms    = [math]::Round((Get-Rank $dr 0.99), 4)
    }
}
if ($StatsOnly) { Get-FrameStats $StatsOnly | ConvertTo-Json; return }

# Paket (EXE und public/grafik neben dem Skript) oder Repo (EXE im Bau-Ordner des Prototyps)
function Find-Exe($name, $repoPath) {
    $p = Join-Path $here "$name.exe"
    if (-not (Test-Path $p)) { $p = Join-Path $here $repoPath }
    $p
}
# Rust-Spiel (rustgame und replay): im Paket neben dem Go-Paket (..\k3c-game), im Repo aus task rust:game:build (dist\k3c-game)
$rustExe = if (Test-Path (Join-Path $here '..\k3c-game\k3c-game.exe')) { Join-Path $here '..\k3c-game\k3c-game.exe' } else { Join-Path $here '..\dist\k3c-game\k3c-game.exe' }
$exes = [ordered]@{
    ebiten = Find-Exe 'k3c-ebiten' 'k3c-ebiten\bin\k3c-ebiten.exe'
    rig    = Find-Exe 'k3c-rig' 'k3c-rig\bin\k3c-rig.exe'
    game   = Find-Exe 'k3c-game' 'k3c-game\bin\k3c-game.exe'
    rustgame = $rustExe
    replay = $rustExe
}
# Go-Spiel im Repo: Bau-Ordner, sonst das Paket nach task native:game:build (dist\k3c-game-go)
if (-not (Test-Path $exes.game)) {
    $goPaket = Join-Path $here '..\dist\k3c-game-go\k3c-game.exe'
    if (Test-Path $goPaket) { $exes.game = $goPaket }
}
$buildTask = @{ ebiten = 'native:ebiten:build'; rig = 'native:rig:build'; game = 'native:game:build'; rustgame = 'rust:game:build'; replay = 'rust:game:build' }
if ($Exe) {
    # Eigene EXE: gilt für die genannte Engine, sonst entscheidet der Dateiname
    if (-not $Engine) {
        $leaf = Split-Path $Exe -Leaf
        $Engine = if ($leaf -like '*rig*') { 'rig' } elseif ($leaf -like '*game*') { 'game' } else { 'ebiten' }
    }
    $exes[$Engine] = $Exe
}
# Go-Spiel ohne EXE (nach RU37): gespeicherte Referenz ausgeben statt zu scheitern (RU58.3, docs/messung/go-referenz.json)
if ($Engine -eq 'game' -and -not (Test-Path $exes.game)) {
    $ref = Join-Path $here '..\docs\messung\go-referenz.json'
    if (-not (Test-Path $ref)) { $ref = Join-Path $here 'go-referenz.json' }
    if (Test-Path $ref) {
        Write-Host "⏳ Referenz, nicht live: keine Go-EXE gefunden, gespeicherte Werte aus $ref (Tag go-referenz-final)"
        Get-Content $ref -Raw -Encoding UTF8
        return
    }
}
$engines = @(switch ($Engine) {
        'ebiten' { 'ebiten' }
        'rig' { 'rig' }
        'game' { 'game' }
        'rustgame' { 'rustgame' }
        'replay' { 'replay' }
        default { $exes.Keys | Where-Object { $_ -notin @('replay', 'rustgame') -and (Test-Path $exes[$_]) } }
    })
if ($engines.Count -eq 0) { throw "Keine EXE gefunden (erst task native:ebiten:build, native:rig:build oder native:game:build)" }
foreach ($e in $engines) {
    if (-not (Test-Path $exes[$e])) { throw "EXE fehlt: $($exes[$e]) (erst task $($buildTask[$e]))" }
}
if (-not $Root) {
    $Root = $here
    if (-not (Test-Path (Join-Path $Root 'public\grafik'))) { $Root = Split-Path $here -Parent }
}
if (-not (Test-Path (Join-Path $Root 'public\grafik'))) { throw "public\grafik fehlt unter $Root (erst task gen:paper)" }
if (-not $Trace) {
    $Trace = Join-Path $here '..\k3c-game\trace\messung-nacht.jsonl.gz'
    if (-not (Test-Path $Trace)) { $Trace = Join-Path $Root '.work\trace\messung-nacht.jsonl.gz' }
}
if ($engines -contains 'replay' -and -not (Test-Path $Trace)) {
    throw "Aufzeichnung fehlt: $Trace (go test ./engine/internal/trace/ -run TestMessungNacht -write)"
}
if ($engines -contains 'replay' -and -not $Vsync) { Write-Host "⏳ Hinweis: Wiedergabe ohne -Vsync misst mit --novsync" }

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

# Startzeit: Zeit vom Prozessstart (Stopwatch vor Start-Process) bis das Fenster sichtbar ist (MainWindowHandle ungleich 0),
# Abfrage alle 50 ms; -1, wenn der Prozess vorher endet oder das Fenster nach $limit Sekunden fehlt
function Wait-Window($proc, $watch, $limit) {
    while (-not $proc.HasExited -and $watch.Elapsed.TotalSeconds -lt $limit) {
        $proc.Refresh()
        if ($proc.MainWindowHandle -ne [IntPtr]::Zero) { return [math]::Round($watch.Elapsed.TotalMilliseconds) }
        Start-Sleep -Milliseconds 50
    }
    -1
}

# Szenen des nativen Spiels: fester Seed, 2 Spieler, Vorlauf über --tick (ohne Fenster), feste Eingabe-Folge.
# tag: Spieler 1 läuft rechts, Spieler 2 links. nacht: erste Nacht, beide stehen an der Burg, die Welle kommt dort an.
$gameScenes = @{
    tag   = @('--seed', 'k3c', '--players', '2', '--tick', '300', '--input', '0@0=R,1@0=L')
    nacht = @('--seed', 'k3c', '--players', '2', '--tick', '15600', '--input', '0@0=-,1@0=-')
}

# Läufe je Engine: Lastfälle `level` und `alle` (Ebitengine), Art × Figurenzahl (Rig-Prototyp) oder Szenen (Spiel)
function Get-Jobs($eng) {
    if ($eng -in @('game', 'rustgame')) { return @($Szenen | ForEach-Object { @{ name = $_; args = $gameScenes[$_] } }) }
    if ($eng -eq 'replay') { return @(@{ name = ((Split-Path $Trace -Leaf) -replace '\.jsonl\.gz$', ''); args = @() }) }
    if ($eng -ne 'rig') { return @($Modes | ForEach-Object { @{ name = $_; args = @('--mode', $_, '--auto') } }) }
    $jobs = @()
    foreach ($n in $Figuren) { foreach ($art in $Arten) { $jobs += @{ name = "$art-$n"; args = @('--art', $art, '--figuren', $n) } } }
    $jobs
}

$laeufe = @()
foreach ($eng in $engines) {
    $jobList = @(Get-Jobs $eng)
    if ($eng -notin @('game', 'rustgame')) { $jobList = $jobList | ForEach-Object { @{ name = $_.name; args = $_.args; lauf = 1 } } }
    else { $jobList = foreach ($j in $jobList) { foreach ($n in 1..$Runs) { @{ name = $j.name; args = $j.args; lauf = $n } } } }
    foreach ($job in $jobList) {
        $mode = $job.name
        $suffix = if ($Runs -gt 1) { "-l$($job.lauf)" } else { '' }
        $tmp = Join-Path $reports "native-proto-$stamp-$eng-$mode$suffix.tmp.json"
        $framesFile = Join-Path $reports "native-$stamp-$eng-$mode$suffix.frames.jsonl"
        $argList = @('--measure', $Seconds, '--warmup', $Warmup, '--report', "`"$tmp`"") + $job.args
        if (-not $Vsync) { $argList += '--novsync' }
        # Das Spiel liest seine Dateien relativ zum Arbeitsordner, die Prototypen über --root
        if ($eng -in @('game', 'rustgame')) { $argList += @('--size', $Size) } elseif ($eng -ne 'replay') { $argList = @('--root', "`"$Root`"") + $argList }
        if ($eng -eq 'replay') {
            # Wiedergabe: Takte statt Sekunden (30 Hz), 60 Takte Aufwärmen fest, ohne -Vsync mit --novsync (wie Go)
            $argList = @('replay', "`"$Trace`"", '--window', '--measure', ($Seconds * 30), '--out', "`"$tmp`"", '--size', $Size)
            if (-not $Vsync) { $argList += '--novsync' }
        }
        if ($Frames -and $eng -in @('game', 'replay')) { $argList += @('--measure-frames', "`"$framesFile`"") }
        # Ebitengine wählt seine Grafik-Bibliothek selbst (Windows: DirectX 11)
        if ($eng -eq 'ebiten') { $argList += $EbitenArgs }
        $warm = if ($eng -eq 'replay') { 2 } else { $Warmup }
        Write-Host "🚀 Messe $eng $mode ($Seconds s nach $warm s Aufwärmen)"
        # Das Rust-Spiel läuft im Ordner seiner EXE (Paket mit data und public\grafik), die anderen im Arbeitsordner -Root
        $cwd = if ($eng -eq 'rustgame') { Split-Path (Resolve-Path $exes[$eng]) -Parent } else { $Root }
        $startWatch = [Diagnostics.Stopwatch]::StartNew()
        $proc = Start-Process -FilePath $exes[$eng] -ArgumentList $argList -WorkingDirectory $cwd -PassThru
        $startMs = if ($eng -in @('game', 'rustgame')) { Wait-Window $proc $startWatch $limit } else { $null }
        $max = Watch-Process $proc $limit
        if ($max.timeout) { Write-Host "❌ $eng $mode nach $limit s nicht fertig, Prozess beendet"; Remove-Item $tmp -ErrorAction SilentlyContinue; continue }
        if (-not (Test-Path $tmp)) { Write-Host "❌ Kein Bericht für $eng $mode (Exit $($proc.ExitCode))"; continue }
        $r = Get-Content $tmp -Raw -Encoding UTF8 | ConvertFrom-Json
        Remove-Item $tmp
        # Die Wiedergabe nennt ihre Szene selbst (`szene`); sie bleibt als `szene_bericht`
        if ($r.PSObject.Properties['szene']) { $r | Add-Member szene_bericht $r.szene; $r.PSObject.Properties.Remove('szene') }
        $r = $r | Select-Object @{ n = 'engine'; e = { $eng } }, @{ n = 'szene'; e = { $mode } }, *
        $r | Add-Member gpu_dediziert_mib_max ([math]::Round($max.ded / 1MB))
        $r | Add-Member gpu_geteilt_mib_max ([math]::Round($max.shr / 1MB))
        $r | Add-Member ram_mib_max ([math]::Round($max.ws / 1MB))
        if ($eng -in @('game', 'rustgame')) {
            $r | Add-Member lauf $job.lauf
            $r | Add-Member start_ms $startMs
            $r | Add-Member exe_mib ([math]::Round((Get-Item $exes[$eng]).Length / 1MB, 1))
        }
        if ($Frames -and (Test-Path $framesFile)) {
            # Go schreibt eigene Kennwerte ins Feld `bilder`; sie bleiben als `bilder_go`, `bilder` rechnet dieses Skript
            if ($r.PSObject.Properties['bilder']) { $r | Add-Member bilder_go $r.bilder -Force }
            $r | Add-Member bilder (Get-FrameStats $framesFile) -Force
            Write-Host ("📄 {0} {1}: Median {2} ms, p99 {3} ms, längstes {4} ms, lang {5} (Programm {6})" -f $eng, $mode, $r.bilder.median_ms, $r.bilder.p99_ms, $r.bilder.max_ms, $r.bilder.lang, $r.bilder.lang_programm)
        }
        $laeufe += $r
        Write-Host ("✅ {0} {1}: {2:N0} FPS Mittel, {3:N0} FPS 1-%-Low, p99 {4:N1} ms, GPU {5} MiB, RAM {6} MiB" -f $eng, $mode, $r.fps_mittel, $r.fps_1pct_low, $r.frametime_ms.p99, $r.gpu_dediziert_mib_max, $r.ram_mib_max)
    }
}

$kind = if ($engines.Count -eq 1 -and $engines[0] -in @('rig', 'game', 'rustgame', 'replay')) { $engines[0] } else { 'proto' }
$out = Join-Path $reports "native-$kind-$($env:COMPUTERNAME)-$stamp.json"
$json = [ordered]@{ zeit = (Get-Date -Format 'o'); rechner = (Get-Machine); laeufe = $laeufe } | ConvertTo-Json -Depth 6
# UTF-8 ohne BOM (Set-Content -Encoding UTF8 schreibt in PowerShell 5.1 eine BOM, an der JSON-Leser scheitern)
[IO.File]::WriteAllText($out, $json, (New-Object Text.UTF8Encoding $false))
Write-Host "💾 Bericht: $out"
