from pydantic import BaseModel
from typing import Literal, Optional, List
from datetime import datetime

class ContactRequest(BaseModel):
    request_id: str
    requester_did: str
    target_did: str
    hop_number: int
    intent: dict
    preferred_outcome: Literal["forward", "summarize", "escrow", "connect"]
    consent_receipts: List[dict]
    signature: str
    encryption: str

class ProxyResponse(BaseModel):
    request_id: str
    decision: Literal["forward", "reject", "summarize", "escrow", "accept-and-connect"]
    summary: Optional[str] = None
    escrow_token: Optional[str] = None
    next_hop_did: Optional[str] = None
    consent_receipt: dict
    error: Optional[dict] = None
