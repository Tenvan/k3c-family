# B-700 · Alte Pixel-Grafiken, Register und Gebäude-Renderer werden nach der Papierschnitt-Umstellung entfernt

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** CU1
- **Projekt:** PAP
- **Erstellt:** 2026-11-06
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-06, Chat (🧑: alte Grafiken als Cleanup zum Schluss)

## Ausgangslage

Nach der Migration (MG1–MG8) und der Umstellung des Spiels (US1.1–US1.2) liegen noch die gekauften Pixel-Packs unter `public/grafik/` und `public/grafik-db32/`, der Atlas `public/atlas/`, das Register `data/assets.json` (ab MC1 eingefroren), `data/sprites.json`, der Gebäude-Renderer `tools/buildings` mit `public/grafik/gebaeude/*.png`, die Zuordnungs-Doku `docs/assets/zuordnung*.md` und Grafik-Zeilen in `public/grafik/CREDITS.md`.

## Ziel

Im Repo gibt es keine alte Pixel-Grafik und keinen Code mehr, der sie erzeugt oder lädt.

## Beteiligte und Zielgruppen

🧑 nimmt ab; Agent der Session US1.4.

## Anforderungen

- Gelöscht wird nur, was per Suche nachweislich ungenutzt ist (Beleg im Ergebnis).
- Mit: Test-Fixtures, Mocks, Taskfile-Tasks (`gen:gebaeude`, `gen:atlas`), CI-Schritte, Codemaps, CLAUDE.md-Zeilen.
- Sound bleibt unberührt.

## Nicht-Ziele

Neue Grafik (MG*), Skalierung für Xbox und Mobile.

## Regeln und Einschränkungen

Letzte Umsetzungs-Session von US1, erst nach der Abnahme der Spiel-Umstellung durch 🧑; Entscheidung 005 › Folgen.

## Beispiele

`rg -l "grafik-db32|atlas/atlas.json|tools/buildings"` liefert nach der Session keine Treffer außerhalb von `docs/sprints/erledigt/` und `docs/decisions/`.

## Ausnahme- und Fehlerfälle

Ein Pack wird doch noch genutzt → nicht löschen, Ticket mit Fundstelle.

## Akzeptanzkriterien

- **AC-01** Keine Datei aus den alten Packs, kein Atlas, kein `tools/buildings`, kein `data/assets.json`-Register mehr im Repo; Suche wie im Beispiel ohne Treffer.
- **AC-02** `task check:all` und `task buildall` grün.
- **AC-03** `CREDITS.md` enthält keine Grafik-Packs mehr.

## Offene Fragen

keine

## Notizen

Projekt PAP, Sprint US1, Session US1.4 (Schluss).
