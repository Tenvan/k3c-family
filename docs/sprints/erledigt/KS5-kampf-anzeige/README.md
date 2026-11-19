# KS5 · CLI · Anzeige im Spiel: Portal-HP, Krone, Münzverlust, Schild, Banner, Auslöser

- **Status:** erledigt
- **Projekt:** KSY
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

KS2–KS4 gemergt (#611, #613, #615): Portale mit HP, Krone und Gold, Gruppen und Banner; Zahlziele `CombatSpots` ohne Anzeige.

## Ziel

Das native Spiel zeigt Portal-HP, Krone, Münzverlust, Schild, Banner und Auslöser.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- Hinweis (B-1143, 2026-11-19): Die Session ist fertig. Das Go-Spiel entfällt mit RU37; die Parität der Anzeige im Rust-Spiel prüft RU60.2 (nach RU52.3), der Abschluss folgt in RU60.5. Die Pfade `native/k3c-game/*` stehen nur noch im Tag `go-referenz-final` und entsprechen `rust/crates/k3c-game/src/*` (Entscheidung 013).
- Grafik nur über `assets_*`; fehlt ein Asset, vorläufig gezeichnet und gemeldet.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/crates/k3c-game/src/**`, `data/paper.json`
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Portal-HP, Krone (am Kopf, fliegend, liegend, getragen), Münzverlust, Schild, Banner und Auslöser sichtbar, 2 Spieler (`B-934/AC-01`, `AC-02`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KS5.1 | `KS5.1-kampf-anzeige.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-19 (Second Worker, RU60.5): Alle Sessions fertig; Go-Abnahme entfällt mit RU37. Anzeige im Spiel (Portal-HP, Krone, Münzverlust, Schild, Banner, Auslöser, B-934): HUD-Tests im Rust-Spiel vorhanden (RU60.2: Skill-Menü, Overlay, Börse, Zahlstellen), Abgleich nach Namen vollständig gedeckt bis auf Go-spezifische Tests.
Offen: Sichtprüfung am TV (Rust-Abnahme-Liste); Befund: Papierschnitt-Assets für Krone (Zustände), Portal-HP und Münzschauer unter `data/paper.json` nicht gefunden (B-940).
