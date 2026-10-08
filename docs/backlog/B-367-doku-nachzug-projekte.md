# B-367 · Begriffe aus den alten Bahnen und Spuren sind nach PJ3 ersetzt und neue Projekt-Begriffe im Glossar

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

PJ3.3 hat § 11.1–11.4 und 11.7 in `docs/plan-weiterentwicklung.md` entfernt. § 11.5 und `docs/arbeitsweise.md` (Golden aktualisieren) sprechen noch von „Spur M/A“, „Bahn SIM/INF“ und „Bahn CLI“; § 11.6 listet Hardware-Sessions mit alten IDs (LT1.3 …) statt HW1.n. Im Glossar fehlen „Ziel-Ticket“ und „ABN (Projekt ohne Rang)“. Befunde L2–L4 aus dem Review PJ3.4.

## Ziel

Die Regeln in § 11.5/11.6 und `arbeitsweise.md` nutzen nur Begriffe, die es noch gibt; das Glossar kennt die Projekt-Begriffe.

## Beteiligte und Zielgruppen

Agenten, die Arbeitsweise und Glossar lesen.

## Anforderungen

Begriffe in § 11.5, § 11.6 und `arbeitsweise.md` › Golden aktualisieren ersetzen; zwei Glossar-Einträge.

## Nicht-Ziele

Abschnittsnummern 11.5/11.6 ändern (Verweise); Regeln inhaltlich ändern.

## Regeln und Einschränkungen

Domäne INF, nur Doku.

## Beispiele

„Golden-Daten ändert nur der Sprint, der gerade in der Bahn SIM/INF läuft“ → „Golden-Daten ändert nur ein aktiver Sprint zur Zeit“.

## Ausnahme- und Fehlerfälle

nicht relevant (reine Doku).

## Akzeptanzkriterien

- **AC-01** `grep -n "Spur [MA]\|Bahn " docs/arbeitsweise.md docs/plan-weiterentwicklung.md` findet nichts mehr; die Golden-Regel lautet „nur ein aktiver Sprint ändert Golden-Daten“.
- **AC-02** § 11.6 verweist für offene Hardware-Sessions auf HW1.n; `docs/glossar.md` enthält „Ziel-Ticket“ und „ABN“; `task test -- planning` grün.

## Offene Fragen

Projekt: WZG vorläufig, weil PRZ mit PJ3 erledigt ist; beim Einplanen ggf. neues PRZ-Folgeprojekt.

## Notizen

Aus dem Review PJ3.4 (2026-10-08).
