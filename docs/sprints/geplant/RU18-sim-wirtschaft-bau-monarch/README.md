# RU18 · SIM · Sim: Wirtschaft, Bau, Hub, Monarch, Skills

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** SIM
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Wirtschaft (576), Bau (521), Hub (503), Monarch (465), Skills (352), Händler, Beutel und Fackeln (rund 230 Zeilen).

## Ziel

Münzen, Bauen, Hub, Monarch und Skills laufen in Rust. Am Ende sichtbar: portierte Tests grün.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 12 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `engine/sim/` → `rust/crates/k3c-sim/src/{economy,merchant,building,hub,monarch,skills}/`; Dateinamen nach Leitfaden § 2.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `engine/sim/`: building_barracks (39, T), building_healer (21, T), building_outpost (40, T), building_plantation (65, T), building_repair (55, T), building_ruin (66, T), building_spell_tower (43, T), building_tavern (44, T), building_upgrades (148, T) | 521 |
| 2 | `engine/sim/`: economy (373), economy_dawn (17), economy_fountain (28), economy_pay (43) | 461 |
| 3 | `engine/sim/`: economy_view (115, T), hub_level (181, T), hub_validate (221), hub_validate_layout (101) | 618 |
| 4 | `engine/sim/`: merchant (170, T), monarch (183, T), monarch_data (131) | 484 |
| 5 | `engine/sim/`: monarch_mount (5, T), monarch_passives (146, T), skills (87, T), skills_caster (80, T), skills_healer (48, T), skills_status (50), skills_tank (41, T), skills_thief (46, T) | 503 |
| 6 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2587 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-sim/src/economy/**`, `rust/crates/k3c-sim/src/building/**`, `rust/crates/k3c-sim/src/hub/**`, `rust/crates/k3c-sim/src/monarch/**`, `rust/crates/k3c-sim/src/skills/**`, `rust/crates/k3c-sim/src/merchant/**`
- **Abhängig von:** RU14, RU15
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Die Go-Tests zu Wirtschaft und Händler sind portiert und grün.
- **AC-02** Die Go-Tests zu Bau und Hub sind portiert und grün.
- **AC-03** Monarch und Skills verhalten sich für zwei Spieler gleichzeitig wie in Go.

## Offene Fragen

Keine zum Port. **Nachzug aus Go-Sprints** (Beschluss 🧑 2026-11-16: Die Go-Engine wächst nicht weiter). Diese Arbeiten gehören nicht in den bitgleichen Port. Sie entstehen nach dem Golden-Abgleich (RU22) als neuer Rust-Code, angefangener Go-Code wird neu geschrieben statt konvertiert. Spec steht im Ticket:

- SI2.3 Doppel-Truhe in der Engine (B-968)
- SI4.3 Reittier-Katalog, Gatter, Kauf und Spielstand (B-970)
- SI5.2 Figur, Hütte, Bauplatz und Boni (B-972)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
