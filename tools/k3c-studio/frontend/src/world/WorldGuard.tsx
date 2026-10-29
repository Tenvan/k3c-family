import { Button, Text } from '@webkit/atoms';
import { useEffect, useState, type ReactNode } from 'react';
import { changesText } from './changes';
import type { PlanState } from './usePlan';
import type { WorldDraftState } from './useWorldDraft';

/** Rückfrage vor einem Wechsel von Welt, Insel, Stufe oder Modus mit offenem Entwurf (B-664/AC-04): Speichern, Verwerfen
 *  oder Abbrechen statt gesperrter Auswahl. „Speichern“ ist gesperrt, solange ein Entwurf ungültig ist. */
export function useGuard(plan: PlanState, world: WorldDraftState): { run: (action: () => void) => void; prompt: ReactNode } {
  const [pending, setPending] = useState<{ action: () => void } | null>(null);
  const dirty = plan.dirty || world.dirty;
  const valid = (!plan.dirty || plan.check.ok) && (!world.dirty || world.check.ok);
  const changes = plan.changes + world.changes;
  useEffect(() => { if (!dirty) setPending(null); }, [dirty]); // ein erledigter Entwurf macht die offene Rückfrage hinfällig

  const finish = () => { const next = pending; setPending(null); next?.action(); };
  const save = async () => {
    if (plan.dirty && !(await plan.save())) { setPending(null); return; } // Stale oder Fehler: Meldung steht im Kopf, der Entwurf bleibt
    if (world.dirty && !(await world.save())) { setPending(null); return; }
    finish();
  };
  const discard = () => { plan.discard(); world.discard(); finish(); };

  const prompt = pending && dirty && (
    <section className="wd-guard" role="alertdialog" aria-label="Ungespeicherte Änderungen">
      <Text size="2">{changesText(changes)} nicht gespeichert. Vor dem Wechsel speichern oder verwerfen?</Text>
      <span className="wd-grow" />
      <Button disabled={!valid} title={valid ? undefined : 'Der Entwurf ist ungültig'} onClick={() => void save()}>Speichern und wechseln</Button>
      <Button variant="outline" color="red" onClick={discard}>Verwerfen und wechseln</Button>
      <Button variant="soft" onClick={() => setPending(null)}>Abbrechen</Button>
    </section>
  );
  return { run: (action) => (dirty ? setPending({ action }) : action()), prompt };
}

export type Guard = ReturnType<typeof useGuard>;
