export interface Arena {
  id: number;
  question: string;
  outcomes: string[];
  status: ArenaStatus;
  totalStakes: number[];
  creator: string;
  createdAt: number;
  resolutionOutcome?: number;
}

export interface Prediction {
  arenaId: number;
  owner: string;
  outcomeIndex: number;
  amount: number;
  placedAt: number;
}

export enum ArenaStatus {
  OPEN = 'OPEN',
  LIVE = 'LIVE',
  RESOLVING = 'RESOLVING',
  CLOSED = 'CLOSED'
}