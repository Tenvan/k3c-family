# MG3.2 · Bogenschütze, Elite-Bogenschütze und Elite-Krieger als figur

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** truppen
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg3/2-bogen-elite
- **Abhängig von:** MG3.1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Bogenschütze (`bogenschuetze`), Elite-Bogenschütze (`elite-bogenschuetze`) und Elite-Krieger (`elite-krieger`) liegen als `figur`-Specs unter `public/grafik/truppen/`, gerendert mit `idle`, `lauf` und `angriff`, nur über die Asset-MCP-Tools angelegt und mit `assets_preview` geprüft.

## Kontext

- Truppen: `data/troops.json` (`archer`, `eliteArcher`, `eliteWarrior`); Elite-Varianten heute als Einfärbung (`tint`) bzw. eigenes Sheet – hier als eigene Assets mit edleren Materialien (gold, koenigsrot), keine Varianten.
- Bogen als eigenes Teil am vorderen Unterarm; Angriff = Bogen spannen (Keyframes an Oberarm/Unterarm).
- Aufbau und Fallstricke wie MG3.1 (dessen Ergebnis lesen); B-707: danach `task gen:paper`.

## Erlaubte Dateien

- `public/grafik/truppen/**`
- `docs/sprints/aktiv/MG3-papier-truppen/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-MG3-Zeilen)

## Nicht-Ziele

Kein `assets_style_set`, keine Spielerfarben, keine anderen Gruppen.

## Schritte

1. Ergebnis MG3.1 lesen, `assets_get` der fertigen Truppen als Vorbild.
2. Je Truppe `assets_create` und `assets_set` (Teile, `idle`, `lauf`, `angriff`).
3. Jede Animation mit `assets_preview` prüfen (Frame 0 und Mitte).
4. `task gen:paper`, `assets_list {gruppe: truppen}`, `assets_review`.
5. `task check:diff`, Ergebnis, Commit `feat(pap): Truppen Bogenschützen und Elite-Krieger (MG3.2)`.

## Fertig, wenn

- [ ] AC-01: `assets_list {gruppe: truppen}` meldet alle sechs Assets gültig.
- [ ] AC-02: `task gen:paper` grün, `golden.json` vollständig; `task test -- paper` grün.
- [ ] AC-03: jede Animation der drei Assets mit `assets_preview` geprüft; `assets_review` ohne roten Eintrag.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
