# GEB12 · REG · Entscheidungen zu GEB9 bis GEB11 (Server-Stufenfolge, Markierer im Snapshot, Wald-Dichte)

- **Status:** geplant
- **Projekt:** GEB
- **Domäne:** REG
- **Reife:** Entwurf
- **Tickets:** B-458
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

GEB9 (Ausbaustufen der Hub-Gebäude, PR 433), GEB10 (Natur-Nachträge, `sprint/geb10`) und GEB11 (Hub wächst, PR 435) wurden am 2026-10-22 komplett autonom umgesetzt. Alles, was Regeln, Server, Protokoll, Spielstand oder Geschmack berührt und dabei nicht von Ralf vorgegeben war, hat Claude mit einem Default umgesetzt oder bewusst offen gelassen. Dieser Sprint legt diese Entscheidungen Ralf vor. Details stehen in den Sprint-READMEs und Session-Ergebnissen von GEB9, GEB10 und GEB11 sowie in B-531 und B-532 (Branch `sprint/geb10`).

## Ziel

Ralf hat die kritischen Entscheidungen aus GEB9 bis GEB11 bestätigt oder geändert, die Folge-Arbeit ist als Tickets oder Sprints geplant. Am Ende sichtbar: je Entscheidung ein Eintrag in `docs/decisions/` bzw. in den betroffenen Specs, offene Tickets mit Rang.

## Beteiligte und Zielgruppen

🧑 Ralf entscheidet; Claude bereitet vor (Optionen, Empfehlung) und plant die Folgearbeit. Spielende am TV sind indirekt betroffen (Aussehen und Regeln des Hubs, Wald, Wimpel).

## Anforderungen

Je Entscheidung eine Vorlage mit Optionen, Folgen und Empfehlung (Abschnitt Offene Fragen). Entscheidungen von Ralf werden in die Specs der betroffenen Sprints und in die Regeldateien nachgezogen; Folgearbeit als Ticket oder eigener Sprint im Projekt GEB.

## Nicht-Ziele

Die Umsetzung der Folgearbeit selbst (eigene Sprints); neue Grafik.

## Regeln und Einschränkungen

Nur Planung und Dokumentation (`docs/`). Protokoll und Spielstand ändern nur mit Entscheidung von Ralf (CLAUDE.md › Spielstruktur, `docs/protocol.md`). Keine Rücksicht auf alte Spielstände, solange Ralf nichts anderes sagt.

## Beispiele

Entscheidung D1 = Option C → Folge-Sprint SIM/REG „Hub-Ausbauschritte“ mit Protokollfeld `hubStep`. Entscheidung D2 = Ja → B-532 wird umgesetzt (Feld `markedBy`).

## Ausnahme- und Fehlerfälle

Ralf lehnt eine autonom umgesetzte Variante ab → Revert oder Anpassung als Ticket im Projekt GEB; die betroffene Abnahme (GEB9.6, GEB10.5, GEB11.6) bleibt offen.

## Akzeptanzkriterien

- **AC-01** Jede Entscheidung D1–D8 ist von Ralf bestätigt, geändert oder verworfen und in der betroffenen Spec bzw. Regeldatei vermerkt.
- **AC-02** Folgearbeit aus den Entscheidungen ist als Ticket oder Sprint mit Projekt GEB und Rang geplant.

## Offene Fragen

Ralf entscheidet; Claude empfiehlt.

- **D1 Feinere Hub-Stufenfolge im Server (GEB11, B-458):** A so lassen, Wachstum nur als Grafik aus `hubUpgrade` (umgesetzt, PR 435); B Unterstufen als zusätzliche `levels` (bricht „Linie k ab Hub-Stufe k“, Claude rät ab); C neue Ausbauschritte je Stufe mit eigener Freischaltung, Protokollfeld `hubStep`, Eintrag im Spielstand. Empfehlung: A, solange die Zwischenschritte nichts freischalten sollen, sonst C.
- **D2 Wimpel-Markierer im Snapshot (GEB10, B-532):** heute leitet der Client die Wimpelfarbe aus `coinGive` ab (nach Neuladen evtl. fehlend, bei mehreren Knoten in Reichweite evtl. falsch). Option: Feld `markedBy` im Snapshot (Protokoll und Spielstand). Empfehlung: erst nach der Abnahme von GEB10.5 entscheiden.
- **D3 Wald dichter, Höhlenbüsche im Level-Generator (GEB10, B-531):** Mehr Bäume je Abschnitt ist ein Wert in `data/` (REG, Golden betroffen); der Höhlenbusch ist im Spiel unsichtbar, weil die Höhle keine Büsche erzeugt. Empfehlung: einplanen.
- **D4 Baumgröße (GEB10):** Bäume 11,25 Kacheln (¾ der Szenenhöhe 15), also höher als Burg (9) und Turm (8). Empfehlung: in der Abnahme ansehen, Maß ggf. auf 9 begrenzen.
- **D5 Plantage-Pappel hinter der Farm (GEB10):** Die hohe Pappel steht jetzt hinter statt vor der Farm. Empfehlung: ansehen.
- **D6 Truhe geschlossen (GEB10):** Ohne Serverzustand umgesetzt: liegende Truhe = geschlossen, beim Aufheben öffnet sie sich kurz (Client). Kein SIM-Ticket. Empfehlung: bestätigen.
- **D7 GEB9-Defaults:** Gebäude folgen der Hub-Stufe (keine eigenen Gebäudestufen), Camp bleibt Grundform, Stufen vor der Freischaltung bleiben erhalten, Heilplatz-Stufen nur dezent (Pfosten, Laterne). Empfehlung: bestätigen; Heilplatz bei der Abnahme ansehen.
- **D8 Gerüst bei Gefahr (GEB11):** Das Baugerüst erscheint auch bei `waitingWorker` durch Gefahr (nachts), also ohne arbeitenden Bauer. Empfehlung: Gerüst nur bei laufendem Ausbau zeigen, sonst Zustand ohne Gerüst.

## Sessions

- GEB12.1 (REG, Mensch) Workshop: D1–D8 entscheiden, Entscheidungen in die Specs von GEB9, GEB10, GEB11 und `docs/decisions/` eintragen (AC-01).
- GEB12.2 (REG) Folgearbeit planen: Tickets bzw. Sprints für die bestätigten Änderungen (D1, D2, D3, D8) mit Rang im Projekt GEB (AC-02).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
