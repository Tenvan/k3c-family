# S4 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Kamera je Stufe und Layouts 1–4
- **Projekt:** –

## Ziel

Jede Zelle zeigt die Stufe ihres Spielers, auch bei Spielern in verschiedenen Stufen; Radar, HUD und die Layouts 1–4 folgen der Stufe der Zelle und halten die Mindest-Schriftgröße. Am Ende sichtbar: 2 Spieler am selben Gerät in verschiedenen Stufen.

## Ergebnis

- 2026-10-03, Review S4.4: AC-01 (S4.1: `cellStages` mit 9 Tests, Zeichnen je Zelle über `StageView`) und AC-02 (S4.2: `cellRadar`, HUD je Zelle mit Biom-Name) sind umgesetzt und im Code geprüft; AC-03 (S4.2: `fontRules.test.ts` gegen `bedienung.md` § 2, Mindestwerte nicht gesenkt) ist grün. - AC-04: angenommen, Validierung offen (S4.3, 🧑 am Gerät; „zwei Spieler in verschiedenen Stufen“ zusätzlich erst nach B-176, Protokoll mit mehreren Stufen, Sprint S2: verschoben). Heute ist nur die Stufe von `client.level` geladen, andere Zellen zeigen den Platzhalter; nichts davon im Browser gesehen. - Ke …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Zeichnen je Zelle aus der Stufe des Spielers; die reine Zuordnungs-Logik ist getestet (B-1 … |
| AC-02 | – | Radar und HUD zeigen je Zelle die richtige Stufe (B-106/AC-02). |
| AC-03 | – | Test: Die Schriftgröße je Layout 1–4 unterschreitet die Mindestgröße aus der Regel (B-136) … |
| AC-04 | – | 🧑 hat 2 Spieler in verschiedenen Stufen sowie das Viertel-Layout am Gerät abgenommen (B-10 … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-106 | erledigt | AC-01 –, AC-02 –, AC-04 – | Jeder Spieler sieht seine Stufe, auch wenn die Spieler in verschiedenen Stufen s … |
