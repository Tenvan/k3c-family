# WZ7.7 · Abnahme des Welt-Betrachters durch Ralf

- **Status:** fertig
- **Typ:** Review
- **Agent:** Mensch
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** wz7/7-abnahme
- **Abhängig von:** WZ7.6
- **Tickets:** B-460
- **Kriterien:** AC-09

## Ziel

🧑 nimmt am PC den Welt-Betrachter ab (AC-09) und entscheidet über die Umplanung von B-092 sowie die Punkte 1 und 4 von B-426.

## Kontext

Sprint WZ7 (`docs/sprints/aktiv/WZ7-welt-betrachter/README.md`), Ticket `docs/backlog/B-460-welt-betrachter-live-vorschau.md` (Spec freigegeben 2026-10-17). Maßgeblich sind die Anforderungen, Beispiele und Ausnahmefälle dort.

- Die Abnahme-Liste steht unten in `### Abnahme-Liste`. Start: `task k3c-studio:build`, dann `bin/k3c-studio.exe`.
- **Umgebung live:** das Studio ist gebaut. Claude bereitet vor und trägt das Urteil von 🧑 ein, urteilt aber nicht selbst. Fenster- oder Browser-Prüfungen durch Claude nur mit Freigabe von 🧑 für diesen Lauf.
- Zu entscheiden: B-092 (Level-Betrachter, HW1) wird umgeplant; B-426 Punkt 1 (Biom-Wahl beim Voll-Ausbau) und Punkt 4 (Taste V mit Strg) gelten für das Studio oder entfallen (`B-460 › Notizen`).

### Abnahme-Liste

Ablauf am PC im Studio-Fenster, Seite „Welt-Betrachter“ (AC-09):

1. Beim Öffnen ist die Chunk-Vorschau maximiert und lässt sich verkleinern (Taste M); Seed und Biom wechseln, Zoom mit + und −, Scrollen mit Pfeiltasten.
2. Live-Vorschau: Hub-Stufe 1 bis 5 umschalten, Gebäude ein und aus, Bürger ein und aus; gleicher Seed zeigt dasselbe Bild.
3. Tageszeit: Tag, Dämmerung, Nacht, Morgengrauen mit Fortschritt; Hintergrund-Schichten mit Parallaxe sichtbar.
4. „Im Spiel öffnen“ (neu und voll ausgebaut) öffnet die Landingpage des lokalen Spielservers; eine Adresse außerhalb von Loopback wird abgelehnt.
5. `dev.html` zeigt keine Kachel „Level“, `testing.html` keinen Abschnitt „Level“.
6. Entscheidung: Umplanung von B-092 (HW1) sowie B-426 Punkt 1 und Punkt 4.

Hinweis des Reviews (2026-10-17): keine schweren Befunde; Frontend-Nachweise nur per Grep und Tests, die Optik ist ungeprüft.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- Diese Session-Datei (Ergebnis, Status) sowie über k3c-dev die Planungsdateien von WZ7, B-460, B-092 und B-426
- Kein Code

## Nicht-Ziele

Kein Code, keine Umsetzung, kein Urteil durch einen Agenten, keine Aktivierung eines Folgesprints.

## Schritte

1. 🧑 arbeitet die Abnahme-Liste (WZ7.6) am PC ab.
2. 🧑 entscheidet über B-092 und B-426 Punkte 1 und 4.
3. Das Urteil und die Entscheidungen werden im Ergebnis eingetragen, Status und Sprint-Abschluss nachgezogen.

## Fertig, wenn

- [ ] AC-09: 🧑 hat den Welt-Betrachter am PC abgenommen und über B-092 und B-426 Punkte 1 und 4 entschieden (Eintrag im Ergebnis).

## Prüfen

Manuelle Abnahme durch 🧑 am PC; keine Befehle.

## Ergebnis

Abnahme durch 🧑 am PC, 2026-10-18 (zwei Screenshots im Chat); der Agent hat nicht geurteilt.
- **Entscheidungen:** B-092 verworfen (Studio und Level-Betrachter brauchen keinen Xbox-Einsatz; AC-05 von B-092 entfällt). B-426 Punkt 1 und Punkt 4: „passt“, verworfen; Punkt 2 und 3 bleiben offen.
- **AC-09 verschoben: B-477.** 🧑 hat den Welt-Betrachter nicht abgenommen. Mängel: maximiert ohne Live-Vorschau, Layout unten und zweite Scrollbar, Chunk- und Live-Vorschau scrollen nicht gemeinsam, getrennte statt gemeinsamer Ansicht, Gebäude-Filter unübersichtlich, kein stufenloser Zoom. B-477 (Spec freigegeben 2026-10-18) nimmt sie auf; die Abnahme des überarbeiteten Betrachters macht 🧑 im Sprint WZ16.
- Neue Tickets: B-477.
