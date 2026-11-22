# B-1230 · Workbench findet ffmpeg nicht, wenn der PATH des Prozesses veraltet ist

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ65
- **Projekt:** WZG
- **Erstellt:** 2026-11-22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, Chat 2026-11-22
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Meldung Ralf 2026-11-22: Der Spieltest startet nicht. ffmpeg 9.1 (WinGet, mit whisper und ddagrab) steht im User-PATH, aber ein Prozess, der vor der Installation gestartet wurde (Terminal, Desktop-App, davon gestartete Workbench), erbt den alten PATH. `exec.LookPath("ffmpeg")` in `internal/playtest` meldet dann „ffmpeg nicht im PATH gefunden“.

## Ziel

Die Workbench findet ffmpeg auch bei veraltetem PATH: erst `K3C_FFMPEG` (Pfad zur ffmpeg.exe), dann der PATH des Prozesses, dann der aktuelle User- und Machine-PATH aus der Registry, dann der WinGet-Ordner (`%LOCALAPPDATA%\Microsoft\WinGet\Packages\Gyan.FFmpeg*\ffmpeg-*\bin`). Die Fehlermeldung nennt, was versucht wurde.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Eine Funktion `FindFFmpeg`, genutzt von Aufnahme (`NewRecorder`) und Auswertung (`NewTranscriber`).
- Nur Windows liest die Registry und den WinGet-Ordner; andere Systeme nutzen nur Variable und PATH.
- Keine neue Abhängigkeit (`golang.org/x/sys` ist schon im Modul).
- Test mit Attrappen für Reihenfolge und Fehlermeldung.

## Nicht-Ziele

ffmpeg installieren oder herunterladen; ffprobe.

## Regeln und Einschränkungen

Schreiben nur im Repo; keine neue Abhängigkeit.

- **Bereich:** `tools/k3c-workbench/internal/playtest/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `FindFFmpeg` prüft Variable, PATH, Registry-PATH und WinGet-Ordner in dieser Reihenfolge (Test).
- **AC-02** Aufnahme und Auswertung nutzen sie; die Fehlermeldung nennt die geprüften Orte.
- **AC-03** `go vet`, `go test` für `internal/playtest` grün.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Umgesetzt 2026-11-22 (Auftrag Ralf): `FindFFmpeg` (`internal/playtest/ffmpeg_find*.go`): `K3C_FFMPEG`, PATH, aktueller PATH aus der Registry, WinGet-Ordner; Fehler nennt die geprüften Orte. Auf Wunsch Ralf zusätzlich Geräte robuster und gegen echte Hardware (`devices.go`): Monitore werden mit einem Probebild von ddagrab geprüft und mit Auflösung angezeigt (auf diesem Rechner: Monitor 1 2560×1440, Monitor 2 3840×2160, Monitor 3 nicht vorhanden), `CheckDevices` prüft Mikro und Monitor vor dem Start gegen die echten Geräte mit klarer Fehlermeldung, Seite mit "Geräte neu suchen", Warnung bei unbekanntem gemerktem Mikro und bei fehlendem Aufnahmegerät. Geprüft: `go vet`, `go test` (playtest, mcpsrv, k3c-workbench), Frontend tsc und Vitest (18 Tests in `src/playtest`). Befund: Auf diesem Rechner sieht ffmpeg (DirectShow) kein Mikrofon, Windows kennt nur Wiedergabegeräte; Mikro-Auswahl ist hier daher leer. `task check:diff` nicht gelaufen (golangci-lint fehlt).
