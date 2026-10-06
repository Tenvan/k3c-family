# PF1.4 · Messung an der Xbox

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** Mensch
- **Branch:** pf1/4-xbox-messung
- **Abhängig von:** PF1.3
- **Tickets:** B-194
- **Kriterien:** AC-01

## Ziel

🧑 hat an der Xbox gemessen, ob der Split-Screen mit zwei Spielern flüssig läuft; FPS, Snapshot-Abstand und Ursache stehen in B-194.

## Kontext

- Hardware-Bahn (`docs/plan-weiterentwicklung.md` § 11.6): Diese Session sperrt nichts und läuft, sobald die Xbox bereitsteht. Ein Agent nimmt sie nicht, er darf nur den Build bereitstellen.
- Gerät wie beim ersten Befund: Xbox, Edge, Server `pi-gaming` (Build mit PF1.2 auf dem Pi), zwei Controller.
- Das Debug-Overlay (`src/scenes/debugOverlay.ts`) zeigt FPS, Snapshot-Hz, „letzter Snapshot … ms“, Puffer und Latenz (und die Frame-Zeit, falls PF1.1 sie ergänzt hat).
- Ziel laut B-194/AC-02: im Mittel ≥ 55 FPS im Split-Screen mit zwei Spielern.
- Die offene Frage aus B-194 (welches Szenario auf `testing.html`, ruckelt auch `game.html`?) klärt 🧑 hier mit: beide Seiten messen.

## Erlaubte Dateien

- `docs/backlog/B-194-*.md`, `docs/sprints/`

## Nicht-Ziele

Code-Änderungen; neue Befunde werden Tickets.

## Schritte

1. Build mit PF1.2 auf `pi-gaming` bereitstellen (`task serve` bzw. Deploy wie üblich).
2. Auf der Xbox `game.html` mit zwei Controllern im Split-Screen starten, je eine Minute bei Tag und bei Nacht spielen; FPS und „letzter Snapshot … ms“ aus dem Debug-Overlay notieren.
3. Dasselbe auf `testing.html` mit dem Szenario vom 2026-10-03, falls 🧑 es noch weiß.
4. Werte, Szenario und Ursache in B-194 › Notizen eintragen, die offene Frage dort als entschieden vermerken.
5. Erreicht: B-194 auf `erledigt`, Sprint abschließen. Nicht erreicht: Messwerte und Ticket, angenommener Wert bleibt (Sprint › Ausnahme- und Fehlerfälle).

## Fertig, wenn

- [ ] AC-01: B-194 nennt FPS im Split-Screen und Snapshot-Abstand am Gerät mit Ursache; im Mittel ≥ 55 FPS mit zwei Spielern (🧑 am Gerät) oder ein Folge-Ticket mit Messwerten.

## Prüfen

```bash
task check
```

Manuelle Prüfung an der Xbox durch 🧑 (diese Datei nennt sie; Freigabe durch 🧑 beim Lauf).

## Ergebnis

–
