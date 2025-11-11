from fastapi import FastAPI

from app.api.routes import router as api_router


def create_app() -> FastAPI:
    """Application factory for FastAPI service."""
    app = FastAPI(title="FinMemory Ingestion API", version="0.1.0")

    app.include_router(api_router, prefix="/v1")
    return app


app = create_app()
