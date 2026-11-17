# SI3 · REG, SIM, PAP, CLI · Wetter je Welt: Regel, Engine, Assets, Anzeige

- **Status:** erledigt
- **Projekt:** KMP
- **Domäne:** REG, SIM, PAP, CLI
- **Reife:** bereit
- **Tickets:** B-969, B-999
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), von 🧑 ausgewählt. Welten unterscheiden sich bisher über Biome, Grafik und Werte-Overrides (`data/worlds/*.json`: standard, sengoku, nil, britannien), nicht über Ereignisse. Nacht-Events (Vollmond, Blutmond, Händler-Überfall) stehen in `data/events.json` und gelten für alle Welten (`engine/sim/events*.go`). Eine Welt Polar gibt es nicht.

## Ziel

Sengoku hat den Taifun, Nil die Flut: selten, einen Tag vorher angekündigt, eine Nacht lang, mit Wirkung auf Spieler und Gegner. Regel, Engine, Assets und Darstellung liegen auf `main`.

## Beteiligte und Zielgruppen

Spieler: ein oder zwei Monarchen auf der Couch. Umsetzung: Claude in autonomen Sessions, Fachprüfung durch `k3c-welten`, `k3c-grafik`, `k3c-sound`. Freigabe der Spec und Abnahme am TV: 🧑.

## Anforderungen

Siehe `B-969 › Anforderungen` (Regel und Engine, mit den Entscheidungen von 🧑 vom 2026-11-15) und `B-999 › Anforderungen` (Darstellung und Ton). Die Auswahl im Welten-Editor ist B-1003 (Sprint SI7).

## Nicht-Ziele

Wetter je Biom, frei einstellbare Wirkung je Welt, Belohnungen durch Wetter, Wetter als reine Kulisse (B-969). Schneesturm und Polar-Welt. Welten-Editor (SI7). Simulation und Balancing (gesperrt).

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/wetter.md`, `docs/rules/welten.md`, `engine/sim/**`, `data/weather.json`, `data/embed.go`, `data/worlds/sengoku.json`, `data/worlds/nil.json`, `public/grafik/hintergruende/**`, `public/audio/**`, `data/paper.json`, `data/sounds.json`, `native/k3c-game/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Sperre:** –
- Kette REG → SIM → PAP → CLI → Review; die Sessions laufen nacheinander auf `sprint/si3`.
- Sim-Code im eigenen Worktree mit eigener Dev-Workbench (B-925, Agent `k3c-sim-dev`). Der Zeitplan wird aus Seed und Tag berechnet, kein neues Feld im Spielstand; sonst anhalten und melden.
- Das Format der Welt-Datei bekommt das optionale Feld `weather`. Werte vorläufig in `data/weather.json`.
- Assets nur über `assets_*`. Die Abnahme am TV (AC-05) sammelt die Ralf-Queue; sie hält den Sprint nicht auf.

## Beispiele

Sengoku, Tag 6: Am Abend ziehen dunkle Wolken auf. Nacht 7: Taifun, Monarchen, Truppen und Gegner sind langsamer, Pfeile treiben ab. Am Morgen ist er vorbei.

## Ausnahme- und Fehlerfälle

Siehe `B-969 › Ausnahme- und Fehlerfälle` (unbekannte Art, Mond-Nächte, schmale Insel, Inselwechsel, Laden in der Wetternacht) und `B-999 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Regeltext `docs/rules/wetter.md` steht (B-969/AC-01).
- **AC-02** Katalog, Feld `weather` der Welt-Datei, Zeitplan, Taifun und Flut sind in der Engine umgesetzt und getestet (B-969/AC-02 bis AC-09).
- **AC-03** Das Asset der Wolkenbank ist angelegt und geprüft (B-999/AC-01).
- **AC-04** Das Spiel zeigt Vorboten, Taifun und Flut und spielt das Wetter-Ambient (B-999/AC-02, AC-03, AC-05).
- **AC-05** Abnahme am TV durch 🧑 (B-999/AC-04).
- **AC-06** Review: `check_run task:check` und `task:check:go` grün, schwere Befunde behoben, Abnahme-Ticket für Sim, Balance, Gen und Race angelegt (B-742).

## Offene Fragen

Keine blockierenden. Defaults in `B-969 › Offene Fragen` und `B-999 › Offene Fragen` (vorläufig, 🧑 bestätigt), vor allem: eigenes System neben den Events, Wirkung nur an der Oberfläche, Flut nimmt die Randzonen, Schneesturm erst mit der Polar-Welt, `britannien` und `standard` ohne Wetter.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SI3.1 | `SI3.1-regeltext-wetter.md` | Umsetzung | autonom | fertig |
| SI3.2 | `SI3.2-engine-wetter.md` | Umsetzung | autonom | verworfen |
| SI3.3 | `SI3.3-asset-wolkenbank.md` | Umsetzung | autonom | fertig |
| SI3.4 | `SI3.4-anzeige-wetter.md` | Umsetzung | autonom | verworfen |
| SI3.5 | `SI3.5-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Review SI3.5 (2026-11-17):
- Erfüllt: Regeltext (SI3.1) und Asset Wolkenbank (SI3.3).
- Nicht erfüllt: Engine (SI3.2) und Anzeige (SI3.4). Beide Sessions sind mit dem Go-Stopp verworfen.
- Offen: Abnahme am TV durch 🧑.
- `task:check`: grün, nachdem der Test-Filter um `wetter-*` erweitert wurde.
- `task:check:go`: rot nur durch B-1030 (Detvec), behoben in #673.
- Sim, Balance, Gen und Race laufen im Abnahme-Ticket B-1031.
