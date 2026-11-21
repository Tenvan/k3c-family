# B-1198 · Erzadern in den Minen müssen erst freigeräumt werden: keine Steine oder Felsen bis zum Hub

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** ECO1
- **Projekt:** WIR
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

In den Minen liegen Erzadern; Steine und Felsen können heute den Weg zwischen Hub und Ader belegen (Level-Generator, `docs/rules/welten.md`). 🧑-Idee 2026-11-21: Erzadern müssen erst freigeräumt werden, also keine Steine oder Felsen auf dem Weg bis zum Hub.

## Ziel

Eine Erzader ist erst abbaubar, wenn der Weg von der Ader bis zum Hub frei ist: Felsen und Steine auf diesem Streifen müssen vorher gehauen werden (durch Handwerker mit Spitzhacke, B-1194).

## Beteiligte und Zielgruppen

🧑 entscheidet über die Dichte; Spieler räumen vor dem Abbau frei.

## Anforderungen

- Der Level-Generator (Rust) legt Felsen und Steine so, dass jede Ader hinter mindestens einem Hindernis liegt, das gehauen werden muss, aber keine unlösbare Sperre (Weg bleibt freiräumbar, deterministisch aus dem Seed).
- Abbau-Aufträge einer Ader starten erst, wenn der Weg zum Hub frei ist.
- Anzeige: gesperrte Ader ist erkennbar.
- 2+ Spieler, Level-Layout und Spielstand-Version in Rust (Go-Golden unverändert).

## Nicht-Ziele

Neue Erzarten, Balancing, Handwerker-Arbeit selbst (B-1194).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, nur Rust.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-level/src/**`, `rust/crates/k3c-sim/src/world/**`, `rust/crates/k3c-sim/src/unit/**`, `data/biomes/mine.json`
- **Abhängig von:** B-1194
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** golden | spielstand

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Test über alle festen Welten: Jede Ader hat Hindernisse auf dem Weg, aber der Weg ist freiräumbar.
- **AC-02** Abbau startet erst nach dem Freiräumen (Test).
- **AC-03** Gesperrte Ader ist im Spiel erkennbar (Prüfung 🧑).

## Offene Fragen

Defaults (autonom entscheidbar): „Weg“ = der gerade Streifen zwischen Ader und Hub-Rand auf derselben Ebene; mindestens ein, höchstens drei Hindernisse je Ader. Abhängigkeit: B-1194 (Handwerker haut Fels). 🧑 bestätigt bei der Freigabe.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
