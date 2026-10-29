# B-664 · Der Welten Editor trennt Welt-Struktur und Stufe bearbeiten und speichert an einer Stelle

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-29 🧑 Chat („jetzt die Seite mit Hilfe der Tickets und Mocks umsetzen“)

## Ausgangslage

`tools/k3c-studio/frontend/src/world/WorldStructure.tsx` zeigt unter der Live-Vorschau in einer Spalte: Welt wählen, anlegen und löschen (`WorldBar`), den Welt-Entwurf (`WorldDraftEditor`, immer offen), Insel und Stufe als Auswahllisten (`IslandBar`) **und** zusätzlich als Baum (`StructureTree`), dazu `GameLauncher`. Es gibt drei Speicher-Stellen: „Welt speichern“ und „Entwurf verwerfen“ (Welt-Entwurf), „Speichern“, „Verwerfen“ und „Neu laden“ der Planung (`PlanEditor.tsx`), dazu Gebäude-Zustände, die gar nicht gespeichert werden. Gesperrte Knöpfe nennen ihren Grund nur im Tooltip („Erst speichern oder verwerfen“).

## Ziel

Wer die Welt ordnet, sieht nur die Welt; wer eine Stufe bearbeitet, sieht nur die Stufe. Die Auswahl steht an einer Stelle, gespeichert wird an einer Stelle, und eine Sperre erklärt sich selbst.

## Beteiligte und Zielgruppen

- 🧑 Ralf baut Welten und plant Stufen im Studio.
- Sessions der Domäne DEV setzen um.

## Anforderungen

- Kopf der Seite mit einem Umschalter für die zwei Modi „Welt-Struktur“ und „Stufe bearbeiten“ (Glossar: Welt-Struktur (Modus)). Der Modus bleibt pro Fenster gemerkt (`../lib/prefs`).
- Auswahlzeile im Kopf: Welt / Insel / Stufe als je ein Auswahl-Knopf. Sie ersetzt `IslandBar` und `StructureTree`. Im Modus Welt-Struktur zeigt sie nur die Welt.
- Speicher-Stelle im Kopf: Zahl der ungespeicherten Änderungen des aktiven Modus, „Verwerfen“, „Speichern“ (Planung bzw. Welt). „Neu laden“ wandert in ein Menü.
- Wechsel von Welt, Insel, Stufe oder Modus mit ungespeichertem Entwurf: Rückfrage „Speichern / Verwerfen / Abbrechen“ statt gesperrter Knöpfe.
- Modus Welt-Struktur, links: Welt (Name, Notizen) und Liste der Inseln, je Insel eine Karte mit Name, Insel-ID, Biomen der Stufen als Farbmarken und Zahl geplanter Chunks; Reihenfolge per ↑/↓ (Ziehen siehe Offene Fragen).
- Modus Welt-Struktur, rechts: die gewählte Insel mit Name, Insel-ID, Biomen als Schaltern, je Stufe eine Karte (Biom, Inselgröße, Zahl geplanter Chunks, „Stufe bearbeiten →“ wechselt in den anderen Modus), Skalierung (HP, Schaden, Tempo) und eingeklappter Vorschau der Welt-Datei. Prüffehler stehen am Feld wie heute (`FieldError`).
- „Neue Welt“, „Kopie“, „Löschen“ im Kopf des Modus Welt-Struktur; Löschen mit Rückfrage, die Standard-Welt bleibt wie heute.
- „Im Spiel öffnen“ als geteilter Knopf im Kopf (Hauptaktion: neues Spiel; Menü: voll ausgebaut und „Spiel läuft auf“). Er ersetzt `GameLauncher` in der Spalte, die gemerkte Wahl (`k3c-studio.world.gameTarget`) bleibt.
- Backend-Aufrufe und Welt-Datei bleiben unverändert (`planLoad`, `planSave`, `planWorld*`).

## Nicht-Ziele

- Layout von Chunk-Vorschau, Live-Vorschau und Inspektor (B-665).
- Neue Bedienung der Gebäude-Zustände (B-666).
- Änderungen am Format der Welt-Datei.

## Regeln und Einschränkungen

- Domäne DEV, nur `tools/k3c-studio/frontend/src/world/`, `styles/world.css` und neue Bausteine in `tools/webkit` (Segment-Umschalter, Pfad-Knopf, Rückfrage-Dialog) auf der passenden Atomic-Design-Ebene.
- Radix nur über `@webkit`, rohe Controls nur in `atoms/` (`tests/atomicDesign.test.ts`).
- Keine neue Abhängigkeit; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.
- Eine Welt aus dem Chat (`useChatDraft`) landet im Modus Welt-Struktur.
- Bestehende Tests (`WorldDraft.test.tsx`, `WorldFiles.test.tsx`, `WorldSwitch.test.tsx`, `ChatDraft.test.tsx`, `GameLauncher.test.tsx`) bleiben grün oder werden mit gleicher Aussage umgestellt.
- Baut auf B-663 (Umbruch nach Seitenbreite) auf.

## Beispiele

- Ralf öffnet den Welten Editor → Modus Stufe bearbeiten, Auswahlzeile „Standard / Insel 1 / Stufe 1 · Oberwelt (Wald)“.
- Ralf schaltet auf Welt-Struktur, wählt Insel 1 und schaltet „Tiefe 3 (Eisenstollen)“ ab → „1 Änderung“, „Welt speichern“ ist aktiv. Klick auf „Stufe bearbeiten →“ bei Stufe 2 → Rückfrage.
- Der Chat liefert eine Welt → der Modus Welt-Struktur öffnet sich mit dem Entwurf.

## Ausnahme- und Fehlerfälle

- Speichern scheitert am Stand (Stale) → der Entwurf bleibt, die Meldung steht im Kopf (heute `Notice` mit `stale`).
- Entwurf ungültig → „Speichern“ gesperrt, im Kopf steht die Zahl der Fehler; ein Klick darauf springt zum ersten Feld mit Fehler.
- Die letzte Insel einer Welt lässt sich nicht entfernen (wie heute).

## Akzeptanzkriterien

- **AC-01** Der Welten Editor hat einen Umschalter „Welt-Struktur“ / „Stufe bearbeiten“. Im Modus Stufe bearbeiten gibt es keine Felder des Welt-Entwurfs, im Modus Welt-Struktur keine Live-Vorschau (Vitest mit Mock).
- **AC-02** Insel und Stufe wählt man nur über die Auswahlzeile im Kopf; `IslandBar` und `StructureTree` gibt es nicht mehr (Vitest).
- **AC-03** Je Modus gibt es genau einen „Speichern“- und einen „Verwerfen“-Knopf, beide im Kopf; daneben steht die Zahl der Änderungen (Vitest).
- **AC-04** Ein Wechsel von Welt, Insel, Stufe oder Modus mit offenem Entwurf zeigt die Rückfrage. „Abbrechen“ lässt Auswahl und Entwurf unverändert, „Verwerfen“ wechselt ohne Speichern, „Speichern“ speichert und wechselt (Vitest).
- **AC-05** Im Modus Welt-Struktur schreiben Inseln umordnen, anlegen, entfernen, Biome umschalten und Skalierung ändern nach „Welt speichern“ dieselbe Welt-Datei wie heute (`WorldDraft.test.tsx`, `WorldFiles.test.tsx` grün).
- **AC-06** Eine Welt aus dem Chat öffnet den Modus Welt-Struktur mit dem Entwurf (`ChatDraft.test.tsx` grün).
- **AC-07** Manuelle Abnahme durch 🧑 im Studio: Welt wechseln, Insel umordnen, Stufe bearbeiten; kein gesperrter Knopf ohne sichtbaren Grund.

## Offene Fragen

- Inseln auch per Ziehen ordnen oder nur ↑/↓? Default: nur ↑/↓, Ziehen nur, wenn es ohne neue Abhängigkeit geht. Entscheidet 🧑.
- Zeigt der Modus Welt-Struktur je Stufe eine kleine Chunk-Vorschau? Default: nein. Entscheidet 🧑.

## Notizen

- Mock: https://claude.ai/artifact/8EESxobV9HwKvqudPmDgbi (Tafeln „Modus Welt-Struktur“ und „Modus Stufe bearbeiten“).
- Glossar-Eintrag „Welt-Struktur (Modus)“ am 2026-10-29 angelegt.
- **Umgesetzt 2026-10-29** (AC-01 bis AC-06 per Vitest, Browser-Pane gesehen; AC-07 offen bei 🧑): `WorldHeader` (Umschalter, Auswahlzeile, Speichern mit Änderungszähler), `WorldGuard` (Rückfrage Speichern/Verwerfen/Abbrechen), `WorldStructure` und `WorldDraftEditor` (Inselliste, Detail, Stufenkarten), `GameLauncher` mit `<details>`-Menü.
- Abweichungen: Biome der Insel sind Schalt-Chips (`aria-pressed`) statt Checkboxen; Ziehen zum Ordnen der Inseln entfällt (nur ↑/↓, Default der offenen Frage); der Modus Welt-Struktur zeigt keine Chunk-Vorschau je Stufe (Default).
- Review-Befunde behoben: verwaiste Rückfrage, Chat-Antwort bei offener Planung (wird mit Hinweis nicht geladen), „Stufe bearbeiten →“ nach Änderung der Insel-ID, Auswahl folgt der Insel beim Umordnen.
