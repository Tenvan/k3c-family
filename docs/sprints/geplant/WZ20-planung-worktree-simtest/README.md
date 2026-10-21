# WZ20 · DEV · k3c-dev: Planungstest für Entwürfe, Worktree-Einträge vollständig und lesend, sim_test im Worktree

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-440, B-484, B-483, B-423
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Go-Planungstest (`tools/k3c-dev/internal/planning/planning_test.go`, `TestRepo`) lehnt Sprints ohne Session-Tabelle ab, auch gültige Entwürfe (B-440). Auf der Planungsseite erscheinen Einträge, die nur in einem Worktree existieren, wie Wurzel-Einträge mit Menüs, die gegen die Wurzel scheitern (B-483); ein neuer Sprint aus einem Worktree erscheint ohne Sessions und Projekt (`diffSessions` in `wtdiff.go`, B-484). `sim_test` rechnet immer mit Code und Daten der Repo-Wurzel, nicht mit dem Worktree der Session (B-423).

## Ziel

Planung und Testläufe aus Worktree-Sessions sind verlässlich: Entwurfs-Sprints bestehen den Planungstest, Worktree-Einträge erscheinen vollständig und nur lesend, `sim_test` rechnet mit dem eigenen Checkout. Am Ende sichtbar: `check_run dev:test` grün, Planungsseite mit vollständigem neuem Worktree-Sprint, `sim_test status` nennt den Checkout.

## Beteiligte und Zielgruppen

Agenten in Worktree-Sessions und 🧑 auf der Planungsseite; Umsetzung durch Agent in DEV; 🧑 gibt die Spec frei.

## Anforderungen

B-440, B-484, B-483, B-423 › Anforderungen (Specs noch Entwurf; Ziele stehen in den Tickets).

## Nicht-Ziele

Schreiben in Worktree-Einträge von der Planungsseite aus; neue Planungs-Tools (`plan_project` usw.); Testläufe selbst (Projekt TST).

## Regeln und Einschränkungen

Domäne DEV (`tools/k3c-dev/`). `sim_test` übernimmt das Argument `checkout` wie `check_run` (B-388); Tests nicht abschwächen. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Sprint `Reife: Entwurf` mit Stichpunkten statt Session-Dateien → `dev:test` grün. Neuer Sprint nur im Worktree → Planungsseite zeigt ihn mit Sessions im richtigen Projekt, Pill „neu“, ohne Menüs. `sim_test start {checkout: "<worktree>"}` → Lauf rechnet mit `data/` und `engine/` des Worktrees.

## Ausnahme- und Fehlerfälle

Unbekannter oder mehrdeutiger `checkout` bei `sim_test` → Ablehnung mit Liste der Worktrees. Sprint ohne Titel → Planungstest scheitert weiter.

## Akzeptanzkriterien

- **AC-01** Der Go-Planungstest akzeptiert Sprints mit `Reife: Entwurf` ohne Session-Dateien; `check_run dev:test` ist grün (B-440).
- **AC-02** Ein Sprint, der nur in einem Worktree existiert, erscheint auf der Planungsseite mit Sessions, Projekt und Pill „neu“ (B-484).
- **AC-03** Einträge nur im Worktree sind als solche erkennbar und nur lesend; Menüs und Rang-Knöpfe, die gegen die Wurzel scheitern würden, erscheinen nicht (B-483).
- **AC-04** `sim_test start` nimmt `checkout` und rechnet mit Code und Daten dieses Checkouts; der Status nennt den Checkout (B-423).

## Offene Fragen

🧑: Baut `sim_test` mit Worktree die Binaries im Worktree neu (Zeit, Firewall-Abfrage bei neuer EXE) oder startet es den Lauf mit `K3C_DATA_DIR` des Worktrees und dem Code der Wurzel?

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- WZ20.1 (DEV) Planungstest für Entwürfe, Worktree-Vergleich mit Sessions und Projekt neuer Sprints (AC-01, AC-02).
- WZ20.2 (DEV) Planungsseite: Worktree-Einträge nur lesend (AC-03).
- WZ20.3 (DEV) `sim_test` mit `checkout` (AC-04).
- WZ20.4 (DEV) Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
