import { Flex, Heading, Text } from '@webkit/atoms';
import { useEffect, useState } from 'react';
import { reviewApi } from './api';
import { text } from './fehlertext';
import { BasisSelect } from './BasisSelect';
import { Compare } from './Compare';
import { ReviewList } from './ReviewList';
import type { Review } from './types';

/** Review-Seite im Bereich Assets (RV1, Vertrag docs/standards/paper-mcp.md › Oberfläche): jede Asset-Änderung gegen
 *  eine wählbare Git-Basis mit Vorher/Nachher, Pixel-Diff-Overlay und markierten Teilen. Ändert nichts. */
export function ReviewPage() {
  const [basis, setBasis] = useState('HEAD');
  const [review, setReview] = useState<Review | null>(null);
  const [fehler, setFehler] = useState('');
  const [auswahl, setAuswahl] = useState<string | null>(null);
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
  }, [basis]);
  const eintrag = review?.assets.find((a) => a.id === auswahl);
  return (
    <Flex direction="column" gap="3" p="4">
      <Heading size="4">Asset-Review</Heading>
      <BasisSelect basis={basis} onChange={setBasis} />
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
      {review && (
        <Flex gap="4" align="start">
          <ReviewList assets={review.assets} auswahl={auswahl} onSelect={setAuswahl} />
          {eintrag && <Compare key={`${eintrag.id}@${review.basis}`} asset={eintrag} basis={review.basis} />}
        </Flex>
      )}
    </Flex>
  );
}
