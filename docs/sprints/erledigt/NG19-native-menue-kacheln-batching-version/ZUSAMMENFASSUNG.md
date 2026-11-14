# NG19 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Natives Spiel: Menü-Kacheln mit Illustrationen, Hintergrund-Batching, Version im Paket
- **Projekt:** NAT

## Ziel

Die Menüs des nativen Spiels zeigen Kacheln mit Illustrationen, zeichnen den Hintergrund mit wenigen Zustandswechseln, und Paket und EXE tragen immer dieselbe Version. Am Ende sichtbar: Haupt- und Pause-Menü mit Illustrationen am PC; `task native:game:paket` im Worktree mit passender Version.

## Ergebnis

2026-11-13: AC-01 bis AC-04 laut Ergebnis von NG19.1, NG19.2, NG19.5; Sichtprüfung NG19.4 und Review-Freigabe durch 🧑 („NG19.3 +4 ok“). Review-Befunde (k3c-reviewer) behoben: Fenster-Lage nach Vollbild, Größe ≤ 90 % des Monitors. `task check` und `task check:go` grün. Neue Tickets: B-826. Kein Abnahme-Ticket für Gen (keine Grafik geändert).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Menü-Kacheln mit Illustrationen (`B-818/AC-01` bis `B-818/AC-03`). |
| AC-02 | verschoben | Hintergrund wechselt den Filter je Bild höchstens einmal, e2e grün (`B-820/AC-01`, `B-820/ … |
| AC-03 | – | Version in EXE und `LIZENZEN.txt` gleich, kein unnötiger Neubau (`B-821/AC-01`, `B-821/AC- … |
| AC-04 | – | Optionen schalten zwischen Fenster, rahmenlos und Vollbild (`B-826/AC-01` bis `B-826/AC-04 … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-818 | erledigt | AC-01 geprüft | Hauptmenü und Pause-Menü des nativen Spiels zeigen Kacheln mit Illustrationen |
| B-820 | erledigt | AC-02 verschoben | Menü-Hintergrund wechselt den Filter je Scheibe und bricht das Batching |
| B-821 | erledigt | AC-03 – | Version in LIZENZEN.txt und EXE können im Worktree auseinanderlaufen |
| B-826 | erledigt | AC-04 – | Die Optionen des nativen Spiels schalten zwischen Fenster, rahmenlos und Vollbil … |
