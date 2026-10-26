# B-635 · Münzpreise liegen auf einer kleinen Skala von 2 bis 30 Gold

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BS2
- **Projekt:** BAS
- **Erstellt:** 2026-10-26
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-26, Chat, durch 🧑 mit BS2/BS3

## Ausgangslage

Goldpreise sind heute grob: Gebäude in `data/buildings.json` von 5 bis 300 Gold (z. B. Farm-Ausbau bis 100), Hub-Stufen in `data/hub.json` bis 200, Truppen bis 50, Bossbeute bis 500, Ziel in `goals.json` 1000. Das Balancing läuft auf dieser Basis nicht mehr durch; alle Werte müssen ohnehin neu gesetzt werden.

## Ziel

Eine kleine, K2C-artige Münzskala: Die günstigste Mauer kostet 2 Münzen, die größten Bauwerke und Ausbaustufen 30. Einnahmen, Beute und Ziele folgen der Skala, sodass ein Balancing-Lauf `coop2` standard wieder durchläuft (Meilenstein 1).

## Beteiligte und Zielgruppen

Spieler (Couch-Koop, Münzen zählbar), Claude (Werte), 🧑 entscheidet die Skala und nimmt ab.

## Anforderungen

- Regel in `docs/rules/wirtschaft.md`: Münzskala 2–30 für reguläre Gebäude, Ausbaustufen, Hub-Stufen und Truppen; Stufen der Skala (Vorschlag 2, 3, 4, 6, 8, 10, 12, 15, 20, 25, 30).
- Mauer Stufe 1 = 2 Münzen; größtes Bauwerk/höchste Ausbaustufe = 30.
- **Ausnahmen** (manuelle/geplante Chunks, Events, Bosse, Ruinen) dürfen andere Preise haben und bekommen später ein eigenes Balancing; bis dahin werden sie proportional mitgezogen und als „vorläufig“ markiert.
- Einnahmen (`economy.json`, Bürger, Beute, Markieren `markCost`) im selben Verhältnis neu, damit Tempo und Spannung erhalten bleiben.
- Zielkorridore (`docs/rules/zielkorridore.md`, `balance-targets.json`) auf die neue Skala umrechnen.

## Nicht-Ziele

Eigenes Balancing für Events, geplante Chunks und Bosse (eigenes Ticket, wenn wir damit anfangen). Materialkosten (Holz, Stein …) nur, soweit Boot/Baumschule sie brauchen (B-636, B-638).

## Regeln und Einschränkungen

Werte nur in `data/`; Spielstand-Kompatibilität entfällt (Beschluss 🧑, keine Migration alter Saves). Golden über `task go:golden` mit Begründung im Commit.

## Beispiele

Holzmauer 2 · Steinmauer ~6 · Turm Stufe 1 ~4 · Hub-Stufe 5 = 30 · Baumschule Stufe 1 ~4.

## Ausnahme- und Fehlerfälle

Preis > 30 in einer regulären Kategorie → Daten-Test schlägt fehl (Ausnahme-Liste explizit). Preis 0 nur bei Geschenken mit Begründung in `notes`.

## Akzeptanzkriterien

- **AC-01** `docs/rules/wirtschaft.md` beschreibt die Skala und die Ausnahmen.
- **AC-02** Ein Daten-Test prüft: reguläre Goldpreise in 2…30, Mauer Stufe 1 = 2, Ausnahmen nur aus der Ausnahme-Liste.
- **AC-03** Ein Balancing-Lauf `coop2`, Welt standard, Seed `k3c-dev` läuft mit den neuen Werten bis zum Ende der Insel durch (Meilenstein 1).

## Offene Fragen

- Genaue Stufen der Skala (Vorschlag oben) → 🧑, Default: Vorschlag.
- Was heißt „durchlaufen“ für Meilenstein 1: Endboss besiegt oder nur kein Abbruch/Scheitern bis Zeitlimit? → 🧑, Default: Endboss der Insel besiegt.

## Notizen

Wunsch 🧑 2026-10-26: „Mauern fangen mit 2 Münzen an … die größten Bauwerke und Ausbaustufen bei 30 Gold.“
