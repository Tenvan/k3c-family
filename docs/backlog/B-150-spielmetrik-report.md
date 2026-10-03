# B-150 · Der Server schreibt je Sitzung einen Spielmetrik-Report nach reports/

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** S2
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Playtests werden nur über Logs ausgewertet (Plan Lücke 19). `reports/` nimmt bisher Gamepad-Berichte auf (`engine/store/reports.go`); Raum-Ereignisse loggt `engine/room/logging.go`.

## Ziel

Am Ende jeder Sitzung liegt ein Report in `reports/` mit den Kennzahlen Tod durch was, Nacht überlebt, Zeit bis zum ersten Bau und weiteren Zahlen laut Frage Q12. Nutzen: Spieleabend (B-151) und Balancing (B-160) haben Messwerte statt Gefühl.

## Beteiligte und Zielgruppen

Entwickler und 🧑 werten aus; Spieler merken nichts davon; 🧑 legt fest, was erfasst wird.

## Anforderungen

- Der Raum sammelt Kennzahlen aus Sim-Ereignissen, ohne die Simulation zu beeinflussen (nur lesen), und schreibt sie beim Raumende als JSON nach `reports/` über `engine/store`.
- Mindestumfang: Dauer, Spieleranzahl und Grad, erreichte Nacht und Stufe, überlebt ja/nein je Nacht, Todesursache je Monarch (Gegnerart), Zeit bis zum ersten Bau, Gold am Ende. Weitere Felder nach Q12.
- Kein Personenbezug: keine Namen, nur Slot-Nummern.
- Das Report-Schema ist versioniert und in `docs/` beschrieben (Ort in der Spec festlegen); Rotation der Reports regelt B-142.

## Nicht-Ziele

Auswertungs-Werkzeug und Abgleich mit dem Simulator (B-160), Fragebogen (B-151).

## Regeln und Einschränkungen

Deterministisch: Messung darf den Spielverlauf nicht ändern (Test: identischer Golden-Hash mit und ohne Sammler). Schichtgrenzen aus `docs/arbeitsweise.md`, Datei ≤ 400 Zeilen, Funktion ≤ 60, 2+ Spieler.

## Beispiele

Zwei Spieler spielen drei Nächte, der Raum schließt → der Report nennt Nacht 3 erreicht, Tod durch „Bogenschütze“ bei Slot 2, erster Bau nach 45 s.

## Ausnahme- und Fehlerfälle

Schreiben schlägt fehl → Fehler im Log, der Raum fährt trotzdem herunter. Raum ohne Spielzeit (sofort verlassen) → kein Report.

## Akzeptanzkriterien

- **AC-01** Test: Ein Raumlauf mit festem Seed erzeugt einen Report mit allen Mindestfeldern und erwarteten Werten.
- **AC-02** Test: Der Golden-Hash der Simulation ist mit und ohne Sammler identisch.
- **AC-03** Test: Der Report enthält keine Namen, nur Slot-Nummern.
- **AC-04** Das Schema ist beschrieben, `task check:go` grün.

## Offene Fragen

Welche Kennzahlen darüber hinaus erfasst werden: 🧑, `docs/fragenkatalog.md Q12`.

## Notizen

Aus Plan Phase 1 (P1) und Lücke 19. Verwandt: B-160, B-142.
