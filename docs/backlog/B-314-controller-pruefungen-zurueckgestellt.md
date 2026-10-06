# B-314 · Alle Controller-Prüfungen sind gesammelt nachgeholt

- **Domäne:** PLAT
- **Typ:** Schuld
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

🧑 kann derzeit nicht mit Controller testen (Chat 2026-10-06: „Control kann ich nicht testen, die können aus allen Sprints und Backlogs erstmal raus“). Abnahmen laufen am PC nur mit Tastatur. Die Controller-Schritte der offenen Abnahme-Sessions sind deshalb hierher verschoben; die Kriterien der Specs bleiben wörtlich stehen und gelten bis dahin als `angenommen, Validierung offen (B-314)`.

## Ziel

Alle zurückgestellten Controller-Prüfungen sind in einem Durchgang nachgeholt, sobald 🧑 wieder mit Controller testen kann. Nutzen: Abnahmen am PC blockieren nicht am fehlenden Controller, und nichts geht verloren.

## Beteiligte und Zielgruppen

🧑 prüft am Gerät; ein Agent führt das Interview und trägt die Ergebnisse in die Sessions ein.

## Anforderungen

- Je Eintrag der Liste unter **Notizen**: Controller-Schritt geprüft, Ergebnis in der genannten Session eingetragen.
- Weicht ein Ergebnis ab, entsteht ein eigenes Ticket.

## Nicht-Ziele

Controller-Funktionen im Code entfernen; Tastatur- und Touch-Prüfungen (laufen weiter am PC).

## Regeln und Einschränkungen

B nicht belegen, View + Menu reserviert (`CLAUDE.md`); `docs/arbeitsweise.md` › Hardware entkoppelt.

## Beispiele

SO3.3: Hörprobenseite mit D-Pad bedienen, B tut nichts, View + Menu führt zur Landingpage → Ergebnis in SO3.3.

## Ausnahme- und Fehlerfälle

Session ist inzwischen erledigt → Ergebnis trotzdem dort ergänzen, kein neuer Sprint nötig.

## Akzeptanzkriterien

- **AC-01** Jeder Eintrag unter **Notizen** hat in seiner Session ein Ergebnis mit Datum und Gerät (Beobachtung durch 🧑).
- **AC-02** Abweichungen sind als Tickets angelegt und hier verlinkt.

## Offene Fragen

- Wann wieder Controller-Tests möglich sind: 🧑.

## Notizen

Zurückgestellt am 2026-10-06 (Controller-Anteil, sonst unverändert):

- SO3.3 (SO3/AC-03): Controller-Bedienung, B frei, View + Menu
- SO1.5: Hörprobe mit Controller am TV
- S3.4 (S3/AC-02, B-124/AC-02): Skill-Menü und Slots mit Controller
- S5.4 (S5/AC-02, AC-04, B-146): Optionen und Pause mit Controller, View + Menu, B frei
- S6.4 (S6/AC-05, B-149/AC-02): Controller-Glyphen
- S7.3: Reittier mit Controller
- GR3.4: Spiel mit zwei Controllern starten
- GR4.3: Messung auf der Xbox mit Controller
- GR5.4 (GR5/AC-06): Vibration, Controller ohne Vibration
- N2.4: Zeitleiste und Vorhersage auf der Xbox mit Controller
- K5.4, W6.4 (geplant): Controller-Schritte der Abnahmen
- B-092/AC-05: Level-Betrachter mit Controller am TV
- B-195/AC-02: Debug-Overlay auf der Xbox mit Controller
