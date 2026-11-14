# PT2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Paper-CLI: Fehlerpfade, Reittier-Boxen, Stil-Option und Ersatz-Materialien
- **Projekt:** PAP

## Ziel

Die Paper-CLI liefert eindeutige Fehlerpfade, Reittier-Boxen und eine wählbare Stil-Datei; die Stil-Datei hat die fehlenden Materialien, und kein Asset nutzt mehr Ersatz-Abbildungen. Am Ende sichtbar: `paper frame monarch --animation galopp` mit Boxen `reittier:*`; `paper render … --stil <datei>`; Grafik-Golden aller Gruppen neu und grün.

## Ergebnis

2026-11-13, autonom (Claude, Klasse A). Kriterien: AC-01 bis AC-03 in PT2.1, AC-04 und AC-05 in PT2.2; `task check` und `task check:go` grün (PT2.3). Review-Befunde behoben: Nachweis für drei umgestellte Assets ergänzt, PT2.2-Ergebnis korrigiert. Neue Tickets: B-845 (`--ohne` mit Reittier-Teilen, Wulst der Bein-Box). Abnahme-Ticket für Sim, Balance und Gen: B-844. Sichtprüfung der 19 umgestellten Assets im Bereich Assets durch 🧑 (nicht blockierend).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Fehlerfeld nennt `animationen.<name>.reittier` (`B-708/AC-01`). |
| AC-02 | geprüft | `paper frame monarch --animation galopp` liefert Boxen mit Präfix `reittier:` (`B-709/AC-0 … |
| AC-03 | geprüft | `paper frame` und `paper render` nehmen `--stil <datei>`, ohne Option gilt `public/grafik/ … |
| AC-04 | geprüft | Vier Materialien in der Stil-Datei, Schema-Test grün (`B-735/AC-01`). |
| AC-05 | geprüft | Kein Asset nutzt die Ersatz-Abbildungen, Golden aller Gruppen neu (`B-735/AC-02`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-708 | erledigt | AC-01 geprüft | Ablehnung reittier-unbekannt nennt das Feld ohne Animationsnamen |
| B-735 | erledigt | AC-04 geprüft, AC-05 geprüft | Weitere Ersatz-Materialien (stroh, lava, geisterweiss, fell-grau) in der Stil-Da … |
