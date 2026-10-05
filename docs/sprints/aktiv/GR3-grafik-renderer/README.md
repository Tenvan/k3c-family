# GR3 · CLI · Grafik im Renderer

- **Status:** aktiv
- **Domäne:** CLI
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-010
- **Start-Commit:** 46aa69b
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 2, durch 🧑; umfasst B-010; nachgeschärft: Lizenz CC0/CC-BY, „zugeordnet“ statt „alle“, Portale/Truhen/Münzen, Stufen erst mit B-112

## Ausgangslage

Gebäude und Bauplätze sind Rechtecke in `src/scenes/worldRenderer.ts`; kein Spiel-Code lädt `public/grafik/`. Voraussetzung: Zuordnung (GR1.3) und Lückenschluss (GR2.3) sind fertig.

## Ziel

Gebäude, Ressourcen und Hintergründe werden mit den zugeordneten Grafiken gezeichnet. Am Ende sichtbar: Hub mit Sprites statt Formen, Parallax je Biom, Hub- und Materialstufen unterscheidbar am TV.

## Beteiligte und Zielgruppen

Spieler am TV und am Handy; Umsetzung durch Agent; 🧑 nimmt am TV ab.

## Anforderungen

B-010 › Anforderungen; Zuordnung aus `docs/assets/zuordnung.md`. Sprint-eigen: Hub-Stufen 1–5 und Mauer-/Turm-Materialstufen sichtbar (sobald B-112 sie liefert, vorher Stufe 1). Das Ticket B-010 hat nur zwei grobe Kriterien; vor der Freigabe prüfen, ob seine Spec nachgeschärft wird.

## Nicht-Ziele

Reittiere (Mechanik fehlt, B-152), Atlas und Lade-Szene (GR4), Effekte (GR5), neue Grafiken (GR2).

## Regeln und Einschränkungen

`src/scenes` zeichnet nur Snapshots und rechnet nichts (`noSim.test.ts`); Seiten-Regeln aus `CLAUDE.md`; B nicht belegen; 2 Spieler im Split-Screen; Datei ≤ 400 Zeilen (`worldRenderer.ts` bei Bedarf teilen).

## Beispiele

Mauer im Hub → Sprite aus der Zuordnung; Werkstatt ohne Grafik → Platzhalter-Form bleibt.

## Ausnahme- und Fehlerfälle

Grafik fehlt oder lädt nicht → Platzhalter-Form, keine leere Stelle.

## Akzeptanzkriterien

- **AC-01** Alle Gebäude, Ressourcen, Portale, Truhen, Münzen und Parallax-Ebenen, die in der Zuordnungstabelle „zugeordnet“ sind, werden als Sprites gezeichnet (B-010/AC-01).
- **AC-02** Die Credits aller eingebauten Grafiken stehen in `public/` (B-010/AC-02).
- **AC-03** Fehlt eine Grafik, bleibt die Platzhalter-Form sichtbar (Test oder Beobachtung).
- **AC-04** Hub-Stufen und Mauer-/Turm-Materialstufen sind unterscheidbar, sobald B-112 sie im Snapshot liefert; vorher genügen Stufe 1 und ein Test der Auswahl Stufe → Sprite. Sicht am TV: Beobachtung durch 🧑, bis dahin `angenommen, Validierung offen`.
- **AC-05** Wald, Höhle und Mine haben je eigene Parallax-Ebenen oder einen dokumentierten Platzhalter (Beobachtung).
- **AC-06** `task check` ist grün, mit 2 Spielern im Split-Screen sind keine Darstellungsfehler zu sehen.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GR3.1 | `GR3.1-gebaeude.md` | Umsetzung | autonom | fertig |
| GR3.2 | `GR3.2-ressourcen-parallax.md` | Umsetzung | autonom | fertig |
| GR3.3 | `GR3.3-review.md` | Review | autonom | fertig |
| GR3.4 | `GR3.4-abnahme-tv.md` | Workshop | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

- 2026-10-04, Review GR3.3: AC-01, AC-02, AC-03, AC-05, AC-06 umgesetzt und im Diff geprüft (Nachweise in GR3.1/GR3.2, `task check` und `task check:go` grün, Browser-Pane im Split-Screen).
- AC-04: Auswahl Stufe → Sprite getestet; ohne Stufe im Snapshot zeigen Mauer und Turm die Stein-Grafik (Holz ist Lücke), bewusste Abweichung. Sicht am TV (AC-04, AC-06): angenommen, Validierung offen (🧑, TV; im Fahrplan unter „Offen am Gerät“).
- Keine schweren Befunde (kein `Math.random()`, Rückfall auf Platzhalter, Credits vollständig, Dateien ≤ 400). B-010 archiviert, keine neuen Tickets.
- Version: v0.11.0 vorgeschlagen (Minor: Sprites und Parallax im Spiel; nach den offenen Vorschlägen bis v0.10.0, aktuell v0.6.0, bei gemeinsamem Setzen anpassen).
