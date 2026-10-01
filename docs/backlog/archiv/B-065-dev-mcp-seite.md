# B-065 · k3c-dev zeigt auf der MCP-Seite Server, Tools, Live-Monitore, Aufruf-Log und Statistik

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** erledigt
- **Sprint:** M5
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat (Pauschalauftrag „beide komplett autonom fertig stellen“: M5 Revision 1 mit B-065, 5 Sessions)

## Ausgangslage

Nach M4 (B-064) hat `k3c-dev` ein Fenster mit Logs-Seite; der Reiter `MCP` zeigt nur einen Hinweis. Zähler und
Aufruf-Log (B-046) und die Nutzungsstatistik (B-062) sind da, aber nur als Daten.

## Ziel

`k3c-dev` zeigt auf der MCP-Seite Server, Tools, Live-Monitore, Aufruf-Log und Statistik. Nutzen: Man sieht, was
Agenten gerade tun, wie lange Tools brauchen und wo es hakt.

## Beteiligte und Zielgruppen

Entwickler, die Agenten-Arbeit beobachten; 🧑 nimmt die Seite ab.

## Anforderungen

- Bausteine, Farben, Mock und Ereignisse aus B-064. Kein Abfrage-Intervall: Nach jedem `mcp:start`/`mcp:call` werden
  Zähler und Statistik neu geladen; `Aktualisieren` lädt von Hand.
- **Unteransichten** `Übersicht` | `Statistik` als Umschalter oben links, gemerkt.
- **Übersicht, Band 1:**
  - Karte `MCP-Server`: Adresse, Badge `Lauscht` bzw. `Fehler`, Knöpfe `Neu starten` (startet den HTTP-Server neu),
    `Instructions` (Dialog mit dem gerenderten `instructions.md`: Überschriften, Listen, `code`, **fett**; als
    React-Elemente, nie als HTML-String) und `Aktualisieren`. Kennzahlen Laufzeit, Aufrufe, Fehler (rot, wenn > 0),
    Ø Dauer (gewichtet über alle Tools). Schlägt ein Nachladen fehl, bleiben die letzten Zahlen stehen, der Grund darunter.
  - Karte `Verbindungen`: Clients, Clients max, Parallel, Parallel max.
- **Übersicht, Band 2 links – Tools:** eine Kachel je Tool mit Name, Anteil-Balken und `N Aufrufe · N Fehler · Ø x`
  (Fehler rot, wenn > 0); sortiert nach Aufrufen, dann Name; Tooltip und `aria-label` mit Beschreibung und letztem Aufruf.
  Die Tool-Menge kommt nur aus den Server-Zählern (kein zweiter Katalog im Frontend).
- **Übersicht, Band 2 rechts – Live:** Umschalter Metrik `Aufrufe`/`Laufzeit` und Zeitraum `15 min`, `1 h`, `24 h`,
  `7 T` (Punkt-Abstand 1 min, 2 min, 30 min, 6 h), beide gemerkt; das Fenster wandert alle 30 s mit der Uhr, neu
  gerechnet wird lokal aus der Minuten-Zeitreihe (B-062).
  - Säulen links: bei `Aufrufe` „Top 5 Tools“ nach Aufrufen im Zeitraum mit der Zahl über der Säule; bei `Laufzeit`
    „Langsamste Tools · max“ mit der Höchstdauer, rot ab Ausreißer. Lange Namen brechen nach `_` um.
  - Liniendiagramm rechts: bei `Aufrufe` Aufrufe je Punkt (Fläche) und Fehler (rot gestrichelt), Kopf
    `N Aufrufe · N Fehler · max N/Punkt`; bei `Laufzeit` Max je Punkt (Linie), Ø (gestrichelt) und rote Punkte für
    Ausreißer, Kopf `max x · N Ausreißer · Ø y`. Achse mit Uhrzeit (bei `7 T` Datum), rechts `jetzt`.
  - Selbst als SVG gezeichnet, keine Diagramm-Bibliothek.
- **Übersicht, Band 3 – Aufruf-Log** (restliche Höhe): Filter `Tool` (Alle und die Tools aus den Zählern) und Schalter
  `nur Fehler`. Je Aufruf zwei Zeilen, Start und Ende, neueste oben: Zeit, Tool, `ref` (je Aufruf eigene Farbe),
  Graph-Spalte, Badge (`Start`, `OK`, `Fehler`, `läuft`), Dauer, beim Start die Argumente, beim Ende die
  Zusammenfassung. Aufklappen zeigt das Argument-JSON eingerückt und den Fehlertext. Der Graph funktioniert wie bei Git:
  jede Spur führt vom Start- zum Endknoten des Aufrufs, parallele Aufrufe liegen auf eigenen Spuren, eine freie Spur wird
  wiederverwendet, ein laufender Aufruf ist gestrichelt und nach oben offen. Fußzeile `N Aufrufe · N laufend · N Fehler`.
- **Statistik:** Umschalter `Sitzung` | `All time`, gemerkt.
  - Kennzahl-Kacheln: Aufrufe, Fehlerquote (rot, wenn > 0), Aufrufe/h, Aktive Tools, Ø Dauer, p50, p95, Max,
    Σ Laufzeit, Ausreißer (rot, wenn > 0). Darunter die Zeile `seit <Uhrzeit bzw. Datum> · Perzentile aus Histogramm,
    ±12 % · Ausreißer: über 2× p95 der Vergleichsgruppe (gleiches Tool und gleiche Argumente, sonst das Tool) und
    mindestens 1,0 s; Vergleichsgruppe ab 8 Aufrufen` (Werte aus einer Stelle im Code, nicht doppelt gepflegt).
  - Tabelle je Tool, sortierbar per Klick auf den Spaltenkopf (Standard: Aufrufe absteigend): Tool, Aufrufe, Anteil
    (Balken und %), Fehler, Quote, Ø, p50, p95, Max, Σ Zeit, Ausreißer. Eine Zeile klappt auf: „Häufigste Argumente ·
    Laufzeit“ (Anzahl, Argument, p95, Max) und „Häufigste Fehler“ (oder `Keine Fehler`).
  - Seitenspalte: `Letzte Ausreißer` und `Langsamste Aufrufe` (Dauer, rot bei Ausreißer; Tool; `x× p95`; `Fehler`,
    falls gescheitert; Zeit; Argumente), `Häufigste Fehler (alle Tools)` (Anzahl, Tool, Meldung).
  - Zeitangaben heute nur als Uhrzeit, sonst Datum und Uhrzeit; Dauer als `ms`, `s` oder `min`.
- Der Mock aus B-064 liefert Zähler, Aufrufe (auch parallele und laufende), Zeitreihe und Statistik.
- Frontend-Logik (Graph-Spuren, Zeitfenster und Reihen, Sortierung, Formatierung, Markdown-Teilmenge) liegt in reinen
  Funktionen mit Tests.

## Nicht-Ziele

Eingriffe in Agenten oder Tools aus der Oberfläche (außer Server-Neustart); Export; Benachrichtigungen; Auswertung je Client.

## Regeln und Einschränkungen

Wie B-064. Die Oberfläche rechnet nur Darstellungswerte (Fenster, Spuren); Perzentile und Ausreißer kommen aus Go (B-062).

## Beispiele

- Zwei Agenten rufen gleichzeitig `check_run` und `logs_query` → im Aufruf-Log zwei nebeneinander liegende Spuren,
  `Parallel` zeigt 2.
- Zeitraum `24 h`, Metrik `Laufzeit` → die Säule von `check_run` ist die höchste, ein roter Punkt markiert einen Ausreißer.

## Ausnahme- und Fehlerfälle

- Keine Aufrufe → Kacheln und Diagramme zeigen `–` bzw. einen leeren Zustand, keine Division durch null.
- Server-Neustart schlägt fehl → Badge rot, Grund in der Karte, die Seite bleibt bedienbar.
- Unbekanntes Tool im Aufruf-Log (nach Neustart) → Zeile wird trotzdem gezeigt.

## Akzeptanzkriterien

- **AC-01** Übersicht mit Server- und Verbindungskarte, Instructions-Dialog und Neustart funktioniert.
- **AC-02** Tool-Kacheln und Live-Monitore zeigen beide Metriken in allen vier Zeiträumen (Tests der Reihen-Berechnung).
- **AC-03** Das Aufruf-Log zeigt Start und Ende mit Graph-Spuren für parallele und laufende Aufrufe, filtert nach Tool
  und Fehlern (Tests der Spur-Berechnung).
- **AC-04** Die Statistik zeigt Kennzahlen, sortierbare Tabelle mit Aufklappen und Seitenspalte für `Sitzung` und
  `All time` (Tests der Sortierung und Formatierung).
- **AC-05** `npm run check:dev` und der CI-Job sind grün, die Grenzen sind eingehalten.

## Offene Fragen

keine

## Notizen

Aus B-046 Revision 2 abgeleitet (2026-09-30).
