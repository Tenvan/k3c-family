# SO3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Hörprobenseite `soundtest.html`
- **Projekt:** SND

## Ziel

Eine Seite spielt Kandidaten ab, bedienbar mit Controller. Am Ende sichtbar: `soundtest.html` auf der Landingpage, auf der Xbox hörbar.

## Ergebnis

2026-10-04, Agent (Claude Opus 5.5) in SO3.4, leichtes Review. AC-01, AC-05: Ergebnis SO3.1; AC-02, AC-06 bis AC-08: Ergebnis SO3.2. AC-03, AC-04: angenommen, Validierung offen (SO3.3, Hörprobe und Controller am TV); ebenso die Browser-Pane-Schritte aus SO3.1/SO3.2 (nicht freigegeben, AC-02 und AC-07 durch Tests belegt). Befunde: keine schweren (`installPageChrome()`, Eintrag in `pages.ts`, kein `requestFullscreen()`/`location`, B und View/Menu nicht belegt, kein `Math.random()`, nur selbst erzeugte Audiodateien). Abweichung SO3.2 (eigener AudioContext mit dem Mixer aus SO1 statt `AudioCore`) …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `soundtest.html` ruft `installPageChrome()` auf und steht in `src/landing/pages.ts`, `test … |
| AC-02 | – | Die Seite listet Kandidaten nach Zustand und Ereignis mit Quelle und Lizenz und spielt sie … |
| AC-03 | – | Die Seite ist mit dem Controller bedienbar, B ist nicht belegt, View + Menu führt zur Land … |
| AC-04 | – | Die Crossfade-Probe zwischen zwei Stücken ist ohne Knacken hörbar (B-169/AC-04). |
| AC-05 | – | Vollbild nur über `toggleFullscreen()`, Seitenwechsel nur über `openPage()` und `goHome()` … |
| AC-06 | – | `task check` ist grün. |
| AC-07 | – | Die Seite ist auch mit Tastatur und Touch (`?touch=1`) bedienbar (B-169 › Anforderungen). |
| AC-08 | geprüft | Jede fremde Audiodatei hat Quelle und Lizenz in den Credits, ohne Credit keine fremde Date … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-169 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 –, AC-05 –, AC-07 – | Eine Hörprobenseite spielt Kandidaten für Musik und Effekte ab |
