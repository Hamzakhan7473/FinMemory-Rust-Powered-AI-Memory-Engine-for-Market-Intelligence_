# FinMemory: Rust-Powered AI Memory Engine for Market Intelligence

This repository hosts a modular platform for ingesting, enriching, and retrieving market intelligence through a hybrid memory engine. The structure mirrors the end-to-end data and reasoning flow.

## Repository Layout

- `docs/` – Architecture notes, specs, and onboarding guides.
- `backend/` – Python services for ingestion, preprocessing, and knowledge persistence.
  - `ingestion_api/` – FastAPI service handling market data and user queries.
  - `pipeline/` – Document chunking and embedding jobs.
  - `vector_store/` – ChromaDB integrations and management utilities.
  - `graph/` – Neo4j lineage tracking adapters and sync tasks.
- `core/` – Rust crates implementing the memory engine and episodic memory logic.
  - `memory_engine/` – Core Rust project for long-term memory storage/retrieval.
  - `episodic_memory/` – Supermemory / MemMachine episodic coordination modules.
- `orchestration/` – LangChain pipelines, toolchains, and retrieval orchestration.
  - `langchain/` – Workflow definitions and agent wiring.
  - `rag/` – Retrieval-Augmented Generation controllers and prompts.
- `llm_clients/` – Client wrappers for OpenAI, Gemini, Claude, and related providers.
- `frontend/` – Market intelligence dashboard built with React.
  - `dashboard/` – React application entrypoint and UI modules.
- `src_shared/` – Cross-service utilities and shared schemas.
- `infrastructure/` – Deployment manifests, infra-as-code, and service configs.
  - `deployment/` – CI/CD pipelines, container specs, and automation scripts.
  - `config/` – Environment templates and secrets management helpers.
- `scripts/` – Operational scripts for maintenance and developer tooling.
- `data/` – Sample datasets and artifacts for local experimentation.
  - `raw/` – Unprocessed input data.
  - `processed/` – Derived datasets ready for ingestion.
- `tests/` – Cross-cutting integration and system tests.

## Getting Started

### FastAPI Ingestion Service
- Install dependencies: `cd backend/ingestion_api && poetry install`
- Run locally: `poetry run uvicorn app.main:app --reload`
- Execute tests: `poetry run pytest`

### Rust Memory Engine
- Build library: `cd core/memory_engine && cargo build`
- Run unit tests: `cargo test`

### React Dashboard
- Install dependencies: `cd frontend/dashboard && npm install`
- Start dev server: `npm run dev`
- Create production build: `npm run build`

## Next Steps

- Define CI workflows and local development environments.
- Integrate persistence layers (ChromaDB, Neo4j) and orchestrate via LangChain.

