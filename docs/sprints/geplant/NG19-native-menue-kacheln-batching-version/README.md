# NG19 · CLI · Natives Spiel: Menü-Kacheln mit Illustrationen, Hintergrund-Batching, Version im Paket

- **Status:** geplant
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** B-818, B-820, B-821
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Aus NG13 (Menüs im Papierschnitt, B-812) sind drei Punkte offen: Kacheln mit Illustrationen in Haupt- und Pause-Menü (B-818, Idee, Spec Entwurf), Filterwechsel je Scheibe im Menü-Hintergrund (B-820, Klasse A) und auseinanderlaufende Version in EXE und `LIZENZEN.txt` im Worktree (B-821, Klasse A). Eingereiht am 2026-11-12 bei der Bereinigung des Backlogs (Entscheidung 009).

## Ziel

Die Menüs des nativen Spiels zeigen Kacheln mit Illustrationen, zeichnen den Hintergrund mit wenigen Zustandswechseln, und Paket und EXE tragen immer dieselbe Version.

Am Ende sichtbar: Haupt- und Pause-Menü mit Illustrationen am PC; `task native:game:paket` im Worktree mit passender Version.

## Beteiligte und Zielgruppen

🧑 Ralf (Freigabe B-818, Sichtprüfung am PC); Claude (Umsetzung).

## Anforderungen

`B-818 › Ziel`, `B-820 › Ziel`, `B-821 › Ziel`.

## Nicht-Ziele

Neue Menüeinträge, Menü-Logik; Browser-Client (Entscheidung 009).

## Regeln und Einschränkungen

Entscheidungen 005, 008, 009; Import-Regeln unter `native/k3c-game` (`game/imports_test.go`); Illustrationen nur über `assets_*`. B-820 und B-821 Klasse A, B-818 braucht Freigabe durch 🧑. `native/Taskfile.yml` teilt sich die Sperre `taskfile` mit NG9 – nicht in derselben Welle wie NG9.

- **Bereich:** `native/k3c-game/menu/**`, `native/Taskfile.yml`, `public/grafik/menue/**`
- **Abhängig von:** NG9
- **Braucht 🧑:** Freigabe
- **Sperre:** taskfile

## Beispiele

Pause-Menü › Eintrag „Speichern“ zeigt seine Illustration; fehlt das Bild, erscheint die Text-Karte.

## Ausnahme- und Fehlerfälle

Illustration fehlt → Text-Karte (B-818/AC-01).

## Akzeptanzkriterien

- **AC-01** Menü-Kacheln mit Illustrationen (`B-818/AC-01` bis `B-818/AC-03`).
- **AC-02** Hintergrund wechselt den Filter je Bild höchstens einmal, e2e grün (`B-820/AC-01`, `B-820/AC-02`).
- **AC-03** Version in EXE und `LIZENZEN.txt` gleich, kein unnötiger Neubau (`B-821/AC-01`, `B-821/AC-02`).

## Offene Fragen

Ablageort der Illustrationen (Gruppe `menue` angenommen, ungeprüft) und Freigabe von B-818: entscheidet 🧑.

## Sessions

Entwurf (Stichpunkte, Session-Dateien vor dem Aktivieren):

- NG19.1 Hintergrund-Batching und Version im Paket (AC-02, AC-03)
- NG19.2 Menü-Kacheln mit Illustrationen (AC-01)
- NG19.3 Sichtprüfung am PC durch 🧑 (`B-818/AC-03`)
- NG19.4 Review (alle)

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
