# KS6 · SIM, PAP · Kampf-Nachträge: Engine-Export für Zahlstellen und Kronen, Papierschnitt-Assets für Kampf

- **Status:** erledigt
- **Projekt:** KSY
- **Domäne:** SIM, PAP
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

KS5 (#616) zeichnet Kampf-Elemente vorläufig und rechnet Zahl-Schlüssel und Preise der Engine nach.

## Ziel

Engine-API für Zahlstellen und Kronen, Papierschnitt-Assets für alle Kampf-Elemente.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- Hinweis (B-1143, 2026-11-19): Alle Sessions sind fertig. Sim in Rust seit RU10 bis RU22, das Go-Spiel entfällt mit RU37; die Parität prüft RU60.2, der Abschluss folgt in RU60.5. Die Pfade `engine/sim` und `native/k3c-game/*` stehen nur noch im Tag `go-referenz-final` und entsprechen `rust/crates/k3c-sim`, `rust/crates/k3c-game/src/*` (Entscheidung 013).
- Sim-Code lief im eigenen Worktree (B-925); Assets nur über `assets_*`.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-game/src/**`, `public/grafik/**`, `data/paper.json`
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Engine-Export für Zahlstellen und Kronen, Spiel nutzt ihn (`B-939`).
- **AC-02** Papierschnitt-Assets ersetzen die gezeichneten Formen (`B-940`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KS6.1 | `KS6.1-engine-export.md` | Umsetzung | autonom | fertig |
| KS6.2 | `KS6.2-kampf-assets.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-19 (Second Worker, RU60.5): Alle Sessions fertig; Go-Abnahme entfällt mit RU37. Engine exportiert Zahlstellen und Kronen-Zustände, das Spiel nutzt sie (B-939); Papierschnitt-Assets für Kampf-Elemente (B-940): Banner, Schild, Auslöser, Horn-Pfahl, Schild-Gestell und Münze stehen in `data/paper.json` und werden gelesen (RU60.2).
Offen: Assets für Krone (Zustände), Portal-HP und Münzschauer nicht belegt (B-940/AC-01, AC-02 teilweise), Sichtprüfung am TV (Rust-Abnahme-Liste). Ticket B-940 bleibt offen.
