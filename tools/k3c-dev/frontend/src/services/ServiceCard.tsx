import { AlertDialog, Button, Flex } from '@radix-ui/themes';
import { useState } from 'react';
import { backend, type ServiceStatus } from '../api';
import { errorText } from '../lib/errors';
import { StatusBadge } from '../ui/parts';
import { LogBox } from './LogBox';
import { badgeFor, buttonsFor, metricsOf, type Command } from './tables';

const ADOPTED_HINT =
  'Vor dem Start von k3c-dev gestartet: keine Konsolenausgabe, kein Auto-Restart. Stoppen nur nach Bestätigung.';

/** Karte eines Dienstes (B-068 › Karten). Den Zustand nach einem Befehl liefert das nächste Ereignis. */
export function ServiceCard({ s }: { s: ServiceStatus }) {
  const [busy, setBusy] = useState<Command | null>(null);
  const [error, setError] = useState('');
  const [confirm, setConfirm] = useState(false);
  const allowed = buttonsFor(s.state);
  const badge = badgeFor(s.state);
  const m = metricsOf(s, Date.now());

  const run = async (cmd: Command, force = false) => {
    setBusy(cmd);
    setError('');
    try {
      if (cmd === 'start') await backend.serviceStart(s.name);
      else if (cmd === 'stop') await backend.serviceStop(s.name, force);
      else await backend.serviceRestart(s.name);
    } catch (e) {
      setError(errorText(e));
    } finally {
      setBusy(null);
    }
  };
  const press = (cmd: Command) => {
    if (cmd === 'stop' && s.state === 'übernommen') setConfirm(true);
    else void run(cmd);
  };
  const button = (cmd: Command, label: string, color?: 'red') => (
    <Button size="2" variant="soft" color={color} loading={busy === cmd}
      disabled={busy !== null || !allowed.includes(cmd)} onClick={() => press(cmd)}>
      {label}
    </Button>
  );

  return (
    <div className="svc-card">
      <div className="svc-head">
        <div>
          <div className="svc-name">{s.name}</div>
          <div className="svc-port">Port {s.port}</div>
        </div>
        <StatusBadge tone={badge.tone}>{badge.label}</StatusBadge>
      </div>
      <code className="svc-health">{s.health}</code>
      <dl className="svc-metrics">
        <Metric label="PID" value={m.pid} />
        <Metric label="CPU" value={m.cpu} />
        <Metric label="SPEICHER" value={m.memory} />
        <Metric label="LAUFZEIT" value={m.uptime} />
      </dl>
      {s.log && <LogBox name={s.name} />}
      {s.restarts > 0 && <p className="svc-note">Neustarts: {s.restarts}</p>}
      {s.lastError && <p className="svc-error">{s.lastError}</p>}
      {s.state === 'übernommen' && <p className="svc-hint">{ADOPTED_HINT}</p>}
      {error && <p className="svc-error">{error}</p>}
      <Flex gap="2" mt="auto">
        {button('start', 'Start')}
        {button('stop', 'Stopp', 'red')}
        {button('restart', 'Neustart')}
      </Flex>
      <ConfirmStop open={confirm} s={s} onOpen={setConfirm} onStop={() => void run('stop', true)} />
    </div>
  );
}

function Metric({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

interface ConfirmProps {
  open: boolean;
  s: ServiceStatus;
  onOpen: (open: boolean) => void;
  onStop: () => void;
}

/** Bestätigung vor dem Stopp eines übernommenen Dienstes (B-068 › Bestätigung). */
function ConfirmStop({ open, s, onOpen, onStop }: ConfirmProps) {
  return (
    <AlertDialog.Root open={open} onOpenChange={onOpen}>
      <AlertDialog.Content maxWidth="440px">
        <AlertDialog.Title>Übernommenen Dienst stoppen?</AlertDialog.Title>
        <AlertDialog.Description size="2">
          {s.name} (PID {s.pid}) läuft nicht unter k3c-dev. Stoppen beendet den fremden Prozess samt Kindprozessen.
        </AlertDialog.Description>
        <Flex gap="3" mt="4" justify="end">
          <AlertDialog.Cancel>
            <Button variant="soft" color="gray">Abbrechen</Button>
          </AlertDialog.Cancel>
          <AlertDialog.Action>
            <Button color="red" onClick={onStop}>Stoppen</Button>
          </AlertDialog.Action>
        </Flex>
      </AlertDialog.Content>
    </AlertDialog.Root>
  );
}
