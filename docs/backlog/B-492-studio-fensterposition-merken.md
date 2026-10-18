# B-492 · Das Asset-Studio merkt sich Fensterposition und Größe und stellt sie sicher wieder her

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Wunsch von 🧑 (2026-10-18): Das Asset-Studio (`tools/k3c-studio`) soll sich Fensterposition und -größe merken wie die Workbench und sie sicher wiederherstellen, auch wenn der Monitor-Index nicht mehr stimmt. Die Workbench (`tools/k3c-dev`) kann das schon: `window.go` (`windowState`, `loadWindow`, `saveWindow`, `placeWindow`, `configPath` unter `<UserConfigDir>/k3c`), `screen_windows.go` (`onScreen` prüft, ob das Fenster auf einem vorhandenen Bildschirm greifbar ist; sonst bleibt die Position Wails überlassen), `screen_other.go`, `beforeClose` in `app.go` (merkt Größe und Position beim Schließen) und verdeckter Start in `main.go`, bis die Position gesetzt ist. Das Studio hat nichts davon. Beide Werkzeuge teilen das Modul `tools/assetkit`.

## Ziel

Das Studio startet dort und in der Größe, in der es geschlossen wurde. Fehlt der gemerkte Bildschirm (abgesteckt, Anordnung oder Index geändert) oder ist die Datei kaputt, erscheint das Fenster sichtbar mit der Vorgabegröße auf dem Hauptbildschirm, nie außerhalb des sichtbaren Bereichs.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Zustand (x, y, Breite, Höhe) in einer eigenen Datei unter `<UserConfigDir>/k3c/` (Annahme: `k3c-studio.json`), nicht im Repo; beim Schließen gespeichert (`beforeClose`), beim Start gelesen.
- Dieselbe Logik wie in der Workbench: Mindestgröße, Vorgabegröße, `onScreen`-Prüfung (Windows), verdeckter Start bis die Position gesetzt ist, `placeWindow` für die Wails-Eigenheit der relativen Position.
- **Keine Kopie:** Die Logik zieht in das gemeinsame Modul `tools/assetkit` (Annahme: Paket `windowstate`), beide Werkzeuge nutzen sie; die Tests der Workbench (`screen_windows_test.go` u. a.) wandern mit.
- Defekte oder fehlende Datei, Fenster teils oder ganz außerhalb aller Bildschirme, negative Positionen für Bildschirme links oder oberhalb des Hauptbildschirms (gültig, wenn der Bildschirm da ist): sicherer Rückfall auf die Vorgabe.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Go-Tests im gemeinsamen Paket: Laden/Speichern (Roundtrip, kaputte Datei, zu kleine Maße werden angehoben), `onScreen` mit vorhandenem und fehlendem Bildschirm (Position weit außerhalb → falsch), Rückfall auf die Vorgabe.
- **AC-02** Die Workbench nutzt das gemeinsame Paket; ihre bisherigen Tests laufen unverändert grün (`task check:dev`).
- **AC-03** Das Studio speichert beim Schließen und stellt beim Start wieder her (Test mit Test-Ersatz für die Wails-Aufrufe); `task check:studio` grün.
- **AC-04** 🧑 prüft am PC: Fenster verschieben und Größe ändern, Studio schließen und starten → gleiche Position und Größe; die Datei auf einen nicht vorhandenen Bildschirm ändern (z. B. x = -30000) → das Fenster erscheint sichtbar mit der Vorgabegröße.

## Offene Fragen

Entscheidet 🧑: Gemeinsames Paket in `tools/assetkit` (Empfehlung, keine Doppelung) oder Kopie ins Studio? Eigener kleiner Sprint oder Einschub in WZ17/WZ18 (DEV, ca. 1–2 Sessions)? Vorgabegröße und Mindestgröße des Studios (Annahme: wie die Workbench, 1280×800, Minimum 900×600).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
