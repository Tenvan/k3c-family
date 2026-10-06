# S7 · CLI · Monarch auf dem Standard-Reittier

- **Status:** aktiv
- **Domäne:** CLI
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-173
- **Start-Commit:** e317292
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1, durch 🧑; umfasst B-173; bestätigt die Vorschläge der Planung in S7.1 und S7.2

## Ausgangslage

Die Simulation kennt nach S1 (B-152) ein Standard-Reittier je Monarch; der Client zeichnet den Monarchen noch als einzelne Figur. Die Reittier-Sprites und Sattelpunkte liegen in `data/sprites.json` › `mounts`.

## Ziel

Der Monarch erscheint beritten, mit Animationen für Stehen, Laufen und Sprint. Am Ende sichtbar: Zwei Spieler im Split-Screen reiten am TV über die Stufe.

## Beteiligte und Zielgruppen

Spieler am TV und am Handy; 🧑 nimmt am Gerät ab.

## Anforderungen

B-173 › Anforderungen.

## Nicht-Ziele

Andere Reittiere als Auswahl, neue Grafiken (GR2), Reittiere für Truppen, Ton.

## Regeln und Einschränkungen

`src/scenes` zeichnet nur Snapshots; Lizenz-Credits der Reittier-Sprites (CC-BY) stehen in `public/sprites/CREDITS.md`; Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Spieler läuft nach links → das Reittier ist gespiegelt, der Reiter sitzt am Sattelpunkt.

## Ausnahme- und Fehlerfälle

Unbekannter Sprite-Schlüssel → bisherige Figur und Log-Eintrag.

## Akzeptanzkriterien

- **AC-01** Auswahl von Sprite, Animation und Reiter-Position ist eine getestete reine Funktion (B-173/AC-01).
- **AC-02** Jeder Monarch erscheint mit Reittier, die Animationen unterscheiden sich je Bewegung (B-173/AC-02).
- **AC-03** Zwei Spieler im Split-Screen zeigen beide Reittiere korrekt, der Client rechnet nichts (B-173/AC-03).
- **AC-04** 🧑 hat die Darstellung am TV und am Handy abgenommen (B-173/AC-04).

## Offene Fragen

keine: Standard-Tier ist das braune Pferd `horse`, Geschwindigkeits- und Sprintfaktor 1,0 (bestätigt im Workshop F1.4, `docs/rules/monarch.md` § 7).

## Sessions

Reihenfolge wie die Nummern. Voraussetzung: S1.3 hat `data/monarch.json › mount` angelegt, S4 ist erledigt (CLI-Bahn), GR3 läuft nicht gleichzeitig (gleicher Renderer). Vorschläge der Planung stehen in den Sessions unter „Entscheidungen dieser Session“ und gelten erst mit der Freigabe.

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| S7.1 | `S7.1-reittier-pose.md` | Umsetzung | autonom | fertig |
| S7.2 | `S7.2-renderer-anbindung.md` | Umsetzung | autonom | fertig |
| S7.3 | `S7.3-abnahme-geraet.md` | Workshop | Mensch | offen |
| S7.4 | `S7.4-review.md` | Review | autonom | fertig |

## Abnahme

- **2026-10-04 (S7.4):** AC-01 (S7.1: `mountPose.ts`, 7 Tests), AC-03 (S7.2: Test mit vier Spielern, `noSim.test.ts`) mit Nachweis. AC-02 (Sichtnachweis) und AC-04: angenommen, Validierung offen (S7.3, 🧑 am Gerät; im Fahrplan unter „Offen am Gerät“).
- **Befunde:** keine schweren: `src/scenes` rechnet nichts (liest `index`, `vx`, `facing`), kein `Math.random()`, Schlüssel aus `MONARCH.mount.sprite`, Rückfall auf die Figur mit Log, LPC-Credits stehen in `public/sprites/CREDITS.md`. `worldRenderer.ts` 402 Rohzeilen, Oxlint (ohne Leer- und Kommentarzeilen) unter 400, grün. Abweichung S7.2 (Reittier-Sheet als einzelne Spritesheets statt Atlas) ist in der Sache unschädlich (`USED` und Atlas unverändert), Ladezeit nur 2 PNGs.
- **Tickets:** B-173 erledigt; neue Tickets: keine. `task check`, `task check:go` grün (`-race` ohne C-Compiler übersprungen).
- **Version:** v0.7.0 vorgeschlagen (Minor; v0.6.0 ist seit 2026-10-04 gesetzt: Monarch erscheint beritten, neue Darstellung im Spiel).
