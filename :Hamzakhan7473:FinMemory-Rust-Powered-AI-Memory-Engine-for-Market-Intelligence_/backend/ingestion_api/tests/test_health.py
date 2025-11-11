import pytest
from httpx import AsyncClient
from fastapi import FastAPI

from app.main import create_app


@pytest.fixture()
def app() -> FastAPI:
    return create_app()


@pytest.mark.asyncio
async def test_health_endpoint(app: FastAPI) -> None:
    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.get("/v1/health")

    assert response.status_code == 200
    body = response.json()
    assert body["status"] == "ok"
    assert "timestamp" in body
