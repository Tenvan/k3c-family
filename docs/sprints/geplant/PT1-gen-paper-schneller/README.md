# PT1 · PAP · gen:paper schneller: Cache je Asset, paralleles Rastern, Fortschritt belegt

- **Status:** geplant
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-740, B-748, B-749, B-750, B-751
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12 (Gruppe von Ralf zur Umsetzung freigegeben; B-748 bis B-750 Klasse A)

## Ausgangslage

`task gen:paper` rendert 127 Assets in gut 3 Minuten, 98 % der Zeit in `resvg` auf einem Kern (B-751). Fortschritt (B-740, #518), Kopieren fehlender Grafik ohne Neurendern (B-748, #520), Startzeile je Asset (B-749, #521) und Ausgaben je Animation (B-750, #522) sind umgesetzt, die Tickets aber offen und nicht belegt. Die Papierschnitt-Pipeline bleibt nach Entscheidung 009 die Grafikquelle des nativen Spiels.

## Ziel

Ein `gen:paper` nach kleiner Änderung dauert Sekunden, ein voller Lauf deutlich unter einer Minute; die Fortschritts-Tickets sind belegt und geschlossen.

Am Ende sichtbar: zweiter `task gen:paper` direkt nach dem ersten meldet 0 gerenderte Assets; voller Lauf mit 8 Kernen höchstens ein Drittel der heutigen Zeit; Konsole zeigt ⏳-, 🔨- und ↳-Zeilen.

## Beteiligte und Zielgruppen

Claude (Umsetzung PT1.1, PT1.2; Review PT1.3); jeder Agent und 🧑, die Grafik rendern.

## Anforderungen

`B-740 › Anforderungen`, `B-748 › Anforderungen`, `B-749 › Anforderungen`, `B-750 › Anforderungen`, `B-751 › Anforderungen`. Sprint-eigen: Ergebnisse bleiben bitgleich (Grafik-Golden unverändert); stdout von `gen.ts` bleibt genau ein JSON-Objekt.

## Nicht-Ziele

Stil, Specs oder Golden ändern (PT2); Renderer durch eine andere Sprache ersetzen (B-751 › Ausgangslage: lohnt nicht); `check:diff` um Gen-Läufe erweitern (B-742).

## Regeln und Einschränkungen

Domäne PAP; Emoji-Log-Konvention; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; Tasks nur über `task`; keine neue Abhängigkeit (Worker über Bun/Node-Bordmittel). Gen-Läufe gehören hier zum Feature (B-742), Grafik-Golden wird nur gelesen. B-748 bis B-750 Klasse A, B-740 und B-751 (Idee) autonom freigegeben, weil 🧑 die Gruppe zur Umsetzung freigegeben hat.

- **Bereich:** `tools/paper/cli/gen*.ts`, `tools/paper/cli/work*.ts`, `tools/paper/cli/fortschritt*.ts`, `tools/paper/cli/pool*.ts`, `tools/paper/bin/gen.ts`, `tools/Taskfile.yml`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

Spec von `figuren/wolf` geändert → nur dieses Asset rendert neu, Rest aus dem Cache. Frischer Checkout mit `.work/grafik` → `gen:paper-sync` kopiert, kein Rendern.

## Ausnahme- und Fehlerfälle

Renderer-Version oder Stil geändert → `renderKey` passt nicht, alle betroffenen Assets rendern neu. Ein Worker stürzt ab → Lauf scheitert mit Asset-Namen und Exit ≠ 0, keine halben Ausgaben in `public/grafik`.

## Akzeptanzkriterien

- **AC-01** Fortschritt je Schritt in Konsole und `.work/grafik/gen.log`, Test prüft Format (`B-740/AC-01` bis `B-740/AC-03`).
- **AC-02** Fehlende Grafik wird kopiert statt neu gerendert (`B-748/AC-01` bis `B-748/AC-03`).
- **AC-03** Startzeile je Asset und Zeilen je Ausgabe großer Assets (`B-749/AC-01`, `B-750/AC-01`).
- **AC-04** Zweiter Lauf rendert 0 Assets; voller Lauf mit 8 Kernen ≤ ein Drittel der Zeit, Golden unverändert grün (`B-751/AC-01`, `B-751/AC-02`).

## Offene Fragen

keine blockierenden. Default (Claude, 2026-11-12) zu `B-751 › Offene Fragen`: Anzahl Worker = verfügbare Kerne minus 1, höchstens 8, übersteuerbar mit `K3C_PAPER_JOBS`; Meldung im Nachgang.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| PT1.1 | `PT1.1-fortschritt-belegen.md` | Umsetzung | autonom | offen |
| PT1.2 | `PT1.2-cache-parallel.md` | Umsetzung | autonom | offen |
| PT1.3 | `PT1.3-review.md` | Review | autonom | offen |

Keine eigenen Sim- oder Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
