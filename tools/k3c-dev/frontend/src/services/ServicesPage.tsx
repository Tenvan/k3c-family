import { Button, Flex, Heading, Text } from '@radix-ui/themes';
import { useEffect, useRef, useState } from 'react';
import { backend, type ServiceStatus, type ServicesView } from '../api';
import { errorText } from '../lib/errors';
import { loadText, savePref } from '../lib/prefs';
import { pickSource } from '../logs/lines';
import { SourceBar } from '../logs/SourceBar';
import { SourcePanel } from '../logs/SourcePanel';
import { logFor, otherGroups } from '../logs/sources';
import { useSources } from '../logs/useSources';
import { NoticeCard } from '../ui/parts';
import { ServiceCard } from './ServiceCard';
import { applyStatus, orderLine } from './tables';

type Bulk = 'start' | 'stop';

/**
 * Reiter `Dienste & Logs` (B-068, B-064): links Sammelaktionen, Dienst-Karten und die übrigen Quellen, rechts
 * Konsole und Log der gewählten Quelle.
 */
export function ServicesPage() {
  const { view, loadError } = useServices();
  const { sources, error: srcError } = useSources();
  const [wanted, setWanted] = useState(() => loadText('source', ''));
  const choose = (name: string) => {
    setWanted(name);
    savePref('source', name);
  };

  if (loadError) return <NoticeCard title="Dienste nicht geladen" tone="error">{loadError}</NoticeCard>;
  if (srcError) return <NoticeCard title="Quellen nicht geladen" tone="error">{srcError}</NoticeCard>;
  if (!view || !sources) return <Text color="gray">Lade Dienste …</Text>;
  if (view.error) return <NoticeCard title="services.json nicht geladen" tone="error">{view.error}</NoticeCard>;
  const selected = sources.find((s) => s.name === pickSource(sources, wanted));
  return (
    <div className="svc-page">
      <div className="svc-side">
        <BulkBar names={view.services.map((s) => s.name)} />
        {view.services.map((s) => (
          <ServiceCard key={s.name} s={s} selected={s.name === selected?.name} onSelect={() => choose(s.name)} />
        ))}
        <SourceBar groups={otherGroups(sources, view.services)} selected={selected?.name ?? ''} onSelect={choose} />
      </div>
      {selected ? (
        <SourcePanel key={selected.name} source={selected} log={logFor(selected, view.services, sources)} />
      ) : (
        <NoticeCard title="Keine Quellen" tone="neutral">Weder Dienste noch Läufe noch Log-Dateien.</NoticeCard>
      )}
    </div>
  );
}

/** Dienste live über `service:state`; Ereignisse vor der Liste werden nachgetragen. */
function useServices() {
  const [view, setView] = useState<ServicesView | null>(null);
  const [loadError, setLoadError] = useState('');
  const pending = useRef<ServiceStatus[]>([]);
  useEffect(() => {
    const off = backend.on('service:state', (st) =>
      setView((v) => {
        if (!v) pending.current.push(st);
        return v && { ...v, services: applyStatus(v.services, st) };
      }),
    );
    backend.services().then(
      (v) => setView({ ...v, services: pending.current.reduce(applyStatus, v.services) }),
      (e) => setLoadError(errorText(e)),
    );
    return off;
  }, []);
  return { view, loadError };
}

/** Kopf: Überschrift, Startreihenfolge, Alle starten / Alle stoppen. */
function BulkBar({ names }: { names: string[] }) {
  const [bulk, setBulk] = useState<Bulk | null>(null);
  const [error, setError] = useState('');
  const run = async (kind: Bulk) => {
    setBulk(kind);
    setError('');
    try {
      await (kind === 'start' ? backend.servicesStartAll() : backend.servicesStopAll());
    } catch (e) {
      setError(errorText(e));
    } finally {
      setBulk(null);
    }
  };
  return (
    <div>
      <Flex justify="between" align="center" gap="3">
        <Heading size="5">Dienste</Heading>
        <Flex gap="2">
          <Button size="1" loading={bulk === 'start'} disabled={bulk !== null} onClick={() => void run('start')}>
            Alle starten
          </Button>
          <Button size="1" color="red" loading={bulk === 'stop'} disabled={bulk !== null} onClick={() => void run('stop')}>
            Alle stoppen
          </Button>
        </Flex>
      </Flex>
      <Text as="p" size="1" className="svc-order">{orderLine(names)}</Text>
      {error && <p className="svc-error">{error}</p>}
    </div>
  );
}
