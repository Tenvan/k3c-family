# RX1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Xbox hält den WebGL-Kontext: Render-Log, Kontextverlust abfangen, Texturen auf 2048 px
- **Projekt:** LST

## Ziel

Die Xbox hält den WebGL-Kontext, und das Render-Verhalten jedes Clients ist im Server-Log lesbar. Am Ende sichtbar: Xbox-Start ohne „WebGL Context lost“ im Server-Log (`logs_query`), Startzeile mit Renderer, GPU und Texturspeicher, Leistungsverlauf alle 30 s, FPS-Wert notiert.

## Ergebnis

Verworfen 2026-11-12 (Entscheidung Ralf): abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009). RX1.1–RX1.4 und RX1.9 fertig, RX1.5–RX1.8 verworfen, alle Tickets verworfen. Status `erledigt`, weil das Werkzeug für Sprints keinen Status `verworfen` kennt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Startzeile mit allen Feldern für WebGL und Canvas (`B-762/AC-01`, Test). |
| AC-02 | – | Kontextverlust und -wiederherstellung erzeugen je eine Zustandszeile (`B-762/AC-02`, Test) … |
| AC-03 | – | Leistungsverlauf höchstens alle 30 s, FPS-Warnung höchstens je Minute (`B-762/AC-03`, Test … |
| AC-04 | – | Fehlgeschlagener Textur-Ladevorgang erzeugt Fehlerzeile mit Datei und Status (`B-762/AC-04 … |
| AC-05 | – | Ein simulierter Kontextverlust löst genau einen Neuaufbau oder Canvas-Neustart aus, ein zw … |
| AC-06 | – | Kein generiertes Sheet ist größer als 2048 px je Kante (`B-764/AC-01`, Test). |
| AC-07 | – | Der Client lädt beim Start nur die Texturen der ersten Stufe (`B-764/AC-02`, Test). |
| AC-08 | – | Xbox-Lauf (🧑): Technik-Anzeige sichtbar, Startwerte, Leistungsverlauf und Ladedaten im Ser … |
| AC-09 | – | Xbox bleibt nach Kontextverlust bedienbar, keine weiße Seite (`B-763/AC-03`). |
| AC-10 | – | Xbox-Start ohne „WebGL Context lost“ im Server-Log, FPS-Wert notiert (`B-764/AC-03`). |
| AC-11 | – | `data/stress.json` mit `typisch` und `worstCase`, alle Typen mit Asset (`B-765/AC-01`, Tes … |
| AC-12 | – | Ungültige Szenarien werden abgelehnt (`B-765/AC-02`, Test). |
| AC-13 | – | Rampe alle 10 s um den Faktor, gleicher Seed gleiche Verteilung (`B-765/AC-03`, Test). |
| AC-14 | – | Je Stufe genau eine Log-Zeile mit `runId`, `device`, FPS, p95, `textureMiB`, `entities`, ` … |
| AC-15 | – | Endzeile mit Dichte-Grenze und Zusammenfassung, Aufwärmzeit zählt nicht (`B-765/AC-05`, Te … |
| … | | weitere in README.md |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-761 | verworfen | AC-08 – | Renderer-Info (WebGL/Canvas, FPS, Texturen) im Spiel anzeigen, WebGL erzwingbar |
| B-762 | verworfen | AC-01 –, AC-02 –, AC-03 –, AC-04 –, AC-08 – | Renderer-, Textur- und Leistungswerte im Client-Log (Start, Laden, Verlauf, Kont … |
| B-763 | verworfen | AC-05 –, AC-09 – | Bei WebGL-Kontextverlust Renderer neu aufbauen statt abschalten |
| B-764 | verworfen | AC-06 –, AC-07 –, AC-10 – | Texturen auf die Xbox als Zielplattform auslegen (höchstens 2048 px, bei Bedarf … |
| B-765 | verworfen | AC-11 –, AC-12 –, AC-13 –, AC-14 –, AC-15 –, AC-16 –, AC-17 – | Stress-Szene Live-Grafik: Dichte-Test mit Szenario-Datei, Rampe und Presets (1 u … |
| B-766 | verworfen | AC-18 –, AC-19 –, AC-20 –, AC-21 –, AC-22 –, AC-23 – | Stresstest des echten Clients: Dev-Aktion spawn und Lauf mit Rampe in der GameSc … |
| B-767 | verworfen | AC-24 –, AC-25 –, AC-26 –, AC-27 – | Nachbesserungen aus dem ersten Xbox-Test: frischer Client, Pad auf allen Seiten, … |
