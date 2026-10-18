# B-486 · Chunk-Editor: Speichern nach externer Änderung von islands.json schweigt

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Gefunden in der Abnahme WZ8.7 (Punkt 5, 2026-10-18, 🧑 am PC): Wird `data/islands.json` von außen geändert und danach im Chunk-Editor des Welt-Betrachters (`tools/k3c-studio`) gespeichert, wird nichts mehr gespeichert und es erscheint keine Meldung. Erwartet laut WZ8 AC-12 (Stand-Prüfung): Ablehnung mit „data/islands.json geändert, bitte neu laden“, der Entwurf bleibt erhalten.

## Ziel

Die Stand-Prüfung beim Speichern meldet eine veraltete Datei sichtbar am Editor, überschreibt sie nie und behält den Entwurf.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Datei seit dem Laden von außen geändert → Speichern wird abgelehnt, der Editor zeigt „data/islands.json geändert, bitte neu laden“, der Entwurf bleibt unverändert. Ursache offen: Die Ablehnung kommt nicht bis zur Oberfläche, oder die Stand-Prüfung greift zu früh oder zu spät (Fehler wird verschluckt).

## Akzeptanzkriterien

- **AC-01** Test (Go und Frontend): Wird die Datei nach dem Laden geändert, lehnt Speichern ab, `data/islands.json` bleibt byte-gleich zur externen Änderung, und die Oberfläche zeigt die Meldung „bitte neu laden“.
- **AC-02** Der Entwurf im Editor bleibt nach der Ablehnung erhalten.
- **AC-03** Abnahme von 🧑 am PC: Punkt 5 der Abnahme-Liste WZ8.7 wiederholt, Ergebnis wie erwartet.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Befund aus WZ8.7, betrifft WZ8 AC-12. Der Review WZ8.6 hat den Editor nur durch Tests geprüft; offenbar deckt kein Test die Anzeige der Ablehnung ab. Blockiert die Abnahme von WZ8 (AC-10).

Untersuchung 2026-10-18 (Claude, statisch): Go (`PlanSave` in `tools/k3c-studio/app_plan.go`) und Oberfläche (`usePlan.ts`, `PlanEditor.tsx`) sind für die Stale-Meldung richtig verdrahtet; `wails.ts` reicht das Ergebnis unverändert durch. Go-Test `TestPlanSaveNachSpeichernLehntExterneAenderungAb` (erst speichern, dann externe Änderung, dann zweites Speichern mit altem Stand) ist grün.

Angaben von 🧑: Speichern war anklickbar, beim Klick keine Meldung, Datei nicht beschrieben; `data/islands.json` wurde in einem Editor geändert, ohne Reset nach Punkt 4. Das Studio-EXE (13:05) ist jünger als der letzte Commit (12:57), ein veralteter Build ist unwahrscheinlich.

Gefunden und behoben (2026-10-18): `usePlan.save` fing keine Ablehnung des Wails-Aufrufs ab. Lehnt `PlanSave` ab (Serialisierung der Argumente, Bindung, Go-Panic), blieb das Speichern ohne jede Meldung, genau wie von 🧑 beobachtet. Test in `PlanEditor.test.tsx` (rot, jetzt grün); der Editor zeigt jetzt „Speichern fehlgeschlagen: <Grund>“ und behält den Entwurf. **Ob dies die Ursache von Punkt 5 war, ist offen:** Der Auslöser (warum der Aufruf nach der externen Änderung ablehnt) ist nicht gefunden; mit der Meldung steht der Grund beim nächsten Versuch am Editor. Nächster Schritt: 🧑 wiederholt Punkt 5 mit neuem Build (`task k3c-studio:build`) und nennt den angezeigten Text; zeigt er die Stale-Meldung, ist B-486 behoben.
