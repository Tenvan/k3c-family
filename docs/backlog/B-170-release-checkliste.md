# B-170 · Eine Release-Checkliste macht jeden Release prüfbar

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** RL1
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 1, durch 🧑 (mit RL1); angeglichen an Q20

## Ausgangslage

Es gibt `task check:all` (Frontend, Go, k3c-dev, Build) und die CI in `.github/workflows/ci.yml`, aber keine festgehaltene Release-Checkliste. Offene Punkte, die ein Release berühren: Golden-Dateien für amd64 und arm64 (F2), Spielstand-Migration (B-137), Dev-Reste (B-098, B-107, B-080), Version im Status und gecachter Xbox-Client (B-141), Credits (B-165), Pi-Betrieb (SP11, B-142).

## Ziel

Eine Checkliste legt fest, wann ein Release gilt (Rhythmus nach Q20) und was dafür grün sein muss. Nutzen: Pi und Xbox laufen mit demselben, geprüften Stand.

## Beteiligte und Zielgruppen

🧑 gibt Releases frei und entscheidet den Rhythmus (Q20 in `docs/fragenkatalog.md`); Entwickler und Agenten arbeiten die Liste ab.

## Anforderungen

- Abschnitt „Release“ in `docs/arbeitsweise.md` (kein neues Prozess-Dokument, `CLAUDE.md`-Regel) mit Schritten und Befehlen.
- Inhalt mindestens: Golden grün auf amd64 und arm64; `task check:all` grün; Spielstand-Migration geprüft (Fixtures aus F2); Dev-Reste (B-098, B-107, B-080) aus; Pi-Image gebaut; Version im Status und auf der Landingpage stimmt mit dem Tag überein (B-141); Credits vollständig (B-165); Tag-Schema laut `docs/arbeitsweise.md` › „Entscheidungen und Versionen“.
- Jeder Punkt ist prüfbar: Befehl oder Datei mit erwartetem Ergebnis.
- Rhythmus und Auslöser stehen fest (Q20, 2026-10-03): Nach jedem fertigen Sprint schlägt die Abnahme eine Version vor, ein Tag nur bei Bestätigung durch 🧑 (B-180); zusätzlich ein Tag nach jedem Spieleabend, mit der Checkliste.

## Nicht-Ziele

itch.io und Englisch (B-023), automatisches Veröffentlichen, Pi-Image selbst (SP11), Version im Status (B-141).

## Regeln und Einschränkungen

Aufgaben nur über `task`; Prozess steht nur in `docs/arbeitsweise.md`; das Taggen und Veröffentlichen löst nur 🧑 aus. Release-Wissen des Plugin-Skills `release` ist kein Ersatz für diese Liste.

## Beispiele

Sprint fertig, Abnahme schlägt `v0.<n+1>.0` vor → Checkliste abarbeiten, alles grün → 🧑 bestätigt und setzt den Tag, Pi und Xbox bekommen denselben Stand.

## Ausnahme- und Fehlerfälle

Ein Punkt rot → kein Tag, Ticket für den Befund. Dev-Rest gefunden → Release blockiert, bis er entfernt ist.

## Akzeptanzkriterien

- **AC-01** `docs/arbeitsweise.md` enthält den Abschnitt „Release“ mit den Punkten der Anforderungen; jeder Punkt nennt Befehl oder Datei und erwartetes Ergebnis.
- **AC-02** Die Checkliste verweist auf Golden amd64 und arm64, `task check:all`, Migration, Dev-Reste, Pi-Image, Version, Credits und Tag-Schema.
- **AC-03** Ein Probelauf der Checkliste (ohne Tag) ist durchgeführt; das Ergebnis steht in der Abnahme des Sprints RL1.
- **AC-04** Rhythmus und Auslöser für Releases sind von 🧑 beschlossen und in der Liste festgehalten.

## Offene Fragen

keine (Q20 geklärt 2026-10-03)

## Notizen

Quelle: `docs/plan-weiterentwicklung.md` Schiene R, Lücken 16 und 22.
