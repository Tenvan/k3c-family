import { Button, Flex, Heading, Text } from '@webkit/atoms';
import { ListDetail } from '@webkit/templates';
import { useEffect, useState } from 'react';
import { reviewApi } from './api';
import { text } from './fehlertext';
import { BasisSelect } from './BasisSelect';
import { Compare } from './Compare';
import { PromptButtons } from './PromptButtons';
import { ReviewList } from './ReviewList';
import type { Review } from './types';

/** Review-Seite im Bereich Assets (RV1, Vertrag docs/standards/paper-mcp.md › Oberfläche): jede Asset-Änderung gegen
 *  eine wählbare Git-Basis mit Vorher/Nachher, Pixel-Diff-Overlay und markierten Teilen. Ändert nichts. */
export function ReviewPage() {
  const [basis, setBasis] = useState('HEAD');
  const [review, setReview] = useState<Review | null>(null);
  const [fehler, setFehler] = useState('');
  const [auswahl, setAuswahl] = useState<string | null>(null);
  const [stand, setStand] = useState(0);
  useEffect(() => {
    let aktiv = true;
    setFehler('');
    reviewApi.review(basis).then(
      (r) => aktiv && setReview(r),
      (e) => {
        if (!aktiv) return;
        setReview(null);
        setFehler(text(e));
      },
    );
    return () => {
      aktiv = false;
    };
  }, [basis, stand]);
  const eintrag = review?.assets.find((a) => a.id === auswahl);
  const kopf = (
    <>
      <Heading size="4">Asset-Review</Heading>
      <Flex gap="3" align="center" wrap="wrap">
        <BasisSelect basis={basis} onChange={setBasis} />
        <Button size="1" variant="soft" onClick={() => setStand((s) => s + 1)}>
          Neu laden
        </Button>
      </Flex>
      {review && (
        <Text size="2" color="gray">
          Basis {review.basis} ({review.basisCommit}) · {review.assets.length} Assets geändert
        </Text>
      )}
      {review?.basisStand === 'lokal' && <Text size="2" color="amber">Basis-Stand: lokal (ohne Fetch)</Text>}
      {fehler && (
        <Text size="2" color="red" role="alert">
          {fehler}
        </Text>
      )}
    </>
  );
  // Liste und Detail scrollen je für sich (WB2/AC-02)
  return (
    <ListDetail header={kopf} listWidth={360} list={review && <ReviewList assets={review.assets} auswahl={auswahl} onSelect={setAuswahl} />}>
      {review && eintrag && (
        <Flex key={`${eintrag.id}@${review.basisCommit}#${stand}`} direction="column" gap="4">
          <Compare asset={eintrag} basis={review.basisCommit} />
          <PromptButtons asset={eintrag} basis={review.basis} />
        </Flex>
      )}
    </ListDetail>
  );
}
