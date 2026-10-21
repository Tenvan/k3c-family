# B-514 · Chat-Entwurf: Planung bei gleicher Insel-id schützen, CLI-Prozessbaum, Test-Riegel

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ21
- **Projekt:** WZG
- **Erstellt:** 2026-10-20
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-21, Ralf im Chat (mit WZ21 Revision 1)

## Ausgangslage

Befunde des Reviews WZ19.4 (nicht schwer, daher Ticket):
1. `tools/k3c-studio/backend/app_world_answer.go:83-107` setzt bei Chat-Entwürfen `From` leer. Bringt die Antwort eine Insel mit schon vorhandener id ohne `plannedChunks`/`chunkCounts` mit, entsteht sie beim Speichern aus der Vorlage neu und die Planung der Datei geht verloren (Kommentar in `app_world_draft.go:33-34` gilt nur für Inseln mit `From`). Gemildert: Speichern ist eine Aktion von 🧑 mit Vorschau, „ersetzt alle Inseln“ ist in WZ19.3 dokumentiert.
2. `chat_cli.go:80,87`: Abbruch beendet nur den direkten Prozess, nicht den Prozessbaum (heute unkritisch: native EXE, keine Werkzeuge).
3. `chat_http.go:37`: kein struktureller Riegel, dass künftige Tests nie den echten `claude` starten (`TestMain` könnte `defaultCLI` sperren).
4. Frage: `tools/k3c-studio/server.go:32,96` liefert `public/`/`dist/` vom selben Origin wie `/api/studio/`; falls der Import SVG mit Skript zulässt, könnte es die Kopfzeile `X-K3C-Studio` setzen. Älter als WZ19, zu klären.

## Ziel

Die offenen Punkte 2 bis 4 und die beiden Nachträge aus den Notizen sind behoben oder begründet verworfen (Punkt 1 ist mit PR #400 erledigt). Ein Abbruch lässt keinen CLI-Prozess zurück, Tests können nie das echte Modell starten, der Mock verhält sich wie Go, ein Chat-Entwurf gehört nie zu einer fremden Welt.

## Beteiligte und Zielgruppen

🧑 im Asset-Studio; Umsetzung durch Agent in DEV (`tools/k3c-studio/`).

## Anforderungen

- **Punkt 2 Prozessbaum:** Abbruch, Zeitlimit und Ende des Servers beenden die CLI samt Nachfahren. Unter Windows über ein Job-Objekt mit `KILL_ON_JOB_CLOSE` (Muster: `engine/devrun/proc_windows.go`, `tools/k3c-dev/internal/proc/proc_windows.go`), sonst über die Prozessgruppe; eigene Dateien `chat_proc_windows.go`/`chat_proc_other.go` im Backend des Studios.
- **Punkt 3 Test-Riegel:** `TestMain` (`chat_fake_test.go`) sperrt den Start des echten Programms strukturell, z. B. über eine Paket-Variable, die `defaultCLI` auf ein nicht vorhandenes Programm lenkt, oder einen Prüfer in `run`; ein Test belegt, dass `defaultCLI().run` im Test nie `claude` startet.
- **Punkt 4 SVG/Herkunft:** prüfen und begründen. Stand der Sichtung (2026-10-21): Der Import nimmt nur `.png` und Audio (`tools/assetkit/import.go` › `importExt`), `public/` enthält kein SVG. Ein Test, der den Import einer `.svg` ablehnt, hält das fest; dann verworfen mit Begründung.
- **Nachtrag Mock:** `mockAnswer` (`frontend/src/api/mockWorldDraft.ts`) setzt `from` für Inseln, deren id schon in der aktiven Welt steht, wie `draftOfAnswer` in Go.
- **Nachtrag Weltwechsel:** Ein ungespeicherter Entwurf, auch aus dem Chat, gehört zur Welt, in der er entstand. Beim Wechsel der Welt wird er verworfen oder vorher nachgefragt (`frontend/src/world/useWorldDraft.ts`); nie erscheint er an der neuen Welt.

## Nicht-Ziele

`writeAtomic` mit Wiederholung bei `Access is denied` (gehört zu B-187); neue Chat-Funktionen; Zusammenlegen der drei Job-Objekt-Umsetzungen (`engine/devrun`, `tools/k3c-dev/internal/proc`, `tools/k3c-starter`) in ein gemeinsames Paket.

## Regeln und Einschränkungen

Domäne DEV; `engine/` importiert nichts aus `tools/`, das Studio-Backend darf `engine` nutzen, aber `engine/devrun` exportiert die Job-Funktionen nicht, daher eigene kleine Umsetzung. Keine neue Abhängigkeit (`golang.org/x/sys` ist nur indirekt da, also `syscall` wie in `devrun`). Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

- Abbruch während der Antwort, die CLI hat einen Enkel gestartet → nach dem Ende von `run` lebt keiner der beiden.
- Ein Test ruft versehentlich `defaultCLI()` → Fehler „CLI nicht gefunden“, nie ein Modell-Aufruf.
- Welt „nordsee“ mit Chat-Entwurf offen, Wechsel auf „standard“ → „standard“ zeigt ihren gespeicherten Stand.

## Ausnahme- und Fehlerfälle

- Job-Objekt lässt sich nicht anlegen → Abbruch wie bisher über den direkten Prozess, Warnung im Log mit ❌.
- Weltwechsel, während eine Chat-Antwort läuft → „Als Entwurf laden“ lädt in die dann aktive Welt, aber kein alter Entwurf wandert mit.

## Akzeptanzkriterien

- **AC-01** Go-Test (Windows) mit dem Test-Ersatz im Modus, der einen Enkel startet: Nach Abbruch lebt kein Prozess des Baums mehr.
- **AC-02** Go-Test: Im Testlauf startet `defaultCLI().run` nie das echte `claude` (Riegel in `TestMain`).
- **AC-03** Punkt 4 begründet verworfen oder behoben, mit Test „Import lehnt `.svg` ab“.
- **AC-04** Vitest: `mockAnswer` setzt `from` für bestehende Insel-ids wie Go.
- **AC-05** Vitest: Ein offener Entwurf erscheint nach Weltwechsel nicht an der neuen Welt.

## Offene Fragen

keine.

## Notizen

Punkt 1 behoben (2026-10-20, PR Tenvan/k3c-family#400): `draftOfAnswer` setzt `From` für Inseln, deren id schon in der Welt-Datei steht, Test `TestAntwortBehaeltPlanungBestehenderInsel`. Offen bleiben Punkt 2 bis 4 sowie aus dem Review des Fixes: `mockWorldDraft.ts:96` setzt `from` noch immer leer (Mock weicht von Go ab); ein offener Chat-Entwurf bleibt beim Weltwechsel stehen (`useWorldDraft.ts:32`, älter).

**Einmaliger Fehlschlag von `go test ./backend` (2026-10-20 untersucht):** Ursache ist `writeAtomic` (`tools/k3c-studio/backend/plan_file.go:210`): `os.Rename` auf die bestehende Welt-Datei scheitert unter Windows mit `Access is denied`, wenn ein Fremdprozess (vermutlich Virenscanner oder Indexer) die frisch geschriebene Datei kurz offen hält. Betroffen war `TestNotizenAnlegenUndZeilenendeDerDatei`. 15 Läufe nacheinander grün; erst bei 8 parallelen Test-Binaries (je `-test.count=5`) trat er in einem von 8 auf, also last- und zeitabhängig, nicht in der Logik. Dasselbe Muster wie B-187 (`engine/store`); der Fehler kann im Betrieb auch ein Speichern der Welt im Studio treffen (Meldung statt Datenverlust, die Datei bleibt unverändert). Lösung gemeinsam mit B-187: Rename mit kurzer Wiederholung (z. B. 5 × 20 ms bei `ERROR_ACCESS_DENIED`/`ERROR_SHARING_VIOLATION`).
