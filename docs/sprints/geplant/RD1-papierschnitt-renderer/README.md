# RD1 · INF · Papierschnitt-Renderer: Rig-Typen, IK, Physik, resvg-Export, Golden-Tests

- **Status:** geplant
- **Projekt:** PAP
- **Domäne:** INF
- **Reife:** Entwurf
- **Tickets:** B-699
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Welle 1, Lane **RD1** (Domäne PAP, läuft parallel zu MC1, WB1, RV1). Verträge aus PV1: `docs/standards/paper-spec.md` (Schema, Pflicht-Teile, Pivots, Ausgabe-Layout, CLI). Referenz-Verhalten im Prototyp `docs/decisions/005/prototyp-monarch-rig.html`.

## Ziel

`node tools/paper/bin/paper.mjs` erfüllt den CLI-Vertrag vollständig und rendert Monarch und Pferd aus ihren Specs wie der Prototyp; `task gen:paper` baut alle Specs.

Am Ende sichtbar: `public/grafik/figuren/monarch/galopp.png` aus der Spec, Golden-Tests grün in der CI.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Entscheidung 005 › 1–3, 7; `docs/standards/paper-spec.md`.

## Nicht-Ziele

MCP-Tools (MC1), Oberfläche (WB1, RV1), Migration weiterer Assets (MG*).

## Regeln und Einschränkungen

Nur `tools/paper/` (außer `schema/` und `fixtures/`, die Vertrag bleiben) und `tools/paper/Taskfile.yml` (Task `gen:paper`). Laufzeit bun (B-701 D-03). Determinismus: fester Zeitschritt 1/120 s, Einschwingzeit 2 s, Struktur-Seed aus der Asset-ID, Koordinaten auf 2 Nachkommastellen, `renderKey` aus `hash.ts` (PV1.3); Golden-Regel aus `paper-spec.md` (exakt oder Toleranz). `gen:paper` schreibt je Gruppe `index.paper.json` und `golden.json`, keinen gemeinsamen Index. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `paper validate` erfüllt alle Regeln aus `paper-spec.md` (005/AC-01).
- **AC-02** Golden-Tests: Spec → PNG-Hash fest für Monarch (5 Animationen), Pferd (3 Gangarten), Mauer (2 Stufen); zwei Läufe byte-gleich (005/AC-02).
- **AC-03** Monarch rendert Idle, Lauf, Schritt, Trab, Galopp mit IK-Hufen, Rückengelenk und Umhang-Physik (005/AC-03); Sichtvergleich mit dem Prototyp im Ergebnis.
- **AC-04** Materialänderung in `stil.paper.json` ändert die Hashes aller betroffenen Assets ohne Spec-Änderung (005/AC-04).
- **AC-05** `frame`, `diff`, `pixeldiff` liefern die Antworten aus dem Vertrag; die Attrappe aus PV1.4 und der echte Renderer bestehen denselben Vertragstest.

## Offene Fragen

keine

## Sessions

Reife Entwurf, Lane `renderer`:

- RD1.1 Opus: Kern – Spec laden, Stil auflösen, Teile-Baum, Lagen `flaeche|falz|punkt`, SVG mit Filtern, `validate`, `render` für `requisite` und `gebaeude` (AC-01, AC-02 Mauer)
- RD1.2 Opus: Rig `figur` – Posen, Keyframes, Umhang-Physik (AC-03 teilweise)
- RD1.3 Opus: Rig `vierbeiner` – IK, Gangarten, Rückengelenk, Beinpfad; Reiten als Kombination (AC-03)
- RD1.4 Sonnet: `frame`, `diff`, `pixeldiff`, `gen:paper`, Gruppen-Index, Vertragstest gegen Attrappe (AC-04, AC-05)
- RD1.5 Review · Opus

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
