# GR1 · CLI · Grafik-Zuordnungstabelle

- **Status:** geplant
- **Domäne:** CLI
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-161
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Zwölf Umgebungs-Packs liegen unter `public/grafik/` (119 PNG), 41 Figuren-Ordner unter `public/sprites/`; gebaut ist nichts davon für Gebäude und Umgebung, Gebäude sind Rechtecke in `src/scenes/worldRenderer.ts`. Die genaue Bestandsaufnahme (welche Objekte Grafik haben, welche nicht) steht in B-161 › Ausgangslage.

## Ziel

Jedes Spielobjekt hat eine Zuordnung zu Asset, Stil und Lizenz oder eine dokumentierte Lücke. Am Ende sichtbar: `docs/assets/zuordnung.md` und ein grüner Vollständigkeits-Test.

## Beteiligte und Zielgruppen

🧑 entscheidet den Grafik-Stil (Q13) und gibt die Zuordnung frei; der Agent erstellt die Tabelle.

## Anforderungen

B-161 › Anforderungen.

## Nicht-Ziele

Suche nach Assets für Lücken (GR2, B-162), Einbau (GR3, B-010), Atlas (GR4, B-163).

## Regeln und Einschränkungen

Nur CC0 oder CC-BY, Credits in `public/*/CREDITS.md`; Datei ≤ 400 Zeilen; `task check` grün. Einschiebbar, läuft früh und parallel zu GR2.

## Beispiele

`workshop` → **keine Grafik**, Ticket B-162; `wall` Stufe 1 → Pack, Datei, CC0, zugeordnet.

## Ausnahme- und Fehlerfälle

Objekt in `data/` ohne Zeile → Test rot. Asset ohne Credit → Test rot.

## Akzeptanzkriterien

- **AC-01** `docs/assets/zuordnung.md` hat zu jeder Objekt-ID aus `data/buildings.json`, `data/enemies.json` und `data/troops.json` eine Zeile mit Status (B-161/AC-01).
- **AC-02** Weitere Objekte (Hub-Stufen, Materialstufen, Materialien, Adern, Plantage, Truhen, Portale, Icons, Hintergründe je Biom) sind erfasst (B-161/AC-02).
- **AC-03** Jedes zugeordnete Asset hat einen Credit-Eintrag (B-161/AC-03).
- **AC-04** Jede Lücke ist fett markiert und verweist auf ein Ticket (B-161/AC-04).
- **AC-05** Jede Zeile nennt Stil und Lizenz, der Stilbeschluss von 🧑 steht im Kopf (B-161/AC-05).
- **AC-06** `task check` ist grün.

## Offene Fragen

- Grundstil (Raster, Palette, Skalierung): Entscheidet 🧑 (`docs/fragenkatalog.md` Q13); blockiert die Freigabe.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- GR1.1 🧑 Workshop (Agent: Mensch): Grundstil festlegen (AC-05).
- GR1.2 Tabelle mit Gebäuden, Gegnern, Truppen und Vollständigkeits-Test (AC-01, AC-03, AC-04).
- GR1.3 Restliche Objekte erfassen (AC-02).
- GR1.4 Review (AC-06).

## Abnahme

–
