# WT1 · DEV · Worktree-Sessions richten k3c-dev-Tools per Argument auf ihren Checkout

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-386
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-10, 🧑 im Chat (Revision 1; checkout nimmt Ordnername und Pfad; umfasst B-386 in Revision 1)

## Ausgangslage

Sessions in Desktop-App-Worktrees schreiben mit k3c-dev in die Repo-Wurzel: Der Client schickt `X-K3C-Root` mit dem Pfad der Wurzel (gemessen 2026-10-10, B-341 › Notizen, B-386 › Ausgangslage). B-275 hat nur die Antwort um die Zeile `Checkout:` ergänzt; `plan_create`, `plan_set`, `svc_*` und `check_run` treffen weiter die falsche Stelle, Agenten übertragen Dateien von Hand und prüfen in der Shell.

## Ziel

Eine Session in einem Worktree richtet jedes schreibende Tool und `check_run` mit dem Argument `checkout` auf ihren Worktree, unabhängig vom Header des Clients.

Am Ende sichtbar: `plan_create {kind: ticket, …, checkout: "<worktree>"}` legt die Datei im Worktree an und lässt die Wurzel unverändert, `check_run` mit `checkout` prüft den Worktree, ein ungültiger Wert wird mit der Liste der Worktrees abgelehnt, und `task check:dev` ist grün.

## Beteiligte und Zielgruppen

Agenten-Sessions in Worktrees; 🧑 räumt heute Dateien in der Wurzel auf, entscheidet die Offene Frage und gibt die Spec frei.

## Anforderungen

B-386 › Anforderungen. Sprint-eigen: Alles läuft in einer Umsetzungs-Session (WT1.1), weil Argument, Auflösung, Log und `instructions.md` an denselben drei Stellen hängen (`workspace.go`, `observe.go`, `tools.go`).

## Nicht-Ziele

Ablehnen ohne Header oder ohne `checkout` (Beschluss 🧑 2026-10-06, B-275). Neue Tools. Konfiguration der Desktop-App. Vergabe der Ticket-Nummern über Worktrees (B-387). Die Ursachenmessung B-341 (die Messung steht dort schon, das Ticket schließt 🧑).

## Regeln und Einschränkungen

- Domäne DEV (`tools/k3c-dev/`), Datei ≤ 400, Funktion ≤ 60 Zeilen, keine neue Abhängigkeit. Go-Tests im Paket `mcpsrv`.
- Proben nur auf Loopback (Firewall-Regel); die Tests brauchen keinen Netzzugriff.
- Nach dem Merge muss k3c-dev neu gebaut und gestartet werden (`task k3c-dev:build`), sonst kennen laufende Sessions das Argument nicht.
- Die Antwort jedes schreibenden Tools nennt weiter den Checkout (B-275/AC-03).

## Beispiele

B-386 › Beispiele. Zusätzlich: `check_run {target: task:test, checkout: "sprint-br1-fortsetzung-1b4aa6"}` führt den Lauf im Worktree aus und nennt ihn im Ergebnis; ohne `checkout` läuft er wie bisher dort, wo der Header hinzeigt.

## Ausnahme- und Fehlerfälle

B-386 › Ausnahme- und Fehlerfälle. Zusätzlich: Nennen `checkout` und Header verschiedene Checkouts, gilt `checkout`; das Log führt beide Werte nicht, nur `checkout_arg=true` und den Checkout, der galt.

## Akzeptanzkriterien

- **AC-01** Ein schreibendes Tool mit `checkout=<worktree>` ändert nur Dateien dieses Worktrees, auch wenn der Header die Wurzel nennt (`B-386/AC-01`, Test in `workspace_test.go`).
- **AC-02** Ein ungültiges oder mehrdeutiges `checkout` wird abgelehnt, ohne zu schreiben (`B-386/AC-02`).
- **AC-03** Ohne `checkout` verhält sich jedes Tool wie vorher, die Antwort nennt den Checkout (`B-386/AC-03`, bestehende Tests grün).
- **AC-04** `instructions.md` nennt das Argument, das Log führt je Aufruf `checkout_arg=true|false` (`B-386/AC-04`).

## Offene Fragen

Entschieden von 🧑 am 2026-10-10 (Chat): `checkout` nimmt Ordnername und absoluten Pfad. Der Name wird zuerst über `git worktree list --porcelain` in der Wurzel aufgelöst, sonst gilt der Wert als Pfad (`workspaceOf`).

Ob das optionale Feld an allen betroffenen Tools ohne Mehraufwand ins Schema kommt (B-386 › Offene Fragen), klärt WT1.1 zuerst; das ist ein Umsetzungsdetail, keine Entscheidung für 🧑.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WT1.1 | `WT1.1-checkout-argument.md` | Umsetzung | autonom | offen |
| WT1.2 | `WT1.2-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
