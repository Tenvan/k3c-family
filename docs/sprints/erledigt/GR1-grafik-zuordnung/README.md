# GR1 · CLI · Grafik-Zuordnungstabelle

- **Status:** erledigt
- **Domäne:** CLI
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-161
- **Start-Commit:** e317292
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1, durch 🧑; umfasst B-161; mit Änderungen aus dem Spec-Review (Q13 geklärt, Reittiere nach Q23, Reihenfolge nach GR2)

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

Nur CC0 oder CC-BY, Credits in `public/*/CREDITS.md`; Datei ≤ 400 Zeilen; `task check` grün. Einschiebbar, läuft früh; GR2 braucht seine Lückenliste (GR2.1 nach GR1.3).

## Beispiele

`workshop` → **keine Grafik**, Ticket B-162; `wall` Stufe 1 → Pack, Datei, CC0, zugeordnet.

## Ausnahme- und Fehlerfälle

Objekt in `data/` ohne Zeile → Test rot. Asset ohne Credit → Test rot.

## Akzeptanzkriterien

- **AC-01** `docs/assets/zuordnung.md` hat zu jeder Objekt-ID aus `data/buildings.json`, `data/enemies.json` und `data/troops.json` eine Zeile mit Status (B-161/AC-01).
- **AC-02** Weitere Objekte (Hub-Stufen, Materialstufen, Materialien, Adern, Plantage, Truhen, Portale, Reittiere, Icons, Hintergründe je Biom) sind erfasst (B-161/AC-02).
- **AC-03** Jedes zugeordnete Asset hat einen Credit-Eintrag (B-161/AC-03).
- **AC-04** Jede Lücke ist fett markiert und verweist auf ein Ticket (B-161/AC-04).
- **AC-05** Jede Zeile nennt Stil und Lizenz, der Stilbeschluss von 🧑 steht im Kopf (B-161/AC-05).
- **AC-06** `task check` ist grün.

## Offene Fragen

keine (Grundstil durch Q13 geklärt, 2026-10-03)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GR1.1 | `GR1.1-workshop-stil.md` | Workshop | Mensch | fertig |
| GR1.2 | `GR1.2-tabelle-test.md` | Umsetzung | autonom | fertig |
| GR1.3 | `GR1.3-restliche-objekte.md` | Umsetzung | autonom | fertig |
| GR1.4 | `GR1.4-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

2026-10-04, Agent (Claude Sonnet) in GR1.4, leichtes Review. AC-01, AC-03 bis AC-05: Ergebnis GR1.2 (Tabelle `docs/assets/zuordnung-objekte.md`, Test `src/tools/zuordnung.test.ts`); AC-02: Ergebnis GR1.3 (`docs/assets/zuordnung-welt.md`); AC-05 Kopf: Workshop GR1.1 mit 🧑; AC-06: `task check` grün.
Schwere Befunde: keine. Neue Tickets: B-251 (Figuren-Skalierung). B-161 archiviert; 24 Lücken gehen an GR2 (B-162), Skill-Icons an B-124, Bosse an B-130.
Version: v0.6.1 vorgeschlagen (Patch: nur Doku und Test, keine Spieländerung); gesetzt erst nach Bestätigung durch 🧑.
