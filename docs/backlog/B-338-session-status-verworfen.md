# B-338 · Sessions können den Status verworfen tragen

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** PJ1
- **Projekt:** –
- **Erstellt:** 2026-10-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-07, Chat, durch 🧑, Revision 1

## Ausgangslage

`docs/arbeitsweise.md` › Sprint-Lebenslauf und `tests/planning.test.ts` (Session-Tabelle) behandeln `verworfen` als Abschluss einer Session, die Vorlage `docs/vorlagen/session.md` erlaubt aber nur `offen | in Arbeit | fertig | blockiert`, und `plan_set` (k3c-dev) lehnt `verworfen` ab. Am 2026-10-07 mussten deshalb acht verworfene Abnahmen (S3.4, S4.3, S5.4, S6.4, S7.3, GR3.4, GR4.3, GR5.4 → B-337) als `fertig` mit Vermerk geschlossen werden.

## Ziel

Eine nicht durchgeführte Session lässt sich ehrlich als `verworfen` schließen, ohne Umweg über `fertig`.

## Beteiligte und Zielgruppen

Agenten und 🧑 beim Planen und Abschließen von Sprints.

## Anforderungen

- Vorlage `docs/vorlagen/session.md`, `tests/planning.test.ts` und `plan_set` (k3c-dev, SRV) kennen `verworfen` für Sessions.
- Optional: die acht Sessions vom 2026-10-07 auf `verworfen` umstellen.

## Nicht-Ziele

Weitere Status-Werte.

## Regeln und Einschränkungen

Domäne INF (Vorlagen, `tests/planning.test.ts`); die Änderung an `plan_set` gehört zu SRV (`tools/k3c-dev/`) und wird beim Einplanen als eigene Session oder Ticket geführt.

## Beispiele

`plan_set S5.4 {"Status": "verworfen"}` → angenommen, Session-Tabelle zeigt `verworfen`, Sprint gilt als abschließbar.

## Ausnahme- und Fehlerfälle

nicht relevant, reine Erweiterung eines Wertebereichs.

## Akzeptanzkriterien

- **AC-01** Vorlage und `tests/planning.test.ts` erlauben `verworfen` für Sessions (`task test -- planning` grün).
- **AC-02** `plan_set` setzt `verworfen` für Sessions (Test in `tools/k3c-dev`).

## Offene Fragen

keine

## Notizen

–
