import { useMemo } from "react";

import { Header } from "../components/Header";
import { PipelineOverview } from "../components/PipelineOverview";

export function App() {
  const pipelineStages = useMemo(
    () => [
      "Ingestion",
      "Chunking",
      "Embedding",
      "Vector Store",
      "Graph Lineage",
      "Memory Engine",
      "Episodic Memory",
      "Orchestration",
      "RAG",
      "LLM Clients"
    ],
    []
  );

  return (
    <div className="app-shell">
      <Header title="FinMemory Market Intelligence" />
      <main>
        <section>
          <h2>System Runbook</h2>
          <p>
            Monitor pipeline health and trigger investigations across ingestion, memory, and
            retrieval components.
          </p>
          <PipelineOverview stages={pipelineStages} />
        </section>
      </main>
    </div>
  );
}
