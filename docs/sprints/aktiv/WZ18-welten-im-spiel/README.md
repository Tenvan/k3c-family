# WZ18 · REG · Welten im Spiel: Welt-Dateien, Welt-Wahl im Raum und Welten-Editor

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** REG
- **Reife:** Entwurf
- **Tickets:** B-488
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`B-488 › Ausgangslage`: Es gibt eine einzige Datei `data/islands.json` mit Insel 1; die Sim lädt sie einmal beim Start (`engine/sim/island_data.go`), der Spielstand Version 5 speichert die Insel als Index darin (`island_save_v5.go`). Welten als eigene Dateien, ein Editor für Welten und Inseln und eine Welt-Wahl im Raum gibt es nicht. Voraussetzung: WZ17 (Chunk-Anzahl je Stufe) ist erledigt. Sprint 1 von 2 der Spec B-488; Sprint 2 ist WZ19.

## Ziel

Welten liegen als Dateien unter `data/worlds/`; der Raum wählt beim Anlegen (Dev-Mode) eine Welt, der Spielstand speichert Welt-ID und Insel-ID (alte Stände werden migriert), mehrere Inseln sind spielbar, und der Welten Editor legt Welten und Inseln an und bearbeitet sie (`B-488 › Ziel`, ohne Chat).

**Am Ende sichtbar:** im Asset-Studio eine Welt mit zwei Inseln als Entwurf, im Spiel im Dev-Mode gewählt und mit Inselwechsel gespielt; in der CI unveränderte Golden-Daten; ein alter Spielstand lädt auf `standard`; von 🧑 abgenommen.

## Beteiligte und Zielgruppen

`B-488 › Beteiligte und Zielgruppen`: 🧑 im Workshop, in der Abnahme und für die Spec-Freigabe; Agenten je Domäne; ein getrennter Reviewer.

## Anforderungen

`B-488 › Anforderungen`: Abschnitte „Welt-Datei und Spiel“ und „Editor Welten und Inseln“. Chat, CLI und Prompt gehören nicht dazu (WZ19).

Nur für den Sprint: Reihenfolge REG → SIM → SRV → DEV; Spielstand-Migration und Protokoll-Version mit Tests zuerst; `sim.CreateOptions`/`IslandOptions` aus WZ8 bleiben getrennt (Welt-ID wird gespeichert, Erzeugungs-Optionen nicht).

## Nicht-Ziele

`B-488 › Nicht-Ziele`, besonders: Chat, Claude Code CLI und Prompt (WZ19); neue Biome, Bosse, Balancing, Ruinen und Gegnerlager (B-468); Welt-Wahl für alle Spielenden (nur Dev-Mode).

## Regeln und Einschränkungen

`B-488 › Regeln und Einschränkungen`. Jede Session eine Domäne (REG, SIM, SRV, DEV); Golden ohne Welt-Wahl byte-gleich; Spielstand-Version +1 mit Migration; keine neue Abhängigkeit in diesem Sprint; Datei ≤ 400, Funktion ≤ 60 Zeilen; mit 2 Spielern gleichzeitig.

## Beispiele

`B-488 › Beispiele` (Welt „Nordmeer“, Inselwechsel, Migration eines Version-5-Stands auf `standard`).

## Ausnahme- und Fehlerfälle

`B-488 › Ausnahme- und Fehlerfälle`: doppelte id, ungültige Welt oder Insel, Welt-Datei extern geändert (Stand-Prüfung), Welt unbekannt beim Anlegen oder Laden eines Raums.

## Akzeptanzkriterien

Aus `B-488`, Sprint 1:
- **AC-01** `B-488/AC-01` Regeltext und Glossar („Welt“, „Welt-Datei“), Format `data/worlds/<id>.json`.
- **AC-02** `B-488/AC-02` Prüfung jeder Welt-Datei; `standard.json` gleich der bisherigen `islands.json`; Golden unverändert.
- **AC-03** `B-488/AC-03` Raum mit gewählter Welt (Dev-Mode), Spielstand mit Welt-ID und Insel-ID, Migration alter Stände, Inselwechsel mit 2 Spielern.
- **AC-04** `B-488/AC-04` Protokoll mit Welt-ID, Beispiel in `docs/protocol.md`, Versionen erhöht.
- **AC-05** `B-488/AC-05` Welten Editor legt Welten und Inseln an und bearbeitet sie; Speichern nur Gültiges, atomar, format-stabil, Stand-Prüfung, REG-Hinweis.
- **AC-06** `B-488/AC-09` (Teil Hand) 🧑 nimmt eine Welt mit zwei Inseln am PC als Entwurf ab und startet sie im Spiel.
- **AC-07** `B-488/AC-10` Das Review weist jedes Kriterium nach; `task check:all` und `task check:go` grün, Golden-Diff und Migration geprüft, keine eingecheckte Welt.

## Offene Fragen

Entschieden: `B-488 › Offene Fragen` (alle). Offen für den Workshop WZ18.1: genaues Format der Welt-Datei, Behandlung der Insel-IDs über Welten hinweg, Verweis des Spielstands. Die Spec dieses Sprints (`Spec: Entwurf`) braucht die Freigabe von 🧑; Voraussetzung WZ17 erledigt.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

- WZ18.1 (REG) Workshop mit 🧑: Format der Welt-Datei, Insel-IDs, Spielstand-Verweis; Regeltext in `docs/rules/`, Glossar (AC-01).
- WZ18.2 (SIM) Welt-Dateien laden und prüfen, Sim lädt über Parameter, `islands.json` → `data/worlds/standard.json` (AC-02).
- WZ18.3 (SIM) Spielstand-Version +1 mit Welt-ID und Insel-ID, Migration alter Stände, Inselwechsel in Welten mit mehreren Inseln (AC-03).
- WZ18.4 (SRV) Raum wählt die Welt (Dev-Mode), Protokoll mit Welt-ID, `docs/protocol.md` (AC-03, AC-04).
- WZ18.5 (DEV) Welten Editor: Welten und Inseln anlegen und bearbeiten, Speichern nach den WZ8-Regeln (AC-05).
- WZ18.6 (DEV) Review: `task check:all`, Diff aller Domänen, PR (AC-07, alle).
- WZ18.7 (DEV) Abnahme durch 🧑 am PC, Agent Mensch (AC-06).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
