import { expect, test } from 'claude-code/testing'

import type { Job, JobRecord, JobStep } from '../types'
import {
  jobTitle,
  FOLD_AT, HISTORY_DAYS, HISTORY_PREFIX, STUCK_MS,
  actOf, addRecord, avgStepMs, continuePrompt, dayKey, dayReport, etaOf, finishJob, isStuck, isWorthShowing, lastQuestion, newJob,
  completeAll, noteAct, noteFile, planUpdate, progressOf, recordsOf, shiftDay, shownPhase, staleKeys, storeKey, toRecord, touch, visibleSteps, weekReport,
} from './glance'

const step = (name: string, status: JobStep['status'], startedAt?: number, doneAt?: number): JobStep => ({
  name, status, files: [], ...(startedAt === undefined ? {} : { startedAt }), ...(doneAt === undefined ? {} : { doneAt }),
})
const base = (over: Partial<Job> = {}): Job => ({ ...newJob('j1', 'Baue das Ding', 'C:\\work\\proj', 1_000), ...over })
const names = (j: Job | null) => j?.steps.map(s => `${s.name}:${s.status}`)

test('Checkliste: newJob startet arbeitend und leer', () => {
  const j = newJob('a', 'Mach\n  bitte   das', '/x', 5)
  expect(j).toEqual({ id: 'a', title: 'Mach bitte das', cwd: '/x', startedAt: 5, phase: 'working', question: '', ask: '', steps: [], act: null, signAt: 5, tools: 0, files: [] })
  expect(newJob('a', 'x'.repeat(100), '/x', 0).title.length).toBe(70)
})

test('Plan: TodoWrite ersetzt die Liste, Zeiten bleiben je Name erhalten', () => {
  const job = base({ steps: [step('A', 'done', 10, 20), step('B', 'active', 30)] })
  const next = planUpdate(job, 'TodoWrite', { todos: [
    { content: 'A', status: 'completed' },
    { content: 'B', status: 'completed' },
    { content: 'C', status: 'in_progress' },
    { content: 'D', status: 'pending' },
    { content: '   ', status: 'pending' },
  ] }, null, 100) as Job
  expect(names(next)).toEqual(['A:done', 'B:done', 'C:active', 'D:upcoming'])
  expect(next.steps[0]).toMatchObject({ startedAt: 10, doneAt: 20 })
  // B lief schon: Start bleibt, Ende setzt der erste Wechsel nach erledigt
  expect(next.steps[1]).toMatchObject({ startedAt: 30, doneAt: 100 })
  expect(next.steps[2]).toMatchObject({ startedAt: 100 })
  expect(next.steps[2]?.doneAt).toBe(undefined)
  expect(next.steps[3]?.startedAt).toBe(undefined)
  // gleicher Name, neue Liste, Ende bleibt beim ersten Mal
  const again = planUpdate(next, 'TodoWrite', { todos: [{ content: 'B', status: 'completed' }] }, null, 999) as Job
  expect(again.steps[0]).toMatchObject({ startedAt: 30, doneAt: 100 })
  expect(planUpdate(job, 'TodoWrite', {}, null, 1)).toBe(null)
  expect(planUpdate(job, 'TodoWrite', null, null, 1)).toBe(null)
})

test('Plan: Namen werden einzeilig und auf 120 Zeichen gekürzt', () => {
  const next = planUpdate(base(), 'TodoWrite', { todos: [{ content: `Zeile eins\nZeile zwei ${'x'.repeat(200)}`, status: 'pending' }] }, null, 1) as Job
  expect(next.steps[0]?.name.length).toBe(120)
  expect(next.steps[0]?.name.startsWith('Zeile eins Zeile zwei')).toBe(true)
})

test('Plan: update_plan mit items ersetzt die ganze Liste', () => {
  const job = base({ steps: [step('Alt', 'done', 1, 2)] })
  const next = planUpdate(job, 'mcp__ccd_turn__update_plan', { items: [
    { content: 'Alt', status: 'completed' },
    { content: 'Neu', status: 'in_progress' },
  ] }, null, 50) as Job
  expect(names(next)).toEqual(['Alt:done', 'Neu:active'])
  expect(next.steps[0]).toMatchObject({ startedAt: 1, doneAt: 2 })
  expect(next.steps[1]).toMatchObject({ startedAt: 50 })
})

test('Plan: update_plan advance schließt den aktiven und startet den nächsten offenen Schritt', () => {
  const job = base({ steps: [step('a', 'done', 1, 2), step('b', 'active', 3), step('c', 'upcoming'), step('d', 'upcoming')] })
  const next = planUpdate(job, 'mcp__ccd_turn__update_plan', { advance: true }, null, 60) as Job
  expect(names(next)).toEqual(['a:done', 'b:done', 'c:active', 'd:upcoming'])
  expect(next.steps[1]).toMatchObject({ startedAt: 3, doneAt: 60 })
  expect(next.steps[2]).toMatchObject({ startedAt: 60 })
  // ohne aktiven Schritt startet advance nur den nächsten
  const idle = planUpdate(base({ steps: [step('x', 'upcoming'), step('y', 'upcoming')] }), 'mcp__ccd_turn__update_plan', { advance: true }, null, 7) as Job
  expect(names(idle)).toEqual(['x:active', 'y:upcoming'])
  // nach dem letzten Schritt bleibt alles erledigt
  const last = planUpdate(base({ steps: [step('x', 'active', 1)] }), 'mcp__ccd_turn__update_plan', { advance: true }, null, 8) as Job
  expect(names(last)).toEqual(['x:done'])
})

test('Plan: update_plan done und start zählen 1-basiert, Unsinn wird ignoriert', () => {
  const job = base({ steps: [step('a', 'upcoming'), step('b', 'upcoming'), step('c', 'upcoming')] })
  const next = planUpdate(job, 'mcp__ccd_turn__update_plan', { done: [1, 3, 99, 0, 'x'], start: 2 }, null, 70) as Job
  expect(names(next)).toEqual(['a:done', 'b:active', 'c:done'])
  expect(next.steps[0]).toMatchObject({ startedAt: 70, doneAt: 70 })
  expect(next.steps[1]).toMatchObject({ startedAt: 70 })
  expect(next.steps.length).toBe(3)
  // ohne bekannte Felder: Job unverändert
  expect(planUpdate(job, 'mcp__ccd_turn__update_plan', { wrap: 1 }, null, 1)).toEqual(job)
})

test('Plan: TaskCreate nimmt die Nummer aus „Task #3 created“, sonst Objekt-id, sonst Position', () => {
  const j0 = base()
  const j1 = planUpdate(j0, 'TaskCreate', { subject: 'Erstens' }, 'Task #3 created successfully: Erstens', 5) as Job
  expect(j1.steps[0]).toMatchObject({ name: 'Erstens', status: 'upcoming', ref: '3' })
  const j2 = planUpdate(j1, 'TaskCreate', { activeForm: 'Zweitens tun' }, { task: { id: 7 } }, 6) as Job
  expect(j2.steps[1]).toMatchObject({ name: 'Zweitens tun', ref: '7' })
  const j3 = planUpdate(j2, 'TaskCreate', { description: 'Drittens' }, { taskId: 'x9' }, 7) as Job
  expect(j3.steps[2]).toMatchObject({ name: 'Drittens', ref: 'x9' })
  const j4 = planUpdate(j3, 'TaskCreate', { subject: 'Viertens' }, 'ohne Nummer', 8) as Job
  expect(j4.steps[3]?.ref).toBe('4')
  const j5 = planUpdate(j4, 'TaskCreate', { subject: 'Fünftens' }, undefined, 9) as Job
  expect(j5.steps[4]?.ref).toBe('5')
  expect(planUpdate(j0, 'TaskCreate', { subject: '  ' }, 'Task #1 created', 1)).toBe(null)
})

test('Plan: TaskUpdate setzt Status, benennt um und löscht über die Nummer', () => {
  let job = base()
  job = planUpdate(job, 'TaskCreate', { subject: 'A' }, 'Task #1 created', 10) as Job
  job = planUpdate(job, 'TaskCreate', { subject: 'B' }, 'Task #2 created', 10) as Job
  job = planUpdate(job, 'TaskUpdate', { taskId: '1', status: 'in_progress' }, null, 20) as Job
  expect(job.steps[0]).toMatchObject({ status: 'active', startedAt: 20, ref: '1' })
  job = planUpdate(job, 'TaskUpdate', { taskId: 1, status: 'completed' }, null, 30) as Job
  expect(job.steps[0]).toMatchObject({ status: 'done', startedAt: 20, doneAt: 30 })
  // ohne status bleibt er, subject benennt um
  job = planUpdate(job, 'TaskUpdate', { taskId: '2', subject: 'B neu' }, null, 31) as Job
  expect(job.steps[1]).toMatchObject({ name: 'B neu', status: 'upcoming', ref: '2' })
  expect(planUpdate(job, 'TaskUpdate', { taskId: '99', status: 'completed' }, null, 40)).toBe(null)
  const gone = planUpdate(job, 'TaskUpdate', { taskId: '1', status: 'deleted' }, null, 50) as Job
  expect(names(gone)).toEqual(['B neu:upcoming'])
})

test('Plan: Werkzeuge außerhalb der Aufgabenliste liefern null', () => {
  for (const tool of ['Read', 'Bash', 'AskUserQuestion', 'mcp__x__update_plan', '']) expect(planUpdate(base(), tool, { items: [] }, null, 1)).toBe(null)
})

test('Aktivität: actOf ordnet bekannte und MCP-Werkzeuge ein', () => {
  expect(actOf('Read')).toBe('read')
  expect(actOf('Grep')).toBe('search')
  expect(actOf('MultiEdit')).toBe('edit')
  expect(actOf('PowerShell')).toBe('shell')
  expect(actOf('WebSearch')).toBe('web')
  expect(actOf('Agent')).toBe('agent')
  expect(actOf('TodoWrite')).toBe('plan')
  expect(actOf('mcp__ccd_turn__update_plan')).toBe('plan')
  expect(actOf('AskUserQuestion')).toBe('ask')
  expect(actOf('mcp__plugin_serena_serena__find_symbol')).toBe('search')
  expect(actOf('mcp__plugin_serena_serena__read_file')).toBe('read')
  expect(actOf('mcp__plugin_serena_serena__replace_content')).toBe('edit')
  expect(actOf('mcp__plugin_context-mode_context-mode__ctx_execute')).toBe('shell')
  expect(actOf('mcp__browser__get_page_text')).toBe('web')
  expect(actOf('mcp__x__fetch_and_index')).toBe('web')
  expect(actOf('Unbekannt')).toBe('other')
})

test('Aktivität: noteAct zählt gleiche Art hintereinander hoch und setzt das Lebenszeichen', () => {
  let j = base()
  j = noteAct(j, 'Read', 2_000)
  j = noteAct(j, 'mcp__serena__read_file', 3_000)
  expect(j.act).toEqual({ kind: 'read', n: 2 })
  expect(j.tools).toBe(2)
  expect(j.signAt).toBe(3_000)
  j = noteAct(j, 'Grep', 4_000)
  expect(j.act).toEqual({ kind: 'search', n: 1 })
  j = noteAct(j, 'Read', 5_000)
  expect(j.act).toEqual({ kind: 'read', n: 1 })
  expect(j.tools).toBe(4)
  expect(touch(j, 9_000).signAt).toBe(9_000)
  expect(touch(j, 9_000).tools).toBe(4)
})

test('Aktivität: noteFile ordnet den Dateinamen dem aktiven Schritt zu, sonst nur dem Job', () => {
  const job = base({ steps: [step('a', 'done'), step('b', 'active', 1), step('c', 'upcoming')] })
  const next = noteFile(job, 'C:\\WORKSPACE\\proj\\src\\app.ts')
  expect(next.steps[1]?.files).toEqual(['src/app.ts'])
  expect(next.steps[0]?.files).toEqual([])
  expect(next.files).toEqual(['src/app.ts'])
  // doppelt zählt nicht
  expect(noteFile(next, '/other/src/app.ts').files).toEqual(['src/app.ts'])
  expect(noteFile(next, '/x/y/README.md').steps[1]?.files).toEqual(['src/app.ts', 'y/README.md'])
  // ohne aktiven Schritt nur am Job
  const idle = noteFile(base({ steps: [step('a', 'upcoming')] }), '/a/b/c.ts')
  expect(idle.steps[0]?.files).toEqual([])
  expect(idle.files).toEqual(['b/c.ts'])
  // leerer Pfad ändert nichts
  expect(noteFile(job, '')).toBe(job)
  expect(noteFile(job, '///')).toBe(job)
  // Obergrenze je Schritt: die letzten zwölf
  let many = job
  for (let i = 0; i < 20; i++) many = noteFile(many, `/d/f${i}.ts`)
  expect(many.steps[1]?.files.length).toBe(12)
  expect(many.steps[1]?.files[11]).toBe('d/f19.ts')
})

test('Rückfrage: lastQuestion findet die Frage in den letzten drei Zeilen, auch vor einer Grußzeile', () => {
  expect(lastQuestion('Alles fertig.\n\nSoll Claude den Branch löschen?\n\nViele Grüße')).toBe('Soll Claude den Branch löschen?')
  expect(lastQuestion('Fertig.\n**Soll Claude weitermachen?**')).toBe('Soll Claude weitermachen?')
  expect(lastQuestion('Ok. Wie jetzt? Merge oder Rebase?')).toBe('Merge oder Rebase?')
  expect(lastQuestion('Erste Frage?\nZweite Zeile\nDritte Zeile\nVierte Zeile')).toBe('')
  expect(lastQuestion('Ergebnis steht.\nNichts offen.')).toBe('')
  expect(lastQuestion('')).toBe('')
  expect(lastQuestion('Ja?\nok')).toBe('')
  expect(lastQuestion(`Welche Option nimmst du denn bitte jetzt wirklich?`, 12)).toBe('Welche Opti…')
})

test('Rückfrage: Fragezeichen in Codeblöcken zählen nicht', () => {
  expect(lastQuestion('Hier der Aufruf.\n```ts\nconst x = a ? b : c; // warum?\n```')).toBe('')
  expect(lastQuestion('Eine Frage vorab: passt das?\n```\nfoo?\n```')).toBe('Eine Frage vorab: passt das?')
})

test('Turn-Ende: finishJob wählt zwischen paused, stopped, needsYou und done', () => {
  const job = base({ tools: 3, signAt: 1_500 })
  const done = finishJob(job, 'Alles erledigt.', false, false, 9_000)
  expect(done).toMatchObject({ phase: 'done', question: '', endedAt: 9_000, signAt: 9_000 })
  const needs = finishJob(job, 'Fertig.\nSoll Claude pushen?', false, false, 9_000)
  expect(needs).toMatchObject({ phase: 'needsYou', question: 'Soll Claude pushen?' })
  const stopped = finishJob(job, 'Soll Claude pushen?', true, false, 9_000)
  expect(stopped).toMatchObject({ phase: 'stopped', question: '' })
  // Pause schlägt Abbruch und Frage
  expect(finishJob(job, 'Soll Claude pushen?', true, true, 9_000).phase).toBe('paused')
  expect(finishJob(job, 'Soll Claude pushen?', false, true, 9_000).phase).toBe('paused')
})

test('Turn-Ende: offene Schritte bleiben offen, nur wrap_up (completeAll) hakt sie ab', () => {
  const job = base({ steps: [step('a', 'active', 1), step('b', 'upcoming')] })
  expect(finishJob(job, 'Alles erledigt.', false, false, 9_000).steps.map(s => s.status)).toEqual(['active', 'upcoming'])
  const all = completeAll(job, 9_000)
  expect(all.steps.map(s => s.status)).toEqual(['done', 'done'])
  expect(all.steps[0]).toMatchObject({ startedAt: 1, doneAt: 9_000 })
  expect(progressOf(all)).toEqual({ done: 2, total: 2 })
})

test('Turn-Ende: die Frage der Desktop-Abschlussbox hat Vorrang', () => {
  const job = base({ ask: 'Mergen?' })
  expect(finishJob(job, 'Fertig.\nSoll Claude pushen?', false, false, 5)).toMatchObject({ phase: 'needsYou', question: 'Mergen?' })
  expect(finishJob(job, 'Alles gut.', false, false, 5)).toMatchObject({ phase: 'needsYou', question: 'Mergen?' })
  // Abbruch bestimmt die Phase, die Frage bleibt
  expect(finishJob(job, 'Alles gut.', true, false, 5)).toMatchObject({ phase: 'stopped', question: 'Mergen?' })
})

test('Phase: hängt erst ab STUCK_MS ohne Lebenszeichen und nur beim Arbeiten', () => {
  const j = base({ signAt: 1_000 })
  expect(isStuck(j, 1_000 + STUCK_MS - 1)).toBe(false)
  expect(isStuck(j, 1_000 + STUCK_MS)).toBe(true)
  expect(shownPhase(j, 1_000 + STUCK_MS)).toBe('stuck')
  expect(shownPhase(j, 2_000)).toBe('working')
  for (const phase of ['needsYou', 'paused', 'stopped', 'done'] as const) {
    expect(isStuck({ ...j, phase }, 9_999_999)).toBe(false)
    expect(shownPhase({ ...j, phase }, 9_999_999)).toBe(phase)
  }
  // Lebenszeichen löst „hängt?“ auf
  expect(isStuck(touch(j, 1_000 + STUCK_MS), 1_000 + STUCK_MS)).toBe(false)
})

test('Phase: eine schnelle Antwort ohne Tools ist keine Zeile wert', () => {
  expect(isWorthShowing(base())).toBe(false)
  expect(isWorthShowing(base({ tools: 1 }))).toBe(true)
  expect(isWorthShowing(base({ steps: [step('a', 'upcoming')] }))).toBe(true)
  expect(isWorthShowing(base({ phase: 'needsYou' }))).toBe(true)
})

test('Fortschritt: progressOf zählt erledigte Schritte', () => {
  expect(progressOf(base())).toEqual({ done: 0, total: 0 })
  expect(progressOf(base({ steps: [step('a', 'done'), step('b', 'active'), step('c', 'upcoming')] }))).toEqual({ done: 1, total: 3 })
})

test('Restzeit: ohne Daten, ohne Plan, fertig oder pausiert keine Zahl', () => {
  const open = [step('a', 'active', 1_000), step('b', 'upcoming')]
  expect(etaOf(base(), 2_000, 5_000)).toBe(null)
  expect(etaOf(base({ steps: open }), 2_000, null)).toBe(null)
  expect(etaOf(base({ steps: [step('a', 'done', 1, 2)] }), 2_000, 5_000)).toBe(null)
  expect(etaOf(base({ steps: open, phase: 'paused' }), 2_000, 5_000)).toBe(null)
})

test('Restzeit: aus den eigenen erledigten Schritten (ab zwei), minus Zeit im laufenden', () => {
  const job = base({ steps: [step('a', 'done', 0, 1_000), step('b', 'done', 1_000, 4_000), step('c', 'active', 5_000), step('d', 'upcoming')] })
  // Mittel 2000 ms, zwei offen, 500 ms im laufenden Schritt
  expect(etaOf(job, 5_500, null)).toEqual({ ms: 3_500, longer: false })
  // der Verlauf wird dann ignoriert
  expect(etaOf(job, 5_500, 60_000)).toEqual({ ms: 3_500, longer: false })
  // abgezogen wird höchstens ein Schrittmittel
  expect(etaOf(job, 8_000, null)).toEqual({ ms: 2_000, longer: false })
  // doppelt so lang wie üblich: keine Zahl mehr
  expect(etaOf(job, 9_001, null)).toEqual({ ms: 0, longer: true })
})

test('Restzeit: mit nur einem gemessenen Schritt zählt der Verlauf', () => {
  const job = base({ steps: [step('a', 'done', 0, 100), step('b', 'active', 1_000), step('c', 'upcoming'), step('d', 'upcoming')] })
  expect(etaOf(job, 1_200, 1_000)).toEqual({ ms: 1_000 * 3 - 200, longer: false })
  expect(etaOf(job, 1_200, null)).toBe(null)
  // ein laufender Schritt ohne Startzeit kostet nichts
  const noStart = base({ steps: [step('a', 'upcoming'), step('b', 'upcoming')] })
  expect(etaOf(noStart, 5_000, 1_000)).toEqual({ ms: 2_000, longer: false })
})

test('Restzeit: avgStepMs braucht drei Jobs dieses Projekts mit gemessenen Schritten', () => {
  const rec = (id: string, cwd: string, ms: (number | undefined)[]): JobRecord => ({
    id, title: id, cwd, startedAt: 0, endedAt: 1, outcome: 'done', question: '', tools: 1, files: [],
    steps: ms.map((m, i) => ({ name: `s${i}`, done: true, ...(m === undefined ? {} : { ms: m }) })),
  })
  const list = [rec('1', '/p', [1_000, 3_000]), rec('2', '/p', [2_000]), rec('3', '/other', [9_000]), rec('4', '/p', [undefined])]
  expect(avgStepMs(list, '/p')).toBe(null)
  expect(avgStepMs([...list, rec('5', '/p', [6_000])], '/p')).toBe(3_000)
  expect(avgStepMs([], '/p')).toBe(null)
})

test('Band: bis FOLD_AT Schritte werden alle gezeigt', () => {
  expect(FOLD_AT).toBe(5)
  const five = ['a', 'b', 'c', 'd', 'e'].map(n => step(n, 'done'))
  const rows = visibleSteps(five)
  expect(rows.length).toBe(5)
  expect(rows.every(r => r.kind === 'step')).toBe(true)
  expect(visibleSteps([])).toEqual([])
})

test('Band: acht Schritte mit drei erledigten falten Erledigtes und Rest', () => {
  const eight = [
    step('1', 'done'), step('2', 'done'), step('3', 'done'), step('4', 'active', 1), step('5', 'upcoming'),
    step('6', 'upcoming'), step('7', 'upcoming'), step('8', 'upcoming'),
  ]
  const rows = visibleSteps(eight)
  expect(rows.map(r => (r.kind === 'fold' ? `fold:${r.n}:${r.isDone}` : `step:${r.index}`))).toEqual(['fold:3:true', 'step:3', 'step:4', 'fold:3:false'])
  const first = rows[1]
  expect(first?.kind === 'step' && first.step.name).toBe('4')
})

test('Band: ohne Erledigtes keine Falte davor, kurz vor Ende keine danach, alles erledigt nur eine Zeile', () => {
  const fresh = Array.from({ length: 8 }, (_, i) => step(String(i), 'upcoming'))
  expect(visibleSteps(fresh).map(r => (r.kind === 'fold' ? `fold:${r.n}:${r.isDone}` : `step:${r.index}`))).toEqual(['step:0', 'step:1', 'fold:6:false'])
  const nearEnd = [...Array.from({ length: 6 }, (_, i) => step(String(i), 'done')), step('6', 'active', 1), step('7', 'upcoming')]
  expect(visibleSteps(nearEnd).map(r => (r.kind === 'fold' ? `fold:${r.n}:${r.isDone}` : `step:${r.index}`))).toEqual(['fold:6:true', 'step:6', 'step:7'])
  const allDone = Array.from({ length: 8 }, (_, i) => step(String(i), 'done'))
  expect(visibleSteps(allDone)).toEqual([{ kind: 'fold', n: 8, isDone: true }])
  // eigene Grenze
  expect(visibleSteps(fresh.slice(0, 3), 2).length).toBe(3)
})

test('Weiter: continuePrompt nennt Titel und offene Schritte', () => {
  const job = base({ title: 'Baue das Ding', steps: [step('a', 'done'), step('b', 'active'), step('c', 'upcoming')] })
  const de = continuePrompt(job, 'de')
  expect(de).toContain('„Baue das Ding“')
  expect(de).toContain('Offen: b; c.')
  expect(de).not.toContain('a;')
  expect(continuePrompt(base({ title: 'T' }), 'de')).not.toContain('Offen')
  expect(continuePrompt(job, 'en')).toContain('b; c')
})

test('Verlauf: dayKey, storeKey und shiftDay rechnen lokal und über Monats- und Jahresgrenzen', () => {
  expect(dayKey(new Date(2026, 2, 4, 23, 59).getTime())).toBe('2026-03-04')
  expect(dayKey(new Date(2026, 0, 9, 0, 1).getTime())).toBe('2026-01-09')
  expect(storeKey('2026-03-04')).toBe(`${HISTORY_PREFIX}2026-03-04`)
  expect(shiftDay('2026-03-01', -1)).toBe('2026-02-28')
  expect(shiftDay('2028-03-01', -1)).toBe('2028-02-29')
  expect(shiftDay('2026-12-31', 1)).toBe('2027-01-01')
  expect(shiftDay('2026-03-04', 0)).toBe('2026-03-04')
  expect(shiftDay('2026-10-24', 2)).toBe('2026-10-26')
  expect(shiftDay('2026-03-28', 3)).toBe('2026-03-31')
})

test('Verlauf: staleKeys liefert nur Tage vor dem Fenster und nur eigene Schlüssel', () => {
  expect(HISTORY_DAYS).toBe(30)
  const today = '2026-03-31'
  const keys = ['glance:2026-03-31', 'glance:2026-03-01', 'glance:2026-02-28', 'glance:2025-12-01', 'andere:2020-01-01', 'glance']
  expect(staleKeys(keys, today)).toEqual(['glance:2026-02-28', 'glance:2025-12-01'])
  expect(staleKeys([], today)).toEqual([])
})

test('Verlauf: toRecord macht aus dem Job einen Eintrag, Laufendes und Pausiertes gelten als gestoppt', () => {
  const job = base({
    title: 'Titel', tools: 4, question: '', files: ['a/b.ts'],
    steps: [step('s1', 'done', 100, 400), step('s2', 'active', 400), step('s3', 'upcoming')],
  })
  const rec = toRecord({ ...job, phase: 'done', endedAt: 5_000 }, 9_999)
  expect(rec).toEqual({
    id: 'j1', title: 'Titel', cwd: 'C:\\work\\proj', startedAt: 1_000, endedAt: 5_000, outcome: 'done', question: '', tools: 4,
    steps: [{ name: 's1', done: true, ms: 300 }, { name: 's2', done: false }, { name: 's3', done: false }], files: ['a/b.ts'],
  })
  expect(toRecord({ ...job, phase: 'needsYou', question: 'Frage?', endedAt: 5_000 }, 9_999)).toMatchObject({ outcome: 'needsYou', question: 'Frage?' })
  expect(toRecord(job, 9_999)).toMatchObject({ outcome: 'stopped', endedAt: 9_999 })
  expect(toRecord({ ...job, phase: 'paused' }, 9_999).outcome).toBe('stopped')
  expect(toRecord({ ...job, phase: 'stopped', endedAt: 2 }, 9_999)).toMatchObject({ outcome: 'stopped', endedAt: 2 })
})

test('Verlauf: addRecord ersetzt gleiche id, hängt sonst an und begrenzt auf 200', () => {
  const rec = (id: string, title = id): JobRecord => ({ id, title, cwd: '/p', startedAt: 0, endedAt: 1, outcome: 'done', question: '', tools: 0, steps: [], files: [] })
  const list = [rec('a'), rec('b')]
  expect(addRecord(list, rec('c')).map(r => r.id)).toEqual(['a', 'b', 'c'])
  const replaced = addRecord(list, rec('a', 'neu'))
  expect(replaced.map(r => r.id)).toEqual(['b', 'a'])
  expect(replaced[1]?.title).toBe('neu')
  expect(list.length).toBe(2)
  let big: JobRecord[] = []
  for (let i = 0; i < 210; i++) big = addRecord(big, rec(`r${i}`))
  expect(big.length).toBe(200)
  expect(big[0]?.id).toBe('r10')
  expect(big[199]?.id).toBe('r209')
})

test('Verlauf: recordsOf filtert Müll aus dem Store', () => {
  const ok = { id: 'x', steps: [] }
  expect(recordsOf([ok, null, 5, { id: 1, steps: [] }, { id: 'y' }, 'str'])).toEqual([ok])
  expect(recordsOf(undefined)).toEqual([])
  expect(recordsOf({ id: 'x', steps: [] })).toEqual([])
})

const R = (id: string, over: Partial<JobRecord> = {}): JobRecord => ({
  id, title: id, cwd: 'C:\\work\\proj', startedAt: 0, endedAt: 60_000, outcome: 'done', question: '', tools: 2, steps: [], files: [], ...over,
})

test('Tagesbericht (de): erledigt, offen, Entscheidung und Zeit; schnelle Fragen ohne Tools fehlen', () => {
  const records = [
    R('Login bauen', { steps: [{ name: 'Maske', done: true }, { name: 'Test', done: true }] }),
    R('Refactoring', { outcome: 'stopped', steps: [{ name: 'Split', done: true }, { name: 'Aufräumen', done: false }, { name: 'Doku', done: false }] }),
    R('Deploy', { outcome: 'needsYou', question: 'Auf Prod?', steps: [{ name: 'Build', done: false }] }),
    R('Nur Tools', { tools: 5, endedAt: 120_000 }),
    R('Kurze Frage', { tools: 0, steps: [], endedAt: 999_999 }),
    R('Anderes Projekt', { cwd: 'C:\\work\\other', steps: [{ name: 'Fremd', done: true }] }),
    R('Gestoppt ohne Schritte', { outcome: 'stopped', tools: 3 }),
  ]
  const text = dayReport(records, '2026-03-04', 'C:\\work\\proj', 'de')
  expect(text.split('\n')[0]).toBe('Tagesbericht · proj · 04.03.2026')
  expect(text).toContain('Erledigt\n• Login bauen: Maske; Test\n• Refactoring: Split\n• Nur Tools')
  expect(text).toContain('Offen\n• Refactoring: Aufräumen; Doku\n• Deploy: Build')
  expect(text).toContain('Braucht eine Entscheidung\n• Deploy: Auf Prod?')
  // 60 + 60 + 60 + 120 + 60 Sekunden, fünf Aufgaben
  expect(text.endsWith('Zeit: 6m 0s · 5 Aufgaben')).toBe(true)
  expect(text).not.toContain('Kurze Frage')
  expect(text).not.toContain('Fremd')
  expect(text).not.toContain('Gestoppt ohne Schritte:')
  expect(text).not.toContain('• Gestoppt ohne Schritte\n')
})

test('Tagesbericht (en): englische Überschriften und ISO-Datum, leere Abschnitte entfallen', () => {
  const text = dayReport([R('Fix bug', { cwd: '/home/me/app', steps: [{ name: 'Repro', done: true }] })], '2026-03-04', '/home/me/app', 'en')
  expect(text.split('\n')[0]).toBe('Day report · app · 2026-03-04')
  expect(text).toContain('Done\n• Fix bug: Repro')
  expect(text).not.toContain('Still open')
  expect(text).not.toContain('Needs a decision')
  expect(text.endsWith('Time: 1m 0s · 1 jobs')).toBe(true)
})

test('Tagesbericht: ohne zählende Aufgaben steht „keine Aufgaben“', () => {
  expect(dayReport([], '2026-03-04', '/p/x', 'de')).toBe('Tagesbericht · x · 04.03.2026\n\nKeine Aufgaben in diesem Zeitraum.')
  expect(dayReport([R('q', { tools: 0 })], '2026-03-04', 'C:\\work\\proj', 'en')).toBe('Day report · proj · 2026-03-04\n\nNo jobs in this period.')
})

test('Wochenbericht (de): Summen, stärkster Tag und größte Aufgaben über sieben Tage', () => {
  const byDay = {
    // 2026-03-04 ist ein Mittwoch
    '2026-03-04': [
      R('A', { steps: [{ name: 'a1', done: true }, { name: 'a2', done: true }, { name: 'a3', done: false }] }),
      R('B', { outcome: 'stopped', cwd: 'C:\\work\\other', steps: [{ name: 'b1', done: true }] }),
      R('Kurz', { tools: 0 }),
    ],
    '2026-03-02': [R('C', { endedAt: 120_000, steps: [{ name: 'c1', done: true }] })],
    // außerhalb des Fensters (acht Tage zurück und Folgetag)
    '2026-02-25': [R('Alt', { steps: [{ name: 'x', done: true }] })],
    '2026-03-05': [R('Spät', { steps: [{ name: 'y', done: true }] })],
  }
  const text = weekReport(byDay, '2026-03-04', 'de')
  const lines = text.split('\n')
  expect(lines[0]).toBe('Deine Woche mit Claude · 26.02.2026 bis 04.03.2026')
  expect(text).toContain('Claude hat 4 Schritte in 3 Aufgaben abgehakt, 2 davon fertig.')
  // 60 + 60 + 120 Sekunden in zwei Projekten
  expect(text).toContain('Arbeitszeit: 4m 0s in 2 Projekten.')
  expect(text).toContain('Stärkster Tag: Mittwoch mit 2 Aufgaben.')
  expect(text).toContain('Größte Aufgaben\n• A (3 Schritte)\n• C (1 Schritte)\n• B (1 Schritte)')
  expect(text).not.toContain('Alt')
  expect(text).not.toContain('Spät')
  expect(text).not.toContain('Kurz')
})

test('Wochenbericht (en): Überschriften, Wochentag englisch; ohne Schritte keine „größten“ Aufgaben', () => {
  const byDay = { '2026-03-04': [R('Q', { tools: 3 }), R('W', { tools: 1 })] }
  const text = weekReport(byDay, '2026-03-04', 'en')
  expect(text.split('\n')[0]).toBe('Your week with Claude · 2026-02-26 to 2026-03-04')
  expect(text).toContain('Claude checked off 0 steps in 2 jobs and finished 2 of them.')
  expect(text).toContain('Time at work: 2m 0s across 1 projects.')
  expect(text).toContain('Busiest day: Wednesday, with 2 jobs.')
  expect(text).not.toContain('Biggest jobs')
})

test('Wochenbericht: leere Woche und nur schnelle Fragen sind „keine Aufgaben“', () => {
  expect(weekReport({}, '2026-03-04', 'de')).toBe('Deine Woche mit Claude · 26.02.2026 bis 04.03.2026\n\nKeine Aufgaben in diesem Zeitraum.')
  expect(weekReport({ '2026-03-04': [R('q', { tools: 0 })] }, '2026-03-04', 'en')).toContain('No jobs in this period.')
})

test('jobTitle: Agent-, Session- und Hintergrund-Nachrichten bekommen einen lesbaren Titel statt der XML-Hülle', () => {
  const agent = '<agent-message from="verifier">\n[Subagent hand-back] The text below is the final report. The report follows:\n  Ok Boss 🤠\n  \n  ## Verifikationsbericht (read-only)\n  rest\n</agent-message>'
  expect(jobTitle(agent)).toBe('↩ verifier: Verifikationsbericht (read-only)')
  expect(jobTitle('<cross-session-message from="api-session">\nBitte Tests fahren\n</cross-session-message>')).toBe('✉ api-session: Bitte Tests fahren')
  expect(jobTitle('<task-notification>\n<task-id>x</task-id>\n<summary>Agent "Docs" finished</summary>\n</task-notification>')).toBe('⚙ Agent "Docs" finished')
  expect(jobTitle('Baue die Checkliste')).toBe('Baue die Checkliste')
})
