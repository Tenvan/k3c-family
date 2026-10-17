# WZ7.3 · Live-Vorschau mit Phaser-Bühne im Studio

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz7/3-live-vorschau
- **Abhängig von:** WZ7.1, WZ7.2
- **Tickets:** B-460
- **Kriterien:** AC-04, AC-08

## Ziel

Die Live-Vorschau zeichnet den Snapshot aus WZ7.1 mit der Spielgrafik auf einer wiederverwendbaren Phaser-Bühne im Studio; Hub-Stufe aus `levels` und Gebäude ein/aus sind schaltbar (AC-04).

## Kontext

Sprint WZ7 (`docs/sprints/geplant/WZ7-welt-betrachter/README.md`), Ticket `docs/backlog/B-460-welt-betrachter-live-vorschau.md` (Spec freigegeben 2026-10-17). Maßgeblich sind die Anforderungen, Beispiele und Ausnahmefälle dort.

- Phaser kommt in der Version der Wurzel-`package.json` ins Studio-Frontend (Auflösung über Wurzel-`node_modules` oder Eintrag in `tools/k3c-studio/frontend/package.json` legt die Session fest); keine weitere Abhängigkeit.
- Die Bühne (Szene + Preload) ist die einzige im Studio; WZ10/WZ11 nutzen sie ohne Kopie wieder (`B-460 › Anforderungen`).
- Zeichnen mit `StageView`/`WorldRenderer` aus `src/scenes/` über `@game`, **ohne** Änderung dort. Braucht das eine Änderung unter `src/scenes/` oder `src/online/`, wird die Session `blockiert` und ein CLI-Ticket angelegt.
- Hub-Stufe: alle Einträge aus `data/hub.json` › `levels`; die Material-Stufen kommen reihum aus dem Snapshot (WZ7.1/WZ7.8), das Frontend rechnet keine Stufen. Gebäude ein/aus ist reine Auswahl für den Snapshot (`kind@x`). Keine Bürger, keine Gegner.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `tools/k3c-studio/**` (Binding auf WZ7.1, Frontend: Phaser, Bühne, Seite, Tests, Codemap)
- diese Session-Datei

## Nicht-Ziele

Keine Änderung unter `src/scenes/`, `src/online/`, `src/model/`, `public/`, `data/`, `engine/`. Keine Tageszeit, Hintergrund-Schichten oder „Im Spiel öffnen“ (WZ7.4). Kein Entfernen von `leveltest.html` (WZ7.5). Keine zweite Bühne.

## Schritte

1. Phaser im Studio-Frontend einführen (Version der Wurzel), Build und Typecheck.
2. Bühne bauen: Szene + Preload der Spielgrafik wie das Spiel (über `@game`), `StageView`/`WorldRenderer` zeichnen den Snapshot; Eingabe nur Snapshot, kein Spielserver.
3. Binding auf `net.Preview` (WZ7.1): Optionen Seed, Biom, Hub-Stufe, Gebäude-Auswahl.
4. Auswahl der Hub-Stufe aus `levels` (zusätzlicher Eintrag in einer Test-Fixture erscheint ohne Code-Änderung), Gebäude-Liste aus dem Snapshot, einzeln ein/aus; gleiche Eingaben → gleiches Bild.
5. Frontend-Tests; `git diff` ohne Änderung unter `src/scenes/`; Codemap.

## Fertig, wenn

- [ ] AC-04: Live-Vorschau zeichnet den Snapshot auf der wiederverwendbaren Phaser-Bühne; Hub-Stufen aus `levels`, Gebäude ein/aus; gleiche Eingaben → gleiches Bild (Frontend-Tests).
- [ ] AC-04: `git diff` ohne Änderung unter `src/scenes/`.
- [ ] AC-08: `check_run` `task:check`, `go:test` und `go:lint` grün.

## Prüfen

`task check` und `task check:go`; über k3c-dev `check_run` (mit `checkout`). Fenster-Beobachtung nur mit Freigabe von 🧑.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
