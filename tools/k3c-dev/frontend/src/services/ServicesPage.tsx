import { Button, Flex, Heading, Text } from '@radix-ui/themes';
import { useEffect, useRef, useState } from 'react';
import { backend, type ServiceStatus, type ServicesView } from '../api';
import { errorText } from '../lib/errors';
import { NoticeCard } from '../ui/parts';
import { ServiceCard } from './ServiceCard';
import { applyStatus, orderLine } from './tables';

type Bulk = 'start' | 'stop';

/** Reiter `Dienste` (B-068): Kopf mit Sammelaktionen, darunter eine Karte je Dienst. */
export function ServicesPage() {
  const [view, setView] = useState<ServicesView | null>(null);
  const [loadError, setLoadError] = useState('');
  const [bulk, setBulk] = useState<Bulk | null>(null);
  const [bulkError, setBulkError] = useState('');
  const pending = useRef<ServiceStatus[]>([]); // Ereignisse, die vor der Liste kamen

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

  const runBulk = async (kind: Bulk) => {
    setBulk(kind);
    setBulkError('');
    try {
      await (kind === 'start' ? backend.servicesStartAll() : backend.servicesStopAll());
    } catch (e) {
      setBulkError(errorText(e));
    } finally {
      setBulk(null);
    }
  };

  if (loadError) return <NoticeCard title="Dienste nicht geladen" tone="error">{loadError}</NoticeCard>;
  if (!view) return <Text color="gray">Lade Dienste …</Text>;
  if (view.error) return <NoticeCard title="services.json nicht geladen" tone="error">{view.error}</NoticeCard>;
  return (
    <div className="svc-page">
      <Flex justify="between" align="center" gap="3">
        <Heading size="5">Dienste</Heading>
        <Flex gap="2">
          <Button loading={bulk === 'start'} disabled={bulk !== null} onClick={() => void runBulk('start')}>
            Alle starten
          </Button>
          <Button color="red" loading={bulk === 'stop'} disabled={bulk !== null} onClick={() => void runBulk('stop')}>
            Alle stoppen
          </Button>
        </Flex>
      </Flex>
      <Text as="p" size="2" className="svc-order">{orderLine(view.services.map((s) => s.name))}</Text>
      {bulkError && <p className="svc-error">{bulkError}</p>}
      <div className="svc-grid">
        {view.services.map((s) => (
          <ServiceCard key={s.name} s={s} />
        ))}
      </div>
    </div>
  );
}
