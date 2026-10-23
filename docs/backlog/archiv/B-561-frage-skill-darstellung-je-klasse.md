# B-561 · Ralf bestätigt die Darstellung der Skills als Ring je Klasse ohne neue Ereignisse

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** KR5
- **Projekt:** KRT
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

SK2.3 (B-503) setzt autonom den Default um: Ein Ring in der Farbe der Linie (Tank, Zauberer, Heiler, Dieb) erscheint am Monarchen, wenn er einen aktiven Skill wirkt (Abklingzeit des Slots steigt) oder einen passiven lernt. Keine neuen Ereignisse, kein Effekt je Skill.

## Ziel

🧑 bestätigt die Darstellung oder wählt eine andere.

## Beteiligte und Zielgruppen

🧑 am TV (SK2.5); CLI, ggf. SRV.

## Anforderungen

- Optionen: (a) Ring je Klasse ohne neue Ereignisse (Default, umgesetzt); (b) eigener Effekt je Skill (Tabelle statt `LINE_COLORS`, CLI, etwa 24 Einträge); (c) neues Ereignis `cast` mit Skill und Ziel (SRV-Session für das Protokoll, genauer, auch Ziel-Ort sichtbar).
- Entscheidet 🧑 (b) oder (c): Folge-Ticket im Projekt SKL; (c) ändert das Protokoll (beide Enden) und die Golden-Ereignisse.

## Nicht-Ziele

Monarch-Animationen (B-505), Sounds je Klasse (B-558).

## Regeln und Einschränkungen

Geschmacksentscheidung bei 🧑.

## Beispiele

nicht relevant: Entscheidungsfrage.

## Ausnahme- und Fehlerfälle

nicht relevant: Entscheidungsfrage.

## Akzeptanzkriterien

- **AC-01** Entscheidung von 🧑 steht hier; bei Abweichung Folge-Ticket.

## Offene Fragen

(a), (b) oder (c)? Default (a).

## Notizen

Sprint KR5, Session KR5.1; TV-Abnahme SK2.5.

**Entscheidung 🧑 (KR5.1, 2026-10-23):** (b), eigener Effekt je Skill. Umsetzung: B-578 (CLI, Tabelle statt `LINE_COLORS`, Ring je Klasse als Rückfall). Der Ring aus SK2.3 gilt bis dahin; die TV-Abnahme SK2.5 prüft ihn weiter.
