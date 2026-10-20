# TS1.7 · Abnahme: Studio-Änderung erscheint im laufenden Server

- **Status:** offen
- **Typ:** Review
- **Agent:** Mensch
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** ts1/7-abnahme
- **Abhängig von:** TS1.6
- **Tickets:** B-507
- **Kriterien:** AC-07

## Ziel

🧑 hat am PC gesehen: eine im Studio gespeicherte Welt erscheint im laufenden Spielserver, das Studio läuft ohne Server, `/studio/` gibt es nicht mehr (AC-07).

## Kontext

Umgebung live: Claude bereitet vor (Server und Studio starten, Adressen nennen) und trägt das Urteil von 🧑 ein, urteilt aber nicht selbst. Punkte: (1) Studio-Fenster starten ohne Spielserver, Welt öffnen, Vorschau/„Im Spiel öffnen“ funktioniert; (2) Spielserver starten, im Studio eine Welt ändern und speichern → nach kurzer Zeit nimmt der Server sie an, Spieler sehen „Server startet neu“ (auch in laufender Runde); (3) `/studio/` im Browser antwortet 404, keine Kachel auf der Landingpage; (4) GitHub-Pages-Seite enthält nur `site/`.

## Erlaubte Dateien

- Planungsdateien von TS1 und B-507, neue Tickets für Befunde, `docs/sprints/README.md`, `docs/projekte/WZG-werkzeuge-testlaeufe.md`

## Nicht-Ziele

Code ändern oder Befunde beheben (→ Ticket); urteilen oder abhaken anstelle von 🧑.

## Schritte

1. Claude bereitet vor: Dienste starten (k3c-dev `svc_start`), Adressen nennen.
2. 🧑 geht die Punkte am PC durch.
3. Claude trägt das Urteil von 🧑 je Punkt mit Datum und Quelle ins Ergebnis; Befunde werden Tickets; bei Abnahme TS1 `erledigt`, B-507 `erledigt`.

## Fertig, wenn

- [ ] AC-07: 🧑 hat die Punkte am PC geprüft (Datum und Quelle im Ergebnis).
- [ ] Befunde sind Tickets oder bewusst verworfen; TS1 `erledigt`.

## Prüfen

```bash
task test -- planning
```
Nur Plan-Test; die Abnahme selbst macht 🧑.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
