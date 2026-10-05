# S2 · SRV · Protokoll für Skills, Speichern beim Verlassen, Spielmetrik

- **Status:** erledigt
- **Domäne:** SRV
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-123, B-147, B-150, B-176
- **Start-Commit:** 7e2b70c
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1, durch 🧑; umfasst B-123, B-147, B-150, B-176; bestätigt die Vorschläge der Planung in den Sessions (Protokoll v4 einmal, Aktionsliste nur am Ort des Spielers, Speichern bei jedem Verlassen); der Rest von Q10 (60-s-Takt, Tagesanbruch, HUD „gesichert“) ist B-186

## Ausgangslage

Protokoll v3 kennt Bewegung, Sprint und Zahlen, aber keine Skills, keinen Schlag, keinen Pool und keine Aktionsliste. Der Spielstand wird nur zu Tagesanbruch geschrieben, und es gibt keinen Spielmetrik-Report. Voraussetzung: S1 (Sim für Schlag und Skills).

## Ziel

Der Client kann Schlag, Skills, Pool und gültige Aktionen je Spieler über das Protokoll bedienen; der Server speichert beim Verlassen und schreibt je Sitzung einen Spielmetrik-Report. Am Ende sichtbar: neue Felder in `docs/protocol.md` mit Beispielen unter `testdata/protocol/`, Spielstand nach Trennung mitten in der Nacht, ein Report in `reports/`.

## Beteiligte und Zielgruppen

Entwickler (Client und Server), Betreiber des Pi, 🧑 (Speicherzeitpunkt, Metrik-Umfang).

## Anforderungen

B-123, B-147 und B-150 › Anforderungen.

## Nicht-Ziele

Darstellung im Client (S3), Protokoll für Berufe, Händler und Lager (W5), Backup außerhalb des Pi und Rotation (B-142).

## Regeln und Einschränkungen

`docs/protocol.md` ist die Quelle; Protokolländerung nur in einer eigenen Session, beide Enden gemeinsam. Spielstand-Änderung mit Versionssprung und Fixture (B-137); deterministisch, Messung ändert den Spielverlauf nicht. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Ein Gerät sendet Skill-Slot 1 → der Server wendet ihn an und meldet Abklingzeit im Snapshot; das letzte Gerät trennt mitten in der Nacht → Spielstand liegt in `saves/`, Report in `reports/`.

## Ausnahme- und Fehlerfälle

Ungültiger Slot oder Beruf → `bad_request`; älterer Client → `version`; Speichern oder Report schlägt fehl → Log, vorheriger Stand bleibt.

## Akzeptanzkriterien

- **AC-01** `docs/protocol.md` beschreibt die neuen Felder, `testdata/protocol/` hat Beispiele, beide Enden parsen sie (B-123/AC-01).
- **AC-02** Der Server prüft Eingaben und lehnt ungültige ab; Protokollversion erhöht, ältere Clients erhalten `version` (B-123/AC-02, B-123/AC-03).
- **AC-03** Verlassen und Trennen des letzten Geräts speichern; ein Schreibfehler lässt den alten Stand unverändert (B-147/AC-01, B-147/AC-02).
- **AC-04** Der Spielstand nennt Zeitpunkt und Tag/Nacht/Stufe, die Liste liefert sie; ein Stand ohne diese Felder lädt weiter (B-147/AC-03, B-147/AC-04).
- **AC-05** Ein Raumlauf erzeugt einen Spielmetrik-Report mit allen Mindestfeldern, ohne Namen, und beeinflusst den Golden-Hash nicht (B-150/AC-01, B-150/AC-02, B-150/AC-03).
- **AC-06** Das Report-Schema ist beschrieben und `task check:go` ist grün (B-150/AC-04).
- **AC-07** Ein Gerät bekommt Level und Zustand jeder Stufe, in der ein lokaler Spieler steht; Beispiele, Tests und Benchmark sind belegt (B-176/AC-01, B-176/AC-02, B-176/AC-03, B-176/AC-04).

## Offene Fragen

Mit der Freigabe entschieden: Aktionsliste nur am Ort des Spielers (B-123), Protokoll v4 einmal (B-176), Speichern bei jedem Verlassen (Q10, Teil); 60-s-Takt, Tagesanbruch und HUD „gesichert“ aus Q10 folgen mit B-186. Metrik-Umfang wie Q12.

## Sessions

Reihenfolge wie die Nummern. Die Protokollversion steigt einmal (S2.1 auf 4, S2.4 bleibt dabei, Vorschlag); die Spielstand-Version steigt einmal (S2.2). Vorschläge der Planung stehen in den Sessions unter „Entscheidungen dieser Session“ und gelten erst mit der Freigabe.

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| S2.1 | `S2.1-protokoll-skills.md` | Umsetzung | autonom | fertig |
| S2.2 | `S2.2-speichern-verlassen.md` | Umsetzung | autonom | fertig |
| S2.3 | `S2.3-spielmetrik-report.md` | Umsetzung | autonom | fertig |
| S2.4 | `S2.4-stufen-je-spieler.md` | Umsetzung | autonom | fertig |
| S2.5 | `S2.5-review.md` | Review | autonom | fertig |

## Abnahme

2026-10-05, Review S2.5 (reviewer-s25, nicht der Umsetzer). AC-01, AC-02: S2.1 (Beruf und Tausch aus B-123/AC-02 verschoben → B-281); AC-03, AC-04: S2.2; AC-05, AC-06: S2.3; AC-07: S2.4 (Ereignisse 10,5 Byte je Tick und Gerät, im Budget Q08).
Keine schweren Befunde im Diff (S2.4) und in den Stichproben (S2.1–S2.3); neue Tickets B-281 (Rest B-123), B-282 (Flake im Lasttest-Test).
Bestätigung durch 🧑 offen: B-278 und B-279 wurden in S2.4 mit `Spec: rückwirkend` ohne Freigabe erledigt und archiviert.
Version: v0.7.0 vorgeschlagen (Minor: Protokoll v4, Speichern beim Verlassen und Spielmetrik-Report wirken im Server; aktuell v0.6.0, die Nummer vergibt 🧑 beim Release); nicht gesetzt (Bestätigung durch 🧑 offen).
