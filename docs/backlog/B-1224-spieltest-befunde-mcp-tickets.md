# B-1224 · MCP-Tools playtest_* und Prompt-Knöpfe machen aus angehakten Befunden Tickets

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WN1
- **Projekt:** WZN
- **Erstellt:** 2026-11-22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, Chat 2026-11-22
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Nach B-1223 liegen Transkript, Marker mit Spielkontext, Bilder und Fragebogen je Session vor. Tickets entstehen nur über `plan_create` (Planung nur über MCP, keine ungeplanten Tickets). Das Muster Prompt-Knöpfe gibt es heute in `frontend/src/assets/prompts/PromptButtons.tsx` (Titel kopiert Prompt, ✦ öffnet neue Code-Session); es wandert mit WZ63 ins Studio bzw. in gemeinsamen Code.

## Ziel

Ralf löst die Auswertung per Knopf aus, Claude schlägt Befunde vor, Ralf hakt an, und erst dann werden daraus Tickets mit Projekt und Sprint. Die Workbench selbst ruft keine KI und hält keinen API-Key.

## Beteiligte und Zielgruppen

Ralf (🧑) entscheidet, welche Befunde Tickets werden; Claude wertet aus und legt an.

## Anforderungen

- MCP-Tools (in `internal/mcpsrv/tools_playtest.go`, Beschreibung mit „Nutze es bei: … Statt: …“):
  - `playtest_list`: Sessions mit Stand, Dauer, Markern.
  - `playtest_get {id}`: Session, Transkript-Segmente, Marker mit Kontext, Fragebogen, Befunde; große Ausgaben mit `limit`/Cursor.
  - `playtest_frame {id, t}`: zieht ein Bild an Zeitpunkt `t` (Unix-ms) aus dem Video, gibt den Pfad zurück.
  - `playtest_findings_set {id, findings}`: schreibt oder aktualisiert `findings.json`.
- Befund: `id`, Zitat, Zeitpunkt, Bild, Spielkontext, Vorschlag Typ/Prio/Projekt/Sprint, `duplicate_of` (offenes Ticket) oder leer, `selected` (Haken von Ralf), `ticket` (ID nach Anlage).
- Prompt-Knopf **Auswerten**: Prompt lässt Claude die Session lesen, Befunde bilden und jeden gegen offene Tickets prüfen (`plan_list`; Treffer → `duplicate_of` statt neuem Ticket).
- Seite zeigt Befunde mit Haken, Bild und Sprung ins Video; Ralf kann Vorschlag für Projekt/Sprint ändern.
- Prompt-Knopf **Tickets anlegen**: Claude legt nur angehakte Befunde ohne `duplicate_of` per `plan_create` an (Spec Entwurf), trägt die ID in `ticket` ein; Duplikate bekommen einen Hinweis im vorhandenen Ticket statt eines neuen.
- Prompt-Knöpfe nutzen das vorhandene Muster (nach WZ63 aus gemeinsamem Code), kein Nachbau.

## Nicht-Ziele

- Anthropic-API oder `claude -p` aus der Workbench (verworfen 🧑 2026-11-22).
- Automatisches Anlegen ohne Haken.
- Auswertung über mehrere Sessions hinweg.

## Regeln und Einschränkungen

- Tickets nur über `plan_create`, immer mit Projekt und Sprint; fehlt ein passender Sprint, schlägt Claude einen vor statt ungeplant anzulegen.
- MCP-Regeln der Workbench: verdichtete Antworten, `checkout`-Argument für schreibende Tools.

- **Bereich:** `tools/k3c-workbench/internal/mcpsrv/tools_playtest.go`, `tools/k3c-workbench/internal/mcpsrv/tools.go`, `tools/k3c-workbench/internal/playtest/**`, `tools/k3c-workbench/frontend/src/playtest/**`
- **Abhängig von:** B-1223
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

- Transkript: „Ich finde den Bauarbeiter nicht“ bei Marker 2 → Befund mit Bild, Vorschlag Typ Problem, Projekt WZG…; Ralf hakt an → Tickets anlegen → neues Ticket mit Zitat, Bild-Pfad und Kontext, `ticket` gesetzt.
- Befund passt zu offenem Ticket B-1100 → `duplicate_of: B-1100`, nach Anlegen steht ein Hinweis in B-1100 statt eines neuen Tickets.

## Ausnahme- und Fehlerfälle

- Session ohne Transkript → `playtest_get` meldet es, Auswerten-Knopf gesperrt.
- `findings.json` geändert, während die Seite offen ist → Seite lädt neu (Ereignis).
- Ticket-Anlage scheitert für einen Befund → die anderen laufen weiter, Fehler steht am Befund.

## Akzeptanzkriterien

- **AC-01** Go-Tests für die vier MCP-Tools mit Beispiel-Session (Format, `limit`, Fehler bei unbekannter ID).
- **AC-02** Beide Prompt-Knöpfe erzeugen den gefüllten Prompt mit Session-ID (Test der Füll-Funktion).
- **AC-03** Durchlauf mit Beispiel-Session: Auswerten → Haken → Tickets anlegen ergibt nur Tickets für angehakte Befunde ohne Duplikat (🧑 prüft das Ergebnis).
- **AC-04** `check_run task:check:diff` grün.

## Offene Fragen

keine

## Notizen

–
