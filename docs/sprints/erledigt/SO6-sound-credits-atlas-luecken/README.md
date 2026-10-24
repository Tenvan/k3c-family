# SO6 · PLAT, INF, CLI · Sound-Credits auf der Credits-Seite, Sound-Atlas und Sounds für die übrigen Ereignisse

- **Status:** erledigt
- **Projekt:** SND
- **Domäne:** PLAT, INF, CLI
- **Reife:** bereit
- **Tickets:** B-493, B-494, B-492, B-556, B-557, B-558
- **Start-Commit:** 7d3ee511
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-22, Revision 1

## Ausgangslage

Die Credits-Seite `lizenzen.html` nennt keine Sounds, kein Test prüft ihre Vollständigkeit (`src/tools/credits.ts`, B-493). Der Audio-Kern liest einen Sound-Atlas, ein Werkzeug, das die gewählten Sounds dazu packt, fehlt (B-494). SO2 deckt elf Ereignis-Slots ab; Münze geben, Bau-Fortschritt, Wiederbeleben durch Mitspieler, Hub-Ausbau, Skill je Klasse, Boss-Auftritt, UI-Klicks und Reiten stehen im Katalog `docs/assets/sounds.md` als Lücke (B-492).

## Ziel

Jeder Sound hat seinen Credit auf der Credits-Seite, die Effekte laden als Atlas, und jedes Ereignis aus dem Katalog hat einen Sound oder eine bestätigte Lücke. Am Ende sichtbar: `lizenzen.html` mit Sounds, Boss-Auftritt, Wiederbeleben und Hub-Ausbau hörbar am TV.

## Beteiligte und Zielgruppen

Spielende am TV; Urheber der CC-BY-Stücke; Agent in PLAT, INF und CLI; 🧑 wählt Sounds im Workshop und hört am TV ab.

## Anforderungen

B-493, B-494, B-492 › Anforderungen (Specs noch Entwurf).

## Nicht-Ziele

Musik (SO4); neue Feedback-Ereignisse im Server, außer 🧑 entscheidet so (dann eigenes Ticket SIM/SRV); Atlas für Musik.

## Regeln und Einschränkungen

Reihenfolge DEV → INF → CLI. **Abhängigkeit:** nach SO2 und SO4 (Code fertig; die Hörproben SO2.5/SO4.5 sind entkoppelt, Hardware). Lizenzen CC0 oder CC-BY, Credits in `public/audio/README.md` (Default, B-556). Der Client rechnet nichts, Auslöser kommen aus Ereignissen oder dem Snapshot (Hub-Ausbau: Anstieg von `hubLevel` im Snapshot, reiner Vergleich). Neue Aufgaben ins Taskfile des Projekts (`tools/Taskfile.yml`, Namensraum `gen`).

## Beispiele

Neue Datei unter `public/audio/kandidaten/` ohne Credit → `credits.test.ts` rot. Spielstart → eine Audio-Datei je Format für alle Effekte. Boss erscheint (`bossSpawned`) → eigener Sound.

## Ausnahme- und Fehlerfälle

Selbst erzeugte Töne (`atlas`, `probe-*`, `public/audio-test/`) brauchen keinen Credit. Fehlt der Atlas → Einzeldateien wie heute. Lücke ohne passenden Sound → bleibt mit Begründung, von 🧑 bestätigt.

## Akzeptanzkriterien

- **AC-01** Die Credits-Seite zeigt die Sounds, der Test prüft die Vollständigkeit unter `public/audio/` (B-493/AC-01, B-493/AC-02).
- **AC-02** Ein Task erzeugt den Sound-Atlas aus dem Slot-Register, der Client lädt für Effekte höchstens eine Datei je Format (B-494/AC-01, B-494/AC-02).
- **AC-03** Jede Lücke im Katalog hat Slot und gewählten Sound oder eine bestätigte Begründung, jede neue Datei einen Credit (B-492/AC-01, B-492/AC-02).
- **AC-04** Am TV lösen Boss-Auftritt, Wiederbeleben durch Mitspieler und Hub-Ausbau je ihren Sound aus (B-492/AC-03).

## Offene Fragen

Autonom vorentschieden (Claude, 2026-10-22), 🧑 bestätigt in KR4:

- Credit-Datei bleibt `public/audio/README.md` (so schreibt k3c-workbench) → B-556.
- Atlas wird gebaut und genutzt; fehlt er oder einer seiner Einträge, laden Einzeldateien wie bisher. Ob er sich lohnt, misst 🧑 am TV → B-557.
- Lücken (B-492): Auswahl aus vorhandenen Kenney-Kandidaten (CC0, keine neue Fremdquelle): Münze geben, Bau-Fortschritt, `revived`, Boss-Auftritt, Hub-Ausbau bekommen einen Slot; Skill je Klasse nutzt den gemeinsamen `sfx:skill`; UI-Klicks bleiben Lücke (kein Ereignis, Q08); Reiten bleibt ohne Einbau (Schleife je Spieler braucht eine Kern-Erweiterung) → B-558.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SO6.1 | `SO6.1-credits-seite-sounds.md` | Umsetzung | autonom | fertig |
| SO6.2 | `SO6.2-sound-atlas-task.md` | Umsetzung | autonom | fertig |
| SO6.3 | `SO6.3-workshop-luecken.md` | Workshop | Mensch | fertig |
| SO6.4 | `SO6.4-luecken-einbau.md` | Umsetzung | autonom | fertig |
| SO6.5 | `SO6.5-review.md` | Review | autonom | fertig |
| SO6.6 | `SO6.6-hoerprobe-tv.md` | Workshop | Mensch | fertig |

## Abnahme

2026-10-24 (SO6.3, SO6.6, Ralf am PC): AC-03 und AC-04 abgenommen: Sounds für Münze geben, Bau-Fortschritt, Wiederbeleben, Boss-Auftritt und Hub-Ausbau ok, Atlas bleibt. SO6.1, SO6.2, SO6.4, SO6.5 früher erledigt. B-492, B-493, B-494 erledigt. Keine neuen Tickets.
