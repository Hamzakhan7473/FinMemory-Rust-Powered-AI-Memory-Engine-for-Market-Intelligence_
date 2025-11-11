from fastapi import APIRouter

from app.api.schemas import HealthResponse, IngestRequest, IngestResponse

router = APIRouter()


@router.get("/health", response_model=HealthResponse)
async def health_check() -> HealthResponse:
    """Basic health probe used for readiness checks."""
    return HealthResponse(status="ok")


@router.post("/ingest", response_model=IngestResponse)
async def ingest_payload(payload: IngestRequest) -> IngestResponse:
    """Accept market data or user queries for downstream processing."""
    # TODO: push payload to task queue / pipeline orchestrator
    return IngestResponse(accepted=True, reference_id=payload.reference_id or "TEMP-REF")
