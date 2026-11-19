# BB1 · PAP · Assets nach Bildaufbau: Maßstab in Monarchenhöhen, Waldwand, Leitmotiv je Welt

- **Status:** aktiv
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-1038, B-1039, B-1040, B-1041, B-1042
- **Start-Commit:** – (wird von der ersten Session gesetzt: `git rev-parse --short origin/main`)
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-19

## Ausgangslage

Neuer Standard `docs/standards/bildaufbau.md` (#686, nach 16 K2C-Screenshots, 2026-11-17). Er misst Größen in Monarchenhöhen (M) und beschreibt Waldwand und Leitmotiv je Welt. Einige Assets weichen ab: Bürger 0,84 M statt ≈ 0,6 M, Schiff 1,35 M statt ≈ 3 M, die Burg (2,44 M) ist kaum größer als die Türme (2,45 M) und die Mauer (1,57 M) ist zu hoch.

Messbasis: Gebäude zeichnen 1 : 1 in Spiel-px (`ausgabe.hoehe_px` ÷ 121,5); Bäume messen sich an der `rahmen`-Höhe in `data/paper.json` › `objekte`; den Anzeige-Maßstab je Asset setzt `data/paper.json` › `skalierung` (Rust-Spiel, `art/scales.rs`, Fußpunkt bleibt), das Skalieren ersetzt dort oft das Neuzeichnen.

## Ziel

Die Assets folgen dem Maßstab des Standards; Wald und Hub-Kulissen haben Waldwand und Leitmotiv. Am Ende sichtbar: Asset-Vorschauen in der Workbench (Szene im Spielmaßstab), am TV die neuen Größen.

## Beteiligte und Zielgruppen

Spieler am TV. Umsetzung: Claude (PAP), Fachprüfung `k3c-grafik`, `k3c-welten`. Freigabe, Motiv-Wahl und Abnahme: 🧑.

## Anforderungen

Sessions (alle Lane hauptrechner, Domäne PAP, offline, Modell sonnet, nacheinander wegen `data/paper.json`):

- BB1.1 Maßstab Figuren und Schiff (AC-01, AC-02)
- BB1.2 Maßstab Hub-Gebäude (AC-03)
- BB1.3 Waldwand und Baumvarianten (AC-04)
- BB1.4 Leitmotiv je Welt (AC-05)
- BB1.5 Review (AC-07)

Ein Vitest `tests/bildaufbauMasstab.test.ts` (neu in BB1.1, erweitert in BB1.2 bis BB1.4) hält die Maße gegen den Standard fest.

## Nicht-Ziele

Zeichnen im Spiel (Gehlinie, Spiegel, Tönung, Weite, Wald nach Rodung): Sprint RU55 (Projekt RSU, Rust-Spiel; BB2 ist darin aufgegangen). Anbindung der neuen Baumvarianten, der Waldwand und der Leitmotive im Rust-Spiel: Folge-Ticket der Review. Rumpf des Schiffs verlängern, Bauplatz-Breiten.

## Regeln und Einschränkungen

- Assets nur über `assets_*` (Entscheidung 005); Golden und `gen:paper` im Abnahme-Ticket (B-742), das die Review anlegt (Lane `zweit`).
- Neue Schlüssel in `data/paper.json`: `waldwand`, `leitmotive` (Zuordnung vorläufig, Entscheidung 010 steht im Entwurf).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `public/grafik/truppen/**`, `public/grafik/gebaeude-paper/**`, `public/grafik/objekte/**`, `public/grafik/hintergruende/**`, `data/paper.json`, `tests/bildaufbauMasstab.test.ts`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** golden

## Beispiele

Hub der Stufe 3: Burg ≈ 4 M, Türme ≈ 2,3 M, Bürger ≈ 0,6 M; dahinter das Leitmotiv der Welt.

## Ausnahme- und Fehlerfälle

Eine Größe lässt sich nicht ohne Neuzeichnen erreichen (Faktor über 2, Unschärfe, Texturbudget): die Spec größer zeichnen, `k3c-grafik` prüft; ein Befund außerhalb der Dateien (Bauplatz, Spiel-Code) ist ein Ticket-Vorschlag im Ergebnis der Session.

## Akzeptanzkriterien

- **AC-01** Bürger und Truppen 0,55–0,65 M (B-1038/AC-01).
- **AC-02** Schiff ≈ 3 M mit Segel (B-1039/AC-01).
- **AC-03** Hub-Gebäude nach Messtabelle angeglichen (B-1040/AC-01, AC-02).
- **AC-04** Waldwand und Baumvarianten (B-1041/AC-01).
- **AC-05** Leitmotiv je Welt (B-1042/AC-01).
- **AC-06** Abnahme am TV (🧑).
- **AC-07** Review: `task:check`, `task:check:go` grün, Abnahme-Ticket angelegt.

## Offene Fragen

Entschieden am 2026-11-19 mit Default (autonom, Claude; Ralf ändert, was ihm nicht gefällt):

- Zielhöhe der Bürger (B-1038): 0,6 M (Truppen 0,58–0,60 M), Gier 0,4 M.
- Leitmotive (B-1042): standard Burgberg, sengoku Vulkanberg mit Torii, nil Pyramide und Sphinx, britannien Steinkreis auf einem Hügel.
- Zuschnitt: BB1.3 des Entwurfs ist in Waldwand und Baumvarianten (BB1.3) und Leitmotive (BB1.4) geteilt; die Review ist BB1.5. Grund: eine Session mit etwa zwölf Assets wäre zu groß.
- Größen entstehen bevorzugt über `skalierung` in `data/paper.json` statt Neuzeichnen; Sperre `golden` für die Abnahme-Läufe der Review.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BB1.1 | `BB1.1-masstab-figuren-schiff.md` | Umsetzung | autonom | offen |
| BB1.2 | `BB1.2-masstab-hub-gebaeude.md` | Umsetzung | autonom | offen |
| BB1.3 | `BB1.3-waldwand-baumvarianten.md` | Umsetzung | autonom | offen |
| BB1.4 | `BB1.4-leitmotive-je-welt.md` | Umsetzung | autonom | offen |
| BB1.5 | `BB1.5-review.md` | Review | autonom | offen |

Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket der Review (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
