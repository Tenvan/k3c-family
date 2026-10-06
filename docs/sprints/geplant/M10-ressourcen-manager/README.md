# M10 · SRV · Ressourcen-Manager für Grafik- und Sound-Slots in k3c-dev

- **Status:** geplant
- **Domäne:** SRV
- **Prio:** mittel
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-298
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Grafik- und Sound-Zuordnung steht verteilt in `docs/assets/` und im Code; es gibt kein Slot-Register (B-298).

## Ziel

🧑 ordnet in k3c-dev jedem Grafik- und Sound-Slot Assets mit Präferenz zu, das Register ist die einzige Quelle. Am Ende sichtbar: k3c-dev zeigt alle Slots, 🧑 ordnet Assets mit Präferenz zu.

## Beteiligte und Zielgruppen

🧑 wählt Assets; der Agent baut Register und Oberfläche.

## Anforderungen

B-298 › Anforderungen.

## Nicht-Ziele

Vorschau im Spielmaßstab (B-299, DBG4), neue Assets suchen.

## Regeln und Einschränkungen

Nur `tools/k3c-dev/` und das Register unter `public/` (Format in der ersten Session festlegen).

## Beispiele

Slot „Bogenschütze laufen“ → zwei Kandidaten, Präferenz 1 wird geladen.

## Ausnahme- und Fehlerfälle

Asset im Register fehlt auf der Platte → Slot rot markiert, Spiel nutzt den Platzhalter.

## Akzeptanzkriterien

- **AC-01** Ein ResourcenManager in k3c-dev ordnet jedem Grafik- und Sound-Slot Assets mit Präferenz zu (B-298/AC-01, B-298/AC-02, B-298/AC-03, B-298/AC-04, B-298/AC-05, B-298/AC-06, B-298/AC-07).

## Offene Fragen

Format und Ort des Slot-Registers: entscheidet die erste Session, 🧑 bestätigt.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- M10.1 Slot-Register und Katalog mit Suche (AC-01).
- M10.2 Oberfläche Grafik und Sound, Präferenz speichern (AC-01).
- M10.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
