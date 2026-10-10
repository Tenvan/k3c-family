# GL1.3 · Abnahme: Sprachwechsel am Gerät

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** PLAT
- **Umgebung:** live
- **Branch:** gl1/3-abnahme-sprache
- **Abhängig von:** GL1.1
- **Tickets:** B-369
- **Kriterien:** AC-06

## Ziel

🧑 hat am Gerät abgenommen, dass die Landingpage nach einem Sprachwechsel in den Optionen und dem Neuladen die gewählte Sprache zeigt.

## Kontext

Seit GL1.1 holt die Landingpage ihre Texte über `t()`. Die Sprache wird in den Optionen des Spiels (`game.html`, ☰ → Sprache) gesetzt und beim Laden der Landingpage aus den Einstellungen gelesen. Dienste über k3c-dev (`svc_start`) bzw. `task dev` und `task start`.

## Erlaubte Dateien

- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Code-Änderungen; Mängel werden Tickets.

## Schritte

1. Gerät notieren (Datum, PC, Xbox …).
2. Landingpage öffnen → Spielen → ☰ → Sprache English → zurück zur Landingpage → Seite neu laden.
3. Kacheln, Abschnitte, Statuszeile, Vollbild-Knopf und Fußleiste sind englisch; zurück auf Deutsch → nach dem Neuladen deutsch.
4. Ergebnis eintragen, Mängel als Tickets; ist alles fertig, Sprint nach `docs/sprints/erledigt/` verschieben.

## Fertig, wenn

- [ ] AC-06: 🧑 hat am Gerät gesehen, dass die Landingpage nach Sprachwechsel und Neuladen Englisch und wieder Deutsch zeigt.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
