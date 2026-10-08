# B-160 · Spielmetrik echter Abende und Simulatorwerte sind abgeglichen

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** BAL4
- **Projekt:** BAL
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Balancing-Tester (B-099, B-157, B-158) misst im Simulator mit Bots; echte Spieleabende (P1, BR1, BR2) liefern Spielmetrik je Sitzung (B-150) und Fragebogen (B-151). Ein Vergleich beider Quellen existiert nicht, die Ziele stammen bisher nur aus Regel-Dateien in `docs/rules/`.

## Ziel

Für jede Kennzahl, die beides liefert, steht Simulatorwert neben dem Wert echter Abende, mit Abweichung und Beschluss. Nutzen: Die Zahlen in `data/` und in den Zielkorridoren passen zu dem, was Familie und Kinder wirklich spielen, nicht nur zu Bots.

## Beteiligte und Zielgruppen

🧑 spielt die Abende und beschließt Wertänderungen (Q12 und Q24 in `docs/fragenkatalog.md`); der Agent wertet aus und schlägt Änderungen vor.

## Anforderungen

- Abgleichtabelle als Datei in `docs/rules/` (Name legt die Session fest): je Kennzahl Simulator (Median, p10–p90), echter Abend (Wert je Sitzung), Abweichung, Toleranz, Beschluss.
- Je Abweichung über der Toleranz: Ursache (Bot zu gut/schlecht, Wert falsch, Messung ungleich) und Vorschlag; Wertänderungen in `data/` nur mit Beschluss von 🧑.
- Nach jeder beschlossenen Änderung läuft `task balance` (B-157); die betroffenen Kennzahlen stehen im Commit.

## Nicht-Ziele

Erfassen der Spielmetrik (B-150), Spieleabend-Fragebogen (B-151), Spielspaß-Bewertung (bleibt Playtest).

## Regeln und Einschränkungen

Werte ändern nur REG mit Beschluss (`docs/arbeitsweise.md`); Metrik nur aus den Berichten in `reports/`. Voraussetzung: B-150, B-157 und mindestens ein Spieleabend (P1).

## Beispiele

Simulator: Überleben Welle 3 median 82 %; echter Abend: 55 % → Abweichung 27 Punkte, Ursache „Bot reagiert schneller als Kinder“ → Vorschlag: Kind-Bot-Profil (B-158) nachziehen oder Wert anpassen.

## Ausnahme- und Fehlerfälle

Zu wenige echte Sitzungen für eine Kennzahl → Eintrag „nicht aussagekräftig“ mit Anzahl, keine Wertänderung. Kennzahl nur auf einer Seite messbar → Lücke als Ticket.

## Akzeptanzkriterien

- **AC-01** Die Abgleichtabelle enthält jede Kennzahl, die der Spielmetrik-Report (B-150) und der Tester gemeinsam haben, mit Simulatorwert, Wert echter Abende und Abweichung.
- **AC-02** Zu jeder Abweichung über der Toleranz steht eine Ursache und ein Beschluss von 🧑 (ändern oder belassen).
- **AC-03** Beschlossene Wertänderungen sind in `data/` umgesetzt; `task balance` und `task check:go` sind danach grün.
- **AC-04** Kennzahlen mit zu wenigen Sitzungen sind als „nicht aussagekräftig“ mit Sitzungsanzahl markiert.

## Offene Fragen

- Welche Toleranz gilt je Kennzahl, und was gilt als „echter Abend“? Entscheidet 🧑, `docs/fragenkatalog.md` Q12 und Q24.

## Notizen

Quelle: `docs/plan-weiterentwicklung.md` Schiene B, BAL4; Lücke 19.
