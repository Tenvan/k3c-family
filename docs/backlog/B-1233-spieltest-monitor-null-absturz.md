# B-1233 · Workbench-Fenster wird beim Start der Spieltest-Aufnahme schwarz (Monitor bekommt null-Listen)

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ65
- **Projekt:** WZG
- **Erstellt:** 2026-11-22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Beim Start der Spieltest-Aufnahme wurde das Workbench-Fenster schwarz. `Stats.clone()` lieferte beim Start nil-Listen, die als `null` im JSON ankamen; `RecordMonitor.tsx` rief darauf `.length` auf, React baute die ganze Seite ab. Eine ErrorBoundary gab es in keiner der beiden Wails-Apps.

## Ziel

Der Start der Aufnahme zeigt die Diagramme; ein Render-Fehler auf einer Seite lässt Kopfzeile und andere Seiten bedienbar.

## Beteiligte und Zielgruppen

Ralf (🧑) und alle, die Workbench oder Studio bedienen.

## Anforderungen

- Go liefert `voiceRate` und `videoRate` nie als `null`; die Oberfläche verkraftet `null`.
- Jede Seite von Workbench und Studio steht in einer `PageBoundary` (`tools/webkit/organisms`), die den Fehler mit Seitennamen zeigt, ihn loggt und „Seite neu laden“ anbietet.

## Nicht-Ziele

Fehlerberichte an einen Dienst; Fehlerbehandlung unterhalb der Seiten.

## Regeln und Einschränkungen

Atomic Design: nur über `@webkit`, Importe nach unten.

- **Bereich:** `tools/webkit/organisms/**`, `tools/k3c-workbench/frontend/src/**`, `tools/k3c-studio/frontend/src/App.tsx`, `tools/k3c-workbench/internal/playtest/watch*.go`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Aufnahme starten: Diagramme erscheinen mit 0 KB/s. Eine Seite wirft beim Rendern: Hinweiskarte mit Fehlertext, andere Reiter funktionieren.

## Ausnahme- und Fehlerfälle

Fehler beim erneuten Laden: dieselbe Karte erscheint wieder.

## Akzeptanzkriterien

- **AC-01** `go test ./internal/playtest`: Stats-JSON ohne `null` (Test `TestCloneLiefertNieNilListen`).
- **AC-02** Vitest: `RecordMonitor` mit `null`-Listen rendert ohne Fehler.
- **AC-03** Vitest `PageBoundary`: zeigt Fehler und lädt nach Klick neu.
- **AC-04** Alle Seiten beider Apps stehen in `PageBoundary`.
- **AC-05** Abnahme durch Ralf: Aufnahme startet ohne schwarzes Fenster.

## Offene Fragen

keine

## Notizen

Ursache des schwarzen Fensters: React-Fehler beim Rendern (per Code gefunden, nicht im Fenster nachgestellt).
