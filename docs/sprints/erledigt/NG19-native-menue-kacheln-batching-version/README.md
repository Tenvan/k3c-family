# NG19 · CLI · Natives Spiel: Menü-Kacheln mit Illustrationen, Hintergrund-Batching, Version im Paket

- **Status:** erledigt
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-818, B-820, B-821, B-826
- **Start-Commit:** 9b2ef1dcb
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-13 (Dialog „NG19 vorziehen“; NG19.2 erst nach Freigabe B-818)

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
- **AC-04** Optionen schalten zwischen Fenster, rahmenlos und Vollbild (`B-826/AC-01` bis `B-826/AC-04`; Wunsch Ralf 2026-11-13).

## Offene Fragen

Ablageort der Illustrationen (Gruppe `menue` angenommen, ungeprüft) und Freigabe von B-818: entscheidet 🧑, vor NG19.2.

Vorgezogen am 2026-11-13 (Dialog Ralf „NG19 vorziehen“): Die Abhängigkeit von NG9 und die Sperre `taskfile` gelten nicht gegen NG9.5 (reine Hardware-Prüfung am Mini-PC).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG19.1 | `NG19.1-batching-version.md` | Umsetzung | autonom | fertig |
| NG19.2 | `NG19.2-menue-kacheln.md` | Umsetzung | autonom | fertig |
| NG19.3 | `NG19.3-review.md` | Review | autonom | fertig |
| NG19.4 | `NG19.4-sichtpruefung-pc.md` | Review | Mensch | fertig |
| NG19.5 | `NG19.5-fenstermodus.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-13: AC-01 bis AC-04 laut Ergebnis von NG19.1, NG19.2, NG19.5; Sichtprüfung NG19.4 und Review-Freigabe durch 🧑 („NG19.3 +4 ok“). Review-Befunde (k3c-reviewer) behoben: Fenster-Lage nach Vollbild, Größe ≤ 90 % des Monitors. `task check` und `task check:go` grün. Neue Tickets: B-826. Kein Abnahme-Ticket für Gen (keine Grafik geändert).
