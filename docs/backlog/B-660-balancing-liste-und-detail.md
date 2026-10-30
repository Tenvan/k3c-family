# B-660 · Läufe und Berichte stehen in einer Liste mit Filter und Mehrfachauswahl, der Bericht in Reitern

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ32
- **Projekt:** WZG
- **Erstellt:** 2026-10-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-29 🧑 Chat (Revision 1, offene Fragen mit Default von Claude)

## Ausgangslage

Die Balancing-Seite hat zwei Listen für dieselbe Sache: „Läufe“ (`balancing/RunQueue.tsx`, nur diese Sitzung) und „Berichte“ (`balancing/ReportList.tsx`). Ein Lauf heißt dort z. B. „offline-headless-balance“, sein Bericht „simtest-run-1“. Am 2026-10-29 lagen 15 Berichte mit nur 3 verschiedenen Namen (`simtest-run-1/2/3`) vor; die Liste zeigt weder Label noch Bot, Spieler, Welt, Tage oder Grad. Es gibt keine Suche und keinen Filter, jede Zeile trägt drei Bedienelemente (Bewertung, „Vergleichsbasis“, ✕) und bricht um. Der Vergleich ist ein Modus (erst Basis setzen, dann zweiten Bericht öffnen), kann nur zwei Berichte und verdrängt die Berichtsansicht. Der Bericht (`ReportView.tsx`, `InsightsView.tsx`) ist eine Spalte mit 16 Ampel-Marken, 13 Hinweisen, 7 Befehlen, 16 Zielen und 4 Diagrammen ohne Gliederung. „Konfiguration übernehmen“ gibt es nur an Läufen dieser Sitzung.

B-612 hat Bewerten, Löschen und die Erklärung des Vergleichs gebracht und Mehrfachauswahl und Filter ausdrücklich ausgeschlossen; dieses Ticket setzt dort an.

## Ziel

Ein Bericht ist in der Liste auf einen Blick erkennbar und schnell zu finden; aus einem Bericht entsteht mit einem Klick der nächste Lauf; mehrere Berichte lassen sich gemeinsam vergleichen, bewerten und löschen.

## Beteiligte und Zielgruppen

Ralf beim Balancing; Agent DEV.

## Anforderungen

### Liste

- Eine Liste für Läufe und Berichte: wartende und laufende Läufe oben (mit Fortschritt, Phase, Abbrechen), fertige darunter, gruppiert nach Heute, Gestern, Älter. Ein fertiger Lauf wird zu seinem Bericht, es gibt keinen zweiten Eintrag.
- Jede Zeile: Label (Ersatz: Bot · Welt · Datum), Status (läuft, wartet, Pass, Fail), Bot, Spieler, Welt oder Tiefe, Tage, Seeds, Anzahl Varianten, Uhrzeit, getroffene Ziele (`3/16 Ziele`), Bewertung als Farbpunkt, Art (Abstimmung, Baseline).
- Suche über Label, Bot, Welt und Varianten; Filter-Chips Status (Alle, Läuft, Pass, Fail) und Art (Balance, Abstimmung, Baseline) über `ListFilter` aus `@webkit/molecules`; Chips gemerkt, Suche nicht.
- Mehrfachauswahl per Checkbox; Leiste unten mit Vergleichen (ab 2), Bewerten, Löschen (eine Rückfrage für alle), Aufheben.
- Keine Bedienelemente außer Checkbox und Name in der Zeile.

### Detail eines Berichts

- Kopf: Label, Status, Auftrag in einer Zeile, Pfad; Aktionen Erneut starten, Bearbeiten und starten (öffnet den Lauf-Editor, B-661), Vergleichen mit …, Export (Markdown, JSON, HTML-Bericht), Bewertung als Umschalter, Menü mit Als Baseline sichern und Löschen.
- Reiter: Überblick (Ampel-Zähler, Kurzfazit, drei Ziele am weitesten daneben mit Korridor-Balken, nächster Lauf mit „Im Editor übernehmen“), Ziele (Tabelle mit Filter Alle/Verletzt/Knapp/Gut), Diagramme, Hinweise (Stellschrauben mit „Als Variante übernehmen“, Befehle), Konfiguration (Auftrag des Laufs). Der Reiter wird gemerkt.
- Detail eines laufenden Laufs: Fortschritt, letzte Zeilen, Auftrag, Abbrechen; nach dem Ende wechselt es auf den Bericht.

### Vergleich

- 2 bis 4 Berichte nebeneinander, A ist die Basis (tauschbar). Je Ziel und Kennzahl Wert und Abstand zu A, gekippte Ziele (Fail ↔ Pass) hervorgehoben; Filter Nur Änderungen, Gekippt, Alle Ziele, Kennzahlen; Export Markdown.
- Varianten der Berichte im Kopf der Spalte (`tree.markCost 1 → 0.5`).

## Nicht-Ziele

- Seitenrahmen, Scroll-Bereiche und Umbruch (B-659, B-658).
- Lauf-Editor und Varianten-Tabelle (B-661).
- Neue Felder und Sammel-Aufrufe der Go-Seite (B-662); bis dahin zeigt die Zeile, was `BalanceReportInfo` heute liefert.
- Papierkorb (wie B-612).

## Regeln und Einschränkungen

- Atomic Design über `@webkit`, rohe Controls nur in `atoms/` (`tests/atomicDesign.test.ts`).
- `docs/standards/workbench-seiten.md` › Gemeinsame Regeln (Gemerkter Zustand, Status-Marken als Pill, Prompt-Knopf bei Befehlen für Claude).
- Löschen nur nach Rückfrage, nur Pfade unter `reports/` (wie B-612).
- Komplexitäts-Budget: Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; `ReportView` und `InsightsView` werden je Reiter geteilt.

## Beispiele

- 15 Berichte von heute → Chip „Fail“ und Suche „Insel“ → 4 Zeilen, jede mit Label, Bot, Welt, Tagen und `x/16 Ziele`.
- Drei Berichte anhaken → „Vergleichen“ → Vergleich A/B/C, Filter „Gekippt“ zeigt die zwei Ziele, die bei C auf Pass kippen.
- Bericht offen → „Erneut starten“ → gleicher Auftrag landet in der Warteschlange, die Zeile erscheint oben unter „Läuft“.
- Hinweis „Holz ≥ 100 ab Tag 3“ → „Als Variante übernehmen“ → Lauf-Editor mit gefülltem Auftrag und der Stellschraube als Variante.

## Ausnahme- und Fehlerfälle

- Älterer Bericht ohne Auftrag → Zeile zeigt Name und Seeds wie heute, „Erneut starten“ ist aus mit Hinweis.
- Abstimm-Ergebnis (`tuning-*.json`) → eigene Reiter (Überblick mit bestem Punkt, Punkte, Diff), kein Vergleich.
- Löschen eines gewählten Berichts → Auswahl springt auf den nächsten Eintrag.
- Filter ohne Treffer → Hinweis „Keine Einträge für diesen Filter“.

## Akzeptanzkriterien

- **AC-01** Vitest (Mock-Backend): Ein laufender Lauf und seine Berichte erscheinen als ein Eintrag; nach dem Ende ersetzt der Bericht den Lauf an derselben Stelle.
- **AC-02** Vitest: Suche und Chips filtern die Liste; Chips überleben ein Neu-Rendern (gemerkt), die Suche nicht.
- **AC-03** Vitest: Mehrfachauswahl von drei Berichten → Vergleichen zeigt drei Spalten mit Abstand zu A; Löschen fragt einmal und entfernt alle drei.
- **AC-04** Vitest: Detail zeigt fünf Reiter; „Erneut starten“ ruft `balanceStart` mit dem Auftrag des Berichts; „Als Variante übernehmen“ öffnet den Lauf-Editor mit der Zeile.
- **AC-05** Beobachtung im Browser-Pane mit `?mock=1`: Aufbau wie im Mock-Board „Vorschlag: Läufe & Berichte“.

## Offene Fragen

- Wie viele Berichte hält die Liste, bevor Älteres eingeklappt wird? Default (Claude): Gruppe „Älter“ eingeklappt ab 30 Einträgen. Entscheidet 🧑.
- Bleibt die Bewertung „Referenz“ neben „Baseline“ (B-640) bestehen, oder wird sie zur Baseline? Default (Claude): beide bleiben, Referenz ist nur eine Markierung. Entscheidet 🧑.

## Notizen

- Mock: Design-Canvas „Balancing-Seite Mock“ (2026-10-29), Boards „Vorschlag: Läufe & Berichte“ und „Vorschlag: Vergleich A/B/C“.
- Braucht für die volle Zeile die Felder aus B-662; ohne sie ist die Liste teilweise umsetzbar.
- Vorgänger: B-612 (Bewerten, Löschen, Vergleich erklären).
