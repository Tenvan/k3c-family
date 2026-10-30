# BS3.5 · Review BS3 und Abnahme Meilenstein 1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** live
- **Branch:** bs3/5-review
- **Abhängig von:** BS3.4, BS3.6
- **Tickets:** B-635, B-638
- **Kriterien:** alle

## Ziel

BS3 ist abgenommen, Meilenstein 1 an 🧑 gemeldet, PR für `sprint/bs3` offen.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session. Besonders: Skala-Test vollständig, Ausnahmen markiert, Meilenstein-Lauf reproduzierbar (zweiter `sim_test`-Lauf mit gleichem Ergebnis).

## Erlaubte Dateien

- Dateien aus BS3.1–BS3.4 (Befunde beheben)
- `docs/sprints/**/BS3-*/**`, `docs/backlog/B-635-*.md`, `B-638-*.md`

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. Kriterien AC-01–AC-05 prüfen; Meilenstein-Lauf über `sim_test` wiederholen (Determinismus).
2. `check_run task:check`, `task:check:go`, `task:check:race`.
3. Abnahme, Sprint abschließen, PR öffnen; Meilenstein 1 per `notify_ui` (success) und im Chat melden.
4. Befunde mit Workbench-Go-Code → **⚠️ Workbench-Neustart nötig** melden.

## Fertig, wenn

- [ ] Alle Kriterien geprüft.
- [ ] Zweiter Lauf identisch.
- [ ] Checks grün, PR offen.

## Prüfen

```bash
task check
task check:go
task check:race
```

## Ergebnis

- AC-01 geprüft: Daten-Test der Münzskala grün (`task check:go`, `task check:diff`); Mauer Stufe 1 = 2, Ausnahmen aus der Liste.
- AC-02 geprüft: Einnahmen, Beute, Markieren, Ziele auf der Skala (`wirtschaft.md` § 8, BS3.1/BS3.2); Zielkorridore der Burg-Ziele unverändert rot/grün wie im Bericht (Richtwerte, entscheiden nicht über den Meilenstein, `zielkorridore.md` § 6).
- AC-03 geprüft: Schiffskosten je Abschnitt in `data/ship.json`, Daten-Test grün.
- AC-04 geprüft: Bericht nennt je Schiff-Abschnitt „bezahlbar ab Tag“ mit Ampel (BS3.3), Beispiel: Kiel T1, Rumpf T16,5, Segel T28 (Seed k3c-dev).
- AC-05 geprüft (Fassung 2026-10-28, BS3 Revision 2): `task balance:milestone` coop2, standard, k3c-dev besiegt den Splitter-Titan an Tag 26; zweiter Lauf mit identischer Ausgabe (md5 gleich). 4 Seeds: Titan 2/4 nach BS12.6.
- Checks: `task check`, `task check:go` grün, `check:race` lokal ohne C-Compiler übersprungen (nicht gelaufen), `check:diff` grün.
- PR: kein PR (BAS läuft auf `feature/bas-remake`, Beschluss 🧑 2026-10-26: keine PRs bis auf Widerruf). Meilenstein 1 an 🧑 gemeldet (Chat, `notify_ui`).
- Abweichungen: Meilenstein 1 bis Splitter-Titan gefasst (🧑 2026-10-28); Endboss dem Stufen-Test überlassen.
