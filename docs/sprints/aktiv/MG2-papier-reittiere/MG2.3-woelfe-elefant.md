# MG2.3 · Wölfe, Höllenhund und Elefant: wolf, weisser-wolf, hoellenhund, elefant

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** reittiere
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg2/3-woelfe-elefant
- **Abhängig von:** MG2.2
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

`wolf`, `weisser-wolf`, `hoellenhund` und `elefant` sind als `vierbeiner`-Specs mit `schritt`, `trab`, `galopp` über die Asset-MCP-Tools angelegt und gerendert.

## Kontext

- Quelle: `data/sprites.json` › `mounts` (`wolf`, `whiteWolf`, `hellhound`/`hellHound` = ein Asset, `elephant`).
- Rig `vierbeiner` ist auf das Pferd zugeschnitten: Wolf und Höllenhund mit eigenem Kopf (Teil `hals`), kürzeren Beinen (`bein`, Pivots) und kleinerem `massstab`; Elefant mit breitem Rumpf, Rüssel im `hals`, dicken Beinen. Wirkt eine Form mit dem Rig nicht, Befund als D-Zeile (Rig-Wunsch an RD1), nicht als neuer Rig-Typ.
- Farben laut D-MG2-01 (wolf `stoff`, weisser-wolf `hermelin`, hoellenhund `maehne` mit `koenigsrot`, elefant `stein`).
- Werkzeuge mit `checkout`; nach Änderungen `task gen:paper` (B-707).

## Erlaubte Dateien

- `public/grafik/reittiere/**`
- `docs/sprints/*/MG2-papier-reittiere/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (Zeilen D-MG2-xx)

## Nicht-Ziele

Kein Code, kein neuer Rig-Typ, kein `assets_style_set`.

## Schritte

1. Je Asset `assets_create`, `assets_set` (Teile, Bein, Animationen, Ausgabe), `assets_preview` je Animation Frame 0 und 4.
2. `task gen:paper`, `assets_review`, `assets_list {gruppe: reittiere}`.
3. `task check:diff`, Commit, Push nach `sprint/mg2`.

## Fertig, wenn

- [ ] AC-01: wolf, weisser-wolf, hoellenhund, elefant gültig.
- [ ] AC-02: `task gen:paper` grün, Golden eingetragen.
- [ ] AC-03: jede Animation per `assets_preview` geprüft, Befunde im Ergebnis; `assets_review` ohne `ungueltig`.

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
