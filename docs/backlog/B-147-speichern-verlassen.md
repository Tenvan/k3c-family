# B-147 · Der Server speichert beim Verlassen und wenn das letzte Gerät getrennt ist, der Spielstand zeigt seinen Speicherstand

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** S2
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint S2

## Ausgangslage

Der Spielstand wird heute nur zu festen Zeitpunkten (Tagesanbruch) gespeichert; Kinder brechen mitten in der Nacht ab und verlieren den Fortschritt (Plan Lücke 21). Speichern liegt in `engine/store/saves.go` und `engine/sim/island_save.go`, Räume verwaltet `engine/room/manager.go`.

## Ziel

Der Raum speichert, wenn ein Gerät den Raum verlässt und wenn das letzte Gerät getrennt ist; der Spielstand nennt, wann und bei welcher Nacht/Stufe gespeichert wurde und kann angezeigt werden. Nutzen: Ein Abbruch mitten in der Nacht kostet kaum Fortschritt.

## Beteiligte und Zielgruppen

Spieler (Kinder, Abbruch jederzeit), Betreiber des Pi; 🧑 entscheidet, wann gespeichert wird.

## Anforderungen

- Speichern beim Verlassen eines Geräts und beim Trennen des letzten Geräts (Raum bleibt oder wird geschlossen laut `engine/room/manager.go`); Atomarität wie bisher (`engine/store/atomic.go`).
- Der Spielstand trägt Zeitpunkt und Angabe Tag/Nacht und Stufe; die Spielstand-Liste (laut `docs/protocol.md`) liefert sie für die Anzeige.
- Format-Änderung nur mit Versionssprung und Fixture nach der Migrationsregel (B-137); alte Stände laden weiter.
- Das Speichern darf den Tick nicht spürbar verlangsamen (Zielwert in der Spec festlegen, Messung im Log oder Test).

## Nicht-Ziele

Zusätzlicher Autospeicher-Takt, Backup außerhalb des Pi (B-142), Anzeige im Client.

## Regeln und Einschränkungen

Schichtgrenzen aus `docs/arbeitsweise.md` (Protokoll als eigene Session), deterministisch (`engine/rng`), 2+ Spieler, Datei ≤ 400 Zeilen, Funktion ≤ 60; `saves/` bleibt die einzige Ablage.

## Beispiele

Zwei Geräte spielen in der Nacht, Gerät 2 trennt → Speicherung; Gerät 1 trennt → Speicherung und Raum schließt; beim nächsten Laden liegt der Stand der Trennung vor, nicht der von Tagesanbruch.

## Ausnahme- und Fehlerfälle

Speichern schlägt fehl (Platte voll) → Fehler im Log, der vorherige Stand bleibt unverändert; der Raum fährt trotzdem herunter. Absturz des Servers ohne Trennung → der letzte erfolgreiche Stand gilt.

## Akzeptanzkriterien

- **AC-01** Test in `engine/room/`: Verlassen eines Geräts und Trennen des letzten Geräts schreiben einen Spielstand; er lädt wieder.
- **AC-02** Test: Schlägt das Schreiben fehl, bleibt der vorherige Stand lesbar und unverändert.
- **AC-03** Der Spielstand enthält Zeitpunkt und Tag/Nacht/Stufe; die Liste der Spielstände liefert sie (Test in `engine/net/`, `docs/protocol.md` aktualisiert).
- **AC-04** Ein Stand ohne diese Felder lädt weiter (Fixture), `task check:go` grün.

## Offene Fragen

Mit der Freigabe von S2 (2026-10-03): Speichern bei jedem Verlassen in S2; 60-s-Takt, Tagesanbruch und HUD „gesichert“ (Q10) folgen mit B-186.

## Notizen

Aus Plan Phase 1 (S5) und Lücke 21.
