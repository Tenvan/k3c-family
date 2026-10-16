# GEB2 · DEV, CLI · Gebäude ohne Grafik: Taverne, Heilplatz, Schmiede, Rüstkammer

- **Status:** aktiv
- **Projekt:** GEB
- **Domäne:** DEV, CLI
- **Reife:** bereit
- **Tickets:** B-435
- **Start-Commit:** 4867877b
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-16, Chat (Ralf), Revision 2

## Ausgangslage

Taverne, Heilplatz, Schmiede und Rüstkammer haben keine Grafik (Slots `tavern`, `healer`, `smithy`, `armory` sind Lücken). Grundregeln und Werkstatt-Vorlage kommen aus GEB1.

## Ziel

Die vier Lücken-Gebäude haben eigene Grafik nach den Grundregeln. Am Ende sichtbar: Vorschau im Asset-Manager im Spielmaßstab, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

🧑 nimmt die vier Gebäude am Bildschirm ab (Vorschau und Spiel). Agenten schreiben Daten-JSON und Zuordnung. Spieler sehen die Gebäude ab Hub-Stufe 3 bzw. 4 und an der Taverne.

## Anforderungen

B-435 › Anforderungen; Regeln aus `docs/standards/gebaeude-grafik.md` (GEB1). Sprint-eigen:
- Alle vier Gebäude haben die Maße der Klasse „kleines Gebäude“: **6 × 4 Kacheln** wie die Werkstatt (Entscheidung 🧑 2026-10-16, Chat). Unterschiede entstehen über Bausteine und Farben aus der Materialsprache.
- Erkennbarkeit am Motiv: Taverne = Holzhaus mit Schild (Krug), Fenster mit warmem Licht, Fass; Heilplatz = offenes Zelt/Vordach mit hellem Stoff, Liege, Kräuterkorb; Schmiede = Esse mit Glut, Amboss, Kamin; Rüstkammer = Steinsockel, Ständer mit Rüstung und Schild. Neue Materialien (Stein, Glut, Glas) werden mit DB32-Farben in die Regeldatei eingetragen.
- **Zuordnung ins Spiel** (Entscheidung 🧑 2026-10-16, Chat): Die Slots `tavern`, `healer`, `smithy`, `armory` und `workshop` zeigen im Spiel die neuen Grafiken in der Spiel-Dichte **32 px je Kachel ×1** (Regeldatei § 7). Die übrigen Gebäude bleiben bei ihrem Pack-Maßstab (×2).
- **Aufräumen** (🧑 2026-10-16, Chat; gilt für alle GEB-Sprints, Projekt GEB › Notizen): Alte Grafik, die das Gebäudesystem im Spiel ersetzt, wird im selben Sprint gelöscht, samt Einträgen in `public/grafik/index.json`, Register, Zuordnungs-Doku, Test-Fixtures/Mocks und CREDITS, sofern keine andere Stelle sie nutzt. In GEB2 betrifft das die alte Werkstatt-Grafik `gothicvania-town/props-einzeln/house-b.png` (Präferenz 2 im Slot `workshop` entfällt damit).

## Nicht-Ziele

Ausbaustufen und Level-Paletten der vier Gebäude; Bäume ins Spiel (`node:tree`, bleibt B-438); Asset-Katalog-Lizenz (B-437); Animationen (Glut, Rauch); TV-Skalierungsfaktor (GR7.4, B-251); weitere Gebäude (GEB3–GEB7).

## Regeln und Einschränkungen

- Domänen nacheinander: **DEV** (GEB2.1, GEB2.2: `public/grafik/gebaeude/`, `tools/gebaeude/` nur bei nötiger Erweiterung, `docs/standards/gebaeude-grafik.md`) → **CLI** (GEB2.3: `data/assets.json` als Register-Eintrag, `src/scenes/buildingSprites.ts`, `src/scenes/siteView.ts`, Tests). `engine/` bleibt unberührt.
- Determinismus wie GEB1: gleiche JSON → byte-gleiche PNGs; kein `Math.random()`.
- Keine neue Abhängigkeit. Budget: Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

- `task gebaeude` mit `taverne.json` → `taverne-32.png` 192×128 px plus 16/64/128; zweiter Lauf ändert nichts.
- `assetvorschau.html?szene=gebaeude` → Streifen „Neue Gebäude (GEB)“ zeigt Werkstatt, Taverne, Heilplatz, Schmiede, Rüstkammer je 32 px ×1 und 16 px ×2.
- Im Spiel ist der Heilplatz gebaut → Bild `gebaeude/heilplatz-32.png` in Maßstab 1, 6 Kacheln breit, Unterkante auf dem Boden; die Kaserne daneben bleibt Pack-Grafik ×2.

## Ausnahme- und Fehlerfälle

- Baustelle oder zerstörtes Gebäude → weiter Platzhalter mit Umriss, Fortschritt und Preisschild (wie heute, `siteTexture` nur bei `built`).
- PNG fehlt im Build → `buildingSprites.test.ts` schlägt fehl (bestehende Prüfung der Dateien).
- Farbe nicht aus DB32 oder Form außerhalb 6 × 4 → `task gebaeude` bricht ab (GEB1-Regeln).

## Akzeptanzkriterien

- **AC-01** Taverne, Heilplatz, Schmiede und Rüstkammer liegen als Daten-JSON und PNG unter `public/grafik/gebaeude/` und entsprechen den Regeln aus GEB1 (6 × 4 Kacheln, DB32, Licht links oben, `fein`-Details); zweiter `task gebaeude`-Lauf ändert nichts; sie erscheinen im Vorschau-Streifen „Neue Gebäude (GEB)“; neue Materialien stehen in `docs/standards/gebaeude-grafik.md`.
- **AC-02** Die Slots `tavern`, `healer`, `smithy`, `armory` und `workshop` sind im Register (`data/assets.json`, Präferenz 1) und in `src/scenes/buildingSprites.ts` den neuen 32-px-PNGs zugeordnet; das Spiel zeichnet sie gebaut in Maßstab 1 (32 px je Kachel), alle anderen Gebäude unverändert ×2; Vitest prüft Zuordnung und Maßstab.
- **AC-03** 🧑 hat die vier Gebäude am Bildschirm abgenommen, in der Vorschau und gebaut im Spiel.
- **AC-04** Ersetzte alte Grafik ist gelöscht: `house-b.png` ist mit allen Verweisen (Grafik-Index, Register, Zuordnungs-Doku, Tests, Mocks, CREDITS) entfernt, belegt durch eine Suche ohne Treffer außerhalb von `docs/sprints/erledigt/`; `task check` und `task check:go` grün.

## Offene Fragen

- Entschieden 2026-10-16 (Chat, Ralf): Zuordnung ins Spiel gehört in GEB2 (eigene CLI-Session), inklusive `workshop`; Bäume bleiben B-438.
- Entschieden 2026-10-16 (Chat, Ralf): alle vier Gebäude 6 × 4 Kacheln wie die Werkstatt.
- Freigabe der Spec setzt 🧑.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB2.1 | `GEB2.1-taverne-heilplatz.md` | Umsetzung | autonom | fertig |
| GEB2.2 | `GEB2.2-schmiede-ruestkammer.md` | Umsetzung | autonom | fertig |
| GEB2.3 | `GEB2.3-zuordnung-spiel.md` | Umsetzung | autonom | fertig |
| GEB2.4 | `GEB2.4-review-abnahme.md` | Review | autonom | blockiert |

- GEB2.1 (DEV) Taverne und Heilplatz als Daten und PNG, Regeldatei ergänzen (AC-01).
- GEB2.2 (DEV) Schmiede und Rüstkammer als Daten und PNG, Stein und Glut in die Regeldatei (AC-01).
- GEB2.3 (CLI) Zuordnung der fünf Slots in Register und Client, Maßstab 1 im Spiel; alte Werkstatt-Grafik löschen (AC-02, AC-04).
- GEB2.4 Review und Abnahme durch 🧑 in Vorschau und Spiel (AC-03, alle).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
