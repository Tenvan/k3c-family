# WZ2.5 · Alte Begriffe ersetzen, Glossar um Ziel-Ticket und ABN

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** sprint/wz2
- **Abhängig von:** –
- **Tickets:** B-367
- **Kriterien:** AC-06

## Ziel

Die Regeln in § 11.5/11.6 und `arbeitsweise.md` nutzen nur Begriffe, die es noch gibt; das Glossar kennt „Ziel-Ticket“ und „ABN“.

## Kontext

- Spec: `docs/backlog/B-367-doku-nachzug-projekte.md` (AC-01, AC-02); Entscheidung 🧑 2026-10-11: bleibt bei Projekt WZG.
- PJ3.3 hat § 11.1–11.4 und 11.7 in `docs/plan-weiterentwicklung.md` entfernt. § 11.5 (ca. Zeilen 196–201) und `docs/arbeitsweise.md` (Golden aktualisieren, ca. Zeile 254) sprechen noch von „Spur M/A“, „Bahn SIM/INF“, „Bahn CLI“; § 11.6 listet Hardware-Sessions mit alten IDs (LT1.3 …) statt HW1.n (ca. Zeile 218).
- Zielwortlaut Golden-Regel: „nur ein aktiver Sprint ändert Golden-Daten“.
- `docs/glossar.md` hat „Ziel-Ticket“ und „ABN (Projekt ohne Rang)“ noch nicht; Begriffe stehen laut Arbeitsweise zuerst im Glossar.
- Nur Doku; Abschnittsnummern 11.5/11.6 bleiben (Verweise).

## Erlaubte Dateien

- `docs/arbeitsweise.md`
- `docs/plan-weiterentwicklung.md` (nur § 11.5 und § 11.6)
- `docs/glossar.md`

## Nicht-Ziele

Regeln inhaltlich ändern; Abschnittsnummern 11.5/11.6 ändern; weitere Begriffe außer „Ziel-Ticket“ und „ABN“ ins Glossar aufnehmen.

## Schritte

1. `grep -n "Spur [MA]\|Bahn " docs/arbeitsweise.md docs/plan-weiterentwicklung.md` ausführen, Treffer notieren.
2. Glossar: Einträge „Ziel-Ticket“ und „ABN (Projekt ohne Rang)“ ergänzen (zuerst).
3. § 11.5 und `arbeitsweise.md` › Golden aktualisieren: „Spur“/„Bahn“ ersetzen; Golden-Regel lautet „nur ein aktiver Sprint ändert Golden-Daten“.
4. § 11.6: offene Hardware-Sessions verweisen auf HW1.n statt alter IDs.
5. Grep erneut ausführen (leer), `task test -- planning` ausführen.

## Fertig, wenn

- [ ] AC-06: `grep -n "Spur [MA]\|Bahn " docs/arbeitsweise.md docs/plan-weiterentwicklung.md` findet nichts; die Golden-Regel lautet „nur ein aktiver Sprint ändert Golden-Daten“ (`B-367/AC-01`).
- [ ] § 11.6 verweist auf HW1.n; `docs/glossar.md` enthält „Ziel-Ticket“ und „ABN“; `task test -- planning` grün (`B-367/AC-02`).

## Prüfen

```bash
task test -- planning
```

Manuelle Prüfungen: keine.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
