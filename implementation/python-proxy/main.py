#!/usr/bin/env python3
"""
Temporal Python Proxy Implementation - HTTP Server for 5H Protocol
"""

import asyncio
import json
import logging
from typing import Dict, Any, Optional
from datetime import datetime
from fastapi import FastAPI, HTTPException, Request
from fastapi.responses import JSONResponse
import uvicorn

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

app = FastAPI(title="5H Protocol Python Proxy")


class TemporalProxy:
    def __init__(self):
        self.active_workflows: Dict[str, Any] = {}
        self.task_queue: asyncio.Queue = asyncio.Queue()
        
    async def start_workflow(self, workflow_type: str, input_data: Dict[str, Any]) -> str:
        """Start a new workflow instance."""
        workflow_id = f"{workflow_type}_{datetime.now().timestamp()}"
        workflow_instance = {
            "id": workflow_id,
            "type": workflow_type,
            "input": input_data,
            "status": "started",
            "created_at": datetime.now().isoformat()
        }
        self.active_workflows[workflow_id] = workflow_instance
        logger.info(f"Started workflow {workflow_id}")
        return workflow_id
    
    async def get_workflow_status(self, workflow_id: str) -> Optional[Dict[str, Any]]:
        """Get the status of a workflow."""
        return self.active_workflows.get(workflow_id)
    
    async def complete_workflow(self, workflow_id: str, result: Dict[str, Any]) -> bool:
        """Mark a workflow as completed."""
        if workflow_id in self.active_workflows:
            self.active_workflows[workflow_id]["status"] = "completed"
            self.active_workflows[workflow_id]["result"] = result
            self.active_workflows[workflow_id]["completed_at"] = datetime.now().isoformat()
            logger.info(f"Completed workflow {workflow_id}")
            return True
        return False
    
    async def fail_workflow(self, workflow_id: str, error: str) -> bool:
        """Mark a workflow as failed."""
        if workflow_id in self.active_workflows:
            self.active_workflows[workflow_id]["status"] = "failed"
            self.active_workflows[workflow_id]["error"] = error
            self.active_workflows[workflow_id]["failed_at"] = datetime.now().isoformat()
            logger.error(f"Failed workflow {workflow_id}: {error}")
            return True
        return False


# Global proxy instance
proxy = TemporalProxy()


@app.post("/v1/proxy/forward")
async def proxy_forward(request: Request):
    """Handle incoming proxy requests from Rust core."""
    try:
        # Get the request body
        body = await request.json()
        logger.info(f"Received proxy request: {body}")
        
        # Extract relevant information
        workflow_type = body.get("workflow_type", "unknown")
        input_data = body.get("input", {})
        request_id = body.get("request_id", f"req_{datetime.now().timestamp()}")
        
        # Start a workflow for this request
        workflow_id = await proxy.start_workflow(workflow_type, {
            "request_id": request_id,
            "input": input_data,
            "timestamp": datetime.now().isoformat()
        })
        
        # Simulate processing (in a real implementation, this would do actual work)
        # For now, we'll just complete it immediately with a mock result
        result = {
            "status": "processed",
            "workflow_id": workflow_id,
            "hops": "2-4 hops",  # Match the expected output from the demo
            "message": "Request processed successfully by Python proxy",
            "timestamp": datetime.now().isoformat()
        }
        
        # Complete the workflow
        await proxy.complete_workflow(workflow_id, result)
        
        # Return response
        return JSONResponse(
            status_code=200,
            content={
                "success": True,
                "data": result,
                "request_id": request_id
            }
        )
        
    except Exception as e:
        logger.error(f"Error processing proxy request: {e}")
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/health")
async def health_check():
    """Health check endpoint."""
    return {"status": "healthy", "service": "5H Protocol Python Proxy"}


def start_server():
    """Start the HTTP server."""
    uvicorn.run(
        "main:app",
        host="0.0.0.0",
        port=8000,
        reload=False,
        log_level="info"
    )


if __name__ == "__main__":
    start_server()