# K2 · SIM · Bosse, Siegvarianten und Inselwechsel

- **Status:** geplant
- **Domäne:** SIM
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-130, B-102, B-103
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Es gibt keine Minibosse und keinen Endboss, Siegvarianten und Niederlage-Modi der Raum-Optionen sind nicht umgesetzt, ein Inselwechsel fehlt (B-130, B-102, B-103).

## Ziel

Je Stufe ein Miniboss, je Insel ein Endboss, wählbare Siegvarianten und Niederlage-Modi sowie der gemeinsame Inselwechsel sind in der Simulation spielbar.

Am Ende sichtbar: Tests je Boss, Siegvariante und Modus grün, Spielstand mit besiegten Bossen und aktueller Insel.

## Beteiligte und Zielgruppen

Spieler (Koop) und REG für Werte; 🧑 gibt die Spec frei.

## Anforderungen

B-130 › Anforderungen, B-102 › Anforderungen, B-103 › Anforderungen.

## Nicht-Ziele

Events (K3), Protokoll (K4), Anzeige (K5), Balancing (BR2).

## Regeln und Einschränkungen

Werte nur in `data/`, Logik und Tests in `engine/sim/`; deterministisch (`engine/rng`), mit 2+ Spielern gleichzeitig. Golden-Daten nach dem Golden-Ablauf (B-137) aktualisieren, Spielstand-Änderungen nach der Migrationsregel (B-137). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`. Der Sprint bleibt in der Domäne SIM. Quelle: `docs/rules/bosse.md`, `docs/rules/stufen.md`.

## Beispiele

Endboss besiegt → Inselwechsel frei → alle lebenden Spieler am Punkt → neue Insel mit neuer Tabelle.

## Ausnahme- und Fehlerfälle

Nicht alle lebenden Spieler am Punkt → Wechsel wartet. Komplett verloren → Raum endet, letzter Spielstand bleibt unverändert.

## Akzeptanzkriterien

- **AC-01** Miniboss erscheint mit Welle 5 bzw. 3, Fähigkeit und Belohnung laut Daten (Test) (B-130/AC-01).
- **AC-02** Endboss wird bei Ankunft ausgelöst, wartet sonst, wechselt die Phasen (Test) (B-130/AC-02).
- **AC-03** Boss-HP skaliert mit der Spieleranzahl der Insel (Test) (B-130/AC-03).
- **AC-04** Der Spielstand merkt besiegte Bosse und die aktuelle Insel, Bosse kehren nie zurück (Test) (B-130/AC-04, B-103/AC-04).
- **AC-05** Je Siegvariante: Bedingung nicht erfüllt, erfüllt, Ereignis genau einmal (Test) (B-102/AC-01).
- **AC-06** Je Niederlage-Modus gelten die Folgen laut Regelwerk, „Komplett verloren“ lässt den letzten Spielstand unverändert, Standard je Grad und Überschreiben je Raum (Test) (B-102/AC-02, B-102/AC-03, B-102/AC-04).
- **AC-07** Endboss besiegt gibt den Inselwechsel frei, Wechsel nur mit allen lebenden Spielern am Punkt, je Stufe ein Miniboss und der Endboss in der tiefsten (Test) (B-103/AC-01, B-103/AC-02, B-103/AC-03).
- **AC-08** Golden-Daten aktualisiert, `task check:go` grün (B-130/AC-05).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- K2.1 Minibosse und Endboss mit Phasen, Skalierung, Spielstand (AC-01, AC-02, AC-03, AC-04).
- K2.2 Siegvarianten und Niederlage-Modi (AC-05, AC-06).
- K2.3 Inseln und gemeinsamer Inselwechsel, Golden (AC-07, AC-08).
- K2.4 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
