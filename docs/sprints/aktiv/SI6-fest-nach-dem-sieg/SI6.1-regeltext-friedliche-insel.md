# SI6.1 · Regeltext Friedliche Insel und Fest (B-973)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si6
- **Abhängig von:** –
- **Tickets:** B-973
- **Kriterien:** AC-01

## Ziel

Die Regel „Friedliche Insel“ steht in `docs/rules/kampf.md`, mit Verweisen in `bosse.md` und Vermerk in `zielkorridore.md` (B-973/AC-01).

## Kontext

Quelle ist `B-973 › Anforderungen` samt Defaults in `B-973 › Offene Fragen` (Entscheidungen 🧑 2026-11-15; Defaults vorläufig). Heute: Kampfregeln in `docs/rules/kampf.md` (§ 2 Portale, § 6 Niederlage durch Kronenraub), Bosse und Events in `docs/rules/bosse.md` (§ 1: der Sieg über den Endboss erfüllt die Bedingung des Inselwechsels; § 2 Events Vollmond, Blutmond, Händler-Überfall), Korridor „Schiff fertig“ in `docs/rules/zielkorridore.md` § 6 (bis 1 Tag nach dem Sieg). In `docs/rules/` steht bisher keine Regel, dass Angriffe mit dem Sieg enden. Tabellenform der Dateien beibehalten.

## Erlaubte Dateien

- `docs/rules/kampf.md`
- `docs/rules/bosse.md`
- `docs/rules/zielkorridore.md`

## Nicht-Ziele

Code, Daten-Dateien, Wert des Korridors ändern (Balancing gesperrt), Darstellung des Fests beschreiben (B-1002).

## Schritte

1. `B-973` und die genannten Abschnitte vollständig lesen.
2. `kampf.md`: neuer Abschnitt oder Zeilen „Friedliche Insel“: ab dem Sieg über den Endgegner, alle Stufen, bis zum Inselwechsel (letzte Insel: bis Spielende); was endet (Wellen, Aggressionspool, Portale, Camps, Nacht-Events mit Gegnern), was mit lebenden Gegnern geschieht (fliehen, Krone fällt), was weiterläuft (Tag und Nacht, Händler, Wetter, Arbeit, Schiffsbau, Fest-Zustand).
3. `bosse.md` § 1 und § 2: Verweis auf die friedliche Insel.
4. `zielkorridore.md` § 6: Vermerk am Korridor „Schiff fertig“ (Wert unverändert, Neubewertung im Balancing).
5. Ausnahmefälle aus B-973; Stand-Zeilen nachziehen.

## Fertig, wenn

- [ ] AC-01: Die drei Dateien enthalten Regel, Verweise und Vermerk aus B-973.
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

- **AC-01** umgesetzt: `kampf.md` § 10 „Friedliche Insel und Fest“ (ab wann, alle Stufen, was endet, Flucht und Krone, was weiterläuft, Fest-Zustand, Ausnahmen); `bosse.md` § 1 (Sieg-Zeile) und § 2 (Events) verweisen darauf; `zielkorridore.md` § 6 Vermerk am Korridor „Schiff fertig“ (Wert unverändert). Geprüft: `check_run task:check:diff` grün.
