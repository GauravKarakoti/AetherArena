import React from 'react';
import { ApolloClient, InMemoryCache, ApolloProvider, gql, useMutation, useSubscription } from '@apollo/client';
import { WebSocketLink } from '@apollo/client/link/ws';
import type { Arena } from './types';
import { ArenaStatus } from './types';
import './App.css'

const wsLink = new WebSocketLink({
  uri: 'ws://localhost:8080/graphql',
  options: {
    reconnect: true,
  },
});

const client = new ApolloClient({
  link: wsLink,
  cache: new InMemoryCache(),
  defaultOptions: {
    watchQuery: {
      fetchPolicy: 'no-cache',
    },
    query: {
      fetchPolicy: 'no-cache',
    },
  },
});

// GraphQL Queries
const GET_ARENAS = gql`
  subscription GetArenas($statusFilter: ArenaStatus) {
    arenas(statusFilter: $statusFilter) {
      id
      question
      outcomes
      status
      totalStakes
      creator
      createdAt
    }
  }
`;

const CREATE_ARENA = gql`
  mutation CreateArena($question: String!, $outcomes: [String!]!) {
    createArena(question: $question, outcomes: $outcomes)
  }
`;

const PLACE_PREDICTION = gql`
  mutation PlacePrediction($arenaId: Uint64!, $outcomeIndex: Int!, $amount: Uint64!) {
    placePrediction(arenaId: $arenaId, outcomeIndex: $outcomeIndex, amount: $amount)
  }
`;

const ArenaList: React.FC = () => {
  // Change this line
  const { loading, error, data } = useSubscription(GET_ARENAS, { // <-- Use useSubscription here
    variables: { statusFilter: null }
  });

  if (loading) return <div className="loading">Loading arenas...</div>;
  if (error) return <div className="error">Error: {error.message}</div>;

  return (
    <div className="arena-list">
      <h2>Active Arenas</h2>
      {data?.arenas?.map((arena: Arena) => (
        <ArenaCard key={arena.id} arena={arena} />
      ))}
  M</div>
  );
};

const ArenaCard: React.FC<{ arena: Arena }> = ({ arena }) => {
  const [placePrediction] = useMutation(PLACE_PREDICTION);

  const handlePrediction = async (outcomeIndex: number) => {
    try {
      await placePrediction({
        variables: {
          arenaId: arena.id,
          outcomeIndex,
          amount: 1000000 // 1 LIN in micro units
        }
      });
    } catch (error) {
      console.error('Failed to place prediction:', error);
    }
  };

  return (
    <div className="arena-card">
      <h3>{arena.question}</h3>
      <div className="outcomes">
        {arena.outcomes.map((outcome, index) => (
          <div key={index} className="outcome">
            <span>{outcome}</span>
            <span>Staked: {arena.totalStakes[index]}</span>
            {arena.status === ArenaStatus.OPEN && (
              <button onClick={() => handlePrediction(index)}>
                Predict
              </button>
            )}
          </div>
        ))}
      </div>
      <div className="arena-status">
        Status: {arena.status}
      </div>
    </div>
  );
};

const CreateArenaForm: React.FC = () => {
  const [createArena] = useMutation(CREATE_ARENA);
  const [question, setQuestion] = React.useState('');
  const [outcomes, setOutcomes] = React.useState(['Yes', 'No']);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await createArena({
        variables: { question, outcomes }
      });
      setQuestion('');
    } catch (error) {
      console.error('Failed to create arena:', error);
    }
  };

  return (
    <div className="create-arena">
      <h2>Create New Arena</h2>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          value={question}
          onChange={(e) => setQuestion(e.target.value)}
          placeholder="Enter prediction question..."
          required
        />
        <div className="outcomes-input">
          {outcomes.map((outcome, index) => (
            <input
              key={index}
              type="text"
              value={outcome}
              onChange={(e) => {
                const newOutcomes = [...outcomes];
                newOutcomes[index] = e.target.value;
                setOutcomes(newOutcomes);
              }}
              placeholder={`Outcome ${index + 1}`}
              required
            />
          ))}
        </div>
        <button type="submit">Create Arena</button>
      </form>
    </div>
  );
};

const App: React.FC = () => {
  return (
    <ApolloProvider client={client}>
      <div className="app">
        <header className="app-header">
          <h1>🏟️ AetherArena</h1>
          <p>Real-Time Prediction Markets</p>
        </header>
        <main className="app-main">
          <CreateArenaForm />
          <ArenaList />
        </main>
      </div>
    </ApolloProvider>
  );
};

export default App;