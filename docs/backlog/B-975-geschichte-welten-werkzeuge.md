# B-975 · Die Welten-Werkzeuge schreiben und prüfen die Geschichte, „Welt anlegen“ erzeugt sie mit

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WN4
- **Projekt:** WZN
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Welt-Dateien ändern nur die Welten-MCP-Tools der Workbench (`world_create`, `world_set`, `world_override_set`, `world_chunks_set`, lesen `world_get`, `world_list`, prüfen `world_check`; `tools/k3c-workbench/internal/mcpsrv/tools_world.go`, Logik in `tools/k3c-workbench/internal/world/`). Der Welten-Editor legt Welten über „Welt anlegen“ an (`tools/k3c-workbench/frontend/src/world/NewWorld.tsx`, Backend `internal/world/edit.go`). Das Feld `story` aus B-965 kennt keines der Werkzeuge. Abgespalten aus B-965 (🧑 2026-11-15, Entstehung).

## Ziel

Claude schreibt Wortschatz und Zeilen einer Welt über die Welten-MCP-Tools; „Welt anlegen“ legt die Geschichte gleich mit an, und `world_check` meldet eine fehlende oder ungültige Geschichte.

## Beteiligte und Zielgruppen

Claude schreibt die Geschichten über die Tools, 🧑 legt Welten im Editor an und nimmt die Texte ab (B-976).

## Anforderungen

Aus B-965 (🧑 2026-11-15, wörtlich):

- **Entstehung: Claude schreibt, 🧑 nimmt ab.** Je Welt ein kleiner Wortschatz und feste Zeilen in der Welt-Datei, geschrieben über die Welten-MCP-Tools; „Welt anlegen“ erzeugt die Geschichte gleich mit.

Ergänzt:

- **R1 `world_story_set`:** Neues MCP-Tool setzt `story` der Welt (`lexicon`, `start`, `etappen`, `end`, `bottles`) und je Insel (`arrival`, `boss`, `done`) in einem Aufruf oder je Teil; es schreibt nur, wenn die Prüfung aus B-965 besteht, und nennt sonst Feld und Zeile. Andere Felder der Welt bleiben byte-gleich.
- **R2 `world_get`:** Zeigt die Geschichte: Wortschatz und je Wendepunkt die Zeilen, fehlende Einträge als „fehlt“.
- **R3 `world_check`:** Meldet Fehler der Geschichte (Prüfungen aus B-965) und als Hinweis jeden fehlenden Eintrag (Welt ohne `story`, ohne `start`, `end` oder `bottles`, Insel ohne `arrival`, `boss` oder `done`). Hinweise sind keine Fehler.
- **R4 `world_create`:** Nimmt die Geschichte als optionales Argument und schreibt sie mit; fehlt sie, legt es die Welt ohne `story` an und die Antwort endet mit dem Hinweis „Geschichte fehlt: world_story_set“.
- **R5 Editor „Welt anlegen“:** Nach dem Anlegen zeigt die Welt-Seite den Stand der Geschichte (vorhanden oder fehlt) und einen Prompt-Button „Geschichte schreiben“, der den Auftrag für Claude in die Zwischenablage legt (Welt-ID, Klang, Verweis auf `world_story_set`). Der Editor bearbeitet die Texte nicht selbst.
- **R6 Beschreibung der Tools** endet nach der Workbench-Regel mit „Nutze es bei: … Statt: …“; `docs/standards/workbench-seiten.md` nennt das neue Tool.

## Nicht-Ziele

Texte schreiben (B-976), Texteditor für Geschichten in der Oberfläche, Format und Prüfregeln selbst (B-965, die Workbench ruft die Engine-Prüfung), Anzeige im Spiel (B-974).

## Regeln und Einschränkungen

- Oberfläche nach Atomic Design (`@webkit`, `tests/atomicDesign.test.ts`); Prüfung durch den Agenten `k3c-workbench`.
- Das neue Tool ist erst nach Merge, Pull und Neubau der Workbench nutzbar (Sperre `exe`; B-976 hängt davon ab).

- **Bereich:** `tools/k3c-workbench/internal/world/**`, `tools/k3c-workbench/internal/mcpsrv/**`, `tools/k3c-workbench/frontend/src/world/**`, `tools/k3c-workbench/frontend/src/api/typesWorld.ts`, `docs/standards/workbench-seiten.md`
- **Abhängig von:** B-965
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** exe

## Beispiele

`world_story_set {world: "sengoku", lexicon: [...], start: ["Oru kamae KRONE no sen. Shiro nu BRENNT."], islands: {island1: {arrival: [...], boss: [...], done: [...]}}}` → Datei geschrieben, Antwort nennt die gesetzten Einträge. `world_check {world: "nil"}` vor B-976 → „Hinweis: Geschichte fehlt (story)“.

## Ausnahme- und Fehlerfälle

- Unbekannte Welt oder Insel → abgelehnt, die Meldung nennt die bekannten IDs.
- Zeile verletzt eine Prüfung (Wort fehlt im Wortschatz, zu lang, falsche Zahl echter Wörter) → nichts geschrieben, Meldung nennt Feld und Zeile.
- Wortschatz wird verkleinert, vorhandene Zeilen nutzen ein entferntes Wort → abgelehnt.
- Leeres Argument für einen Eintrag (`[]`) → Eintrag wird entfernt.
- Aufruf aus einem Worktree ohne `checkout` → Verhalten wie bei den übrigen `world_*`-Tools.

## Akzeptanzkriterien

- **AC-01** Go-Test (`tools/k3c-workbench/internal/mcpsrv`): `world_story_set` schreibt Welt- und Insel-Geschichte; die übrigen Felder der Datei sind danach byte-gleich; eine ungültige Zeile wird mit Feld und Zeile abgelehnt und nichts geschrieben.
- **AC-02** Go-Test: `world_get` zeigt Wortschatz und Zeilen, `world_check` nennt fehlende Einträge als Hinweis und Verstöße als Fehler.
- **AC-03** Go-Test: `world_create` mit Geschichte schreibt sie mit; ohne Geschichte endet die Antwort mit dem Hinweis auf `world_story_set`.
- **AC-04** Vitest (`WorldPage.test.tsx`): Nach „Welt anlegen“ zeigt die Seite „Geschichte fehlt“ und den Prompt-Button; der kopierte Text nennt Welt-ID und `world_story_set`.
- **AC-05** `check_run task:check:dev` grün.

## Offene Fragen

- „Welt anlegen erzeugt die Geschichte gleich mit“: Ein Formular kann keine Texte erfinden. → Default: `world_create` nimmt die Geschichte als Argument (Claude schreibt sie beim Anlegen mit); der Editor zeigt nach dem Anlegen „Geschichte fehlt“ und einen Prompt-Button für Claude (vorläufig, 🧑 bestätigt).
- Eigenes Tool oder `world_set` erweitern? → Default: eigenes `world_story_set`, weil die Prüfung und die Antwort eigene sind (vorläufig).

## Notizen

Abgespalten aus B-965 (2026-11-15, Zerlegung wegen Größe L).
