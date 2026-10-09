# B-383 · Die Welt stellt Boss-Phase, Warnkreis, Event-Restzeit und Inselwechsel für das Protokoll bereit

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** K4
- **Projekt:** KMP
- **Erstellt:** 2026-10-09
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-09, 🧑 im Chat, mit Sprint K4 Revision 2

## Ausgangslage

K4.1 (B-154) braucht Boss, Phase, Warnkreis, Event mit Restzeit und Inselwechsel im Zustand. `net.stateOf` sieht nur `sim.World` (JSON der Welt, kein Zugriff auf das unexportierte `World.island`). Heute steht im JSON: Boss-HP, `maxHp`, `boss`, `kind`, `aoeIn` in `enemies[]` (Boss vorhanden), `cycle` (Phase, Tag, `secondsLeft`). Es fehlen: die Endboss-Phase (nur `Island.endboss.phase`, unexportiert), ein Warnkreis (kein Feld, nirgends in `engine/sim/`; nur `aoe.radius` in `data/bosses.json` und `Enemy.AoeIn`), das aktive Event (nur als Ereignis `eventStarted`/`eventEnded`; `eventActive` ist unexportiert) und der Inselwechsel (`Island.GateOpen`, `SwitchReady`, `switchProgress` nur in `Island`).

## Ziel

Die Welt spiegelt diese Werte als Felder mit `json`-Tag (wie `SkillPoints`), damit das Protokoll sie ohne Regel im Server nennen kann.

## Beteiligte und Zielgruppen

Entwickler (Server K4.1, Client K5); 🧑 entscheidet die Namen und was der Warnkreis ist.

## Anforderungen

- Endboss-Phase (ab 1) und Warnkreis des nächsten Flächenschlags (Ort und Radius in Units, Restzeit) je Boss in der Welt.
- Aktives Event der Stufe (Name wie in `data/events.json`) mit Restzeit in Sekunden.
- Inselwechsel: Wechselpunkt offen, Fortschritt 0..1, bereit; fehlt = keiner.
- Felder fehlen im JSON, wenn es sie nicht gibt (`omitempty`); `TestEventsBudgetJeTick` und Golden bleiben grün.

## Nicht-Ziele

Protokoll und Client (K4.1), Darstellung (K5), Händler-Überfall (B-381).

## Regeln und Einschränkungen

Kein rng, Spielstand unverändert (nur Spiegel); Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Endboss in Phase 2, nächster Flächenschlag in 3 s bei x 120 mit Radius 5 → Felder der Welt nennen Phase 2 und Kreis (120, 5, 3 s).

## Ausnahme- und Fehlerfälle

Kein Boss, kein Event, kein Wechsel → Felder fehlen.

## Akzeptanzkriterien

- **AC-01** Test in `engine/sim/`: JSON einer Welt mit laufendem Endboss enthält Phase und Warnkreis, in der Vollmondnacht das Event mit Restzeit, nach dem Endboss-Sieg den Wechselzustand.

## Offene Fragen

Entschieden 2026-10-09 (🧑, Vorschlag übernommen): am Boss-Gegner `phase` (Endboss, ab 1) und `warn {x, r, in}` (Units, Sekunden bis zum nächsten Flächenschlag); in der Welt `event {id, secondsLeft}` (Restzeit aus dem Zyklus, in der Sim berechnet) und `islandSwitch {open, progress, ready}`; alles `omitempty`.

## Notizen

Gefunden in K4.1; K4.1 bleibt bis dahin `blockiert`.
