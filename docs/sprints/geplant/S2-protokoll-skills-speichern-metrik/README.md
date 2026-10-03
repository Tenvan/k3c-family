# S2 · SRV · Protokoll für Skills, Speichern beim Verlassen, Spielmetrik

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-123, B-147, B-150
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

## Offene Fragen

Umfang der Aktionsliste: B-123 › Offene Fragen; Speicherzeitpunkt `docs/fragenkatalog.md Q10`; Metrik-Umfang `docs/fragenkatalog.md Q12`.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- S2.1 Protokoll: Eingaben und Zustand für Schlag, Skills, Pool, gültige Aktionen, `docs/protocol.md`, `testdata/protocol/`, Versionssprung (AC-01, AC-02).
- S2.2 Speichern beim Verlassen und beim Trennen des letzten Geräts, Zeitpunkt im Spielstand und in der Liste (AC-03, AC-04).
- S2.3 Spielmetrik-Report beim Raumende, Schema beschreiben (AC-05, AC-06).
- S2.4 Review des Sprints (Code-Sprint) (AC-01, AC-02, AC-03, AC-04, AC-05, AC-06).

## Abnahme

–
