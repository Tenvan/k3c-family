# BI2.1 · Kristall-Adern in drei Farben als Papierschnitt

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/bi2
- **Abhängig von:** –
- **Tickets:** B-900
- **Kriterien:** AC-01

## Ziel

Kristall-Adern in drei Farben als Papierschnitt-Specs (B-900).

## Kontext

`ader-kristall` (Material `kristall`) ist blau. Stil-Datei bleibt unverändert.

## Erlaubte Dateien

- `public/grafik/objekte/ader-kristall-violett.paper.json`, `public/grafik/objekte/ader-kristall-gruen.paper.json`, `public/grafik/objekte/index.paper.json`
- `data/paper.json`

## Nicht-Ziele

Einbau ins Spiel (BI2.2); Stil-Datei.

## Schritte

1. `assets_create` je Farbe, dann `assets_set` mit den Lagen von `ader-kristall`, Kristall-Lagen auf `purpur` bzw. `smaragd`.
2. `data/paper.json` um beide Einträge ergänzen.
3. `assets_review`, `check:diff`, Commit.

## Fertig, wenn

Beide Specs gültig, Maße wie `ader-kristall`, `check:diff` grün.

## Prüfen

`assets_list gruppe objekte`, `assets_review`, `check_run task:check:diff`.

## Ergebnis

AC-01 erfüllt (B-900/AC-01, AC-02): `ader-kristall-violett` (`purpur`) und `ader-kristall-gruen` (`smaragd`) als Kopie von `ader-kristall` (blau) angelegt, Ausgabe 70 px, Rahmen [-36, -64, 72, 66], beide gültig; `data/paper.json` ergänzt. Sichtprüfung im Abnahme-Ticket.
