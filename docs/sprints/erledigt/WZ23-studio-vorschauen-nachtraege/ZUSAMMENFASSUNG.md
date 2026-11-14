# WZ23 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Studio-Vorschauen: Reittiere aus dem Spielcode, Monarch-Seite, Boss-Anzeigen, Gebäude-Planer abgenommen
- **Projekt:** WZG

## Ziel

Die Vorschauen im Studio zeigen mit dem Code des Spiels, was das Spiel zeigt, und lassen sich bequem vergleichen. Am Ende sichtbar: Reittiere, Monarchen mit Zoom und Boss-Anzeigen im Studio ohne nachgebauten Spielcode, Gebäude-Planer von 🧑 abgenommen.

## Ergebnis

2026-10-22: AC-01 bis AC-05 nachgewiesen (WZ23.1–WZ23.5): AC-05 von Ralf am PC abgenommen, AC-04 (Boss-Anzeigen) mit dem Zeichencode aus K5.1 umgesetzt, der unverändert aus `origin/sprint/k5` übernommen ist (siehe WZ23.3, beim Merge von K5 beachten). Event-Banner für Boss-Ereignisse bleibt Lücke (das Spiel zeichnet es nicht). Entscheidungen für den Nachgang: Reittiere auf Anforderung, Monarch-Sheets wie im Spiel, K5.1 vorab übernommen.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `sprites.ts` lädt beliebige Reittiere auf Anforderung und legt ihre Animationen an, `creat … |
| AC-02 | geprüft | `renderReference` liefert eine Abmeldung, das Studio meldet beim Abbau ab (B-471/AC-01, B- … |
| AC-03 | geprüft | Die Monarch-Seite hat Zoom und zweispaltiges Layout; die Sheets folgen der Entscheidung vo … |
| AC-04 | geprüft | Die Gegner-Seite zeigt Warnkreis, Boss-Leiste, Phasen und Event-Banner mit dem Zeichencode … |
| AC-05 | – | 🧑 hat Gebäude-Planer, Monarch- und Gegner-Seite am PC abgenommen oder Nachbesserungen als … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-476 | erledigt | AC-01 geprüft | preloadSprites und createSpriteAnims laden beliebige Reittiere statt nur das Sta … |
| B-471 | erledigt | AC-02 geprüft | renderReference beendet seine Animationsschleife, wenn die Karten verschwinden |
| B-502 | erledigt | AC-03 geprüft | Monarch-Seite: einheitliches Sheet, Zoom, zweispaltige Ansicht |
| B-469 | erledigt | AC-04 geprüft | Die Gegner-Seite zeigt Warnkreis, Boss-Leiste, Phasen und Event-Banner aus K5.1 |
| B-501 | erledigt | AC-05 – | Gebäude-Planer: Live-Vorschau unten fest, gewähltes Gebäude markiert |
