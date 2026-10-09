# TR2.2 · Review und Abnahme des Sprints TR2

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PLAT
- **Umgebung:** offline
- **Branch:** tr2/2-review
- **Abhängig von:** TR2.1
- **Tickets:** B-349
- **Kriterien:** alle

## Ziel

TR2 ist nach `docs/arbeitsweise.md` › Review-Session geprüft, B-349 archiviert, der PR des Sprints offen.

## Kontext

Leichtes Review: Feed nur Loopback oder Host der Seite, keine Eingabe ohne Parameter verändert, B-Taste und Home-Kombi unberührt.

## Erlaubte Dateien

- Dateien von TR2.1 nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, neue Funktionen.

## Schritte

1. `Status: in Arbeit`, `task check` grün.
2. Diff `origin/develop...origin/sprint/tr2` lesen, Befunde behandeln.
3. Abnahme, B-349 archivieren, Sprint nach `erledigt/`, Fahrplan, merge, push, PR.

## Fertig, wenn

- [x] AC-01 und AC-02 mit Nachweis; PR offen.

## Prüfen

```bash
task check
```

## Ergebnis

- Review durch einen unabhängigen Reviewer-Agenten (code-reviewer, nur lesend), weil TR2.1 im selben Chat entstand; `task check` grün.
- **AC-01** geprüft: `botInput.test.ts` deckt Aktionen je Slot, Leerlauf ohne Feed und Abbruch ab; Nachrichtenformat passt zu `botfeed.Cmd` (Slots 0…n-1, Skill 0–4).
- **AC-02** geprüft: Host-Prüfung über den URL-Parser (nur `ws:`/`wss:`, Loopback oder Host der Seite), keine Lücke gefunden; die von `sim_test` erzeugte Adresse `ws://127.0.0.1:<port>/bot/<lauf>/<client>` wird angenommen. „Eingabe unverändert“ ist bis zur Einbindung nur über `botInputs() = []` gezeigt; das Spiel selbst prüft B-353 (TR3).
- Keine schweren Befunde. Leichte Befunde (kurze Drücke gehen zwischen zwei Frames verloren, stummer Feed hält die letzte Bewegung, `👋` je Sekunde ohne Feed) → **B-354**. Beispiel-URL in B-349 auf `/bot/<lauf>/<client>` berichtigt.
- B-349 archiviert, Sprint nach `erledigt/`, PR des Sprints geöffnet.

