# WZ7.4 · Live-Optionen und Im Spiel öffnen

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz7/4-live-optionen-im-spiel
- **Abhängig von:** WZ7.3
- **Tickets:** B-460
- **Kriterien:** AC-05, AC-06

## Ziel

Die Live-Vorschau bietet Tageszeit mit vier Einträgen und Hintergrund-Schichten mit Parallaxe; „Im Spiel öffnen“ startet das Level neu oder voll ausgebaut im Browser über die Landingpage (AC-05, AC-06).

## Kontext

Sprint WZ7 (`docs/sprints/geplant/WZ7-welt-betrachter/README.md`), Ticket `docs/backlog/B-460-welt-betrachter-live-vorschau.md` (Spec freigegeben 2026-10-17). Maßgeblich sind die Anforderungen, Beispiele und Ausnahmefälle dort.

- Tageszeit: Oberfläche mit Tag, Dämmerung, Nacht, Morgengrauen (alle wählbar, Fortschritt p in [0, 1)). Das Frontend bildet Morgengrauen als Sim-Phase `night` mit Fortschritt 0,9 + 0,1·p ab (so zeichnet es `daylight()` in `src/scenes/viewRules.ts`); p = 1 gibt es nicht. Reine Abbildungsfunktion, Test gegen `daylight()` mit `toBeCloseTo`. Die Setzfunktion in Go (`sim.PreviewCycle`) kennt nur die drei Sim-Phasen.
- Abhängigkeit B-472: ändert `daylight()` (dunklere Nacht); wer später landet, zieht Test und Vorschau nach.
- Hintergrund-Schichten ein/aus mit Parallaxe beim Scrollen (Scroll-Faktoren wie im Spiel); Biom ohne Schichten → Schalter ausgegraut.
- „Im Spiel öffnen“: neu oder voll ausgebaut (B-315, `engine/net/dev.go` › `/api/dev/fullsave`, nur Dev-Mode) im Browser über die Landingpage; nur Loopback. Hub-Stufe, Gebäude-Auswahl und Tageszeit gehen nicht ins Spiel.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `tools/k3c-studio/**` (Frontend, Go-Anbindung für „Im Spiel öffnen“, Tests, Codemap)
- diese Session-Datei

## Nicht-Ziele

Keine Änderung unter `src/scenes/`, `src/online/`, `src/model/`, `public/`, `data/`, `engine/`. Kein Entfernen von `leveltest.html` (WZ7.5). Keine Bürger- oder Gegner-Schalter. Keine Änderung der Nachtdarstellung (B-472).

## Schritte

1. Test zuerst für die Abbildungsfunktion der vier Tageszeit-Einträge auf Phase und Fortschritt (Morgengrauen mit p = 0, 0,5, 0,9 → `night` mit 0,9, 0,95, 0,99; Helligkeit gleich `daylight()`; `toBeCloseTo`).
2. Tageszeit-Auswahl in der Live-Vorschau, Binding auf `sim.PreviewCycle`.
3. Hintergrund-Schichten ein/aus mit Parallaxe.
4. „Im Spiel öffnen“ (neu / voll ausgebaut) über die Landingpage, nur Loopback; Fehlerfälle (Dev-Mode aus, Raum belegt) anzeigen.
5. Tests, Codemap.

## Fertig, wenn

- [ ] AC-05: vier Tageszeit-Einträge, Abbildung von Morgengrauen und Helligkeit getestet (`toBeCloseTo`); Hintergrund-Schichten mit Parallaxe schaltbar.
- [ ] AC-06: „Im Spiel öffnen“ neu und voll ausgebaut über die Landingpage, nur Loopback (Test der Anbindung).
- [ ] CI: `check_run` `task:check`, `go:test` und `go:lint` grün.

## Prüfen

`task check` und `task check:go`; über k3c-dev `check_run` (mit `checkout`). Fenster- oder Browser-Beobachtung nur mit Freigabe von 🧑.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
