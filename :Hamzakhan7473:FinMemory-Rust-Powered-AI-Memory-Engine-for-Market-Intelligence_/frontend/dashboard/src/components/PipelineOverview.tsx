interface PipelineOverviewProps {
  stages: string[];
}

export function PipelineOverview({ stages }: PipelineOverviewProps) {
  return (
    <div className="pipeline-overview">
      {stages.map((stage) => (
        <article key={stage} className="pipeline-card">
          <h3>{stage}</h3>
          <p>Status: <span className="status-tag status-tag--pending">Pending integration</span></p>
        </article>
      ))}
    </div>
  );
}
