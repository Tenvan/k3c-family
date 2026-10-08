# BAL1.2 · Replay-Format: Aufnahme, Wiedergabe, Versions- und Datenstand-Prüfung

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** bal1/2-replay-format
- **Abhängig von:** BAL1.1
- **Tickets:** B-159
- **Kriterien:** AC-03, AC-04, AC-05

## Ziel

Der Tester schreibt auf Wunsch je Lauf eine Replay-Datei; die Wiedergabe rechnet ohne Bot denselben Lauf nach und liefert denselben Endzustand-Hash; unbekannte Version ist ein Fehler, abweichender Datenstand eine Warnung.

## Kontext

- Anforderungen: B-159 › Anforderungen und Ausnahme- und Fehlerfälle. Format JSON mit Version im Kopf: Seed, Spielerzahl, Szenario-Parameter, Hash des Datenstands (`data/*.json`), `PlayerCommand` je Tick. Beschädigte Datei → Fehler mit Zeilennummer.
- Ort: wie in BAL1.1 entschieden (Vorschlag: `engine/balance/replay*.go`). Die Wiedergabe braucht nur `engine/sim`, keinen Bot.
- **Datenstand-Hash:** über die eingebetteten Daten (`data/embed.go`, `go:embed`), sortiert nach Dateiname, damit er auf jedem Rechner gleich ist.
- **Endzustand-Hash:** Hash einer festen JSON-Form des Zustands am Ende (z. B. `json.Marshal` der Insel bzw. ihrer Welten; Structs, keine Maps in der Ausgabe). Falls `engine/sim` schon eine Zusammenfassung bietet, die nutzen; sonst im Balance-Paket, nicht in `engine/sim`.
- Dateigröße: Eingaben je Tick als Lauflängen (Wechsel statt jedes Ticks) halten die Datei klein; das ist erlaubt, solange die Wiedergabe exakt dieselben Befehle je Tick ergibt (Test).
- Spielstand-Versionen aus F2 (B-137) bleiben unberührt; das Replay ist ein eigenes Format mit eigener Version.

## Erlaubte Dateien

- `engine/balance/` (Replay mit Tests), `cmd/k3c-balance/` (Flag zum Schreiben und Abspielen)
- `testdata/replay/` (neu: kleine Beispiel-Datei für Tests)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Aufnahme echter Sitzungen aus dem Browser, Protokoll-Schnittstelle, Wiedergabe in k3c-dev (BAL1.3), Änderungen an `engine/sim`.

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Format (Typ mit Version), Schreiben aus dem Tester (Flag), Lesen mit Fehler bei unbekannter Version und Zeilennummer bei kaputtem JSON.
3. Wiedergabe: Insel aus Seed und Parametern, Befehle je Tick, Endzustand-Hash und Burgfall-Tick.
4. Tests: (a) Aufnahme eines Bot-Laufs und Wiedergabe → gleicher Endzustand-Hash; (b) Datei enthält Seed, Parameter, Datenstand-Hash, Eingaben; (c) unbekannte Version → Fehler mit Versionsangabe; anderer Datenstand-Hash → Warnung, Wiedergabe läuft.
5. `task check:go`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-03: Test belegt gleichen Endzustand-Hash bei Aufnahme und Wiedergabe.
- [x] AC-04: Replay-Datei enthält Seed, Parameter, Datenstand-Hash und Eingaben (Test).
- [x] AC-05: Tests für Versions-Fehler und Datenstand-Warnung.
- [x] `task check:go` grün.

## Prüfen

```bash
task check:go
```

## Ergebnis

- **Ort:** wie in BAL1.1 entschieden (Domänen-Ausnahme der Sprint-Spec) im Paket `tools/k3c-dev/internal/balance/` statt `engine/balance/` und `cmd/k3c-balance/`: `replay.go` (Format, Lesen, Wiedergabe, Hashes), `replay_test.go`, Aufnahme in `run.go` (`runOne`, gemeinsame Inselerzeugung `newIsland`), Flags in `cmd/main.go`. Keine Beispiel-Datei unter `testdata/replay/` nötig: die Tests nehmen einen Bot-Lauf auf und lesen ihn zurück.
- **Format (Version 1):** JSON mit `version`, Szenario (`seed`, `players`, `bot`, `depth`, `days`), `dataHash`, `ticks`, `endHash`, `castleFallTick` und `inputs` (je Spieler `PlayerCommand` als Lauflängen `{n, moveX, sprint, pay}`). Datenstand-Hash: SHA-256 über `data.Files` nach Pfad sortiert, mit LF-Zeilenenden. Endzustand-Hash: SHA-256 von `json.Marshal` der Insel (`sim.Island`).
- **Befehl:** `task balance:run -- … --replay-dir DIR` schreibt je gültigem Lauf `<seed>-p<spieler>-<bot>-d<tiefe>.replay.json`; `--play DATEI` spielt ohne Bot ab und gibt `ticks`, `endHash`, `castleFallTick`, `warnings` als JSON aus (Warnungen zusätzlich auf stderr). Probe: Seed 12, 2 Spieler, sparsam, 1 Tag → 18 KB, Wiedergabe liefert denselben `endHash`.
- **AC-03** umgesetzt, geprüft mit `TestAufnahmeUndWiedergabeGleicherHash` (Bot-Lauf → Datei → Lesen → Wiedergabe: gleicher Endzustand-Hash, gleiche Ticks, gleicher Burgfall-Tick) und `TestLauflaengenExakt` (Lauflängen ergeben exakt dieselben Befehle je Tick).
- **AC-04** umgesetzt, geprüft mit `TestDateiEnthaeltSeedParameterHashEingaben` (Version, Seed, Spieler, Bot, Tiefe, Tage, Datenstand-Hash, Eingaben je Spieler).
- **AC-05** umgesetzt, geprüft mit `TestUnbekannteVersionUndKaputteDatei` (Version 99 → Fehler „unbekannte Version 99“; Typ- und Syntaxfehler → Fehler mit Zeilennummer) und `TestAndererDatenstandWarnt` (Warnung, Wiedergabe läuft bis zum gleichen Endzustand).
- `task check:go`, `task check:dev`, `task check` grün (2026-10-04, Worktree).
