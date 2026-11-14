# WZ16 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Welt-Betrachter: eine gemeinsame Ansicht mit Sprungleiste, Zoom und Gebäude-Filter
- **Projekt:** WZG

## Ziel

Der Welt-Betrachter zeigt Chunk-Streifen und Live-Vorschau in einer Ansicht, die gemeinsam scrollt und zoomt, mit Sprungleiste und gruppiertem Gebäude-Filter; maximiert sind beide Vorschauen sichtbar und nichts wird abgeschnitten. Am Ende sichtbar: im Studio-Fenster die überarbeitete Seite Welt-Betrachter; `task check:all` und CI grün; von 🧑 abgenommen (damit gilt auch WZ7/AC-09 als erfüllt).

## Ergebnis

2026-10-18 (Review WZ16.3, Abnahme WZ16.4 durch 🧑 am PC): AC-01 bis AC-05 mit Nachweis aus Tests und Browser-Pane (Ergebnis WZ16.1 bis WZ16.3); AC-07 erfüllt (WZ16.4, damit auch WZ7/AC-09). AC-06 teilweise: `task check:all` lokal nur an `assetkit` `TestDownloadHit` rot (B-480); CI auf `develop` nach dem Merge. Befunde: keine schweren. Neue Tickets: B-480. Version: v0.19.0 gesetzt (2026-10-18, Minor wegen Wirkung im Werkzeug, zusammen mit WZ7; Tag auf `ea750844`). `task check:all` lokal nur an B-480 rot, die CI-Läufe auf `develop` und der Release-Lauf sind grün.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `B-477/AC-01` gemeinsame Ansicht mit gemeinsamem Scrollen, eine Scrollbar. |
| AC-02 | – | `B-477/AC-02` Sprungleiste (links, Mitte, rechts, Hub, Tor, Portal, Ausgang). |
| AC-03 | – | `B-477/AC-03` Zoom stufenlos und in festen Stufen für beide Vorschauen. |
| AC-04 | – | `B-477/AC-04` maximiert beide Vorschauen sichtbar, keine abgeschnittenen Bedienelemente, k … |
| AC-05 | – | `B-477/AC-05` Gebäude-Filter nach Art gruppiert. |
| AC-06 | teilweise | `B-477/AC-06` `task check:all` und CI grün. |
| AC-07 | erfüllt | 🧑 nimmt den überarbeiteten Welt-Betrachter am PC ab (Abnahme-Session; erfüllt auch WZ7/AC- … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-477 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 –, AC-05 –, AC-06 teilweise | Der Welt-Betrachter zeigt Chunk- und Live-Vorschau in einer gemeinsamen, synchro … |
