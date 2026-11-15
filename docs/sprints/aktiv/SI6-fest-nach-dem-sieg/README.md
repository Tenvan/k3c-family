# SI6 · REG, PAP, SIM, CLI · Fest nach dem Sieg und friedliche Insel: Regel, Assets, Engine, Anzeige, Festmusik

- **Status:** aktiv
- **Projekt:** KMP
- **Domäne:** REG, PAP, SIM, CLI
- **Reife:** bereit
- **Tickets:** B-973, B-1002
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), von 🧑 ausgewählt. Der Sieg über den Endgegner setzt `Island.EndbossDefeated` und öffnet den Wechselpunkt am Steg (`engine/sim/island_switch.go`); gefeiert wird nicht, und in `docs/rules/` steht keine Regel, dass Angriffe mit dem Sieg enden (gesucht 2026-11-15, Verhalten im Code ungeprüft).

## Ziel

Mit dem Sieg über den Endgegner enden alle Angriffe auf der Insel, und im Hub beginnt ein Fest im Stil der Welt mit Festmusik; es dauert, bis das Schiff ablegt. Regel, Assets, Engine, Anzeige und Musik liegen auf `main`.

## Beteiligte und Zielgruppen

Spieler: ein oder zwei Monarchen auf der Couch. Umsetzung: Claude in autonomen Sessions, Fachprüfung durch `k3c-reviewer`, `k3c-grafik`, `k3c-sound`. Freigabe der Spec und Abnahme am TV: 🧑.

## Anforderungen

Siehe `B-973 › Anforderungen` (Regel und Engine, mit den Entscheidungen von 🧑 vom 2026-11-15) und `B-1002 › Anforderungen` (Darstellung und Festmusik).

## Nicht-Ziele

Ein Fest für alle Welten, Fest nur für eine Nacht, kurzer Jubel-Moment ohne Zustand, Belohnungen durch das Fest (B-973). Nordlicht und Polar-Welt. Neubewertung des Korridors „Schiff fertig“ (Balancing, gesperrt).

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/kampf.md`, `docs/rules/bosse.md`, `docs/rules/zielkorridore.md`, `engine/sim/**`, `testdata/golden/**`, `public/grafik/objekte/**`, `public/audio/**`, `data/paper.json`, `data/native.json`, `data/sounds.json`, `native/k3c-game/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Sperre:** golden
- Kette REG → PAP → SIM → CLI (Anzeige) → CLI (Festmusik) → Review; die Sessions laufen nacheinander auf `sprint/si6`.
- Sim-Code im eigenen Worktree mit eigener Dev-Workbench (B-925, Agent `k3c-sim-dev`). Gewollte Regeländerung: `task go:golden` nur in SI6.3 mit Begründung im Commit. Kein neues Feld im Spielstand (Frieden aus `EndbossDefeated`).
- Keine Simulation, kein Balancing; keine neue Abhängigkeit für die Musik.

## Beispiele

Sengoku, Insel 1: Der Endgegner fällt. Im Hub steigen Laternen auf, die Bürger tanzen, Festmusik spielt. Die Monarchen bauen das Segel fertig und beladen das Schiff; mit dem Ablegen endet das Fest.

## Ausnahme- und Fehlerfälle

Siehe `B-973 › Ausnahme- und Fehlerfälle` (getragene Krone, Sieg in anderer Stufe, letzte Insel, Laden nach dem Sieg) und `B-1002 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Regel „Friedliche Insel“ steht in `kampf.md`, mit Verweisen in `bosse.md` und Vermerk in `zielkorridore.md` (B-973/AC-01).
- **AC-02** Die Fest-Assets sind angelegt und geprüft (B-1002/AC-01).
- **AC-03** Frieden und Fest-Zustand sind in der Engine umgesetzt und getestet, Golden begründet (B-973/AC-02 bis AC-08).
- **AC-04** Das Spiel zeigt Jubel, Tanz und den Himmels-Effekt je Welt (B-1002/AC-02, AC-03, AC-06).
- **AC-05** Die Festmusik spielt von Sieg bis Abfahrt (B-1002/AC-04).
- **AC-06** Abnahme am TV durch 🧑 (B-1002/AC-05).
- **AC-07** Review: `check_run task:check` und `task:check:go` grün, schwere Befunde behoben, Abnahme-Ticket für Sim, Balance, Gen und Race angelegt (B-742).

## Offene Fragen

Keine blockierenden. Defaults in `B-973 › Offene Fragen` und `B-1002 › Offene Fragen` (vorläufig, 🧑 bestätigt), vor allem: Frieden gilt für alle Stufen, restliche Gegner fliehen, Wetter und Händler laufen weiter, `standard` und `britannien` feiern ohne Himmels-Effekt, Nordlicht erst mit der Polar-Welt.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SI6.1 | `SI6.1-regeltext-friedliche-insel.md` | Umsetzung | autonom | offen |
| SI6.2 | `SI6.2-assets-fest.md` | Umsetzung | autonom | offen |
| SI6.3 | `SI6.3-engine-friedliche-insel.md` | Umsetzung | autonom | offen |
| SI6.4 | `SI6.4-anzeige-fest.md` | Umsetzung | autonom | offen |
| SI6.5 | `SI6.5-festmusik.md` | Umsetzung | autonom | offen |
| SI6.6 | `SI6.6-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

–
