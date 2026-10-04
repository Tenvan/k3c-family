# BAL3.1 · Workshop: Profile, Fehlerrate und Grad-Kurven beschließen

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Branch:** bal3/1-workshop-profile
- **Abhängig von:** –
- **Tickets:** B-158
- **Kriterien:** AC-05

## Ziel

🧑 hat die Profile, die Fehlerrate des Kind-Bots und die Form der Grad-Kurven entschieden; der Beschluss steht mit Datum in der Dokumentation des Testers.

## Kontext

Ein Agent bereitet nur vor; 🧑 entscheidet. Nichts davon darf der Agent erfinden.

- **Schon beschlossen (Q19, `docs/fragenkatalog.md` › Beschlüsse vom 2026-10-03):** Pflicht-Profile sind passiv, sparsam, Mauern zuerst, Wirtschaft zuerst, Koop 2 und 4 Spieler; ein „Kind-Bot“ kommt „später“. BAL1 liefert „passiv“ und „sparsam“ (`BAL1.1-bots-kennzahlen.md`), BAL3 die übrigen fünf nach B-158: Wirtschaft zuerst, Mauern zuerst, Koop 2, Koop 4, Kind-Bot.
- **Spannung zwischen Spec und Q19:** B-158 und die Spec zählen den Kind-Bot zu den fünf neuen Profilen von BAL3, Q19 nennt ihn „später“. Das klärt 🧑 hier zuerst (Kind-Bot in BAL3 ja oder nein); bei „nein“ ändert sich die Spec (Revision + 1, neue Freigabe), nicht stillschweigend die Session-Zuordnung.
- **Offen, nicht durch Beschlüsse beantwortet:** Fehlerarten und **Fehlerrate des Kind-Bots** (B-158 nennt als Beispiele „verpasste Zahlung, späte Reaktion“), Werte der Wahrscheinlichkeiten, Bedeutung von „Koop 2/4 Spieler“ (Rollenverteilung der Bots), Form und Umfang der **Grad-Kurven** (welche Kennzahlen über die Tage, z. B. Überleben je Welle), welche Werte aus `data/*.json` der Sensitivitäts-Lauf standardmäßig variiert.
- **Vorlagen für das Gespräch:** Zielkorridore `docs/rules/zielkorridore.md` § 1–3 (Kennzahlen, Grade Dev/Leicht/Hart/Ultra aus `data/difficulty.json`), Kennzahlen des Testers aus `BAL1.1-bots-kennzahlen.md` und B-099 › Anforderungen.
- **Ort der Tester-Dokumentation:** Vorschlag der Planung (nicht beschlossen): `engine/balance/README.md`; das Paket entsteht in BAL1 (Ort dort noch von 🧑 zu bestätigen, siehe `BAL1.1-bots-kennzahlen.md`). Legt BAL1 das Werkzeug anderswo ab, gilt dessen Ort.

## Erlaubte Dateien

- Tester-Dokumentation (Vorschlag `engine/balance/README.md`, nur der Abschnitt „Profile und Grad-Kurven“)
- `docs/sprints/` (Status dieser Session), `docs/backlog/` (Status und neue Tickets)

## Nicht-Ziele

Kein Code, keine Änderung von `data/*.json`, keine Änderung der Spec oder der Korridore.

## Schritte

1. Agent stellt die offenen Fragen aus „Kontext“ als Liste bereit (je Frage Optionen und eine Empfehlung, als Vorschlag gekennzeichnet).
2. 🧑 entscheidet je Frage; der Agent trägt mit.
3. Beschluss mit Datum („Beschlossen von 🧑 am …“) in die Tester-Dokumentation: Profile (Name, Verhalten in einem Satz), Kind-Bot (Fehlerarten, Fehlerrate je Art), Grad-Kurven (Kennzahlen, Tage), Standard-Werte für die Sensitivität.
4. Offene Punkte als Ticket oder Offene Frage führen, nichts raten.

## Fertig, wenn

- [ ] AC-05: Die Tester-Dokumentation nennt Profile, Fehlerrate des Kind-Bots und Grad-Kurven mit „Beschlossen von 🧑 am <Datum>“.
- [ ] Unbeantwortete Fragen sind als Ticket oder Offene Frage geführt.

## Prüfen

Manuell durch 🧑 (Gespräch); danach `task check`.

## Ergebnis

–
