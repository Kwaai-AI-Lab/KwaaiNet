# API quickstart: call KwaaiNet like OpenAI

_Audience: application developers, agent builders_

KwaaiNet exposes an OpenAI-compatible HTTP API for chat-completion style inference, backed by CandelEngine's sharded LLM compute. This guide shows how to list models and send chat-completion requests from curl, Python, and JavaScript.

> **Prerequisite:** you have a node running following [`getting-started-node.md`](getting-started-node.md), and its HTTP API is reachable (for example at `http://localhost:11435`).

---

## 1. Endpoints and compatibility

KwaaiNet implements a subset of the OpenAI API surface:

- `GET /v1/models` — list available models on your node and the network.
- `POST /v1/chat/completions` — create chat completions, optionally with streaming.

Differences to be aware of:

- **API keys:** in early versions, the node may accept a dummy API key or use local auth; consult node config and release notes for the current auth model.
- **Models:** model IDs reflect locally hosted and network-available models (e.g. `llama3.1:8b`), not proprietary OpenAI models.
- **Behavior:** routing is trust- and capability-aware; the node may choose local vs network shards based on configuration.

---

## 2. Discover available models

```bash
curl http://localhost:11435/v1/models
```

You should see JSON like:

```json
{
  "data": [
    {
      "id": "llama3.1:8b",
      "object": "model",
      "owned_by": "local",
      "kwaainet": {
        "sharded": true,
        "trust_tier": "Verified"
      }
    }
  ],
  "object": "list"
}
```

Pick a `model` id for the next steps.

---

## 3. Call chat completions with curl

Basic non-streaming request:

```bash
MODEL_ID="llama3.1:8b"  # replace with a model from /v1/models

curl http://localhost:11435/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d "{
    \"model\": \"${MODEL_ID}\",
    \"messages\": [
      {\"role\": \"user\", \"content\": \"Explain Layer 8 in one paragraph.\"}
    ]
  }"
```

KwaaiNet will:

1. Treat this as an intent to run `MODEL_ID` under default trust and latency constraints.
2. Resolve a shard chain of nodes that satisfy the trust and capability requirements.
3. Run sharded inference and return a standard OpenAI-style `chat.completion` response.

Streaming (Server-Sent Events):

```bash
curl http://localhost:11435/v1/chat/completions \
  -H "Content-Type: application/json" \
  -N \
  -d "{
    \"model\": \"${MODEL_ID}\",
    \"stream\": true,
    \"messages\": [
      {\"role\": \"user\", \"content\": \"List three benefits of decentralized AI.\"}
    ]
  }"
```

---

## 4. Use the OpenAI Python client

```bash
pip install openai
```

```python
from openai import OpenAI

client = OpenAI(
    base_url="http://localhost:11435/v1",
    api_key="sk-local-placeholder",  # see node config for actual auth behavior
)

MODEL_ID = "llama3.1:8b"  # replace with a /v1/models id

resp = client.chat.completions.create(
    model=MODEL_ID,
    messages=[
        {"role": "user", "content": "Give me a one-sentence description of KwaaiNet."}
    ],
)

print(resp.choices[0].message.content)
```

Streaming:

```python
stream = client.chat.completions.create(
    model=MODEL_ID,
    stream=True,
    messages=[{"role": "user", "content": "Explain what Layer 8 is in 3 bullet points."}],
)

for chunk in stream:
    delta = chunk.choices[0].delta.content or ""
    print(delta, end="", flush=True)
print()
```

---

## 5. Use JavaScript / TypeScript

```bash
npm install openai
```

```js
import OpenAI from "openai";

const client = new OpenAI({
  baseURL: "http://localhost:11435/v1",
  apiKey: "sk-local-placeholder", // see node config for actual auth behavior
});

const MODEL_ID = "llama3.1:8b"; // replace with a /v1/models id

const resp = await client.chat.completions.create({
  model: MODEL_ID,
  messages: [
    { role: "user", content: "What makes KwaaiNet different from a normal LLM API?" },
  ],
});

console.log(resp.choices[0].message.content);
```

---

## 6. Mapping OpenAI parameters to KwaaiNet

KwaaiNet maps common OpenAI parameters directly into CandelEngine's sampling and routing configuration:

| Parameter | Behavior |
|-----------|----------|
| `model` | Selects the logical model; maps to one or more shard chains. |
| `messages` | Standard chat history; KwaaiNet maintains a per-session KV cache with TTL. |
| `temperature`, `top_p`, `top_k`, `max_tokens` | Control sampling strategy and output length. |
| `stream` | Enables SSE streaming from the API. |

KwaaiNet may also introduce Layer 8-specific extensions in the future, such as:

- Trust requirements (e.g. `"kwaainet_trust_tier": "Verified"`).
- Latency or geography hints.
- Policies for where knowledge (VPK) may be queried.

Any such extensions will be documented in `docs/api-reference.md` once stabilized.

---

## 7. Troubleshooting

If requests fail:

- Check `kwaainet logs --follow` for errors related to model loading, DHT connectivity, or API binding.
- Confirm that `GET /v1/models` works locally before debugging chat completions.
- Verify firewall/port settings if you're calling the node from another machine.
- Compare your `base_url` and `api_key` handling with the examples above and current release notes.

If you hit confusing errors, please open an issue with log snippets (redacted where needed), platform details, and your command or code snippet.

---

## 8. Next steps

- [`docs/roadmap.md`](roadmap.md) — Find roadmap items related to API, routing, or trust you might want to help with.
- `docs/knowledge-and-VPK.md` *(planned)* — Add private knowledge and test encrypted vector search.
- [`docs/contributor-guide.md`](contributor-guide.md) — Build a demo app or agent and contribute example code to the repo.
