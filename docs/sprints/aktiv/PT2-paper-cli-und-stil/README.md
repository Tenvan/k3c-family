# PT2 · PAP · Paper-CLI: Fehlerpfade, Reittier-Boxen, Stil-Option und Ersatz-Materialien

- **Status:** aktiv
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-708, B-735
- **Start-Commit:** a96eee64c
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

Aus MG1 und RV1 sind Restbefunde an der Paper-CLI offen: Fehlerfeld ohne Animationsnamen (B-708), keine Teil-Boxen des Reittiers in `paper frame` (B-709, Teil CLI), Stil-Datei immer aus der Repo-Wurzel, sodass das Review-Vorher-Bild mit dem falschen Stil rendert (B-705, Teil CLI). Aus RD2 fehlen Ersatz-Materialien (Stroh, Lava, Geisterweiß, Fell grau; B-735). B-705 und B-709 schließt WZ43 (Workbench-Teil); dieser Sprint liefert ihren CLI-Teil.

## Ziel

Die Paper-CLI liefert eindeutige Fehlerpfade, Reittier-Boxen und eine wählbare Stil-Datei; die Stil-Datei hat die fehlenden Materialien, und kein Asset nutzt mehr Ersatz-Abbildungen.

Am Ende sichtbar: `paper frame monarch --animation galopp` mit Boxen `reittier:*`; `paper render … --stil <datei>`; Grafik-Golden aller Gruppen neu und grün.

## Beteiligte und Zielgruppen

Claude (Umsetzung PT2.1, PT2.2; Review PT2.3); Agenten, die Assets über `assets_*` bearbeiten; 🧑 sieht die neuen Materialien im Bereich Assets.

## Anforderungen

`B-708 › Ziel`, `B-735 › Ziel`, CLI-Teil von `B-709 › Ziel` und `B-705 › Regeln und Einschränkungen` (Option `--stil`). Sprint-eigen: Vertragsänderung `docs/standards/paper-spec.md` › 8 (CLI) nur für `--stil` und das Box-Präfix `reittier:`.

## Nicht-Ziele

Workbench-Teil von B-705 und B-709 (WZ43), `assets_set` mit Teil-Patch (B-710, WZ43), gen:paper-Tempo (PT1), neue Assets oder Neugestaltung (MG10, MG11).

## Regeln und Einschränkungen

Domäne PAP, Entscheidung 005; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; keine neue Abhängigkeit. Grafik-Golden wird in PT2.2 neu geschrieben (Gen am Feature, B-742; `docs/arbeitsweise.md` › Golden aktualisieren). Alle Teile Klasse A (Problem/Schuld, PAP bzw. DEV, ohne Spielregeln, Protokoll, Spielstand, Abhängigkeiten).

- **Bereich:** `tools/paper/rig/**`, `tools/paper/cli/frame*.ts`, `tools/paper/bin/paper.ts`, `tools/paper/bin/paper.test.ts`, `tools/paper/render/stil*.ts`, `tools/paper/schema/**`, `public/grafik/stil.paper.json`, `public/grafik/*/*.paper.json`, `tools/paper/golden/**`, `docs/standards/paper-spec.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** golden

## Beispiele

`assets_set monarch {animationen: {schritt: {reittier: esel}}}` → `feld: animationen.schritt.reittier`. Lava-Golem nutzt Material `lava` statt `stoff`.

## Ausnahme- und Fehlerfälle

`--stil` zeigt auf eine fehlende oder ungültige Datei → Exit ≠ 0 mit Pfad im Fehler. Ein Asset sieht mit neuem Material deutlich anders aus → Golden neu, Hinweis im Ergebnis für 🧑 (Sichtprüfung im Bereich Assets, nicht blockierend).

## Akzeptanzkriterien

- **AC-01** Fehlerfeld nennt `animationen.<name>.reittier` (`B-708/AC-01`).
- **AC-02** `paper frame monarch --animation galopp` liefert Boxen mit Präfix `reittier:` (`B-709/AC-01`).
- **AC-03** `paper frame` und `paper render` nehmen `--stil <datei>`, ohne Option gilt `public/grafik/stil.paper.json` (`B-705/AC-01`).
- **AC-04** Vier Materialien in der Stil-Datei, Schema-Test grün (`B-735/AC-01`).
- **AC-05** Kein Asset nutzt die Ersatz-Abbildungen, Golden aller Gruppen neu (`B-735/AC-02`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| PT2.1 | `PT2.1-cli-vertraege.md` | Umsetzung | autonom | offen |
| PT2.2 | `PT2.2-ersatz-materialien.md` | Umsetzung | autonom | offen |
| PT2.3 | `PT2.3-review.md` | Review | autonom | offen |

Keine eigenen Sim- oder Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
