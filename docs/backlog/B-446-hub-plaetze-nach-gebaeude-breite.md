# B-446 · Hub-Gebäude überdecken sich: Kaserne und Schmiede stehen 4 Units auseinander, sind aber 6 Kacheln breit

- **Domäne:** REG
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WRT
- **Erstellt:** 2026-10-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

W11.1 (B-409) hat die Treppen auf ≥ 8 Units Abstand zu jedem Zahlziel gebracht (`data/hub.json › sites`, `TestSiteDataTreppenAbstand`). Die übrigen Hub-Plätze liegen weiter auf dem 4-Units-Raster. Mit den Maßen aus GEB (`docs/standards/gebaeude-grafik.md` § 1, `kacheln.b` in `public/grafik/gebaeude/*.json`, 1 Kachel = 1 Unit) überdecken sich Gebäude, obwohl die Zahlziele das Mindestmaß halten (Rechnung vom 2026-10-16, Hub-Stufe 5, rechte Seite, Mitte ± Breite/2):

| Paar | Mitte x | Breite | Spanne | Überdeckung |
|---|---|---|---|---|
| Kaserne `barracks` | +24 | 6 | 21 … 27 | |
| Schmiede `smithy` | +28 | 6 | 25 … 31 | **2 Units mit der Kaserne** |
| Turm Linie 1 | +36 | 3 | 34,5 … 37,5 | |
| Rüstkammer `armory` | +40 | 6 | 37 … 43 | 0,5 mit dem Turm |
| Taverne `tavern` | −32 | 6 | −35 … −29 | 0,5 mit dem Turm Linie 1 links (−37,5 … −34,5) |
| Heilplatz `healer` | −40 | 6 | −43 … −37 | 0,5 mit demselben Turm |

Treppe hoch (+16, 4 × 4, 14 … 18) und Treppe runter (+8, 4 × 3, 6 … 10) liegen frei (kleinster Rand 2 Units zur Burg 8 × 9). GEB5 nennt die Kaserne-Schmiede-Überdeckung als bestätigte Annahme und verschiebt keine Hub-Plätze (GEB5 › Nicht-Ziele). Ob Preisschild und Wartegrund-Symbol (Breite im Client, `src/scenes/siteView.ts`) breiter sind als das Gebäude, ist ungeprüft.

## Ziel

Kein Hub-Gebäude überdeckt ein anderes (Breite aus der Gebäude-JSON), und die Offsets in `data/hub.json` sind daraus abgeleitet. Das schließt den Rest von B-409 (Beschluss 🧑 2026-10-11: Mindestabstand nach der Breite der Gebäude-Grafiken festlegen).

## Beteiligte und Zielgruppen

Spielende am TV sehen die Gebäude einzeln; Agent in REG ändert Werte in `data/hub.json`; 🧑 entscheidet, ob Plätze verschoben oder das Mindestmaß je Paar erhöht wird.

## Anforderungen

- Ein Test prüft für jeden Hub-Platz und die Linien-Plätze: Mitte ± halbe Breite (`kacheln.b` der Gebäude-JSON, Treppen 4) überdeckt kein anderes Gebäude, bei voller Streuung der Linien, je Tiefe (Treppe hoch nur ab Tiefe 1, Händler nur in Tiefe 0).
- Die Zahlziel-Regel (≥ 4 Units, Treppen ≥ 8) bleibt; die Breiten-Regel kommt dazu.
- Nach der Änderung: Golden und `testdata/saves/v*/familie.json` auf die neuen Offsets (Vorgehen wie W11.1), Balancing über `sim_test`.

## Nicht-Ziele

Neue Gebäude, neue Grafik (GEB), Client-Änderungen; Linien-Offsets (Kristallhöhle: Portale ab 135, Camps ohne freien Chunk, siehe W11 › Offene Fragen); Turm-Überdeckung von 0,5 Units, solange 🧑 sie nicht als Fehler sieht (Turm ragt hoch, Gebäude sind niedrig; ungeprüft am TV).

## Regeln und Einschränkungen

Domäne REG: nur Werte in `data/hub.json`, `docs/rules/materialien-gebaeude.md` § 3, Test in `engine/sim/site_data_test.go`, Golden und Spielstand-Fixtures. Erst nach GEB5 (Breiten von Lager, Kaserne, Farm und Treppen stehen dann auf `develop`); eigener Golden-Commit (`docs/arbeitsweise.md` › Golden aktualisieren). Keine Rücksicht auf alte Spielstände (Beschluss 🧑). Nur ein aktiver Sprint ändert Golden-Daten.

## Beispiele

Kaserne +24 (21 … 27) und Schmiede +28 (25 … 31) → nach der Änderung mindestens 6 Units Abstand der Mitten (z. B. Schmiede +30: 27 … 33, Turm ab 34,5; Angebots-`dx` anpassen); der Test meldet das Paar, solange es überdeckt.

## Ausnahme- und Fehlerfälle

Gebäude ohne `kacheln.b` in der JSON (Platzhalter-Grafik) → Test nimmt 6 an (Klasse „kleines Gebäude“) und meldet das. Angebots-Zahlziele (`dx`) haben kein Sprite und zählen nur für das Mindestmaß der Zahlziele.

## Akzeptanzkriterien

- **AC-01** Go-Test (`engine/sim/site_data_test.go`): Kein Hub-Gebäude überdeckt ein anderes oder die Burg (Türme ausgenommen), gemessen mit `kacheln.b` der Gebäude-JSON, bei voller Streuung und je Tiefe; vorher rot für Kaserne ↔ Schmiede.
- **AC-02** `data/hub.json` und `docs/rules/materialien-gebaeude.md` § 3 tragen die neuen Offsets, Golden und Fixtures sind nachgezogen, `task check:go` und `sim_test` Balancing ohne Einbruch.
- **AC-03** 🧑 hat den Hub am TV gesehen: keine überlappenden Gebäude.

## Offene Fragen

1. Verschieben (Vorschlag, ungeprüft: Schmiede nach +30 und Rüstkammer nach außen; ob zwischen Kaserne, Turm Linie 1 und Rüstkammer genug Platz bleibt, rechnet der Test in AC-01) oder die Gebäude schmaler zeichnen (GEB, 4 statt 6 Kacheln)? Entscheidet 🧑.
2. Gehört das Ticket in W11 (als REG-Session vor W11.4) oder in einen eigenen Sprint? Vorschlag: eigener Sprint nach GEB5, damit W11 klein bleibt.

## Notizen

Entstanden aus der Prüfung „W11 und GEB5“ (2026-10-16); Rechnung aus `data/hub.json` und `kacheln.b` auf `origin/develop`. Treppen-Breiten (4) sind GEB5-Annahmen, bis GEB5.3 die JSONs liefert. Nummer B-446 wurde im Worktree vergeben; bei Kollision mit einem parallel angelegten Ticket beim Merge umnummerieren.
