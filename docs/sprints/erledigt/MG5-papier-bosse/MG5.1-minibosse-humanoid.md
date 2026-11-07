# MG5.1 · Minibosse goblin-anfuehrer, troll-koenig, ratten-koenigin

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** bosse
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg5/1-minibosse-humanoid
- **Abhängig von:** RD1, MC1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

`goblin-anfuehrer`, `troll-koenig` und `ratten-koenigin` sind als `figur`-Specs mit `idle`, `lauf`, `angriff` und `hoehe_px` 288 über die Asset-MCP-Tools angelegt und gerendert (Gruppe `bosse`).

## Kontext

- Quelle: `data/bosses.json` › `bosses` (goblinLeader, trollKing, ratQueen; Minibosse der Stufen 0–2).
- Grundform: Truppe `krieger` (`public/grafik/truppen/krieger.paper.json`, MG3): Figur mit Schwert, Schild, Helm und `angriff`-Keyframes (D-MG3-01). Je Boss Materialien getauscht und Zusatzteile (Ohren, Krone, Schwanz) mit `eltern`.
- Größe: `hoehe_px` 288 statt 192 (Bosse 1,5-mal so groß wie Truppen, D-MG5-01).
- Farben nur aus der Stil-Datei (D-MG5-02): goblin `smaragd`-Haut; troll `stein`-Haut, Krone `gold`, Keule `holz`; ratten-koenigin Fell `stoff`, Robe `purpur`, Krone `gold`, Schwanz `haut`.
- Werkzeuge mit `checkout`; nach Änderungen `task gen:paper` (B-707), fremde Gruppen danach zurücksetzen (D-MG3-06).

## Erlaubte Dateien

- `public/grafik/bosse/**`
- `docs/sprints/*/MG5-papier-bosse/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (Zeilen D-MG5-xx)

## Nicht-Ziele

Kein Code, kein `assets_style_set`, keine neuen Rig-Typen, keine anderen Gruppen.

## Schritte

1. `assets_guide {rig: figur}`, `assets_get krieger {felder: [animationen, ausgabe]}`.
2. Je Boss `assets_create {rig: figur, gruppe: bosse}`, `assets_set` (Teile, Animationen, Ausgabe vom Krieger, Materialien und Zusatzteile), `assets_preview` je Animation (idle, lauf, angriff; Frame 0 und 4).
3. `task gen:paper`, fremde Gruppen zurücksetzen, `assets_review`, `assets_list {gruppe: bosse}`.
4. `task check:diff`, Commit, Push nach `sprint/mg5`.

## Fertig, wenn

- [ ] AC-01: drei Bosse gültig.
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

- AC-01 geprüft: `assets_list {gruppe: bosse}` → goblin-anfuehrer, troll-koenig, ratten-koenigin gültig (Rig `figur`, `hoehe_px` 288).
- AC-02 geprüft: `task gen:paper` ok (erster Lauf brach ohne Meldung ab, zweiter grün), `golden.json`/`index.paper.json` der neuen Gruppe `bosse`; fremde Gruppen zurückgesetzt.
- AC-03 geprüft: goblin idle 0, lauf 4, angriff 4; troll idle 0, lauf 2, angriff 3; ratten-koenigin idle 4, lauf 6, angriff 5. Grundform Krieger mit getauschten Materialien; Zusatzteile `ohren` (goblin, ratten-koenigin), Krone statt Helm (troll, ratten-koenigin), `hauer` (troll), `schwanz` (ratten-koenigin). Befund: Schwanz ragte im `angriff` links aus dem Rahmen → Rahmen links erweitert (−68/158). `assets_review` (HEAD): 3× `neu`, kein `ungueltig`.
- `task check:diff`: Client 3173/3173; rot nur Workbench-Frontend `AnimationSync` (D-MG2-07, nicht MG5).
- Hinweis: Bosse wirken wie verkleidete Krieger (gleiche Silhouette, nur Kopf und Farben anders); Sichtprüfung in PS1.
