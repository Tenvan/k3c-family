# B-205 · Agenten pflegen Tickets, Sprints und Sessions über MCP-Tools von k3c-dev

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** M8
- **Erstellt:** 2026-10-04
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 1, durch 🧑; umfasst B-205, B-206, B-207 (Sprint-Revision 2 bestätigt)

## Ausgangslage

Agenten legen Tickets, Sprints und Sessions heute von Hand an: Vorlage kopieren, nächste Nummer suchen, Kopf-Felder setzen, Index-Zeile in `docs/backlog/README.md`, Session-Tabelle der Sprint-README und Fahrplan `docs/sprints/README.md` nachziehen, erledigte Tickets per `git mv` nach `archiv/`. Jeder Schritt kostet Kontext und geht oft schief (rot in `tests/planning.test.ts`). k3c-dev liest die Planung schon (`tools/k3c-dev/internal/planning/`), schreibt aber nicht.

## Ziel

Ein Agent erledigt jede Planungsänderung mit einem MCP-Aufruf; Vorlage, Nummer, Index, Tabellen und Ablage stimmen danach von selbst. Der Handweg bleibt nur Rückfall, wenn k3c-dev nicht läuft.

## Beteiligte und Zielgruppen

Agenten (autonome Sessions, Planung), 🧑 (Freigaben bleiben bei 🧑).

## Anforderungen

- **Lesen:** Liste von Tickets, Sprints und Sessions mit Filter (Art, Status, Domäne, Sprint); ein Dokument als Markdown nach ID (`B-205`, `M8`, `M8.1`).
- **Anlegen:** Ticket, Sprint (in `geplant/`) und Session als Kopie der Vorlage aus `docs/vorlagen/`, mit nächster freier Nummer (Ticket) bzw. gegebener ID; Index-Zeile, Session-Tabelle und Fahrplan-Zeile entstehen mit.
- **Ändern:** Kopf-Felder (`Status`, `Prio`, `Sprint`, `Reife`, `Spec`, `Revision`, `Freigabe`, …) und einzelne `## `-Abschnitte. Folgeänderungen laufen mit: Index-Zeile, Session-Status in der Sprint-README, Ticket auf `erledigt`/`verworfen` → `backlog/archiv/` und Index-Abschnitt „Archiv“, Sprint-Status → Ordner `geplant/` ↔ `aktiv/` → `erledigt/` und Fahrplan.
- **Löschen:** nur Sprints und Sessions mit `Spec: Entwurf` im Ordner `geplant/`; Tickets werden nie gelöscht, sondern `verworfen`.
- Antworten knapp (eine Zeile je Änderung mit Pfad), wie die übrigen k3c-dev-Tools.
- Nach dem Sprint steht in `docs/arbeitsweise.md`, `CLAUDE.md` und den MCP-Instructions: Planung nur über diese Tools, Handarbeit nur als Rückfall.

## Nicht-Ziele

Commits, Push und PRs (bleiben beim Agenten bzw. `git`); Freigabe-Logik (die Zustimmung von 🧑 prüft kein Tool); Bearbeiten in der Oberfläche (B-206 zeigt nur an); erledigte Sprints umschreiben.

## Regeln und Einschränkungen

Domäne SRV, nur `tools/k3c-dev/`. Pfade entstehen nur aus geprüften IDs (`^B-\d{3}$`, `^[A-Z]+\d+$`, `^[A-Z]+\d+\.\d+$`) und Kurznamen (`^[a-z0-9-]+$`), nie aus freiem Text; geschrieben wird nur unter `docs/sprints/` und `docs/backlog/`. `Spec: freigegeben` setzt das Tool nur zusammen mit einem nicht leeren `Freigabe`-Text. Format bleibt das von `docs/vorlagen/` und `tests/planning.test.ts`. Keine neue Abhängigkeit, Komplexitäts-Budget.

## Beispiele

- `plan_create {kind: ticket, domain: SRV, typ: Idee, prio: mittel, title: "…", slug: "kurzname"}` → `B-208 angelegt: docs/backlog/B-208-kurzname.md (Index ergänzt)`.
- `plan_set {id: B-205, fields: {Status: erledigt}}` → Datei nach `backlog/archiv/`, Index-Zeile in „Archiv“.
- `plan_set {id: M8.1, fields: {Status: fertig}}` → Session-Datei und Zeile der Session-Tabelle auf `fertig`.
- `plan_section {id: B-208, section: Ausgangslage, text: "…"}` → nur dieser Abschnitt ersetzt.

## Ausnahme- und Fehlerfälle

- Unbekannte ID, unbekanntes Feld, Abschnitt nicht in der Vorlage → Fehler, keine Datei geändert.
- Ungültiger Wert (z. B. `Status: fertig` bei einem Ticket, Domäne `XYZ`) → Fehler mit erlaubten Werten.
- Löschen eines freigegebenen oder aktiven Sprints → Fehler mit Hinweis auf `verworfen`.
- Zwei Aufrufe gleichzeitig → nacheinander (eine Sperre im Prozess), nie halb geschriebene Dateien.

## Akzeptanzkriterien

- **AC-01** Go-Tests in `internal/planning` legen in einem Temp-Repo Ticket, Sprint und Session an, ändern Felder und Abschnitte, archivieren ein Ticket, verschieben einen Sprint und löschen einen Entwurf; danach ist der Stand gültig nach den Regeln von `tests/planning.test.ts` (Index, Tabellen, Ordner).
- **AC-02** Die MCP-Tools `plan_list`, `plan_get`, `plan_create`, `plan_set`, `plan_section`, `plan_delete` sind registriert, haben Instructions-Abschnitt und Roundtrip-Test.
- **AC-03** Pfad- und Werteprüfung: Tests mit `../`, absoluten Pfaden, falscher ID und ungültigem Wert ändern keine Datei.
- **AC-04** `docs/arbeitsweise.md`, `CLAUDE.md` und `internal/mcpsrv/instructions.md` nennen die Tools als Weg für Planungsänderungen, Handarbeit nur als Rückfall.

## Offene Fragen

keine

## Notizen

–
