import type { PlanDoc, PlanningData, PlanSession, PlanTicket } from './types';

// Erfundene Planung für den Mock: ein aktiver Sprint mit Session-Tabelle, geplante Entwürfe, Tickets und zwei kurze
// Dokumente, die dieselben Markdown-Formen haben wie docs/plan-weiterentwicklung.md und docs/fragenkatalog.md.

const s = (nr: string, typ: string, agent: string, status: string, titel = ''): PlanSession => ({ nr, typ, agent, status, titel });
const tk = (nr: string, title: string, domain: string, prio: string, status: string, sprint: string, spec = 'Entwurf'): PlanTicket =>
  ({ nr, title, domain, typ: 'Idee', prio, status, sprint, spec });

const DATA: PlanningData = {
  done: 31,
  sprints: [
    { id: 'SP11', title: 'Raspberry Pi', domain: 'SRV', status: 'aktiv', reife: 'bereit', spec: 'freigegeben',
      tickets: ['B-028', 'B-035', 'B-042'],
      sessions: [s('SP11.1', 'Umsetzung', 'autonom', 'fertig'), s('SP11.2', 'Workshop', 'Mensch', 'offen'),
        s('SP11.3', 'Workshop', 'Mensch', 'offen'), s('SP11.4', 'Review', 'autonom', 'offen')] },
    { id: 'F1', title: 'Zielkorridore und Bedienungsregeln', domain: 'REG', status: 'geplant', reife: 'Entwurf', spec: 'Entwurf',
      tickets: ['B-134', 'B-135', 'B-136'],
      sessions: [s('F1.1', '', '', 'entwurf', 'Workshop Zielkorridore'), s('F1.2', '', '', 'entwurf', 'Workshop Bedienung 1'),
        s('F1.3', '', '', 'entwurf', 'Workshop Bedienung 2')] },
    { id: 'F2', title: 'Golden-Ablauf, Migration, Determinismus', domain: 'INF', status: 'geplant', reife: 'Entwurf', spec: 'Entwurf',
      tickets: ['B-137', 'B-071'],
      sessions: [s('F2.1', '', '', 'entwurf', 'Golden-Task'), s('F2.2', '', '', 'entwurf', 'Spielstand-Migration')] },
    { id: 'S1', title: 'Monarch-Schlag und Skills', domain: 'SIM', status: 'geplant', reife: 'Entwurf', spec: 'Entwurf',
      tickets: ['B-118', 'B-119'], sessions: [s('S1.1', '', '', 'entwurf', 'Schlag und Pool'), s('S1.2', '', '', 'entwurf', 'Skills')] },
  ],
  tickets: [
    tk('B-006', 'Gamepad-Test auf der Xbox ist ausgewertet', 'PLAT', 'hoch', 'eingeplant', 'X1', 'freigegeben'),
    tk('B-007', 'Skill-Baum mit Tank und Zauberer ist spielbar', 'SIM', 'hoch', 'offen', '–'),
    tk('B-011', 'Spiel hat Sound und Musik', 'CLI', 'mittel', 'offen', '–'),
    tk('B-035', 'Server läuft auf dem Raspberry Pi im Docker', 'SRV', 'hoch', 'eingeplant', 'SP11', 'freigegeben'),
    tk('B-112', 'Hub-Ausbau mit Mauerstufen', 'SIM', 'mittel', 'offen', '–'),
    tk('B-134', 'Jede Kennzahl hat einen Zielkorridor als Zahl', 'REG', 'hoch', 'eingeplant', 'F1'),
    tk('B-137', 'Golden-Hashes ändern sich nur mit Begründung', 'INF', 'hoch', 'eingeplant', 'F2'),
  ],
};

const PLAN = `# Plan: K3C nach den Grundlagen

- **Status:** Richtung von 🧑 gewählt; Umsetzung in Tickets B-134 bis B-170 und den Sprints F1 bis RL1.
- **Leitlinie:** früh spielbar, dann Tiefe.

## 1. Ausgangslage

- **Fertig:** Insel/Stufen-Kern (SP12), Optionen (SP13), Raum auf Insel (SP14), Regelwerk R1–R4.
- **Offen:** Hub-Ausbau B-112, Skills B-119, Bosse B-130.

## 3. Phasen und Sprints

### Phase 0 – Fundament

| Sprint | Domäne | Inhalt | Am Ende sichtbar |
|---|---|---|---|
| **F1** | REG | Zielkorridore, Schriftgrößen, Pause-Semantik | Regeln mit Pass/Fail-Zahlen |
| **F2** | INF | Golden-Ablauf, Spielstand-Migration | Rote CI bei Drift |
| **F3** | SIM | Feedback-Events, Benchmark | Events im Debug-Overlay |
`;

const FRAGEN = `# Fragenkatalog

Zu klärende Punkte für die Planung. Eine Frage = ein Absatz mit Optionen und einer **Empfehlung**.

| Nr. | Thema | Blockiert | Block | Status |
|---|---|---|---|---|
| Q01 | Pause im gemeinsamen Raum | F1, S5 | 1 | offen |
| Q02 | Zielkorridore als Zahlen | F1, BAL2 | 1 | offen |
| Q06 | Skill-Tasten am Controller | X1, S3 | 1 | geklärt |

## Block 1 – blockiert F1

### Q01 · Pause im gemeinsamen Raum

- Option A: jeder Spieler kann pausieren, der Raum hält an.
- Option B: nur der Host pausiert.
- **Empfehlung:** A, mit sichtbarem Hinweis, wer pausiert hat.
`;

export function mockPlanning() {
  return {
    planning: async (): Promise<PlanningData> => DATA,
    planningDoc: async (name: PlanDoc): Promise<string> => (name === 'plan' ? PLAN : FRAGEN),
  };
}
