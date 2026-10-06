# H1 · SIM · Holz-Startvorrat

- **Status:** erledigt
- **Domäne:** SIM
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-177
- **Start-Commit:** fc8aa19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1; umfasst B-177; Startvorrat 100 Holz bestätigt

## Ausgangslage

Der Insel-Vorrat startet leer, alle Gebäude der Hub-Stufe 1 kosten Holz plus Gold; ein Spielstart ohne Holzfällen kann nichts bauen (B-177 › Ausgangslage). Voraussetzung: F2 abgeschlossen (`task golden:update`, Spielstand-Fixtures).

## Ziel

Eine neue Insel startet mit 100 Holz aus den Daten. Am Ende sichtbar: Go-Tests (neue Insel, geladener Stand, Datenprüfung, Bauen ohne Holzfällen), aktualisierte Golden-Daten, Regel in den Regelwerken.

## Beteiligte und Zielgruppen

Spieler am TV und am Handy; 🧑 gibt die Spec frei und bestätigt die Zahl.

## Anforderungen

B-177 › Anforderungen.

## Nicht-Ziele

Gold-only-Gebäude, geänderte Baukosten, Balancing (B-155).

## Regeln und Einschränkungen

Domäne SIM (`engine/sim/`, `data/hub.json`). **Domänen-Ausnahme (Freigabe dieser Spec erlaubt sie):** H1.1 darf die Regel in `docs/rules/materialien-gebaeude.md` und den Verweis in `docs/game-design.md` (REG) nachführen. Golden-Updates nur mit `task golden:update` und Begründung im Commit. Der Sprint läuft in der SIM-Bahn nach F2 und vor F3.

## Beispiele

Neue Insel → Holz 100, zwei Mauern sofort bezahlbar. Ältere Spielstände behalten ihren Vorrat.

## Ausnahme- und Fehlerfälle

Startvorrat über dem Lager-Maximum → Datenprüfung scheitert; Golden-Lauf lässt sich nicht erzeugen → Session `blockiert` mit Ticket.

## Akzeptanzkriterien

- **AC-01** Eine neue Insel hat Holz 100 und sonst 0 aus `data/hub.json`, deterministisch (B-177/AC-01).
- **AC-02** Ein geladener Spielstand behält seinen Vorrat (B-177/AC-02).
- **AC-03** Eine Datenprüfung begrenzt den Startvorrat auf das Lager-Maximum (B-177/AC-03).
- **AC-04** Beide Mauern und ein Turm sind ohne Holzfällen baubar (B-177/AC-04).
- **AC-05** Golden-Daten aktualisiert, `task check:go` grün (B-177/AC-05).
- **AC-06** Regel und Verweis stehen in `docs/rules/materialien-gebaeude.md` und `docs/game-design.md` (B-177/AC-06).

## Offene Fragen

Höhe des Startvorrats (100 Holz ist ein Vorschlag): 🧑 bestätigt mit der Freigabe.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| H1.1 | `H1.1-startvorrat-umsetzen.md` | Umsetzung | autonom | fertig |
| H1.2 | `H1.2-review.md` | Review | autonom | fertig |

## Abnahme

2026-10-03 (H1.2): `task check` und `task check:go` grün, Diff `fc8aa19..origin/main` geprüft, keine schweren Befunde.
AC-01 bis AC-06: Nachweise im Ergebnis von H1.1 (`island_start_test.go`, Regel in `docs/rules/materialien-gebaeude.md` und `docs/game-design.md`).
Keine behobenen Befunde, keine neuen Tickets.
Version: v0.4.0 gesetzt (2026-10-03, auf `80dfb6b`, von 🧑 bestätigt; Minor: neue Insel startet mit 100 Holz).
