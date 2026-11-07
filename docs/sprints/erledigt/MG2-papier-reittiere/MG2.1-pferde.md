# MG2.1 · Pferde: pferd prüfen, rappe, schimmel, albtraum

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** reittiere
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg2/1-pferde
- **Abhängig von:** RD1, MC1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Die Pferde der Gruppe `reittiere` sind als `vierbeiner`-Specs mit `schritt`, `trab`, `galopp` gerendert: `pferd` (aus RD1) geprüft, `rappe`, `schimmel`, `albtraum` neu, alle nur über die Asset-MCP-Tools.

## Kontext

- Quelle: `data/sprites.json` › `mounts` (`horse`/`horseWalk` = pferd, `blackHorse` = rappe, `whiteHorse` = schimmel, `nightmare` = albtraum).
- Vorlage für die Form ist `pferd` (`public/grafik/reittiere/pferd.paper.json`, 17 Teile inkl. `sattel`, `bein`, Gangarten). Neue Pferde: `assets_create {rig: vierbeiner, gruppe: reittiere}`, dann `assets_set` mit `teile`, `bein`, `animationen`, `ausgabe` vom Pferd und getauschten Materialien.
- Farben nur aus der Stil-Datei (`assets_style_get`); Abbildung laut D-MG2-01 in B-701 (rappe: Körper `maehne`, Mähne `linie`; schimmel: Körper `hermelin`, Mähne `stoff`; albtraum: Körper `maehne`, Mähne `purpur`, Auge `koenigsrot`).
- `pferd` ist das Reittier des Monarchen (MG1): Änderungen daran nur nach Absprache mit MG1.
- Werkzeuge immer mit `checkout` = Worktree. `assets_set` zieht `golden.json` nicht nach (B-707): danach `task gen:paper` (D-MG1-02).

## Erlaubte Dateien

- `public/grafik/reittiere/**` (Specs, PNGs, Manifeste, `index.paper.json`, `golden.json`)
- `docs/sprints/*/MG2-papier-reittiere/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (Zeilen D-MG2-xx)

## Nicht-Ziele

Kein Code, kein `assets_style_set`, keine Änderung am `pferd` ohne MG1, keine anderen Gruppen.

## Schritte

1. `assets_guide {rig: vierbeiner}`, `assets_list {gruppe: reittiere}`, `assets_get pferd`.
2. `pferd`: `assets_preview` je Animation Frame 0 und 4; Befunde notieren.
3. Je neues Pferd: `assets_create`, `assets_set` (Teile, Bein, Animationen, Ausgabe aus dem Pferd, Materialien getauscht), `assets_preview` je Animation Frame 0 und 4.
4. `task gen:paper`, `assets_review`, `assets_list {gruppe: reittiere}` ohne `ungültig`.
5. `task check:diff`, Commit, Push nach `sprint/mg2`.

## Fertig, wenn

- [ ] AC-01: `assets_list {gruppe: reittiere}` meldet pferd, rappe, schimmel, albtraum gültig.
- [ ] AC-02: `task gen:paper` grün, `golden.json` enthält alle vier.
- [ ] AC-03: jedes Asset in jeder Animation per `assets_preview` geprüft, Befunde im Ergebnis; `assets_review` ohne `ungueltig`.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 geprüft: `assets_list {gruppe: reittiere}` → 4 Assets, 0 ungültig (pferd, rappe, schimmel, albtraum).
- AC-02 geprüft: `task gen:paper` ok, `golden.json` und `index.paper.json` der Gruppe mit allen vier; Zeilenend-Änderungen an `figuren/` und `pferd/manifest.json` zurückgenommen (fremde Gruppe, kein Inhalt).
- AC-03 geprüft: Preview pferd galopp 4, schritt 0, trab 4; rappe galopp 4, schritt 0, trab 2; schimmel trab 4, galopp 0, schritt 6; albtraum schritt 4, galopp 2, trab 0 – Form wie pferd, Farben laut D-MG2-01, Beine folgen dem Rumpf-Material. `assets_review` (HEAD): 3× `neu`, kein `ungueltig`.
- Befunde (D-MG2-02 bis -05 in B-701): Schweif ragt in `galopp` links aus dem Rahmen (alle Pferde, aus RD1); `assets_set` sortiert die Schlüssel des Patches (Animationen alphabetisch, erste ist jetzt `galopp`); `assets_set` wiederholt jedes geänderte Feld in der Antwort (sehr lang bei ganzen Teilen); `assets_create` mit Rig `vierbeiner` rendert nicht (Vorlage ohne Gangart).
- Arbeit nur über MCP (`assets_create`, `assets_set`, `assets_preview`, `assets_review`, `assets_list`); `task gen:paper` wegen B-707.
