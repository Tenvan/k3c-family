# AZ1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Tasten von Spieler 2 an der Tastatur, Pause-Anzeige
- **Projekt:** BED

## Ziel

Jede Zelle zeigt, was für ihren Spieler gerade gilt: die Tasten seines Tastatur-Layouts und einen angehaltenen Raum. Am Ende sichtbar: Am PC mit zwei Tastatur-Spielern nennt das Feld von Spieler 2 Enter, Pfeile und Ziffernblock; nach „⏸ Pause“ in `/dm` steht in jeder Zelle eine Pause-Anzeige und alle Figuren stehen still.

## Ergebnis

- 2026-10-11 (AZ1.3, Review durch unabhängigen Reviewer-Agenten): AC-01 geprüft (AZ1.1, `glyphs.test.ts`), AC-02 geprüft (AZ1.1, Browser-Pane), AC-03 und AC-04 angenommen; `task check` und `task check:go` grün. - 2026-10-11 (AZ1.4, 🧑 am PC): AC-03 und AC-04 erfüllt, Tasten von Spieler 2 ok, Zusatzprobe „Raum in der Pause verlassen“ ok. - Behoben: Pause beim Verlassen des Raums ließ alle Animationen global angehalten (`HudScene.ts`, Shutdown-Handler ruft `resumeAll()`). - Neue Tickets: keine. B-371 und B-333 erledigt und archiviert. - Version: v0.18.0 vorgeschlagen (Minor: Pause-Anzeige und Tas …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Test: Die Glyph für `confirm` und die Slot-Beschriftungen von Spieler 2 an der Tastatur ne … |
| AC-02 | geprüft | Browser-Pane: Im Feld von Spieler 2 stehen keine Tasten von Spieler 1 (`B-371/AC-02`). |
| AC-03 | – | Bei angehaltenem Raum zeigt jeder Spieler-Bildschirmbereich eine Pause-Anzeige, läuft der … |
| AC-04 | erfüllt | Während der Pause stehen die Figuren-Animationen still und laufen nach „Weiter“ wieder (`B … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-371 | erledigt | AC-01 geprüft, AC-02 geprüft | Hinweise und Glyphen zeigen für Spieler 2 an der Tastatur dessen Tasten |
| B-333 | erledigt | AC-03 –, AC-04 erfüllt | Der Client zeigt einen angehaltenen Raum deutlich an und hält die Figuren-Animat … |
