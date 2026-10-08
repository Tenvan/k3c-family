# SO5 · CLI · Audio-Kern: ganze Dateien mit Crossfade, Ambient-Lautstärke

- **Status:** geplant
- **Projekt:** –
- **Domäne:** CLI
- **Prio:** hoch
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-250, B-218
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Hörprobe spielt ganze Dateien an SO1 vorbei (B-250); die Optionen kennen keinen Ambient-Bus (B-218).

## Ziel

Hörprobe und später die Musik (SO4) nutzen denselben Audio-Kern, Ambient ist einstellbar. Am Ende sichtbar: Hörprobe nutzt den Kern für ganze Dateien, Optionen regeln den Ambient-Bus.

## Beteiligte und Zielgruppen

🧑 hört am TV; Agent baut in `src/scenes/` und Audio-Kern.

## Anforderungen

B-250 › Anforderungen; B-218 › Anforderungen.

## Nicht-Ziele

Musik-Auswahl (SO4), SFX (SO2).

## Regeln und Einschränkungen

Einschiebbar; vor SO4.

## Beispiele

Hörprobe Stück A → B → Crossfade über Kurve, kein Knacken.

## Ausnahme- und Fehlerfälle

Datei fehlt → Stille, Warnung im Client-Log.

## Akzeptanzkriterien

- **AC-01** Der Audio-Kern spielt ganze Dateien mit Crossfade, die Hörprobe nutzt ihn (B-250/AC-01).
- **AC-02** Die Optionen-Szene regelt auch die Lautstärke des Ambient-Busses (B-218/AC-01).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SO5.1 Wiedergabe ganzer Dateien mit Crossfade, Ambient-Regler (AC-01, AC-02).
- SO5.2 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
