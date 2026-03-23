#!/usr/bin/env bash
set -euo pipefail

# Apply dependency hints between bootstrap tickets using field updates.
# This script uses metadata hints until dedicated edge commands are fully wired.

ticket update --id 2a1fa2f2-56ce-45cc-a5d4-915d90e6b7a2 --field blocked_by=4f2d2a5e-5df1-4bd8-9b65-0d4de0a0a5c1
ticket update --id de6c3391-27c2-4e27-bde8-1456f0eb3f43 --field blocked_by=4f2d2a5e-5df1-4bd8-9b65-0d4de0a0a5c1
ticket update --id 77f1eb5c-dc38-4221-89e9-2bdf2b8d3ca4 --field blocked_by=4f2d2a5e-5df1-4bd8-9b65-0d4de0a0a5c1
ticket update --id ee43f72e-53ef-4937-8216-92e17f185d85 --field blocked_by=4f2d2a5e-5df1-4bd8-9b65-0d4de0a0a5c1
ticket update --id 5e4727f9-53a6-4d36-a98f-4c9a6db81216 --field blocked_by=4f2d2a5e-5df1-4bd8-9b65-0d4de0a0a5c1
ticket update --id be1a3de7-f44f-496d-b4c6-b4f8a120dc97 --field blocked_by=5e4727f9-53a6-4d36-a98f-4c9a6db81216
ticket update --id 9d0258de-bf87-4b7e-b8f0-e78f4fdf0b58 --field blocked_by=4f2d2a5e-5df1-4bd8-9b65-0d4de0a0a5c1
ticket update --id c91a334e-26cf-4cf2-9212-4288a07bbf09 --field blocked_by=2a1fa2f2-56ce-45cc-a5d4-915d90e6b7a2
ticket update --id 48ea4df8-25f5-46ce-b2cc-ff00d32ddd47 --field blocked_by="2a1fa2f2-56ce-45cc-a5d4-915d90e6b7a2,de6c3391-27c2-4e27-bde8-1456f0eb3f43,77f1eb5c-dc38-4221-89e9-2bdf2b8d3ca4,ee43f72e-53ef-4937-8216-92e17f185d85"

ticket update --id a8d6c1d2-2b64-4d9a-9f1d-1e2a3b4c5d61 --field blocked_by=2a1fa2f2-56ce-45cc-a5d4-915d90e6b7a2
ticket update --id b1f3e2a4-6c7d-4e8f-9a0b-2c3d4e5f6a72 --field blocked_by=a8d6c1d2-2b64-4d9a-9f1d-1e2a3b4c5d61
ticket update --id c2a4b6d8-7e9f-4a1b-8c2d-3e4f5a6b7c83 --field blocked_by=a8d6c1d2-2b64-4d9a-9f1d-1e2a3b4c5d61
ticket update --id d3b5c7e9-8f1a-4b2c-9d3e-4f5a6b7c8d94 --field blocked_by=c2a4b6d8-7e9f-4a1b-8c2d-3e4f5a6b7c83
ticket update --id e4c6d8f1-9a2b-4c3d-8e4f-5a6b7c8d9ea5 --field blocked_by=c2a4b6d8-7e9f-4a1b-8c2d-3e4f5a6b7c83
ticket update --id f5d7e9a2-ab3c-4d5e-9f5a-6b7c8d9eaf16 --field blocked_by="d3b5c7e9-8f1a-4b2c-9d3e-4f5a6b7c8d94,77f1eb5c-dc38-4221-89e9-2bdf2b8d3ca4"
ticket update --id 48ea4df8-25f5-46ce-b2cc-ff00d32ddd47 --field blocked_by="2a1fa2f2-56ce-45cc-a5d4-915d90e6b7a2,de6c3391-27c2-4e27-bde8-1456f0eb3f43,77f1eb5c-dc38-4221-89e9-2bdf2b8d3ca4,ee43f72e-53ef-4937-8216-92e17f185d85,f5d7e9a2-ab3c-4d5e-9f5a-6b7c8d9eaf16"

echo "Applied bootstrap dependency hints."
