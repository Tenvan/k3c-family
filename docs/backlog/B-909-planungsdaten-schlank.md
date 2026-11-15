# B-909 · Die Planungsseite lädt bei einer Änderung nur das Geänderte statt 5,5 MiB PlanningData

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ61
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Gemessen in WZ60.1 (B-895): `PlanningData` ist 5,5 MiB groß und braucht 1,9 s; `planning:changed` feuert bei laufenden Agenten alle 4–5 s. Ohne Drossel zog die Planungsseite 644 KiB/s, mit der Drossel aus B-895 (höchstens alle 15 s) noch bis zu ~370 KiB/s. Im Fenster geht jede Antwort als Skript durch den WebView2-Renderer.

## Ziel

Die Planungsseite bleibt bei laufenden Agenten aktuell, ohne je Änderung mehrere MiB zu laden.

## Beteiligte und Zielgruppen

🧑 und Agenten an der Planungsseite der Workbench; Agent DEV (Agent `k3c-workbench` prüft).

## Anforderungen

- Größe von `PlanningData` aufschlüsseln (Anteil je Teil: Sprints, Sessions, Tickets, Archiv, Fließtexte) und im Ergebnis nennen.
- Die Listen-Daten tragen nur, was Listen und Karten zeigen (Kopf-Felder, Titel, Zähler); Fließtexte lädt die Seite erst beim Öffnen eines Dokuments (`PlanningGet`).
- `planning:changed` nennt die geänderten IDs; die Seite lädt nur diese nach und ersetzt sie in ihrem Zustand. Ohne IDs (erster Aufruf, unklare Änderung) lädt sie einmal alles.
- Die Drossel aus B-895 bleibt; Fenster (Wails) und Browser-Dev (Vite, `/api`) verhalten sich gleich.
- Anzeige, Filter und Zähler der Planungsseite bleiben gleich (bestehende Tests grün).

## Nicht-Ziele

Neue Funktionen der Planungsseite; Änderungen an den MCP-Tools `plan_*`; Cache auf der Platte.

## Regeln und Einschränkungen

Oberfläche nach Atomic Design über `@webkit` (`tests/atomicDesign.test.ts`); keine neue Abhängigkeit. Die Dateien unter `docs/` bleiben die Quelle.

- **Bereich:** `tools/k3c-workbench/internal/planning/**`, `tools/k3c-workbench/app_planning.go`, `tools/k3c-workbench/app.go`, `tools/k3c-workbench/uiapi.go`, `tools/k3c-workbench/frontend/src/planning/**`, `tools/k3c-workbench/frontend/src/api/**`, `tools/k3c-workbench/frontend/src/lib/useLiveReload.ts`, `tools/k3c-workbench/frontend/wailsjs/**`
- **Abhängig von:** WZ60.1
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** exe

## Beispiele

Ein Agent setzt per `plan_set` den Status eines Tickets → die Seite lädt nur dieses Ticket nach (wenige KiB) und zeigt den neuen Status.

## Ausnahme- und Fehlerfälle

Datei gelöscht oder ins Archiv verschoben → der Eintrag verschwindet bzw. wechselt die Liste. Viele Änderungen auf einmal (Merge, Branchwechsel) → einmal alles laden. Nachladen scheitert → alter Stand bleibt sichtbar, nächster Versuch mit dem nächsten Ereignis.

## Akzeptanzkriterien

- **AC-01** Ein Nachladen nach einer einzelnen Ticket-Änderung ist kleiner als 500 KiB und die Listen-Daten ohne Fließtexte sind kleiner als 1,5 MiB (Go-Test gegen das echte `docs/`, Messung im Ergebnis).
- **AC-02** Vitest: Nach `planning:changed` mit einer ID ruft die Seite nur das Nachladen dieser ID auf, nicht `PlanningData`.
- **AC-03** Die Aufschlüsselung der 5,5 MiB steht im Ergebnis der Session (Tabelle Teil → Größe).

## Offene Fragen

- Stempel je Datei, Teil-Daten oder kleinere Nutzlast? → Default: beides in dieser Reihenfolge: Fließtexte aus den Listen-Daten nehmen, dann nur geänderte IDs nachladen (vorläufig, 🧑 bestätigt).
- Grenze für die Listen-Daten? → Default: 1,5 MiB (vorläufig, nach der Aufschlüsselung anpassbar).

## Notizen

Messwerte aus WZ60.1 (B-895). 2026-11-15 von WZ60 (alle Sessions fertig) nach WZ61 verschoben.
