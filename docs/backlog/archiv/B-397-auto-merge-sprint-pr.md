# B-397 · Der PR eines Sprints wird mit Auto-Merge geöffnet

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** REL
- **Erstellt:** 2026-10-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-11, 🧑 im Chat, Revision 1

## Ausgangslage

Den PR eines Sprints öffnet die Review-Session; gemergt hat ihn bisher 🧑 von Hand (`docs/arbeitsweise.md` › Review-Session, Schritt 5). Im PR läuft nur der Check „Merge-Konflikte“; die fünf erforderlichen Checks von `develop` sind dort `SKIPPED` und gelten als erfüllt (`.github/workflows/ci.yml`). Auto-Merge ist im Repo eingeschaltet (`allow_auto_merge`), ein Squash-Merge löscht den Branch.

## Ziel

Der PR eines Sprints wird beim Öffnen mit Auto-Merge (Squash) versehen und mergt, sobald der Konflikt-Check grün ist. Nutzen: kein Handgriff mehr pro Sprint, der Push-CI auf `develop` läuft wie bisher.

## Beteiligte und Zielgruppen

🧑 (Beschluss 2026-10-11, Chat: „Auto-Merge pro PR, das ist ok. In den PRs soll ja auch nicht getestet werden.“); Agenten, die PRs öffnen.

## Anforderungen

- Die Review-Session (bzw. die letzte Session eines Doku-Sprints) schaltet Auto-Merge am PR des Sprints ein: `gh pr merge --auto --squash`.
- Ist der Konflikt-Check schon grün und der PR `CLEAN`, lehnt GitHub `--auto` ab; dann `gh pr merge --squash`.
- Ein Entwurfs-PR einer blockierten Session wird **nicht** automatisch gemergt.
- Der Merge läuft unter dem Konto des Nutzers, damit der Push-CI auf `develop` und Pages starten (ein Merge durch `GITHUB_TOKEN` löst keine Workflows aus).

## Nicht-Ziele

Tests im Pull Request (bleibt: nur Konflikt-Check); ein Workflow, der Auto-Merge serverseitig einschaltet (bräuchte ein Token des Nutzers als Secret); Änderung der Branch-Protection; automatischer Release-Tag.

## Regeln und Einschränkungen

`main` bleibt geschützt: nur 🧑 gleicht ihn ab, kein PR dorthin (`docs/arbeitsweise.md` › Branches). Prozess steht nur in `docs/arbeitsweise.md`, keine weiteren Prozess-Dokumente (CLAUDE.md).

## Beispiele

Review-Session öffnet den PR `sprint/az2` → `develop` und ruft `gh pr merge --auto --squash` auf; sobald „Merge-Konflikte“ grün ist, mergt GitHub, löscht den Branch, der Push-CI läuft auf `develop`.

## Ausnahme- und Fehlerfälle

Konflikt-Check rot oder Merge-Konflikt: Auto-Merge wartet, der Agent holt `develop` per `git merge` nach und pusht. PR bereits `CLEAN`: `gh pr merge --squash`. Sprint mit offener Hardware-Session: der PR wird trotzdem gemergt (Hardware entkoppelt).

## Akzeptanzkriterien

- **AC-01** `docs/arbeitsweise.md` nennt in der Review-Session (Schritt 5) den Befehl `gh pr merge --auto --squash` und den Rückfall `gh pr merge --squash`; der Autonome Ablauf (Schritt 5) verweist darauf; „Gemergt wird er von 🧑“ steht dort nicht mehr.
- **AC-02** `task check` grün.

## Offene Fragen

Entschieden 2026-10-11 (🧑, Chat): Auto-Merge je PR, im PR keine Tests. Serverseitiger Workflow bewusst nicht (Token). Sonst keine.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
