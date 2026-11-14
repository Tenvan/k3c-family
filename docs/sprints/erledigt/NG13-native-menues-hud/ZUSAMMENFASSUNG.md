# NG13 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Natives Spiel: Menüs und HUD nachziehen
- **Projekt:** NAT

## Ziel

Menüs und HUD des nativen Spiels haben den Funktionsumfang des Browser-Clients (ohne Kachel-Illustrationen) und zeigen die Befunde aus NG9.1 nicht mehr. Am Ende sichtbar: Tests in `menu`, `hud`, `fx`, `local` grün, `task native:game:e2e` mit zwei gleichen Läufen, Lizenzdatei in `.work/native-game/`.

## Ergebnis

2026-11-12 Review (Claude, `k3c-reviewer`, NG13.6): AC-01 bis AC-17 belegt in den Ergebnissen von NG13.1 bis NG13.5; AC-16 mit additiver Änderung in `engine/room` und `local/step.go` (begründet in NG13.5). Behoben: stufenlose Bannergröße (`fitPx` in 2-px-Stufen), Lizenz-Volltexte in `LIZENZEN.txt`, `game:build`-sources um `engine/**`. Neue Tickets: B-820 (Batching-Bruch im Menü-Hintergrund), B-821 (Version `LIZENZEN.txt`/EXE im Worktree). Abnahme-Ticket für Sim, Balance und Gen: keins nötig (kein Gen-Feature; Sim gesperrt laut Beschluss 2026-11-11). Sichtprüfung durch 🧑 Ralf bei der nächsten P …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Seite Tasten im Pause-Menü, eine Zeile je Aktion, Zurück ins Pause-Menü (`B-807/AC-01`). |
| AC-02 | – | Importregel-Test grün (`B-807/AC-02`). |
| AC-03 | – | Ende-zu-Ende-Lauf gleich in allen Stationen (`B-807/AC-03`). |
| AC-04 | – | Lizenz-Seite nennt Ebitengine, Schrift und Audio (`B-808/AC-01`). |
| AC-05 | – | Hauptmenü zeigt Version, ohne Angabe „unbekannt“ (`B-808/AC-02`). |
| AC-06 | – | Lizenzdatei im Paket (`B-808/AC-03`). |
| AC-07 | – | Schalter Blitze und Anleitung bleiben nach Speichern und Laden (`B-809/AC-01`). |
| AC-08 | – | Anleitung aus → kein Hinweis (`B-809/AC-02`). |
| AC-09 | – | „Anleitung gesehen“ überlebt einen Neustart (`B-809/AC-03`). |
| AC-10 | – | Karten im Bild ohne Überlappung, Auswahl hervorgehoben (`B-812/AC-01`). |
| AC-11 | – | Farben aus `stil.paper.json` (`B-812/AC-02`). |
| AC-12 | – | Ende-zu-Ende-Lauf mit Papier-Menü, Bilder abgelegt (`B-812/AC-03`). |
| AC-13 | – | Burgfall-Banner im Bild, Verluste des Modus `lost` (`B-813/AC-01`). |
| AC-14 | – | Skill-Menü ohne Überlappung (`B-813/AC-02`). |
| AC-15 | – | Erstes Draw nach „Spiel verlassen“ ohne falschen Zustand (`B-813/AC-03`). |
| … | | weitere in README.md |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-807 | erledigt | AC-01 –, AC-02 –, AC-03 – | Das Pause-Menü des nativen Spiels zeigt die Tastenbelegung |
| B-808 | erledigt | AC-04 –, AC-05 –, AC-06 – | Das native Spiel zeigt Lizenzen und Version und liefert die Lizenzdatei im Paket … |
| B-809 | erledigt | AC-07 –, AC-08 –, AC-09 – | Die Optionen des nativen Spiels bieten Blitze und Anleitung an und merken sich „ … |
| B-812 | erledigt | AC-10 –, AC-11 –, AC-12 – | Das Menü des nativen Spiels erscheint im Papierschnitt-Stil mit Illustrationen |
| B-813 | erledigt | AC-13 –, AC-14 –, AC-15 –, AC-16 – | Burgfall-Banner, Skill-Menü, Hauptmenü nach dem Verlassen und Siegmeldung zeigen … |
| B-811 | erledigt | AC-17 – | Plantagen-Bäume, gegnerische Geschosse und die Anleitung sehen im nativen Spiel … |
