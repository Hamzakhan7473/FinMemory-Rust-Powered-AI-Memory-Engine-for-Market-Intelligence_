from datetime import datetime
from typing import Literal, Optional

from pydantic import BaseModel, Field


class HealthResponse(BaseModel):
    status: Literal["ok"]
    timestamp: datetime = Field(default_factory=datetime.utcnow)


class IngestRequest(BaseModel):
    payload_type: Literal["market_data", "user_query", "news"]
    content: str
    metadata: dict[str, str] | None = None
    reference_id: Optional[str] = None


class IngestResponse(BaseModel):
    accepted: bool
    reference_id: str
