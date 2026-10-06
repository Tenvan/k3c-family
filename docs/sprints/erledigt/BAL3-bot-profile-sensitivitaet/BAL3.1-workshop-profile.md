# BAL3.1 · Workshop: Profile, Fehlerrate und Grad-Kurven beschließen

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Umgebung:** live
- **Branch:** bal3/1-workshop-profile
- **Abhängig von:** –
- **Tickets:** B-158
- **Kriterien:** AC-05

## Ziel

🧑 hat die Profile und die Form der Grad-Kurven entschieden; der Beschluss steht mit Datum in der Dokumentation des Testers.

## Kontext

Ein Agent bereitet nur vor; 🧑 entscheidet. Nichts davon darf der Agent erfinden.

- **Schon beschlossen (Q19, `docs/fragenkatalog.md` › Beschlüsse vom 2026-10-03):** Pflicht-Profile sind passiv, sparsam, Mauern zuerst, Wirtschaft zuerst, Koop 2 und 4 Spieler; ein „Kind-Bot“ kommt „später“. BAL1 liefert „passiv“ und „sparsam“ (`BAL1.1-bots-kennzahlen.md`), BAL3 die übrigen vier nach B-158: Wirtschaft zuerst, Mauern zuerst, Koop 2, Koop 4.
- **Kind-Bot:** In der Spec-Prüfung 2026-10-04 geklärt: nicht in BAL3 (Q19, später; B-158/AC-02). Fehlerrate des Kind-Bots ist hier nicht zu beschließen.
- **Offen, nicht durch Beschlüsse beantwortet:** Bedeutung von „Koop 2/4 Spieler“ (Rollenverteilung der Bots), Form und Umfang der **Grad-Kurven** (welche Kennzahlen über die Tage, z. B. Überleben je Welle), welche Werte aus `data/*.json` der Sensitivitäts-Lauf standardmäßig variiert.
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
3. Beschluss mit Datum („Beschlossen von 🧑 am …“) in die Tester-Dokumentation: Profile (Name, Verhalten in einem Satz), Grad-Kurven (Kennzahlen, Tage), Standard-Werte für die Sensitivität.
4. Offene Punkte als Ticket oder Offene Frage führen, nichts raten.

## Fertig, wenn

- [x] AC-05: Die Tester-Dokumentation nennt Profile und Grad-Kurven mit „Beschlossen von 🧑 am <Datum>“.
- [x] Unbeantwortete Fragen sind als Ticket oder Offene Frage geführt.

## Prüfen

Manuell durch 🧑 (Gespräch); danach `task check`.

## Ergebnis

2026-10-05, Workshop im Chat (🧑 hat je Frage aus vier Optionen gewählt, jeweils die Empfehlung).

- AC-05: umgesetzt – `tools/k3c-dev/internal/balance/README.md` › „Profile und Grad-Kurven“ mit „Beschlossen von 🧑 am 2026-10-05“: Koop = Rollen teilen (Mauern/Wirtschaft im Wechsel), Grad-Kurven = Burg hält, zerstörte Gebäude und Gold zur Dämmerung je Nacht 1–5, Sensitivität = vier Kernwerte.
- Zusatzbeschluss: Der Sensitivitäts-Lauf darf die Sim-Daten im Speicher neu laden (`sim.UseData` in `engine/sim/data.go`); BAL3.3 bekommt diese Datei als erlaubte Datei.
- Ort der Tester-Dokumentation: das Paket liegt seit BAL1 unter `tools/k3c-dev/internal/balance/`, nicht unter `engine/balance/`.
- Keine offenen Fragen übrig.
