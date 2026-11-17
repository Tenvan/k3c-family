# BB1 · PAP · Assets nach Bildaufbau: Maßstab in Monarchenhöhen, Waldwand, Leitmotiv je Welt

- **Status:** geplant
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** Entwurf
- **Tickets:** B-1038, B-1039, B-1040, B-1041, B-1042
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Neuer Standard `docs/standards/bildaufbau.md` (#686, nach 16 K2C-Screenshots, 2026-11-17). Er misst Größen in Monarchenhöhen (M) und beschreibt Waldwand und Leitmotiv je Welt. Einige Assets weichen ab: Bürger 0,84 M statt ≈ 0,6 M, Schiff 1,35 M statt ≈ 3 M, die Burg ist kaum größer als die Werkstätten.

## Ziel

Die Assets folgen dem Maßstab des Standards; Wald und Hub-Kulissen haben Waldwand und Leitmotiv. Am Ende sichtbar: Asset-Vorschauen in der Workbench (Szene im Spielmaßstab), am TV die neuen Größen.

## Beteiligte und Zielgruppen

Spieler am TV. Umsetzung: Claude (PAP), Fachprüfung `k3c-grafik`, `k3c-welten`. Freigabe, Motiv-Wahl und Abnahme: 🧑.

## Anforderungen

Siehe `B-1038` bis `B-1042 › Anforderungen`.

## Nicht-Ziele

Zeichnen im Spiel (Gehlinie, Spiegel, Tönung, Weite, Wald nach Rodung): Sprint BB2.

## Regeln und Einschränkungen

- **Bereich:** `public/grafik/truppen/**`, `public/grafik/gebaeude-paper/**`, `public/grafik/objekte/**`, `public/grafik/hintergruende/**`, `data/paper.json`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe, Entscheidung (Leitmotive), Abnahme
- **Sperre:** –
- Assets nur über `assets_*` (Entscheidung 005); Golden und `gen:paper` im Abnahme-Ticket (B-742).

## Beispiele

Hub der Stufe 3: Burg ≈ 4 M, Türme ≈ 2,3 M, Bürger ≈ 0,6 M; dahinter das Leitmotiv der Welt.

## Ausnahme- und Fehlerfälle

Siehe Tickets.

## Akzeptanzkriterien

- **AC-01** Bürger und Truppen 0,55–0,65 M (B-1038/AC-01).
- **AC-02** Schiff ≈ 3 M mit Segel (B-1039/AC-01).
- **AC-03** Hub-Gebäude nach Messtabelle angeglichen (B-1040/AC-01, AC-02).
- **AC-04** Waldwand und Baumvarianten (B-1041/AC-01).
- **AC-05** Leitmotiv je Welt (B-1042/AC-01).
- **AC-06** Abnahme am TV (🧑).
- **AC-07** Review: `task:check`, `task:check:go` grün, Abnahme-Ticket angelegt.

## Offene Fragen

Motive der Leitmotive (B-1042) und Zielhöhe der Bürger (B-1038), Defaults stehen in den Tickets.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

- BB1.1 Maßstab Figuren und Schiff (AC-01, AC-02)
- BB1.2 Maßstab Hub-Gebäude (AC-03)
- BB1.3 Waldwand und Leitmotive (AC-04, AC-05)
- BB1.4 Review (AC-07)

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
