# B-179 · Das Debug-Overlay bedient Gold, Material und Zeitraffer

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** DBG2
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Debug-Overlay (`src/scenes/debugOverlay.ts`, `debugOverlayView.ts`, Taste Ö oder Stick-Klick, B-093) zeigt nur Verbindung, Takt und Entitäten. Mit B-178 gibt es serverseitig Dev-Aktionen für Gold, Material und Zeitraffer.

## Ziel

Das Overlay bietet im Dev-Mode Aktionen zum Gold droppen, Material geben und für den Zeitraffer an und zeigt den aktuellen Zeitfaktor. Nutzen: Tests am PC, am Handy und am Controller ohne Tastenkombinationen aus der Entwicklerkonsole.

## Beteiligte und Zielgruppen

🧑 und Entwickler beim Testen; Domäne CLI (`src/scenes/`, `src/online/`).

## Anforderungen

- Die Aktionsliste erscheint nur, wenn das Overlay aktiv ist (`?dev` nicht 0, `debugEnabled`) und der Raum im Dev-Mode läuft (Server akzeptiert `dev`); sonst bleibt sie verborgen.
- Aktionen: Gold droppen (10, 50, 100) für den gewählten lokalen Spieler, Material geben (Holz, Stein, Kupfer, Eisen, Kristall je 50), Zeitfaktor 1×, 2×, 4×, 8×.
- Bedienung mit Maus, Touch (Schaltflächen) und Controller (D-Pad wählt, A löst aus); Belegung nicht B, nicht View + Menu, nicht X; der Overlay-Toggle Ö bleibt.
- Das Overlay zeigt den aktuellen Zeitfaktor des Raums.
- Der Client sendet nur Nachrichten, er rechnet nichts.

## Nicht-Ziele

Serverseitige Aktionen (B-178), Wechsel des Schwierigkeitsgrads (B-107), Stufenwechsel und Neustart (B-080).

## Regeln und Einschränkungen

`src/scenes` zeichnet nur Snapshots (`noSim`-Test); Dateien ≤ 400 Zeilen, Funktionen ≤ 60; Seitenregeln aus `CLAUDE.md` (B nicht belegen, View + Menu reserviert). B-098: Vor dem Release ist das Overlay wieder nur mit `?dev=1` verfügbar.

## Beispiele

Overlay mit Ö öffnen → „Gold 50“ wählen → 50 Münzen fallen vor den Spieler. „Zeit 8×“ → der Faktor steht im Overlay und die Welt läuft schneller.

## Ausnahme- und Fehlerfälle

Raum ohne Dev-Mode → keine Aktionsliste, keine Meldung. Server antwortet `forbidden` → kurzer Hinweis im Overlay, keine Wiederholung.

## Akzeptanzkriterien

- **AC-01** Test: Eine reine Funktion liefert aus der Overlay-Auswahl die zu sendende `dev`-Nachricht (Gold, Material, Zeitfaktor) und bei unbekannter Auswahl nichts.
- **AC-02** Test: Die Aktionsliste ist nur im Dev-Mode des Raums und bei aktivem Overlay sichtbar.
- **AC-03** Das Overlay zeigt den Zeitfaktor aus dem Zustand.
- **AC-04** Der `noSim`-Test und `task check` sind grün.
- **AC-05** 🧑 hat Gold droppen, Material geben und Zeitraffer am PC, am Handy und mit Controller ausprobiert.

## Offene Fragen

keine

## Notizen

Hängt an B-178 (Protokoll). Anlass: Wunsch von 🧑 am 2026-10-03.
