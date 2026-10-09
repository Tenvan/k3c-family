# M10 · SRV · Ressourcen-Manager für Grafik- und Sound-Slots in k3c-dev

- **Status:** geplant
- **Projekt:** GRA
- **Domäne:** SRV
- **Reife:** Entwurf
- **Tickets:** B-298, B-299
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 2
- **Freigabe:** –

## Ausgangslage

Grafik- und Sound-Zuordnung steht verteilt in `docs/assets/` und im Code; es gibt kein Slot-Register (B-298).

## Ziel

🧑 ordnet in k3c-dev jedem Grafik- und Sound-Slot Assets mit Präferenz zu, das Register ist die einzige Quelle. Am Ende sichtbar: k3c-dev zeigt alle Slots, 🧑 ordnet Assets mit Präferenz zu.

## Beteiligte und Zielgruppen

🧑 wählt Assets; der Agent baut Register und Oberfläche.

## Anforderungen

B-298 › Anforderungen; B-299 › Anforderungen (aus DBG4, PJ3).

## Nicht-Ziele

Neue Assets suchen. Die Vorschau im Spielmaßstab (B-299) gehört seit PJ3 zu diesem Sprint (AC-02).

## Regeln und Einschränkungen

Nur `tools/k3c-dev/` und das Register unter `public/` (Format in der ersten Session festlegen).

## Beispiele

Slot „Bogenschütze laufen“ → zwei Kandidaten, Präferenz 1 wird geladen.

## Ausnahme- und Fehlerfälle

Asset im Register fehlt auf der Platte → Slot rot markiert, Spiel nutzt den Platzhalter.

## Akzeptanzkriterien

- **AC-01** Ein ResourcenManager in k3c-dev ordnet jedem Grafik- und Sound-Slot Assets mit Präferenz zu (B-298/AC-01, B-298/AC-02, B-298/AC-03, B-298/AC-04, B-298/AC-05, B-298/AC-06, B-298/AC-07).
- **AC-02** Eine Dev-Seite zeigt die Asset-Zuordnung je Kategorie als Mini-Szene im Spielmaßstab (B-299/AC-01, B-299/AC-02, B-299/AC-03, B-299/AC-04; aus DBG4 AC-01, PJ3).

## Offene Fragen

Format und Ort des Slot-Registers: entscheidet die erste Session, 🧑 bestätigt.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- M10.1 Slot-Register und Katalog mit Suche (AC-01).
- M10.2 Oberfläche Grafik und Sound, Präferenz speichern (AC-01).
- M10.3 Review (Code-Sprint): alle Kriterien prüfen.
- M10.4 Dev-Seite mit Mini-Szenen je Kategorie, Domäne PLAT (AC-02; aus DBG4.1, PJ3). Beim Bereitmachen vor das Review ordnen.

## Abnahme

–
