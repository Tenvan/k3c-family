# NG9 · CLI · Natives Spiel: Zusammenführung, Messung und Abnahme

- **Status:** geplant
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-802, B-786
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

NG2–NG8 haben ihre Pakete einzeln gebaut und getestet. Zusammenspiel, Messung und Abnahme fehlen; die Mini-PC-Prüfung wartet seit NV2 (B-786). Einzelheiten: B-802, B-786.

## Ziel

Das native Spiel läuft vom Start bis zum Spielstand durch, ist gemessen, als Paket gebaut und von 🧑 am PC abgenommen; der Mini-PC-Termin ist vorbereitet. Am Ende sichtbar: `task native:game:e2e` grün, Messbericht unter `reports/`, Paket unter `.work/`, Abnahme in den Notizen von B-802.

## Beteiligte und Zielgruppen

🧑 Ralf (Abnahme PC und Mini-PC), Familie, Claude (Zusammenführung, Messung).

## Anforderungen

`B-802 › Anforderungen`, `B-786 › Anforderungen` (Mini-PC, mit dem Mini-PC-Teil von `B-780/AC-01`).

## Nicht-Ziele

`B-802 › Nicht-Ziele`; Abschalten von Server und Browser-Client.

## Regeln und Einschränkungen

Leistungsmessung der Darstellung je einmal; keine Sim-, Balance- und Lasttest-Läufe. Hardware entkoppelt: Die Mensch-Sessions sind keine Abhängigkeit des Reviews; der Sprint bleibt aktiv, bis sie fertig sind. Abschluss des Projekts: Abnahme-Ticket NAT (B-742) legt das Review an.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `native/k3c-game/game/**`, `native/k3c-game/e2e/**`, `native/Taskfile.yml`, `native/measure.ps1`
- **Abhängig von:** NG2, NG3, NG4, NG5, NG6, NG7, NG8
- **Braucht 🧑:** Freigabe, Abnahme, Gerät
- **Sperre:** taskfile

## Beispiele

`B-802 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-802 › Ausnahme- und Fehlerfälle`. Mini-PC nicht verfügbar → NG9.5 bleibt offen, der Sprint bleibt aktiv (Hardware entkoppelt).

## Akzeptanzkriterien

- **AC-01** Ende-zu-Ende-Test ohne Fenster grün (`B-802/AC-01`).
- **AC-02** Funktionsabgleich mit `src/scenes/`, Lücken als Tickets (`B-802/AC-02`).
- **AC-03** Messbericht Entwicklungs-PC und Paket für den Mini-PC (`B-802/AC-03`).
- **AC-04** Abnahme 🧑 am PC zu zweit (`B-802/AC-04`).
- **AC-05** Sichtprüfung und Messung am Mini-PC mit Entscheidung (`B-786/AC-01`, Mini-PC-Teil von `B-780/AC-01`).

## Offene Fragen

- Mini-PC, Auflösung, Gamepads, Weg für Paket und Bericht (`B-786 › Offene Fragen`). Entscheidet 🧑, sobald der Mini-PC feststeht.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG9.1 | `NG9.1-ende-zu-ende.md` | Umsetzung | autonom | offen |
| NG9.2 | `NG9.2-messung-paket.md` | Umsetzung | autonom | offen |
| NG9.3 | `NG9.3-review.md` | Review | autonom | offen |
| NG9.4 | `NG9.4-abnahme-pc.md` | Review | Mensch | offen |
| NG9.5 | `NG9.5-mini-pc.md` | Review | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
