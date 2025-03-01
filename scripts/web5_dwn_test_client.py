#!/usr/bin/env python3
"""
Test client for Web5 DWN functionality with Bitcoin anchoring.

This script demonstrates how to use the Web5 API with enhanced DWN functionality,
including message processing with Bitcoin anchoring, querying with anchoring
verification, and status checking.

Usage:
    python web5_dwn_test_client.py
"""

import argparse
import json
import requests
import time
import sys
from typing import Dict, Any, List, Optional, Union
from uuid import uuid4

# Default server URL
DEFAULT_SERVER_URL = "http://localhost:8000"

class Web5Client:
    """Client for interacting with the Web5 API."""
    
    def __init__(self, server_url: str = DEFAULT_SERVER_URL):
        """Initialize the Web5 client."""
        self.server_url = server_url
        self.session = requests.Session()
    
    def create_did(self) -> Dict[str, Any]:
        """Create a new DID."""
        url = f"{self.server_url}/api/web5/did"
        response = self.session.post(url)
        response.raise_for_status()
        return response.json()
    
    def resolve_did(self, did: str) -> Dict[str, Any]:
        """Resolve a DID."""
        url = f"{self.server_url}/api/web5/did/{did}"
        response = self.session.get(url)
        response.raise_for_status()
        return response.json()
    
    def create_dwn(self, did: str) -> Dict[str, Any]:
        """Create a new DWN for a DID."""
        url = f"{self.server_url}/api/web5/dwn/create"
        data = {"did": did}
        response = self.session.post(url, json=data)
        response.raise_for_status()
        return response.json()
    
    def process_dwn_message(self, did: str, message: Dict[str, Any]) -> Dict[str, Any]:
        """Process a DWN message."""
        url = f"{self.server_url}/api/web5/dwn/process"
        data = {"did": did, "message": message}
        response = self.session.post(url, json=data)
        response.raise_for_status()
        return response.json()
    
    def process_dwn_message_enhanced(
        self, 
        did: str, 
        message: Dict[str, Any], 
        options: Dict[str, Any]
    ) -> Dict[str, Any]:
        """Process a DWN message with enhanced features."""
        url = f"{self.server_url}/api/web5/dwn/process/enhanced"
        data = {"did": did, "message": message, "options": options}
        response = self.session.post(url, json=data)
        response.raise_for_status()
        return response.json()
    
    def query_dwn(self, did: str, query: Dict[str, Any]) -> List[Dict[str, Any]]:
        """Query a DWN."""
        url = f"{self.server_url}/api/web5/dwn/query"
        data = {"did": did, "query": query}
        response = self.session.post(url, json=data)
        response.raise_for_status()
        return response.json()
    
    def query_dwn_anchored(
        self, 
        did: str, 
        query: Dict[str, Any], 
        min_confirmations: int
    ) -> List[Dict[str, Any]]:
        """Query a DWN with anchoring verification."""
        url = f"{self.server_url}/api/web5/dwn/query/anchored"
        data = {"did": did, "query": query, "min_confirmations": min_confirmations}
        response = self.session.post(url, json=data)
        response.raise_for_status()
        return response.json()
    
    def anchor_dwn_message(self, did: str, message_id: str) -> Dict[str, Any]:
        """Anchor a DWN message to Bitcoin."""
        url = f"{self.server_url}/api/web5/dwn/anchor/{did}/{message_id}"
        response = self.session.post(url)
        response.raise_for_status()
        return response.json()
    
    def get_dwn_anchoring_status(self, did: str, message_id: str) -> Dict[str, Any]:
        """Get anchoring status for a DWN message."""
        url = f"{self.server_url}/api/web5/dwn/status/{did}/{message_id}"
        response = self.session.get(url)
        response.raise_for_status()
        return response.json()
    
    def verify_dwn_anchoring(self, did: str, message_id: str) -> Dict[str, Any]:
        """Verify Bitcoin anchoring for a DWN message."""
        url = f"{self.server_url}/api/web5/dwn/verify/{did}/{message_id}"
        response = self.session.get(url)
        response.raise_for_status()
        return response.json()

    def issue_credential(
        self,
        issuer_did: str,
        subject_did: str,
        credential_type: str,
        claims: Dict[str, Any],
        valid_for_days: Optional[int] = None,
        anchor_to_bitcoin: bool = False
    ) -> Dict[str, Any]:
        """Issue a verifiable credential."""
        url = f"{self.server_url}/api/web5/credential/issue"
        data = {
            "issuer_did": issuer_did,
            "subject_did": subject_did,
            "credential_type": credential_type,
            "claims": claims,
            "valid_for_days": valid_for_days,
            "anchor_to_bitcoin": anchor_to_bitcoin
        }
        response = self.session.post(url, json=data)
        response.raise_for_status()
        return response.json()

def create_records_collection_message(did: str) -> Dict[str, Any]:
    """Create a DWN message for creating a records collection."""
    return {
        "id": str(uuid4()),
        "type": "RecordsCreate",
        "from": did,
        "to": did,
        "recordId": str(uuid4()),
        "data": json.dumps({
            "name": "Test Collection",
            "description": "A test collection for Web5 DWN"
        }),
        "dataFormat": "application/json",
        "schema": "https://schema.org/Collection",
        "parentId": None,
        "published": True,
        "dateCreated": int(time.time() * 1000)
    }

def create_record_message(did: str, data: Dict[str, Any]) -> Dict[str, Any]:
    """Create a DWN message for creating a record."""
    return {
        "id": str(uuid4()),
        "type": "RecordsWrite",
        "from": did,
        "to": did,
        "recordId": str(uuid4()),
        "data": json.dumps(data),
        "dataFormat": "application/json",
        "schema": None,
        "parentId": None,
        "published": True,
        "dateCreated": int(time.time() * 1000)
    }

def create_query_message(did: str) -> Dict[str, Any]:
    """Create a DWN query message."""
    return {
        "id": str(uuid4()),
        "type": "RecordsQuery",
        "from": did,
        "to": did,
        "schema": None,
        "dataFormat": "application/json",
        "published": True
    }

def demo_web5_dwn_functionality():
    """Demonstrate Web5 DWN functionality with Bitcoin anchoring."""
    client = Web5Client()
    
    print("Creating a new DID...")
    did_result = client.create_did()
    did = did_result.get("id")
    print(f"Created DID: {did}")
    
    print("\nCreating a DWN for the DID...")
    dwn_result = client.create_dwn(did)
    print(f"DWN created: {dwn_result}")
    
    print("\nCreating a record in the DWN with Bitcoin anchoring...")
    record_data = {
        "title": "Test Document",
        "content": "This is a test document with Bitcoin anchoring",
        "tags": ["test", "web5", "bitcoin"]
    }
    record_message = create_record_message(did, record_data)
    
    # Enhanced options with Bitcoin anchoring
    enhanced_options = {
        "anchor_to_bitcoin": True,
        "wait_for_broadcast": True,
        "include_anchoring_status": True
    }
    
    result = client.process_dwn_message_enhanced(did, record_message, enhanced_options)
    message_id = record_message["id"]
    print(f"Record created with ID: {message_id}")
    print(f"Result: {json.dumps(result, indent=2)}")
    
    print("\nChecking anchoring status...")
    time.sleep(1)  # Wait a bit for anchoring to start
    
    status = client.get_dwn_anchoring_status(did, message_id)
    print(f"Anchoring status: {json.dumps(status, indent=2)}")
    
    print("\nQuerying the DWN...")
    query_message = create_query_message(did)
    results = client.query_dwn(did, query_message)
    print(f"Query results: {json.dumps(results, indent=2)}")
    
    print("\nVerifying anchoring...")
    verified = client.verify_dwn_anchoring(did, message_id)
    print(f"Anchoring verification: {json.dumps(verified, indent=2)}")
    
    print("\nDemonstration complete!")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Test client for Web5 DWN functionality")
    parser.add_argument("--server", default=DEFAULT_SERVER_URL, help="Server URL")
    args = parser.parse_args()
    
    try:
        demo_web5_dwn_functionality()
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
