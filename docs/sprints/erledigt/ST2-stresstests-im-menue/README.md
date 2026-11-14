# ST2 · CLI · Stresstests im Spielmenü (Testing)

- **Status:** erledigt
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-896
- **Start-Commit:** 59fdb136b
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: nächste autonome Welle)

## Ausgangslage

Der Phaser-Client hatte Stresstest-Seiten, die mit dem Browser-Client entfallen sind (B-848). Das native Spiel (`native/k3c-game`) hat Cheats (F9, Dev-Aktionen `spawn`, `buildAll`, `timescale` des Raums), die Leistungsanzeige F2, das Debug-Overlay F10 und Berichte nach `reports/` (Sitzungszusammenfassung), aber keinen einstellbaren Stresstest.

## Ziel

Ralf stellt im Hauptmenü unter „Testing“ einen Stresstest ein (oder wählt eine Vorgabe) und startet ihn; das Spiel läuft mit diesen Werten, zeigt F2 und F10 und schreibt am Ende einen Leistungsbericht.

**Am Ende sichtbar:** Hauptmenü-Karte „Testing“ (Entwickler-Build oder Optionen › Entwicklermodus), Einstell-Seite mit Vorgaben aus `data/stresstests.json`, nach dem Lauf `reports/native-stress-*.json` (lesbar über `report_read`).

## Beteiligte und Zielgruppen

- 🧑 Ralf startet Stresstests selbst (manuell, nicht über `sim_test`).
- Claude setzt um (autonom).

## Anforderungen

- `B-896 › Anforderungen`.
- Sprint-eigen: Mehr Gegner je Welle entstehen über die Dev-Aktion `spawn` des Raums bei jeder neuen Welle (der Raum wird nicht geändert, WR2); Truppen und Ausbau vorab über `spawn` und `buildAll`, Zeitraffer über `timescale`.

## Nicht-Ziele

- Automatische Testläufe über `sim_test` (gesperrt), Balancing-Auswertung.
- Bots für fehlende Spieler und mehr als `view.MaxPlayers` (heute 2) Spieler: Spieler ohne Gerät bekommen einen Platz, der auf ein Gerät wartet und stillsteht.
- Mehrere Inseln (B-103): „Insel“ ist die eine Insel der Standard-Welt, einstellbar ist die Stufe (Tiefe).
- Änderungen an `engine/room`, `engine/sim`, `cmd/k3c-sim`.

## Regeln und Einschränkungen

- Seiten navigieren nicht weg; alles im Spielmenü. Werte der Vorgaben in `data/stresstests.json`, nicht im Code.
- Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

- **Bereich:** `native/k3c-game/menu/**`, `native/k3c-game/game/**`, `native/k3c-game/view/**`, `native/k3c-game/local/settings*.go`, `data/stresstests.json`
- **Abhängig von:** B-888, EP1
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

- Vorgabe „Viele Gegner“ → Spiel startet mit Ausbau, Truppen und Zeitraffer der Vorgabe; jede Welle bringt zusätzlich Gegner nach der Kurve; nach n Wellen steht das Spiel, der Bericht liegt in `reports/`.
- Eigene Werte geändert → beim nächsten Öffnen von Testing stehen sie wieder da.

## Ausnahme- und Fehlerfälle

- `data/stresstests.json` fehlt oder ist kaputt → Testing zeigt eine Meldung und die eingebauten Grundwerte, kein Absturz.
- Gegnerzahl über der Obergrenze der Dev-Aktion (200) → begrenzt.
- Spiel endet vorher (Niederlage, Verlassen) → Bericht mit Ursache.

## Akzeptanzkriterien

- **AC-01** Das Menü zeigt „Testing“ mit allen genannten Einstellungen; Gamepad und Tastatur bedienen es (Test) (`B-896/AC-01`).
- **AC-02** Ein gestarteter Test läuft mit den eingestellten Werten (Go-Test: Konfiguration → Spiel und Dev-Aktionen) (`B-896/AC-02`).
- **AC-03** Am Ende liegt ein Bericht in `reports/` mit den Leistungswerten (Test) (`B-896/AC-03`).

Sichtprüfung durch 🧑 (`B-896/AC-04`) im Abnahme-Ticket des Sprints.

## Offene Fragen

keine. Entschieden mit Default (autonom, Claude, 2026-11-14): Sichtbarkeit per Einstellung „Entwicklermodus“, im Entwickler-Build (ohne Versionsangabe) immer an; „eigene Einstellungen merken“ = die zuletzt gestarteten Werte stehen in den Einstellungen.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| ST2.1 | `ST2.1-testing-menue.md` | Umsetzung | autonom | fertig |
| ST2.2 | `ST2.2-stresstest-lauf-bericht.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-14 (autonom, Claude): AC-01 siehe ST2.1 › Ergebnis, AC-02 und AC-03 siehe ST2.2 › Ergebnis. Review (k3c-reviewer, Sonnet): behoben, dass der Test schon beim Start der letzten Welle endete (Gegner liegen erst in der SpawnQueue) und dass Wellen aller Stufen zählten; die Minutenfrist nach Wanduhr samt Pausen bleibt bewusst. Abnahme-Ticket für die Sichtprüfung: B-914 (Sim und Balance gesperrt, Gen und Race nicht betroffen).
