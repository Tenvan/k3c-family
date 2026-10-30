# WZ33 · DEV · Eine Seite Assets statt Grafik, Sound und Quellen

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-677
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Grafik, Sound und Quellen sind drei Seiten des Studios mit sieben Unter-Reitern. Es gibt drei Kandidaten-Listen, sechs Abspielstellen und vier Filterleisten. Eine Netz-Datei in einen Slot zu bringen braucht sechs Schritte und zwei Seitenwechsel. Details stehen in B-677 › Ausgangslage.

## Ziel

🧑 verwaltet alle Asset-Slots und Dateien auf einer Seite Assets und stellt sie per Fokus-Chip auf ein Thema scharf. Eine Lücke lässt sich aus dem Netz in zwei Schritten füllen.

**Am Ende sichtbar:** Im Studio-Fenster gibt es statt Grafik, Sound und Quellen eine Seite Assets mit folgenden Teilen:

- Fokus-Chips samt eigenen Chips
- den Sichten Slots, Dateien und Szene
- einem Inspektor mit Kandidaten
- einer Abspielleiste unten

## Beteiligte und Zielgruppen

- 🧑 Ralf arbeitet mit der Seite und nimmt sie ab.
- Claude setzt in autonomen DEV-Sessions um.

## Anforderungen

- B-677 › Anforderungen (Seite und Navigation, Inspektor, Abspielen und Werkzeuge, Vokabular).
- Die neue Seite entsteht neben den alten, bis WZ33.6 sie entfernt. Bis dahin bleibt das Studio in jedem Commit benutzbar.

## Nicht-Ziele

- B-677 › Nicht-Ziele.
- Die kompakten Seiten Planer, Monarch und Gegner (B-670–B-672) kommen in einem eigenen Sprint.

## Regeln und Einschränkungen

- B-677 › Regeln und Einschränkungen.
- WZ29 und WZ30 ändern dieselben Ordner (`graphics/`, `sound/`, `sources/`). WZ33 wird erst aktiv, wenn beide erledigt sind oder 🧑 ihre offenen Sessions verwirft.
- Ein PR für den ganzen Sprint (Branch `sprint/wz33`), ein Commit je Session.
- Das Code-Review vor dem Commit läuft in WZ33.1 (Backend, Pfade, Download) und ab etwa 150 geänderten Zeilen.

## Beispiele

B-677 › Beispiele.

## Ausnahme- und Fehlerfälle

B-677 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Eine Seite Assets ersetzt Grafik, Sound und Quellen (B-677/AC-01).
- **AC-02** Feste und eigene Fokus-Chips (B-677/AC-02).
- **AC-03** Sichten und Gruppierungen (B-677/AC-03).
- **AC-04** Filterzustand bleibt erhalten (B-677/AC-04).
- **AC-05** Kandidaten-Liste je Slot ohne Dubletten (B-677/AC-05).
- **AC-06** „Zuordnen“ in einem Aufruf mit Bestätigung in der Zeile (B-677/AC-06).
- **AC-07** Eine Abspielleiste, Hörprobe entfällt (B-677/AC-07).
- **AC-08** Werkzeuge-Fläche (B-677/AC-08).
- **AC-09** Sprünge und Chat-Kontext (B-677/AC-09).
- **AC-10** Alte Seiten entfernt, Prüfungen grün (B-677/AC-10).
- **AC-11** Layout im Browser-Pane ohne Scrollen (B-677/AC-11).
- **AC-12** Manuelle Abnahme durch 🧑 (B-677/AC-12).

## Offene Fragen

keine. Die vier Fragen aus dem Mock hat 🧑 am 2026-10-30 entschieden (B-677 › Offene Fragen).

## Sessions

Entwurf; vor dem Aktivieren wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

- **WZ33.1 · Backend: Kandidaten und Zuordnen** in einem Aufruf: `assetkit` führt Katalog, Vorrat, Recherche und Vorschläge je Slot zusammen. RPC `Candidates` und `CandidateAssign` mit Stand-Prüfung auf Basis von `VorratAdoptIf` (AC-05, AC-06 Backend).
- **WZ33.2 · webkit-Bausteine:** FocusBar, FacetRail, Inspektor-Vorlage, TransportBar mit Tastatur. Abspielzustand seitenweit, höchstens ein Ton (AC-04 Teil, AC-07 Teil).
- **WZ33.3 · Seite Assets** mit Sichten Slots und Dateien: feste und eigene Fokus-Chips in `prefs`, Suche, Facetten, Gruppieren, Raster bzw. Liste mit ▶ (AC-01 hinter Schalter, AC-02, AC-03, AC-04).
- **WZ33.4 · Inspektor** mit Belegung, Kandidaten und Details: Zuordnen mit Bestätigung in der Zeile, Fehler und erneuter Versuch in der Zeile (AC-05, AC-06).
- **WZ33.5 · Sicht Szene, Abspielleiste und Werkzeuge-Fläche:** Mischprobe, Lautheit, Abstand, Vorrat aufräumen, Aus Recherche übernehmen. Hörprobe in die Leiste überführen (AC-03 Szene, AC-07, AC-08).
- **WZ33.6 · Umschalten und Aufräumen:** `PAGES` auf `assets` umstellen, Sprünge, Chat- und Ticket-Kontext umstellen, alte Seiten `graphics/`, `sound/` und `sources/SourcesPage` entfernen (AC-01, AC-09, AC-10).
- **WZ33.7 · Review** mit allen Kriterien, `task check:studio`, `task check:diff`, Browser-Pane 5196 (AC-11).
- **WZ33.8 · Abnahme am PC**, Agent Mensch: 🧑 füllt eine Lücke aus dem Netz und merkt sich eine Ansicht (AC-12).

## Abnahme

–
