# FinMemory: Rust-Powered AI Memory Engine for Market Intelligence

This repository hosts a modular platform for ingesting, enriching, and retrieving market intelligence through a hybrid memory engine. The structure mirrors the end-to-end data and reasoning flow.

## Vision

**FinMemory** is a memory-powered AI brain for financial institutions that helps analysts, traders, and funds remember, reason, and act across oceans of market data.

## Problem

- Market data, filings, and analyst notes live in siloed systems that do not talk to each other.
- Research teams re-discover old insights; repeat analyses consume 30–40% of their time.
- Traditional LLMs forget context beyond a single session, creating memoryless intelligence.
- Compliance teams struggle to explain how an insight or recommendation was formed.

## Solution Overview

- **Rust-based Memory Core** – ultra-fast, concurrent persistence with lineage tracking (source → insight → decision).
- **Neo4j Graph Provenance** – maps how each insight was derived, including data inputs, models, and human contributors.
- **ChromaDB + RAG** – embeds filings, earnings calls, transcripts, and research notes for retrieval-augmented reasoning.
- **Supermemory + MemMachine** – provides episodic recall so AI agents remember prior analyses and market regimes.
- **React Dashboard** – gives quants and PMs a visualization surface for reasoning chains, comparisons, and chat with the firm’s knowledge base.

## Business Model

| Segment | Value Delivered | Monetization |
| --- | --- | --- |
| Hedge Funds | 10× faster research cycles, persistent context | Per-seat SaaS ($2k–10k/month) |
| Investment Banks | Deal memory and compliance explainability | Enterprise license or private cloud |
| Retail Brokers | Explainable AI trading assistants | API integrations with per-call pricing |
| Research Startups | Memory + LLM API for bespoke analytics | Usage-based API tiers |

## Tech Stack Advantage

| Layer | Tech | Purpose |
| --- | --- | --- |
| Core | Rust | Speed, concurrency, safety for memory persistence |
| Ingestion | FastAPI | Real-time capture of market and user data |
| Storage | ChromaDB + Neo4j | Hybrid vector + graph context storage |
| Orchestration | LangChain + MemMachine | Context persistence, agent coordination, RAG |
| Frontend | React (Vite) | Insight visualization and analyst workflows |
| Models | OpenAI, Claude, Gemini, local LLMs | Multi-model reasoning and summarization |

## Market Opportunity

- Financial AI market projected to exceed $40B by 2030.
- Buy-side research and quant teams spend $10B+ annually on data feeds and analytics.
- Regulatory demand for explainability and lineage is accelerating adoption of memory-first systems.

## Competitive Edge

| Competitor | Focus | FinMemory Advantage |
| --- | --- | --- |
| Kensho (S&P Global) | Query-first analytics | Adds persistent memory graph and explainability |
| AlphaSense | NLP search | Provides episodic intelligence and lineage-aware retrieval |
| BloombergGPT | Model-centric | Overlays a memory and provenance layer on top of models |
| Palantir Foundry | Data fusion platform | Rust-speed memory engine purpose-built for AI workloads |

## Why Now

- Rising compliance scrutiny on AI recommendations demands full lineage.
- LLM proliferation highlights the limits of short-term context windows.
- Financial firms need tools that combine speed, accuracy, and explainability to stay competitive.

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

