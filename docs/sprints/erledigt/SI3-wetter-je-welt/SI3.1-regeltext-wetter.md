# SI3.1 · Regeltext Wetter je Welt (B-969)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si3
- **Abhängig von:** –
- **Tickets:** B-969
- **Kriterien:** AC-01

## Ziel

Die neue Regeldatei `docs/rules/wetter.md` beschreibt Wetter-Katalog, Zeitplan, Vorboten, Taifun, Flut und den geplanten Schneesturm (B-969/AC-01).

## Kontext

Quelle ist `B-969 › Anforderungen` samt Defaults in `B-969 › Offene Fragen` (Entscheidungen 🧑 2026-11-15; Defaults vorläufig). Heute: Nacht-Events in `docs/rules/bosse.md` § 2 und `data/events.json` (Vollmond jede 7., Blutmond jede 13. Nacht), Welten und Welt-Datei in `docs/rules/welten.md` (nur lesen; das Feld `weather` trägt SI3.2 dort ein). Welten: standard, sengoku, nil, britannien; eine Welt Polar gibt es nicht. Aufbau der Regeldateien an `docs/rules/bosse.md` ablesen: Kopf mit Stand, Tabellen (Regel, Begründung, Daten, Zielkorridor), Abschnitt „Offen und Annahmen“.

## Erlaubte Dateien

- `docs/rules/wetter.md`

## Nicht-Ziele

Code, Daten-Dateien, `welten.md`, Wetter je Biom, Belohnungen durch Wetter, Werte abstimmen.

## Schritte

1. `B-969` vollständig lesen.
2. `docs/rules/wetter.md` anlegen: Grundsatz (Katalog, die Welt wählt eine Art oder keins, trifft Spieler und Gegner gleich), Zeitplan (aus Seed und Tag, `everyNights`, `firstNight`, Mond-Nächte ausweichen, Vorboten am Vortag, Dauer eine Nacht), Geltung (Zeitplan je Insel, Wirkung nur an der Oberfläche).
3. Taifun und Flut mit Wirkung und Datenfeldern `data/weather.json`; Schneesturm als geplant (mit der Polar-Welt).
4. Zuordnung: `sengoku` Taifun, `nil` Flut, `standard` und `britannien` keins.
5. Ausnahmefälle aus B-969 und Abschnitt „Offen und Annahmen“ mit den Defaults; alle Zahlen als vorläufig.

## Fertig, wenn

- [ ] AC-01: `docs/rules/wetter.md` existiert in Tabellenform mit allen Regeln aus B-969.
- [ ] `check_run task:check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01: umgesetzt, `docs/rules/wetter.md` mit Katalog, Zeitplan, Vorboten, Taifun, Flut, geplantem Schneesturm, Zuordnung der Welten und „Offen und Annahmen“; alle Zahlen als vorläufig markiert.
- `check_run task:check:diff` grün (exit 0, 13 s).
