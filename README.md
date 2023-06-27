For Form Recognizer version 3.0 

How to use:
1) Build the request and response clients.
2) Pass requests into the request client. Each request will return a promise of a response (Reqwest).
3) Enable feature flags for the following features: automatic deserialization (Serde/serde_json), blocking requests (reqwest), short polling (backoff??), etc.

API Documentation: https://westus.dev.cognitive.microsoft.com/docs/services/form-recognizer-api-2022-08-31/operations/AnalyzeDocument