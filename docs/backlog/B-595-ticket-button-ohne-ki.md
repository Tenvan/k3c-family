# B-595 · Jede Seite hat einen Button, der ohne KI ein Ticket zur Seite anlegt

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WZ28
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-24, Ralf im Chat (mit WZ28 Revision 1)

## Ausgangslage

Tickets entstehen heute über Claude (`plan_create`) oder von Hand in `docs/backlog/`. Ralf (2026-10-24): Er muss auf einem Notebook ohne Internet und ohne Claude testen und dabei Tickets anlegen können.

## Ziel

Auf jeder Seite von Workbench und Studio legt ein Button ohne KI ein Ticket an, das den Seitenkontext schon enthält.

## Beteiligte und Zielgruppen

Ralf beim Testen unterwegs; Agent DEV; später arbeitet Claude die Tickets ab.

## Anforderungen

- Ein Button „Ticket“ (immer sichtbar, Kopfzeile) öffnet einen Dialog: Titel, Beschreibung, Typ (Idee, Problem, Frage), Prio, Domäne (Vorgabe aus der Seite); Senden legt das Ticket mit der Ticket-Vorlage an (nächste freie Nummer, Index) wie `plan_create`.
- Seitenkontext wird automatisch eingetragen: App, Seite, ausgewählter Eintrag (z. B. Slot, Chunk, Lauf), Version, Zeit.
- Funktioniert offline: nur lokale Dateien, kein Netzwerk, kein Claude, kein MCP.
- Gleicher Baustein in allen Wails-Apps über `@webkit` (Atomic Design), Backend über das gemeinsame Planungs-Paket.
- Anzeige der zuletzt angelegten Tickets im Dialog; Ticket später in Planung bearbeitbar.
- Anlegen im aktuellen Checkout; Hinweis, wenn es kein Git-Checkout ist.

## Nicht-Ziele

Tickets abarbeiten oder Specs schreiben (bleibt bei Claude); Synchronisation zwischen Rechnern (geht über Git).

## Regeln und Einschränkungen

Vorlage und Nummernvergabe wie die Planungswerkzeuge (`tests/planning.test.ts` prüft das); der Kern dazu zieht nach `tools/assetkit/ticket`, weil das Studio `tools/k3c-workbench/internal/planning` nicht importieren kann (WZ28 › Regeln); Studio schreibt Tickets nur im Backlog-Ordner; Atomic Design (`docs/arbeitsweise.md` › Oberflächen der Wails-Programme), der Button sitzt in `tools/webkit/organisms/AppHeader.tsx`; Texte zweisprachig.

## Beispiele

Auf der Grafik-Seite Slot `shardling` gewählt → Dialog mit Kontext „Studio · Grafik · Slot shardling“, Ralf tippt „sieht aus wie Pilz“ → B-nnn im Backlog.

## Ausnahme- und Fehlerfälle

Backlog nicht beschreibbar → Fehlermeldung, Eingabe bleibt erhalten.

## Akzeptanzkriterien

- **AC-01** Ein Baustein `TicketButton` in `@webkit` und ein gemeinsames Backend legen ein Ticket nach Vorlage an (Go-Test, Vitest).
- **AC-02** Der Button steht auf allen Seiten von k3c-workbench und k3c-studio (Test über die Seitenliste).
- **AC-03** Das Ticket enthält den Seitenkontext (Test).
- **AC-04** Ohne Netzwerk funktioniert der Ablauf (Test ohne Netz, Beobachtung am Notebook).

## Offene Fragen

keine. Entschieden (🧑, 2026-10-24): Ralf arbeitet allein, der Stand ändert sich nur durch ihn; nächste freie Nummer genügt, keine Kollisionsbehandlung nötig.

## Notizen

Aus Ralfs Wunsch 2026-10-24. Spec: Entwurf. Sprint WZ28.
