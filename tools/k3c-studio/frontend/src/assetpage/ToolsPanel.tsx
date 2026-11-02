import { Button, Flex, Heading, IconButton, Text } from '@webkit/atoms';
import { useState } from 'react';
import type { SourceAddView } from '../api';
import { Proben } from '../sound/Proben';
import { CleanupDialog, ResearchDialog } from '../sources/SourceDialogs';
import '../styles/assetTools.css';

// Werkzeuge-Fläche der Assets-Seite (B-677): öffnet rechts statt des Inspektors. Proben und die beiden Vorrat-Dialoge
// sind die bisherigen Komponenten, das Verhalten bleibt wie auf den alten Seiten Sound und Quellen.

interface Props {
  onClose: () => void;
  /** nach erfolgreichem Aufräumen oder Übernehmen: Seite lädt Slots und Vorrat neu */
  onChanged: () => void;
  /** offline ist die Übernahme aus der Recherche gesperrt (Größe per HEAD nicht möglich) */
  offline: boolean;
}

export function ToolsPanel({ onClose, onChanged, offline }: Props) {
  const [cleanup, setCleanup] = useState(false);
  const [research, setResearch] = useState(false);
  const [done, setDone] = useState<SourceAddView | null>(null);
  return (
    <Flex direction="column" gap="3" className="ap-tools" role="region" aria-label="Werkzeuge">
      <Flex justify="between" align="center">
        <Heading size="3">Werkzeuge</Heading>
        <IconButton variant="ghost" color="gray" aria-label="Werkzeuge schließen" onClick={onClose}>×</IconButton>
      </Flex>
      <section className="ap-tools-sec">
        <Heading size="2">Proben</Heading>
        <Proben />
      </section>
      <section className="ap-tools-sec">
        <Heading size="2">Vorrat</Heading>
        <Flex gap="2" wrap="wrap">
          <Button variant="soft" onClick={() => setCleanup(true)}>Vorrat aufräumen …</Button>
          <Button variant="soft" disabled={offline} onClick={() => setResearch(true)}>Aus Recherche übernehmen …</Button>
        </Flex>
        {offline && <Text size="1" color="gray">Offline: Aus Recherche übernehmen ist gesperrt (Größe per HEAD nicht möglich).</Text>}
        {done && <Text size="2" role="status" data-testid="research-done">Aus Recherche: {done.uebernommen.length} übernommen, {done.uebersprungen.length} übersprungen.</Text>}
      </section>
      <CleanupDialog open={cleanup} onClose={() => setCleanup(false)} onDone={() => { setCleanup(false); onChanged(); }} />
      <ResearchDialog open={research} source="" onClose={() => setResearch(false)} onDone={(v) => { setResearch(false); setDone(v); onChanged(); }} />
    </Flex>
  );
}
