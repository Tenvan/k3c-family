# B-437 · Asset-Katalog von k3c-dev kennt eigene Gebäude-Grafik nicht

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** GEB
- **Erstellt:** 2026-10-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`tools/k3c-dev/internal/assets/catalog.go` (Zeilen 62–82) nimmt jede PNG unter `public/grafik/**` mit Pack = Ordnername auf. Für `gebaeude/` (eigene Grafik aus `task gebaeude`, GEB1.2) gibt es keine Credit-Zeile; der Katalog setzt `License = "unbekannt"` und markiert die Gebäude-PNGs vermutet als Lizenzproblem (ungeprüft, Befund aus dem Review von GEB1.2). Die Vitest-Ausnahmen in `credits.test.ts` und `grafikPacks.test.ts` decken den Katalog nicht ab.

## Ziel

Der Asset-Katalog führt `public/grafik/gebaeude/` als eigene Grafik (analog `selfMade` bei Sounds), ohne Lizenzwarnung.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
