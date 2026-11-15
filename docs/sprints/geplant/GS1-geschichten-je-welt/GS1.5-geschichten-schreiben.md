# GS1.5 · Texte: Wortschatz und Zeilen der vier Welten (B-976)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Lane:** –
- **Modell:** opus
- **Umgebung:** live
- **Branch:** sprint/gs1
- **Abhängig von:** GS1.4
- **Tickets:** B-976
- **Kriterien:** AC-06

## Ziel

`standard`, `sengoku`, `nil` und `britannien` haben Wortschatz und alle Zeilen der Geschichte; 🧑 hat die Liste zur Abnahme.

## Kontext

Spec: `docs/backlog/B-976-geschichten-vier-welten.md` (R1–R5, AC-01 bis AC-04), Format und Regeln: `docs/rules/welten.md` § 2, § 6.2 und B-965 › Beispiele (die Zeilen für Sengoku und Nil von 🧑 werden übernommen). **Hauptcheckout, kein Worktree:** `sengoku`, `nil`, `britannien` sind nicht eingecheckt (`.gitignore` › `data/worlds/*`) und liegen nur in der Repo-Wurzel. **Voraussetzung:** Workbench neu gebaut und gestartet, `world_story_set` ist als MCP-Tool vorhanden (sonst abbrechen und im Ergebnis melden). Inseln: Sengoku Reisfeld-Ebene, Bambuswälder, Bergfestung; Nil Oasen am Fluss, Wüstenrand, Grabkammern; Britannien Hochmoor, Steinkreis-Hügel, Hügelgräber; Standard Insel 1. Endboss überall: Kristallherz-Wächter (`data/bosses.json`). Klang: Sengoku japanisch, Nil ägyptisch, Britannien keltisch-altenglisch, Standard märchenhaft. Je Zeile 2–3 echte deutsche Wörter in Großbuchstaben, höchstens 64 Zeichen, Wortschatz höchstens 24 Wörter.

## Erlaubte Dateien

- `data/worlds/**`

## Nicht-Ziele

Neue Welten, Änderungen an Inseln, Overrides oder Chunks, Mechanik der Flaschenpost, Golden neu schreiben, Welt „Polar“.

## Schritte

1. `world_list`, `world_get` je Welt: Inseln, Etappen, Stand der Geschichte.
2. Je Welt Wortschatz festlegen (eigene Laute, höchstens 2 Wörter gemeinsam mit einer anderen Welt).
3. Zeilen schreiben: `start`, `end`, je Etappe, drei `bottles`, je Insel `arrival`, `boss`, `done`; ein Bogen je Welt.
4. `world_story_set` je Welt; `world_check` ohne Fehler und Hinweise.
5. `check_run task:check:go` (Golden unverändert), `check_run task:check:diff`.
6. Ergebnis: Liste aller Zeilen je Welt mit einer deutschen Deutung je Zeile für die Abnahme durch 🧑; Commit nennt die REG-Änderung an `standard.json`.

## Fertig, wenn

- [ ] AC-06: Vier Welten vollständig, `world_check` ohne Fehler und Hinweise, Golden unverändert (B-976/AC-01 bis AC-03).
- [ ] Liste für 🧑 im Ergebnis; B-976/AC-04 (Abnahme) bleibt offen bei 🧑.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
