# B-437 · Asset-Katalog von k3c-dev kennt eigene Gebäude-Grafik nicht

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** GEB3
- **Projekt:** GEB
- **Erstellt:** 2026-10-16
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-16, Chat (Ralf), mit Sprint GEB3 Revision 1

## Ausgangslage

`tools/k3c-dev/internal/assets/catalog.go` (Zeilen 62–82) nimmt jede PNG unter `public/grafik/**` mit Pack = Ordnername auf. Für `gebaeude/` (eigene Grafik aus `task gebaeude`, GEB1.2) gibt es keine Credit-Zeile; der Katalog setzt `License = "unbekannt"` und markiert die Gebäude-PNGs vermutet als Lizenzproblem (ungeprüft, Befund aus dem Review von GEB1.2). Die Vitest-Ausnahmen in `credits.test.ts` und `grafikPacks.test.ts` decken den Katalog nicht ab.

## Ziel

Der Asset-Katalog führt `public/grafik/gebaeude/` als eigene Grafik (analog `selfMade` bei Sounds), ohne Lizenzwarnung.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `tools/k3c-dev/internal/assets/catalog.go` (`asset`): PNG unter `public/grafik/gebaeude/` bekommt die Lizenz „eigene Grafik (tools/gebaeude, kein Credit nötig)“ statt „unbekannt“, analog `selfMade` bei Sounds; `Flag` leer.
- Geprüft 2026-10-16 (Claude): `LicenseFlag("unbekannt")` liefert kein NC/ND/SA-Flag; das Problem ist die Lizenzangabe „unbekannt“ in Katalog und Asset-Manager, kein falsches Flag.
- Go-Test in `catalog_test.go`: eine Datei unter `grafik/gebaeude/` hat die eigene Lizenz, eine Pack-Datei ohne Credit bleibt „unbekannt“.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Der Asset-Katalog führt jede PNG unter `public/grafik/gebaeude/` mit eigener Lizenz statt „unbekannt“; `go test` in `tools/k3c-dev/internal/assets` belegt eigene Grafik und unveränderten Fallback für Packs ohne Credit.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
