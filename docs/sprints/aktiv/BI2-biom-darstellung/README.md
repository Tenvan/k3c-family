# BI2 · PAP, CLI · Biom-Eigenschaften sichtbar: Kristallfarben, Decke, Tropfsteine, Verfall, Glut, Leuchten

- **Status:** aktiv
- **Projekt:** NAT
- **Domäne:** PAP, CLI
- **Reife:** bereit
- **Tickets:** B-900, B-901, B-902, B-903, B-904, B-905, B-906
- **Start-Commit:** 6802a69d9
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: nächste autonome Welle)

## Ausgangslage

Seit BI1 (B-891) haben die Biome Natur-Eigenschaften (`traits`, `weights` in `data/biomes/*.json`); der Level trägt die geltenden Werte je Stufe (`level.Layout.Nature`). Das native Spiel zeigt davon nur die Baumart. Kristall-Adern tragen eine Farbe (`crystalColors`: blau, violett, gruen), es gibt aber nur ein Asset `ader-kristall`. Die reinen Darstellungs-Eigenschaften (`display: true`) liest niemand.

## Ziel

Kristallfarben, Deckenhöhe, Tropfsteine, Stützbalken und Verfall, Hitze und Glut sowie Leuchtstärke sind im nativen Spiel sichtbar, jeweils nach dem geltenden Wert der Stufe.

## Beteiligte und Zielgruppen

Spieler (sehen unterschiedliche Höhlen je Welt), Claude (Umsetzung, autonom), Ralf 🧑 (Sichtprüfung im Abnahme-Ticket).

## Anforderungen

Siehe B-900 bis B-906. Gemeinsam: Wert der Eigenschaft aus `World.Level.Nature`, fehlt er, gilt der `default` des Bioms; Biome ohne die Eigenschaft zeigen nichts davon. Formen und Orte deterministisch aus der Position (`mix`), kein Zufall. Alles mit zwei Spielern (je Zelle eigene Kamera).

## Nicht-Ziele

Neue Eigenschaften; Balancing (gesperrt); Änderungen an Licht und Nacht in `native/k3c-game/world/light.go`; Papierschnitt-Assets für Decke, Tropfsteine, Balken und Glut (gezeichnet als Formen, Assets ggf. später als eigenes Ticket); Browser-Client.

## Regeln und Einschränkungen

Domäne CLI (B-900: PAP, Assets nur über `assets_*`). Stil-Datei bleibt unverändert (keine Lane stil): violett und grün nutzen die vorhandenen Materialien `purpur` und `smaragd`. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Nicht anfassen: `native/k3c-game/input`, `menu`, `hud`, `devtools`, `engine/room`, `cmd/k3c-sim`. Gen-Läufe (`gen:paper`, Golden) und `check:race` nur im Abnahme-Ticket.

- **Bereich:** `native/k3c-game/props/**`, `public/grafik/objekte/**`, `data/paper.json`
- **Abhängig von:** B-891
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

- Kristallhöhle mit Gewichten blau 0, violett 5, gruen 0 → alle Kristall-Adern violett.
- Höhle mit `ceilingHeight` 1 → breites Felsband oben im Bild; mit 3 → nur ein schmaler Rand.
- Höhle mit `stalactites` 0 → keine Tropfsteine; mit 3 → dreimal so viele wie mit 1.
- Mine mit `decay` 0 → alle Stützbalken heil; mit 3 → die meisten schief oder gebrochen.
- Eisenstollen mit `glow` 0 → keine Glut; mit 3 → viele pulsierende Glutpunkte.
- Kristallhöhle mit `brightness` 3 → großer heller Schein um jede Ader.

## Ausnahme- und Fehlerfälle

- Level ohne Natur-Werte (alter Spielstand, Online-Welt ohne `Nature`) → `default` des Bioms.
- Unbekannte Variante einer Kristall-Ader → Bild `ader-kristall` (blau).
- Tests ohne Grafik (`lib` = nil) → Formen werden trotzdem geplant, Sprites fallen wie bisher weg.

## Akzeptanzkriterien

- **AC-01** Kristall-Adern gibt es als Papierschnitt in drei Farben (B-900/AC-01).
- **AC-02** Jede Kristall-Ader zeigt das Bild ihrer Farbe (B-901/AC-01, AC-02).
- **AC-03** Die Höhle zeigt eine Decke, deren Höhe dem Wert folgt (B-902/AC-01, AC-02).
- **AC-04** Die Höhle zeigt Tropfsteine, deren Anzahl dem Wert folgt (B-903/AC-01, AC-02).
- **AC-05** Die Mine zeigt Stützbalken, deren Verfall dem Wert folgt (B-904/AC-01, AC-02).
- **AC-06** Der Eisenstollen zeigt Glut, deren Menge dem Wert folgt (B-905/AC-01, AC-02).
- **AC-07** Die Kristallhöhle zeigt einen Schein um die Adern, dessen Stärke dem Wert folgt (B-906/AC-01, AC-02).

## Offene Fragen

keine (Defaults autonom entschieden: blau = vorhandenes `ader-kristall`; violett/grün mit `purpur`/`smaragd` ohne Stil-Änderung; Darstellung als Formen statt Assets; Leuchten als Schein in props statt über `world/light.go`).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BI2.1 | `BI2.1-kristall-assets.md` | Umsetzung | autonom | fertig |
| BI2.2 | `BI2.2-kristallfarbe-im-spiel.md` | Umsetzung | autonom | fertig |
| BI2.3 | `BI2.3-hoehle-decke-tropfsteine.md` | Umsetzung | autonom | fertig |
| BI2.4 | `BI2.4-verfall-glut-leuchten.md` | Umsetzung | autonom | fertig |
| BI2.5 | `BI2.5-review.md` | Review | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
