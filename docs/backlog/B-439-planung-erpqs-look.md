# B-439 · Die Planungsseite von k3c-dev zeigt und bedient die Planung wie die erp-qs-Workbench

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ5
- **Projekt:** WZG
- **Erstellt:** 2026-10-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Planungsseite von k3c-dev (`tools/k3c-dev/frontend/src/planning/`, `styles/planning.css`) stammt aus der ErpApi-Workbench und ist älter als die Workbench in erp-qs-automation (`tools/dev-workbench/frontend`, Spec `workbench-seiten.md` 1.3). Dort gibt es eine nach Wirkung gegliederte Filterleiste mit Checklisten und Chips, Splitter zwischen Projekten und Backlog, Pills, Sprint-Panels mit Session-Zeilen, ein Detail nur bei Auswahl und einen Lesemodus für Tickets. In k3c stehen die Filter als lange Chip-Reihen in den Spalten, das Detail zeigt nur Markdown oder Listen, einen Lesemodus gibt es nicht.

## Ziel

Die Planungsseite sieht aus und bedient sich wie die erp-qs-Workbench, mit k3c-Daten und den Teilen, die nur k3c hat. Wer zwischen beiden Workbenches wechselt, findet Filter, Karten, Backlog, Detail und Lesemodus an derselben Stelle.

## Beteiligte und Zielgruppen

🧑 plant und entscheidet in k3c-dev; Agenten bekommen Prompts aus der Seite.

## Anforderungen

- Filterleiste oben, nach Wirkung gegliedert (Gemeinsam: Suche · Projekte: Zustand, Sprints · Backlog: Prio, Umgebung, Einplanung, Projekt, Domänen), je Gruppe eine Dropdown-Checkliste mit Zähler, angehakte Werte als Chips mit ×, „alle zurücksetzen“, Trefferzahl. Gemerkt ohne Freitext.
- Projekte links, Backlog rechts, dazwischen ein verstellbarer, gemerkter Splitter; das Detail erscheint nur bei Auswahl unter dem Backlog, mit eigenem Splitter, schließbar.
- Projektkarte: Rang-Kachel mit hoch/runter, Pills, Sprint-Chips, Fortschrittsbalken, „Nächste Session“ mit Prompt, eingebettete Sprint-Panels mit Session-Zeilen (Checkbox, Status-Pill, Nr., Titel, Modus, Abhängigkeiten, Prompt).
- Backlog je Domäne einklappbar, Sammel-Checkbox je Domäne, Ziel-Auswahl, Prompt für markierte Tickets; Prio, Projekt, Umgebung direkt änderbar (FieldMenu).
- Lesemodus für Ticket, Sprint und Session: Dialog mit Inhaltsverzeichnis, Scroll-Spy, Suche (Strg+F), j/k, Fortschritt der Akzeptanzkriterien, Esc.
- Bleibt: GitHub-Leiste (PR/CI/Merge je Sprint, develop-CI), Dokumente-Reiter, Spec und Reife, Domänen, ABN und „Ohne Projekt“, Live-Update über `planning:changed`.
- Helles und dunkles Theme lesbar.

## Nicht-Ziele

- Datenmodell der Planung (`planning.Data`, Typen in `api/types.ts`) bleibt; nur ein Lese-Binding für den Volltext kommt dazu.
- Was k3c nicht hat, entfällt: Contributors, Rolle, Jira, Bearbeitung-Filter, Projekt-Worktree-Edits, Radar-Änderungslog.
- Übernahme der Spec `workbench-seiten.md` 1.3 nach k3c (eigenes Ticket, falls gewünscht).
- Keine neue Abhängigkeit (kein react-markdown); der Lesemodus nutzt `ui/MarkdownView`.

## Regeln und Einschränkungen

Domäne DEV; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; reine Logik mit Vitest; React 19 und Radix Themes 3.3 wie bisher; Planungsänderungen nur über `planningSet`.

## Beispiele

- „Prio ▾“ öffnen, „hoch“ anhaken → Chip „Prio: hoch ×“ erscheint, Backlog zeigt nur hohe Tickets, Trefferzahl sinkt.
- Doppelklick auf den Titel im Detail eines Tickets → Lesemodus mit Rubriken links, AC-Fortschritt oben.

## Ausnahme- und Fehlerfälle

- Volltext nicht lesbar (Datei weg) → Lesemodus zeigt einen Fehlerhinweis statt leer.
- Gemerkter Filterwert ohne Daten (Domäne weg) → fällt still aus der Auswahl.
- Auswahl verschwindet nach `planning:changed` → Detail schließt.

## Akzeptanzkriterien

- **AC-01** Die Filterleiste hat die drei Bereiche mit Checklisten, Chips mit ×, „alle zurücksetzen“ und Trefferzahl; `planning/filters.test.ts` prüft Gruppen, Umschalten und Chips.
- **AC-02** Projekte und Backlog sind durch einen gemerkten Splitter getrennt; das Detail erscheint nur bei Auswahl, mit eigenem Splitter und Schließen-Knopf (Beobachtung im Mock).
- **AC-03** Projektkarten, Sprint-Panels und Session-Zeilen zeigen Rang, Pills, Fortschritt, nächste Session und Prompt-Knöpfe wie erp-qs (Screenshot im Mock, hell und dunkel).
- **AC-04** Der Backlog ist je Domäne einklappbar, hat Sammel-Checkboxen, Ziel-Auswahl und Prompt; `planning/Backlog.test.tsx` deckt Markieren ab.
- **AC-05** Der Lesemodus öffnet für Ticket, Sprint und Session den Volltext aus `PlanningGet` mit Inhaltsverzeichnis, Suche, j/k, AC-Fortschritt und Esc; die reine Logik prüft ein Vitest.
- **AC-06** GitHub-Leiste, Dokumente-Reiter, Spec/Reife, FieldMenu und Rang hoch/runter funktionieren weiter; `task check` und `dev:test` sind grün.

## Offene Fragen

- Soll die Spec `docs/standards/workbench-seiten.md` auf 1.3 (erp-qs) gehoben werden, ohne Contributors/Rolle/Jira? Entscheidet 🧑.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
