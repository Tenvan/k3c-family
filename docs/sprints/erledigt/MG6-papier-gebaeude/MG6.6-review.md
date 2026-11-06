# MG6.6 · Sprint-Review MG6

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** gebaeude
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** mg6/6-review
- **Abhängig von:** MG6.1, MG6.2, MG6.3, MG6.4, MG6.5
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Sprint MG6 ist reviewt (frischer Reviewer, Opus), Befunde behoben oder als D-Zeile/Ticket, Abnahme in der README.

## Kontext

Diff `origin/feature/pap..sprint/mg6`: nur `public/grafik/gebaeude-paper/` (neue `*.paper.json`, Ausgabe-Ordner, `index.paper.json`, `golden.json`) und Planung. Kriterien AC-01–AC-03 der README.

## Erlaubte Dateien

- `public/grafik/gebaeude-paper/*.paper.json` der Sprint-Gebäude, ihre Ausgabe-Ordner, `index.paper.json`, `golden.json`
- `docs/sprints/aktiv/MG6-papier-gebaeude/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Keine Sichtprüfung durch 🧑 (PS1).

## Schritte

1. Reviewer-Agent über den Sprint-Diff (Specs, Stufen-Ausgabe, Golden).
2. Befunde beheben (über MCP) oder D-Zeile/Ticket.
3. `task check`, Abnahme-Abschnitt füllen, Commit.

## Fertig, wenn

- [ ] AC-01–AC-03 mit Nachweis aus MG6.1–MG6.5 bestätigt.
- [ ] `task check` grün, Abnahme ausgefüllt.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **Review:** frischer Reviewer (Opus) über den Sprint-Diff: kein kritischer, 3 wichtige, 9 kleine Befunde.
- **Behoben (wichtig):** Eisenbänder der Stufe 4 blieben in Stufe 5 sichtbar → Bänder in `stoff/schatten`, werden in Stufe 5 dunkler Kristall (mauer, tor, turm, werkstatt, schmiede, lager, ruestkammer, farm); Runenband am Tor als Falz unsichtbar → Fläche; Lagerfeuer nicht mittig → Feuerstelle und Grill mittig (Zahlziel), Zelt und Hütte links verkleinert, Holzstapel rechts. Alle mit Kontaktbogen geprüft.
- **Zurückgestellt (klein):** D-MG6-08 in B-701 (u. a. leuchtende Fugen der Kristallstufe, 1–2 px über dem Rahmen, Taverne-Schild, Burg-Banner-Reste). D-MG6-07 neu bewertet (ungeprüft). MG6.1-Kriterien auf `gebaeude-paper` korrigiert.
- **AC-01–AC-03 bestätigt:** 20 Assets gültig, `golden.json` und `index.paper.json` mit 20 Einträgen (Reviewer: SHA-256 gleich den PNGs); `assets_review` gegen HEAD ohne roten Eintrag.
- **Prüfungen:** `task check` unter Last 2 rote Vitest-Tests, einzeln 3173/3173 grün; `task check:go` grün; `task check:race` lokal übersprungen (kein C-Compiler, 0,4 s).
- **Merge:** `origin/feature/pap` (MG2, MG3) eingemergt; Golden-Test-Lockerung aus MG3 übernommen (D-MG6-06).
