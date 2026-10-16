# GEB6.4 · Truhe, Portal, Ausgang und Rekrutierungslager

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb6/4-truhe-portal-lager
- **Abhängig von:** GEB6.3
- **Tickets:** B-435
- **Kriterien:** AC-01, AC-04

## Ziel

`truhe.json` (1 × 1), `portal.json` und `ausgang.json` (je 2 × 3) und `rekrutierungslager.json` (3 × 2, Zelt und Feuerstelle) liegen mit PNGs in `public/grafik/gebaeude/` und erscheinen in der Vorschau; die Regeldatei nennt alle Maße.

## Kontext

- **Spec:** GEB6 AC-01, AC-04, › Anforderungen (Truhe, Portal, Ausgang, Rekrutierungslager), Offene Fragen 1, 4, 5, 7. Regeln: `docs/standards/gebaeude-grafik.md` (§ 1, § 3, § 5, § 7).
- **Heute im Spiel:** `pickup:chest` = offene Truhe mit Gold (`icons/8.png`, 16 × 16, ×2); `portal` = 4-Frame-Animation 32 × 48 px (×1); `exit` = Torbogen `fort-door.png` 96 × 80 (×2) mit Beschriftung darüber; `camp:recruit` = Zelt-Ausschnitt 63 × 79 (×2) plus daneben ein Lagerfeuer (`camp:recruit:fire`, 4 Frames, 90 px rechts vom Zelt). Neu sind alle vier Bilder **statisch** (Animationen sind Nicht-Ziel, Offene Frage 5).
- **Truhe** (`truhe`, 1 × 1): Holz (`rope`, Licht `twine`, Schatten `oiled-cedar`), Eisenbänder (Eisen: `heather`, Licht `light-steel`, Schatten `dim-gray`), offen mit sichtbarem Gold (Gold: `tahiti-gold`, Licht `golden-fizz`, Schatten `brown`; Offene Frage 4, 7).
- **Portal** (`portal`, 2 × 3, 64 × 96 px): Steinbogen (Stein `dim-gray`, `smokey-ash`, Fuge `valhalla`) mit leuchtender Fläche in Kristallfarben (`royal-blue`, `cornflower`, `viking`, Glanz `white`); Gegner kommen daraus, es soll bedrohlich-magisch wirken, nicht wie der Ausgang.
- **Ausgang** (`ausgang`, 2 × 3, 64 × 96 px): Stein- oder Holzrahmen eines Durchgangs mit dunkler Öffnung (`black`, `valhalla`), Stufe nach oben angedeutet; klar vom Portal unterscheidbar (kein Leuchten, andere Form). Die Beschriftung („Tiefer“/„Gesperrt“) setzt das Spiel darüber (GEB6.5).
- **Rekrutierungslager** (`rekrutierungslager`, 3 × 2, 96 × 64 px): links ein Zelt (Zelttuch = „Stoff hell“ `pancho`, Falte `white`, Schatten `twine`, Pfosten `oiled-cedar`), rechts die Feuerstelle (Steinring `dim-gray`, Holzscheite `rope`, Glut `tahiti-gold`/`golden-fizz`, Öffnung `black`); statisch ohne Flammenfrisur. Hier warten Landstreicher (`data/economy.json › recruitCamp`).
- **Materialien:** Zeilen Zelttuch und Gold stehen nach GEB6.1 in § 3; sonst Offene Frage 4.
- **Parallel:** GEB5 hat `index.json` und § 1 geändert; nach dem Merge `task gebaeude` neu laufen lassen.

## Erlaubte Dateien

- `public/grafik/gebaeude/` (neu: `truhe.json`, `portal.json`, `ausgang.json`, `rekrutierungslager.json` und PNGs; `index.json`)
- `tools/gebaeude/` (nur nötige Erweiterungen, mit Test)
- `src/tools/assetvorschau*.ts` (nur nötige Erweiterungen mit Test)
- `docs/standards/gebaeude-grafik.md`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Darstellung im Spiel (GEB6.5), keine Animation (Portal-Flackern, Flammen), keine Biom-Looks, keine Änderung an vorhandenen JSONs und PNGs, keine Münzen, Icons oder Sterne.

## Schritte

1. `sprint/geb6` auschecken, `Status: in Arbeit`, committen, pushen; md5 aller vorhandenen PNGs in `public/grafik/gebaeude/` sichern.
2. Vorlage zuerst (§ 5), dann Bausteine: `truhe.json` (1 × 1), `portal.json` (2 × 3), `ausgang.json` (2 × 3), `rekrutierungslager.json` (3 × 2); keine `stufen`.
3. `task gebaeude`; PNGs nebeneinander sichten (Portal und Ausgang klar unterscheidbar, Lager mit Zelt und Feuerstelle lesbar), nachbessern; md5 der vorhandenen PNGs gleich; zweiter Lauf ohne Änderung.
4. Regeldatei § 1 (Maße Truhe, Portal, Ausgang, Lager), § 4 (Bausteine); alle erscheinen in der Vorschau.
5. Code-Review (getrennter Agent), Ergebnis ausfüllen, `Status: fertig`, ein Commit `feat(dev): Truhe, Portal, Ausgang, Rekrutierungslager (GEB6.4)`, `git fetch`, `git merge origin/develop`, push.

## Fertig, wenn

- [ ] AC-01/AC-04: `truhe-32.png` (32 × 32 px), `portal-32.png` und `ausgang-32.png` (je 64 × 96 px), `rekrutierungslager-32.png` (96 × 64 px), je plus 16/64/128, alle JSONs mit `vorlage`, ohne `stufen`.
- [ ] AC-04: md5 aller vorher vorhandenen PNGs gleich; ein zweiter `task gebaeude`-Lauf ändert nichts.
- [ ] AC-04: Regeldatei § 1 (Maße) und § 4 (Bausteine) nachgetragen; alle vier erscheinen in der Vorschau.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task gebaeude
task check
task check:go
```

Mit laufender Workbench stattdessen `check_run` (mit `checkout`). Keine manuellen Prüfungen; die Sichtprüfung gehört GEB6.6.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
