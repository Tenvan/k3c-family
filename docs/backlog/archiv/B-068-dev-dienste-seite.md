# B-068 · k3c-dev zeigt die Dienste als Karten mit Zustand, Metriken und Log-Level

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** M4
- **Projekt:** –
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat (M4 Revision 1 mit B-064, B-068, Wails v2.16.0, React 19.3.0, Radix Themes 3.3.0, plugin-react 6.1.1, Ausnahmen und 5 Sessions)

## Ausgangslage

Nach M3 (B-067) führt `k3c-dev` die Dienste und bietet `svc_*`-Tools; eine Oberfläche dafür entsteht mit B-064.

## Ziel

`k3c-dev` zeigt die Dienste als Karten mit Zustand, Metriken und Log-Level. Nutzen: Start, Stopp und Zustand aller
Dienste auf einen Blick, ohne Terminal.

## Beteiligte und Zielgruppen

Entwickler am Entwickler-PC; 🧑 nimmt die Seite ab.

## Anforderungen

- Reiter `Dienste` als erster in der Kopfzeile (vor `Logs` und `MCP`, B-064); Bausteine, Farben, Mock und Ereignisse
  aus B-064; Zustandswechsel kommen als Ereignis `service:state`, Metriken alle 2 s mit.
- **Kopf der Seite:** Titel `Dienste`, rechts `Alle starten` (Akzent) und `Alle stoppen` (Gefahr); darunter die Zeile
  `„Alle starten“ startet parallel · „Alle stoppen“ rückwärts: Heimnetz → Vite` (Reihenfolge aus der Konfiguration).
  Während einer Sammelaktion sind beide gesperrt, der gedrückte zeigt den Ladezustand.
- **Karten** im Raster (breit drei Spalten, schmal eine), je Dienst:
  - Name, `Port <n>`, Zustands-Badge (`Läuft` grün, `Startet` blau, `Übernommen` gold, `Stoppt` neutral, `Gestoppt`
    neutral, `Fehlgeschlagen` rot; unbekannter Zustand → neutral), darunter die Health-Adresse in Festbreitenschrift.
  - Kasten mit `PID`, `CPU`, `SPEICHER`, `LAUFZEIT` (`–`, solange nichts läuft; deutsch formatiert, z. B. `3,1 %`,
    `480,0 MB`, `2 h 13 min`).
  - Nur für Dienste mit Log: Kasten `LOG · LETZTE 60 MINUTEN`, Anzahl Einträge, Knopf `Aktualisieren`, ein
    Balken mit Anteilen je Level und die Zahlen je Level (`WARN` und `ERROR` immer, `ERROR` rot, wenn > 0).
  - `Neustarts: n`, wenn > 0; der letzte Fehler rot (z. B. `Port 8080 bereits belegt (PID 8812)`).
  - Bei `übernommen` der Hinweis `Vor dem Start von k3c-dev gestartet: keine Konsolenausgabe, kein Auto-Restart.
    Stoppen nur nach Bestätigung.`
  - Knöpfe `Start`, `Stopp` (Gefahr), `Neustart`, freigegeben nach einer Tabelle Zustand → Knöpfe; nur der gedrückte
    zeigt den Ladezustand, alle sind währenddessen gesperrt.
- **Bestätigung:** Stopp eines übernommenen Dienstes öffnet einen Dialog (Titel, Dienst, PID, `Stoppen`/`Abbrechen`).
- **Fehler der Konfiguration** → statt des Rasters eine Hinweiskarte mit Pfad und Ursache.
- Die Tabellen Zustand → Badge und Zustand → Knöpfe sind reine Daten mit Tests; der Mock liefert alle Zustände.

## Nicht-Ziele

Dienste anlegen oder ändern; Modus-Umschalter; Konsole auf der Karte (die steht auf der Logs-Seite).

## Regeln und Einschränkungen

Wie B-064. Die Seite zeigt nur; Zustand, Metriken und Log-Zähler kommen aus Go (B-067).

## Beispiele

- Vite läuft, Heimnetz gestoppt → Vite-Karte grün mit Metriken, `Stopp` und `Neustart` frei; Heimnetz-Karte nur `Start`.
- `Alle stoppen` → Heimnetz, dann Vite gehen auf `Stoppt` und `Gestoppt`.

## Ausnahme- und Fehlerfälle

- Befehl schlägt fehl → Meldung an der Karte, Zustand aus dem nächsten Ereignis.
- Ereignis für einen unbekannten Dienst (Konfiguration geändert) → wird ignoriert, beim nächsten Laden stimmt die Liste.

## Akzeptanzkriterien

- **AC-01** Kopf mit Sammelaktionen und Karten mit Badge, Metriken, Log-Kasten und Knöpfen nach Zustand (Tests der Tabellen).
- **AC-02** Stopp eines übernommenen Dienstes nur über den Bestätigungsdialog; Fehler der Konfiguration als Hinweiskarte.
- **AC-03** Zustände und Metriken aktualisieren sich über Ereignisse; im Browser läuft die Seite gegen den Mock.

## Offene Fragen

keine

## Notizen

Wunsch von 🧑 (2026-09-30, Chat); Seite im Oberflächen-Sprint M4, Backend B-067 in M3.
