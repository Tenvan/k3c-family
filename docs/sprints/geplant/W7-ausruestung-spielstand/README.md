# W7 · SIM · Ausrüstung ohne Unverwundbarkeit, Spielstand vollständig

- **Status:** geplant
- **Domäne:** SIM
- **Prio:** hoch
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-312, B-313, B-202, B-201, B-075
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Sofortiges Wiederaufheben macht die Burg bei passivem Spiel unverwundbar (B-312); W4.3b hängt an `sites_test.go` (B-313); Spielstand v3 ist zwischen S1 und W1 unklar (B-202) und stellt W2–W4 nicht vollständig her (B-201); der Golden-Spielstand hat keinen ausgebauten Hub (B-075).

## Ziel

Ohne Handlung verliert man die Burg im Zielkorridor, und Speichern/Laden verliert nichts aus W2–W4. Am Ende sichtbar: Passive Burg kann fallen, Spielstand stellt W2–W4 wieder her, Golden-Hub geprüft.

## Beteiligte und Zielgruppen

🧑 entscheidet B-313; Agent baut in `engine/sim/`.

## Anforderungen

B-312 › Anforderungen; B-313 › Anforderungen; B-202 › Anforderungen; B-201 › Anforderungen; B-075 › Anforderungen.

## Nicht-Ziele

Neue Gegner (K1), Bosse (K2), Migration alter Stände (Beschluss: keine Rücksicht).

## Regeln und Einschränkungen

SIM; nach W4. Golden-Diffs nur mit Begründung in der Session.

## Beispiele

Bot „passiv“ über 5 Nächte → Burg fällt im Korridor je Grad.

## Ausnahme- und Fehlerfälle

Spielstand aus älterer Version → Fehler 🚫 mit Version, kein stiller Verlust.

## Akzeptanzkriterien

- **AC-01** Sofortiges Wiederaufheben fallengelassener Ausrüstung macht die Burg bei passivem Spiel unverwundbar (B-312/AC-01, B-312/AC-02).
- **AC-02** W4.3b kann den Vermerk „Wirkung offen“ nur mit einer Änderung an sites_test.go ersetzen (B-313/AC-01).
- **AC-03** S1 und W1 teilen sich die Spielstand-Version 3 eindeutig (B-202/AC-01, B-202/AC-02, B-202/AC-03).
- **AC-04** Der Spielstand stellt Plantage, Berufe, Krieger, Elite, Rüstung, Schwerter und Händler nach dem Laden wieder her (B-201/AC-01, B-201/AC-02, B-201/AC-03, B-201/AC-04).
- **AC-05** Der Golden-Spielstand enthält einen gebauten und veränderten Hub (B-075/AC-01).

## Offene Fragen

- B-313: Darf W4.3b `sites_test.go` ändern? Entscheidet 🧑; blockiert W7.1.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- W7.1 Wiederaufheben mit Sperrzeit, Entscheidung zu W4.3b (AC-01, AC-02).
- W7.2 Spielstand v3 eindeutig und vollständig, Golden mit Hub (AC-03, AC-04, AC-05).
- W7.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
