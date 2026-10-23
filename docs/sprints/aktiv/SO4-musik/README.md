# SO4 · CLI · Musik je Zustand

- **Status:** aktiv
- **Projekt:** SND
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-168, B-250, B-218
- **Start-Commit:** 2ea58efa
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-08, Chat, durch 🧑, Revision 2

## Ausgangslage

Audio-Kern (SO1) und Hörprobenseite (SO3) stehen; die Effekte (SO2) sind noch nicht gestartet, Musik fehlt. Die Hörprobe gibt es zusätzlich als Reiter der Sound-Seite im Asset-Studio (WZ6.4, am PC). Details in B-168.

## Ziel

Die Musik wechselt je Spielzustand mit Crossfade. Am Ende sichtbar: Die 8 Zustände Tag, Abend (= Dämmerung, Q65), Nacht, Kampf, Tiefe/Höhle, Boss, Lobby und Niederlage/Sieg haben je 1–2 eigene Stücke, Warnungen senken die Musik.

## Beteiligte und Zielgruppen

🧑 wählt Stücke mit `soundtest.html` am TV (Q16) oder im Reiter „Hörprobe“ der Sound-Seite des Asset-Studios am PC; Agent baut ein; Spieler am TV.

## Anforderungen

B-168 › Anforderungen; B-250 › Anforderungen; B-218 › Anforderungen (B-250, B-218 aus SO5, PJ3).

## Nicht-Ziele

Eigene Kompositionen, Effekte (SO2), dynamische Layer.

## Regeln und Einschränkungen

`src/scenes` rechnet nichts; CC0 oder CC-BY mit Credits (Credits-Seite B-165); eine Musik je Gerät; B nicht belegen.

## Beispiele

Nacht beginnt → Crossfade zu „Nacht“.

## Ausnahme- und Fehlerfälle

Stück fehlt → vorheriger Zustand oder Stille, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Der Zustands-Automat bildet jeden der 8 Zustände auf 1–2 Stücke ab (Q16) (B-168/AC-01).
- **AC-02** Der Wechsel Tag → Dämmerung → Nacht → Morgengrauen (Q65) ist hörbar ohne Pause und Knacken (B-168/AC-02).
- **AC-03** Ein Warn-Sound senkt die Musik ab und stellt sie wieder her (B-168/AC-03).
- **AC-04** Jede Musik-Datei hat einen Credit-Eintrag, CC-BY-Stücke stehen auf der Credits-Seite (B-168/AC-04).
- **AC-05** Die Lautstärke des Busses „Musik“ ist getrennt von den Effekten einstellbar (B-168/AC-05).
- **AC-06** `task check` ist grün.
- **AC-07** Die Lautheit der Musik ist am TV geprüft (Q16) (B-168/AC-06).
- **AC-08** Der Audio-Kern spielt ganze Dateien mit Crossfade, die Hörprobe nutzt ihn (B-250/AC-01; aus SO5 AC-01, PJ3).
- **AC-09** Die Optionen-Szene regelt auch die Lautstärke des Ambient-Busses (B-218/AC-01; aus SO5 AC-02, PJ3).

## Offene Fragen

- Welches Stück läuft im Morgengrauen? Q65 führt die Phase ein, Q16 nennt dafür keinen eigenen Zustand (B-168 › Offene Fragen). Die Phase selbst baut B-470; bis dahin gibt es `day`, `dusk` und `night`, das Morgengrauen läuft als Ende der Nacht. Entscheidet 🧑 mit den Hörproben.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SO4.1 | `SO4.1-workshop-stuecke.md` | Workshop | Mensch | offen |
| SO4.2 | `SO4.2-zustandsautomat-crossfade.md` | Umsetzung | autonom | fertig |
| SO4.3 | `SO4.3-ducking-dateien-credits.md` | Umsetzung | autonom | fertig |
| SO4.4 | `SO4.4-review.md` | Review | autonom | fertig |
| SO4.5 | `SO4.5-hoerprobe-tv.md` | Workshop | Mensch | offen |
| SO4.6 | `SO4.6-audiokern-dateien-ambient.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

- 2026-10-22, Review SO4.4 (Reviewer-Agent, Diff `2ea58efa..HEAD`): keine schweren Befunde; `task check` grün.
- AC-01, AC-04 (Datei-Credits), AC-05, AC-06, AC-08, AC-09 geprüft (SO4.2, SO4.3, SO4.6); Credits-Seite → SO6.1 (B-493).
- AC-02, AC-03, AC-07: angenommen, Validierung offen (SO4.5, TV). Stückwahl vorentschieden (SO4.1 offen, B-554, B-555).
- Sprint bleibt `aktiv`, B-168 `eingeplant`. Version: v0.20.0 vorgeschlagen (Minor: Musik im Spiel), gesetzt erst nach Bestätigung durch 🧑.
