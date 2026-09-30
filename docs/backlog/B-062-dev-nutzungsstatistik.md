# B-062 · k3c-dev wertet MCP-Aufrufe über Sitzungen aus: Perzentile, Ausreißer und Zeitreihe

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** erledigt
- **Sprint:** M2
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat (M2 Revision 1 mit B-062 und B-063)

## Ausgangslage

Nach M1 (B-046) zählt `k3c-dev` Aufrufe nur im Speicher: je Tool Anzahl, Fehler und Ø Dauer, dazu ein Ring der letzten
200 Aufrufe. Wie lange ein Tool typischerweise braucht, welcher Aufruf aus der Reihe fällt und wie sich die Last über
Stunden und Tage verteilt, sieht niemand, und nach einem Neustart ist alles weg.

## Ziel

`k3c-dev` wertet MCP-Aufrufe über Sitzungen aus: Perzentile, Ausreißer und Zeitreihe. Nutzen: Die Statistik und die
Live-Monitore der MCP-Seite (B-065) haben ihre Daten, langsame oder fehleranfällige Tools fallen auf.

## Beteiligte und Zielgruppen

Entwickler, die Agenten-Arbeit beobachten; die MCP-Seite (B-065) als einziger Leser der Daten.

## Anforderungen

- Paket `tools/k3c-dev/internal/usage`, kennt den Server nicht. Die Middleware aus B-046 gibt jeden beendeten Aufruf
  hinein, mit den **rohen** Argumenten (das Aufruf-Log kürzt sie auf 120 Zeichen, gekürztes JSON ließe sich nicht
  normieren). Ein Ereignis: Zeitpunkt, Tool, Argumente (JSON), Dauer in ms, ok, Fehlermeldung.
- **Zwei Bereiche:** `session` (seit Programmstart) und `allTime` (seit dem ersten Start, gespeichert). Je Bereich:
  - je Tool: Aufrufe, Fehler, Σ Dauer, Ø, Max, p50, p95, Ausreißer; häufigste Argumente mit Anzahl und je Argument
    Ø, p95 und Max; häufigste Fehlermeldungen mit Anzahl. Argumente normiert (Schlüssel sortiert, kompaktes JSON),
    Texte auf 120 Zeichen gekürzt; höchstens 50 verschiedene Werte je Tool und Liste, der Rest zählt unter `(weitere)`.
  - über alle Tools: Aufrufe, Fehler, Ø, p50, p95, Max, Σ; die 10 häufigsten Fehler (mit Tool); Ausreißer gesamt;
    die 10 letzten Ausreißer und die 10 langsamsten Aufrufe (Zeitpunkt, Tool, Argumente, Dauer, ok, Vergleichswert).
- **Perzentile aus Histogramm:** Buckets mit Faktor 1,25 zwischen zwei Grenzen (Fehler eines Perzentils höchstens rund
  12 %), p50 und p95 aus dem Histogramm, nie über dem echten Maximum.
- **Ausreißer:** ein Aufruf ist einer, wenn er länger dauert als 2 × p95 seiner Vergleichsgruppe und mindestens 1 s.
  Vergleichsgruppe: gleiches Tool mit gleichen Argumenten, wenn diese schon 8 Aufrufe hat, sonst das Tool (ab 8
  Aufrufen), sonst wird nicht bewertet. Der Vergleichswert wird vor dem Eintragen des Aufrufs gebildet.
- **Zeitreihe:** je Minute der letzten 7 Tage Aufrufe je Tool, Fehler, Σ Dauer je Tool, Max je Tool, Ausreißer je Tool;
  ältere Minuten fallen weg, leere Minuten werden nicht gespeichert.
- **Speichern:** `allTime` und Zeitreihe in `<os.UserConfigDir()>/k3c/mcp-usage.json` mit Versionsfeld; gebündelt
  (höchstens alle 2 s) und beim Beenden; atomar (temporäre Datei, dann umbenennen).
- **Schnappschuss:** eine Struktur `{session, allTime, minutes}` als JSON für die Oberfläche; Minuten mit Zeitstempel in ms.
- `workbench_status` (B-046) nennt zusätzlich p95 und Ausreißer der Sitzung.

## Nicht-Ziele

Oberfläche (B-065); Auswertung nach Client oder Agent; Export; Speicherung im Repo.

## Regeln und Einschränkungen

Nur Standardbibliothek. Komplexitäts-Budget aus `docs/arbeitsweise.md`. Die Zeit kommt als Parameter bzw. Uhr-Funktion
herein, damit Tests ohne echtes Warten laufen. Nebenläufig sicher.

## Beispiele

- 20 Aufrufe `logs_query` à 100 ms, dann einer mit 1,5 s → Ausreißer (2 × p95 = 0,2 s, über 1 s).
- 20 Aufrufe `check_run {npm:check}` à 40 s, einer mit 50 s → kein Ausreißer (unter 2 × p95 der Argumentgruppe).
- Programm neu gestartet → `session` leer, `allTime` und Zeitreihe wie vorher.

## Ausnahme- und Fehlerfälle

- Datei kaputt oder fremde Version → neu beginnen, alte Datei als `mcp-usage.json.bak` behalten, Warnung ins eigene Log.
- Datei nicht schreibbar → Statistik läuft im Speicher weiter, eine Warnung je Programmlauf.
- Weniger als 8 Aufrufe → kein Ausreißer, Perzentile trotzdem.

## Akzeptanzkriterien

- **AC-01** p50 und p95 aus dem Histogramm liegen bei einer bekannten Verteilung höchstens 12 % neben dem echten Wert
  und nie über dem Maximum (Test).
- **AC-02** Die Ausreißer-Regel trifft genau die Beispiele und Grenzfälle (Faktor, 1 s, 8 Aufrufe, Argumentgruppe vor
  Toolgruppe) (Tests).
- **AC-03** Ranglisten für Argumente, Fehler, letzte Ausreißer und langsamste Aufrufe stimmen, der Deckel von 50 Werten
  greift mit `(weitere)` (Tests).
- **AC-04** Die Zeitreihe hält 7 Tage je Minute und verwirft Älteres (Test mit gestellter Uhr).
- **AC-05** Nach einem Neustart sind `allTime` und Zeitreihe da, `session` ist leer; kaputte Datei → `.bak` und
  Neubeginn; geschrieben wird atomar (Tests mit temporärem Ordner).
- **AC-06** `npm run check:dev` ist grün, die Grenzen sind eingehalten.

## Offene Fragen

keine

## Notizen

Aus B-046 Revision 2 abgeleitet (2026-09-30).
