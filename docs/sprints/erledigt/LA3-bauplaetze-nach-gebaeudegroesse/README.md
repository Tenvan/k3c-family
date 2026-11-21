# LA3 · SIM · Bauplätze im Hub richten sich nach der Gebäudegröße

- **Status:** erledigt
- **Projekt:** RSU
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** B-1182
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 Ralf, 2026-11-21

## Ausgangslage

Die festen Bauplätze im Hub stehen an festen Positionen und kennen die Breite der Gebäude nicht; Gebäude überlappen sich (🧑 2026-11-21, B-1182). Seit den Größenstufen (LA1.1) und je Welt sind die Breiten verschieden.

## Ziel

Die Plätze richten sich nach der Gebäudebreite der Welt, nichts überlappt. Am Ende sichtbar: Hub jeder festen Welt am PC ohne überlappende Gebäude.

## Beteiligte und Zielgruppen

Hauptrechner setzt um (Sim und Golden betroffen); 🧑 entscheidet über alte Spielstände und nimmt am PC ab.

## Anforderungen

`B-1182 › Anforderungen`.

## Nicht-Ziele

Neue Gebäude, Grafik, Balancing.

## Regeln und Einschränkungen

Neue Mechanik nur in Rust (`k3c-sim`, Entscheidung 013); Go-Golden nie zugunsten von Rust ändern; Spielstand-Format prüfen; 2 Spieler; deterministisch.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `data/hub.json`, `rust/crates/k3c-sim/**`, `rust/crates/k3c-level/**`
- **Abhängig von:** LA1
- **Braucht 🧑:** Entscheidung | Abnahme
- **Sperre:** golden | spielstand

## Beispiele

Schmiede 4,8 Units, Taverne 6 Units: Abstand der Mittelpunkte mindestens 5,4 Units plus Lücke.

## Ausnahme- und Fehlerfälle

Zu wenig Platz im Hub: Reihenfolge und Mauern behalten Vorrang, Meldung im Log.

## Akzeptanzkriterien

- **AC-01** (B-1182/AC-01) Test: in jeder festen Welt überlappen sich keine zwei gebauten Plätze (Breiten aus den Assets).
- **AC-02** (B-1182/AC-02) Abnahme 🧑 am PC.

## Offene Fragen

Keine offen. 🧑 2026-11-21: Alte Spielstände sind egal, keine Migration, keine Kompatibilität; die Plätze folgen der neuen Abstandsregel, alte Spielstände dürfen bei der neuen Regel brechen oder neu geladen werden.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| LA3.1 | `LA3.1-bauplatz-abstaende.md` | Umsetzung | autonom | fertig |
| LA3.2 | `LA3.2-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-21, Hauptrechner: AC-01, AC-02 umgesetzt und geprüft (siehe LA3.1 und LA3.2 › Ergebnis); `task check` grün, Go-Tests grün, `golangci-lint` fehlte lokal. Behobene Befunde: Treppen- und Zone-1-Regel (Review LA3.1), Lint-Fehler im Workbench-Frontend. Neues Abnahme-Ticket: B-1189 (PC-Sicht, Race, Lint, rote Rust-Tests).
