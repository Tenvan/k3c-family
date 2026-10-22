# B-516 · Prompt-Grundgerüst des Studio-Chats ist eine feste Instruktion im Code statt Text im Eingabefeld

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ21
- **Projekt:** WZG
- **Erstellt:** 2026-10-20
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-21, Ralf im Chat (mit WZ21 Revision 1)

## Ausgangslage

Abnahme WZ19.5 (2026-10-20, Ralf im Chat): Der Button „Prompt-Grundgerüst“ schreibt das Gerüst (Format, Regeln, Daten) heute ins Eingabefeld, Ralf ergänzt seine Wünsche und sendet alles zusammen. Gewünscht: Das Gerüst gehört als feste Instruktion in den Code. Der Chat schickt es bei Welt-Aufträgen selbst mit, und Ralf schreibt nur seinen Wunsch („Erstelle eine Welt Nordsee“). Aufruf der CLI siehe `tools/k3c-studio/backend/chat_cli.go` (`chatArgs`, z. B. `--append-system-prompt`, Verfügbarkeit in der CLI-Version prüfen); Gerüst heute im Frontend des Welten Editors (WZ19.3).

## Ziel

🧑 schreibt im Studio-Chat nur den Wunsch („Erstelle eine Welt Nordsee“) und bekommt eine Welt-Datei, die „Als Entwurf laden“ annimmt. Das Gerüst (Format, Regeln, Daten) kopiert niemand mehr von Hand ins Eingabefeld.

## Beteiligte und Zielgruppen

🧑 im Asset-Studio (Welten Editor mit Chat); Umsetzung durch Agent in DEV (`tools/k3c-studio/`).

## Anforderungen

- Das Gerüst entsteht weiter in Go aus den Daten (`backend/app_world_prompt.go`, heute `PlanWorldPrompt`), aber als Instruktion ohne den Schluss „MEINE WÜNSCHE:“.
- Eine Chat-Anfrage trägt die Art des Auftrags (`chatRequest`, neues Feld, z. B. `task: "welt"`; leer = freier Chat). Bei einem Welt-Auftrag hängt das Backend das Gerüst als System-Instruktion an den CLI-Aufruf (`--append-system-prompt-file`, Datei im Arbeitsordner des Chats; die Kommandozeile unter Windows ist auf 32 KiB begrenzt). Ein unbekannter Wert wird abgelehnt.
- Das Frontend sendet den Welt-Auftrag, solange der Welten Editor einen Entwurfs-Lader angemeldet hat (`chatBridge.ts` › `registerDraftLoader`); der Chat zeigt dann sichtbar, dass das Welt-Gerüst mitgeht.
- Jede Nachricht eines Welt-Auftrags trägt das Gerüst (die CLI kennt keinen Verlauf), der gespeicherte Verlauf enthält es nicht.
- Der Knopf „Prompt-Grundgerüst“ entfällt (Entscheidung 🧑, 2026-10-21), samt `fillChat`, wenn es sonst niemand nutzt.
- Mock (`mockWorldDraft.ts`) und Go-Backend verhalten sich gleich.

## Nicht-Ziele

Weitere Auftragsarten außer Welten; Werkzeuge der CLI; Chat in k3c-dev; Ändern des Gerüst-Inhalts über das Weglassen von „MEINE WÜNSCHE:“ hinaus.

## Regeln und Einschränkungen

Domäne DEV, Entscheidung 004 (Studio getrennt vom Spielserver). Oberfläche nach Atomic Design (`tools/webkit`). Keine neue Abhängigkeit. Die Sicherheits-Argumente von `chatArgs` bleiben unverändert. Flag gegen die installierte CLI geprüft: `claude --help` 2.1.304 kennt `--append-system-prompt` und `--append-system-prompt-file`. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

- Welten Editor offen, Eingabe „Erstelle eine Welt Nordsee“ → die CLI bekommt das Gerüst als Instruktion, stdin enthält nur Verlauf und Wunsch; die Antwort ist eine Welt-Datei.
- Studio auf einer anderen Seite (kein Entwurfs-Lader) → freier Chat ohne Gerüst.
- Folgenachricht „Mach die zweite Insel größer“ im Welt-Auftrag → Gerüst geht erneut mit.

## Ausnahme- und Fehlerfälle

- Die CLI kennt das Flag nicht (ältere Version, Meldung „unknown option“) → Fehler im Chat: „Diese Claude-Code-Version kennt --append-system-prompt-file nicht, bitte Claude Code aktualisieren“, keine Antwort ohne Gerüst.
- Gerüst-Datei nicht schreibbar → Fehler im Chat, die CLI startet nicht.
- Anfrage mit unbekannter Auftragsart → 400 mit gültigem Wertebereich.

## Akzeptanzkriterien

- **AC-01** Go-Test mit dem Test-Ersatz der CLI (`chat_fake_test.go`): Ein Welt-Auftrag startet die CLI mit `--append-system-prompt-file`, die Datei enthält das Gerüst ohne „MEINE WÜNSCHE:“, stdin enthält es nicht; ein freier Chat startet ohne das Flag.
- **AC-02** Go-Test: Meldet die CLI eine unbekannte Option, endet die Antwort mit dem Status `fehler` und dem Hinweis auf das Update.
- **AC-03** Vitest: Mit angemeldetem Entwurfs-Lader sendet der Chat den Welt-Auftrag und zeigt den Hinweis; ohne Lader nicht. Der Knopf „Prompt-Grundgerüst“ ist weg, `ChatDraft.test.tsx` ist angepasst.
- **AC-04** Abnahme durch 🧑 im Studio: „Erstelle eine Welt Nordsee“ ohne Gerüst im Feld liefert einen ladbaren Entwurf.

## Offene Fragen

keine (Knopf entfällt, 🧑 2026-10-21).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
