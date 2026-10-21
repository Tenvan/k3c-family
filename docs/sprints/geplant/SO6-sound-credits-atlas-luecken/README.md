# SO6 · PLAT · Sound-Credits auf der Credits-Seite, Sound-Atlas und Sounds für die übrigen Ereignisse

- **Status:** geplant
- **Projekt:** SND
- **Domäne:** PLAT
- **Reife:** Entwurf
- **Tickets:** B-493, B-494, B-492
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Credits-Seite `lizenzen.html` nennt keine Sounds, kein Test prüft ihre Vollständigkeit (`src/tools/credits.ts`, B-493). Der Audio-Kern liest einen Sound-Atlas, ein Werkzeug, das die gewählten Sounds dazu packt, fehlt (B-494). SO2 deckt elf Ereignis-Slots ab; Münze geben, Bau-Fortschritt, Wiederbeleben durch Mitspieler, Hub-Ausbau, Skill je Klasse, Boss-Auftritt, UI-Klicks und Reiten stehen im Katalog `docs/assets/sounds.md` als Lücke (B-492).

## Ziel

Jeder Sound hat seinen Credit auf der Credits-Seite, die Effekte laden als Atlas, und jedes Ereignis aus dem Katalog hat einen Sound oder eine bestätigte Lücke. Am Ende sichtbar: `lizenzen.html` mit Sounds, Boss-Auftritt, Wiederbeleben und Hub-Ausbau hörbar am TV.

## Beteiligte und Zielgruppen

Spielende am TV; Urheber der CC-BY-Stücke; Agent in PLAT, INF und CLI; 🧑 wählt Sounds im Workshop und hört am TV ab.

## Anforderungen

B-493, B-494, B-492 › Anforderungen (Specs noch Entwurf).

## Nicht-Ziele

Musik (SO4); neue Feedback-Ereignisse im Server, außer 🧑 entscheidet so (dann eigenes Ticket SIM/SRV); Atlas für Musik.

## Regeln und Einschränkungen

Reihenfolge DEV → INF → CLI. **Abhängigkeit:** nach SO2 (Hörprobe SO2.5) und SO4. Lizenzen CC0 oder CC-BY, Credits in `public/audio/README.md`. Der Client rechnet nichts, Auslöser kommen aus Ereignissen oder dem Snapshot. Neue Aufgaben ins Taskfile des Projekts.

## Beispiele

Neue Datei unter `public/audio/kandidaten/` ohne Credit → `credits.test.ts` rot. Spielstart → eine Audio-Datei je Format für alle Effekte. Boss erscheint (`bossSpawned`) → eigener Sound.

## Ausnahme- und Fehlerfälle

Selbst erzeugte Töne (`atlas`, `probe-*`, `public/audio-test/`) brauchen keinen Credit. Fehlt der Atlas → Einzeldateien wie heute. Lücke ohne passenden Sound → bleibt mit Begründung, von 🧑 bestätigt.

## Akzeptanzkriterien

- **AC-01** Die Credits-Seite zeigt die Sounds, der Test prüft die Vollständigkeit unter `public/audio/` (B-493/AC-01, B-493/AC-02).
- **AC-02** Ein Task erzeugt den Sound-Atlas aus dem Slot-Register, der Client lädt für Effekte höchstens eine Datei je Format (B-494/AC-01, B-494/AC-02).
- **AC-03** Jede Lücke im Katalog hat Slot und gewählten Sound oder eine bestätigte Begründung, jede neue Datei einen Credit (B-492/AC-01, B-492/AC-02).
- **AC-04** Am TV lösen Boss-Auftritt, Wiederbeleben durch Mitspieler und Hub-Ausbau je ihren Sound aus (B-492/AC-03).

## Offene Fragen

🧑: Credit-Datei `public/audio/README.md` oder eigene `CREDITS.md` (B-493)? Lohnt der Atlas nach der Messung in SO2.5 (B-494)? Auslöser für Hub-Ausbau und UI-Klicks, Umfang „Skill je Klasse“ und „Münze geben“, Weg für Reiten (B-492 › Offene Fragen).

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SO6.1 (DEV) Credits-Seite mit Sounds und Vollständigkeits-Test (`src/tools/credits.ts`; AC-01).
- SO6.2 (INF) Task für den Sound-Atlas (AC-02).
- SO6.3 (CLI, Mensch) Workshop: Sounds für die Lücken hören und wählen (AC-03).
- SO6.4 (CLI) Slots, Dateien, Credits und Einbau der neuen Sounds (AC-02, AC-03).
- SO6.5 (CLI) Review (Code-Sprint): alle Kriterien prüfen.
- SO6.6 (CLI, Mensch) Hörprobe am TV (AC-04).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
