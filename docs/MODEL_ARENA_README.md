# AgentX Model Arena 🏟️

**A comprehensive LLM benchmarking and comparison system for evaluating model performance side-by-side.**

---

## Overview

The Model Arena allows you to:
- ⚡ **Compare multiple LLMs** running identical tasks simultaneously
- 📊 **Benchmark performance** with automated metrics (speed, accuracy, efficiency)
- 👁️ **View results side-by-side** with live streaming output
- 🎯 **Create custom tests** tailored to your specific use cases
- 🏆 **Track leaderboards** based on aggregate model performance

This feature helps you select the optimal model for your AgentX workflows by providing empirical data on:
- Code generation quality
- Research and analysis capabilities
- Debugging effectiveness
- API design skills
- Response speed and token efficiency

---

## Architecture

```
┌───────────────────────────────────────────────────────┐
│              Browser UI (React)                       │
│  ┌─────────────┬──────────────┬──────────────────┐   │
│  │ Model       │ Test         │ Live Results     │   │
│  │ Selection   │ Selection    │ Table            │   │
│  ├─────────────┴──────────────┴──────────────────┤   │
│  │ Side-by-Side Output Comparison                │   │
│  │ ┌──────────────┬──────────────┐               │   │
│  │ │ qwen2.5:3b   │ codellama:7b │               │   │
│  │ │ def sum(...) │ def total... │               │   │
│  │ │ ⚡ 2.1s      │ ⚡ 3.4s      │               │   │
│  │ └──────────────┴──────────────┘               │   │
│  └─────────────────────────────────────────────────┘   │
└───────────────────────────┬───────────────────────────┘
                            │ WebSocket + REST API
┌───────────────────────────┴───────────────────────────┐
│          AgentX Server (Rust/Axum)                    │
│  ┌──────────────────────────────────────────────┐     │
│  │ Arena API Endpoints                          │     │
│  │ POST /api/v1/arena/benchmark                │     │
│  │ GET  /api/v1/arena/benchmark/:id            │     │
│  │ GET  /api/v1/arena/models                   │     │
│  │ POST /api/v1/arena/test                     │     │
│  │ GET  /api/v1/arena/leaderboard              │     │
│  └──────────────────────────────────────────────┘     │
│  ┌──────────────────────────────────────────────┐     │
│  │ Benchmark Execution Engine                   │     │
│  │ - Concurrent model execution                │     │
│  │ - Automatic metric calculation              │     │
│  │ - WebSocket event broadcasting              │     │
│  │ - Result persistence                        │     │
│  └──────────────────────────────────────────────┘     │
└───────────────────────────┬───────────────────────────┘
                            │
┌───────────────────────────┴───────────────────────────┐
│              Ollama API                               │
│  ┌──────────┬──────────┬──────────┬──────────┐       │
│  │ qwen2.5  │ llama3.2 │codellama │ deepseek │       │
│  │ :3b      │ :3b      │ :7b      │ :6.7b    │       │
│  └──────────┴──────────┴──────────┴──────────┘       │
└───────────────────────────────────────────────────────┘
```

---

## Features

### 1. Multi-Model Comparison

Run 2-8 models concurrently on identical prompts:

```bash
# Example: Compare 4 models
POST /api/v1/arena/benchmark
{
  "models": [
    "qwen2.5-coder:3b",
    "codellama:7b", 
    "deepseek-coder:6.7b",
    "llama3.2:3b"
  ],
  "test_ids": ["code_gen_sum", "research_compare"],
  "config": { "parallel": true }
}
```

### 2. Benchmark Test Suite

**Pre-built test categories:**

#### 🔧 Code Generation (10 tests)
- Implement functions from docstrings
- Fix buggy code snippets
- Refactor legacy code
- Generate unit tests
- Create CLI scripts
- Implement algorithms
- Parse/validate input
- Generate regex patterns
- Create data structures
- Write async functions

#### 📚 Research & Analysis (8 tests)
- Summarize technical documentation
- Compare technologies (A vs B)
- Extract key information
- Generate research outlines
- Explain technical concepts
- List pros/cons
- Suggest alternatives
- Create glossaries

#### 🐛 Debugging (6 tests)
- Analyze error messages
- Find code issues
- Suggest fixes
- Explain bug causes
- Review code quality
- Scan security vulnerabilities

#### 🔌 API Design (6 tests)
- Design REST endpoints
- Create OpenAPI specs
- Design GraphQL schemas
- Write API documentation
- Design error responses
- Version API schemas

### 3. Evaluation Metrics

**Automatically calculated:**

| Metric | Description | Range |
|--------|-------------|-------|
| **Response Time** | Time to generate complete response | milliseconds |
| **Tokens/Second** | Generation speed | tokens/sec |
| **Total Tokens** | Output length | count |
| **Correctness** | Keyword matching against expected terms | 0.0-1.0 |
| **Efficiency** | Token usage relative to max threshold | 0.0-1.0 |
| **Clarity** | Readability and structure | 0.0-1.0 |
| **Overall Score** | Weighted composite | 0.0-1.0 |
| **Code Execution** | Does generated code run? | true/false |

**Weighted scoring formula:**
```
overall_score = 
  (correctness × correctness_weight) +
  (efficiency × efficiency_weight) +
  (clarity × clarity_weight)

# Default weights:
# Code gen: 50% correctness, 30% efficiency, 20% clarity
# Research: 60% correctness, 20% efficiency, 20% clarity
```

### 4. Side-by-Side Comparison View

Visual comparison showing:
- Syntax-highlighted output
- Response time badges
- Token count indicators
- Accuracy percentages
- Winner highlighting

### 5. Custom Test Creation

Define your own benchmark tests:

```typescript
{
  "id": "my_custom_test",
  "name": "Parse JSON Config",
  "category": "custom",
  "prompt": "Write a Python function to parse and validate a JSON config file...",
  "expected_criteria": {
    "keywords": ["json", "load", "validate"],
    "max_tokens": 100,
    "max_time_ms": 5000,
    "code_executes": true
  },
  "rubric": {
    "correctness_weight": 0.5,
    "efficiency_weight": 0.3,
    "clarity_weight": 0.2
  }
}
```

### 6. Leaderboard & Historical Tracking

Track aggregate model performance:

```
┌─────────────────────────────────────────────────────────┐
│ Model Leaderboard - Code Generation Category           │
├──────┬──────────┬─────┬──────────┬──────────┬──────────┤
│ Rank │ Model    │ Wins│ Avg Time │ Accuracy │ Overall  │
├──────┼──────────┼─────┼──────────┼──────────┼──────────┤
│  1   │codellama │  42 │   3.2s   │   92%    │   0.89   │
│  2   │qwen2.5   │  38 │   2.1s   │   87%    │   0.86   │
│  3   │deepseek  │  35 │   4.8s   │   91%    │   0.84   │
│  4   │llama3.2  │  27 │   5.2s   │   79%    │   0.76   │
└──────┴──────────┴─────┴──────────┴──────────┴──────────┘
```

---

## API Reference

### Create Benchmark Run

```http
POST /api/v1/arena/benchmark
Content-Type: application/json

{
  "models": ["qwen2.5-coder:3b", "codellama:7b"],
  "test_ids": ["code_gen_sum", "research_compare"],
  "config": {
    "temperature": 0.7,
    "top_p": 0.9,
    "max_tokens": 500,
    "parallel": true
  }
}

Response:
{
  "benchmark_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "running",
  "models": ["qwen2.5-coder:3b", "codellama:7b"],
  "tests": ["code_gen_sum", "research_compare"],
  "started_at": "2026-02-05T18:00:00Z"
}
```

### Get Benchmark Status

```http
GET /api/v1/arena/benchmark/:id

Response:
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "completed",
  "progress": 1.0,
  "results": {
    "qwen2.5-coder:3b": [
      {
        "test_id": "code_gen_sum",
        "output": "def calculate_sum(numbers: list) -> int:\n    return sum(numbers)",
        "metrics": {
          "response_time_ms": 2100,
          "total_tokens": 18,
          "tokens_per_sec": 8.57,
          "correctness_score": 1.0,
          "efficiency_score": 0.82,
          "clarity_score": 0.9,
          "overall_score": 0.89
        }
      }
    ],
    "codellama:7b": [...]
  },
  "leaderboard": [...]
}
```

### List Available Models

```http
GET /api/v1/arena/models

Response:
{
  "models": [
    {
      "name": "qwen2.5-coder:3b",
      "size": 1900000000,
      "modified_at": "2026-02-01T12:00:00Z"
    },
    {
      "name": "codellama:7b",
      "size": 3800000000,
      "modified_at": "2026-01-28T10:00:00Z"
    }
  ]
}
```

### Create Custom Test

```http
POST /api/v1/arena/test
Content-Type: application/json

{
  "id": "my_custom_test",
  "name": "Implement Binary Search",
  "category": "code_gen",
  "prompt": "Implement binary search in Python...",
  "expected_criteria": {
    "keywords": ["def", "binary_search", "while", "mid"],
    "max_tokens": 80,
    "max_time_ms": 5000,
    "code_executes": true
  },
  "rubric": {
    "correctness_weight": 0.6,
    "efficiency_weight": 0.3,
    "clarity_weight": 0.1
  }
}

Response:
{
  "id": "my_custom_test",
  "message": "Test created successfully"
}
```

### Get Leaderboard

```http
GET /api/v1/arena/leaderboard?category=code_gen&limit=10

Response:
{
  "models": [
    {
      "model_id": "codellama:7b",
      "total_tests": 50,
      "wins": 42,
      "avg_response_time_ms": 3200,
      "avg_tokens_per_sec": 12.5,
      "avg_correctness": 0.92,
      "avg_efficiency": 0.85,
      "avg_clarity": 0.91,
      "overall_score": 0.89
    }
  ],
  "category": "code_gen",
  "updated_at": "2026-02-05T18:30:00Z"
}
```

---

## WebSocket Events

Subscribe to real-time benchmark updates:

```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  
  switch (message.type) {
    case 'benchmark_started':
      console.log('Benchmark started:', message.benchmark_id);
      console.log('Models:', message.models);
      break;
      
    case 'model_response':
      // Live streaming token chunks
      console.log(`${message.model_id}: ${message.chunk}`);
      break;
      
    case 'test_complete':
      console.log('Test completed:', message.test_id);
      console.log('Metrics:', message.metrics);
      break;
      
    case 'benchmark_complete':
      console.log('All tests finished!');
      console.log('Leaderboard:', message.leaderboard);
      break;
  }
};
```

---

## Usage Examples

### Example 1: Quick Model Comparison

Compare two models on a simple code generation task:

```bash
curl -X POST http://localhost:8080/api/v1/arena/benchmark \
  -H "Content-Type: application/json" \
  -d '{
    "models": ["qwen2.5-coder:3b", "codellama:7b"],
    "test_ids": ["code_gen_sum"],
    "config": { "parallel": true }
  }'
```

### Example 2: Comprehensive Benchmark

Run full test suite across 4 models:

```typescript
const benchmark = await fetch('/api/v1/arena/benchmark', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    models: [
      'qwen2.5-coder:3b',
      'codellama:7b',
      'deepseek-coder:6.7b',
      'llama3.2:3b'
    ],
    test_ids: [
      'code_gen_sum',
      'code_gen_refactor',
      'research_compare',
      'debugging_error'
    ],
    config: {
      temperature: 0.7,
      parallel: true
    }
  })
});

const { benchmark_id } = await benchmark.json();
console.log('Benchmark started:', benchmark_id);
```

### Example 3: Check Results

```typescript
const status = await fetch(`/api/v1/arena/benchmark/${benchmark_id}`);
const data = await status.json();

if (data.status === 'completed') {
  // Find best model
  const winner = data.leaderboard[0];
  console.log(`Winner: ${winner.model_id}`);
  console.log(`Score: ${winner.overall_score}`);
  console.log(`Avg Time: ${winner.avg_response_time_ms}ms`);
}
```

---

## UI Components

### Model Selection Panel

```tsx
<ModelSelector
  availableModels={['qwen2.5:3b', 'codellama:7b', 'deepseek:6.7b']}
  selected={selectedModels}
  onChange={setSelectedModels}
  min={2}
  max={8}
/>
```

### Test Selection Grid

```tsx
<TestSelector
  categories={['code_gen', 'research', 'debugging', 'api_design']}
  selected={selectedTests}
  onChange={setSelectedTests}
/>
```

### Live Results Table

```tsx
<ResultsTable
  benchmark={currentBenchmark}
  sortBy="overall_score"
  highlightWinner
/>
```

### Side-by-Side Comparison

```tsx
<ComparisonView
  benchmark={currentBenchmark}
  models={['qwen2.5:3b', 'codellama:7b']}
  testId="code_gen_sum"
/>
```

---

## Performance Characteristics

**Target Performance:**
- **Concurrent Models:** 2-8 simultaneous executions
- **Benchmark Suite Completion:** <5 minutes (10 tests × 4 models)
- **Real-time Streaming Latency:** <100ms per token
- **Database Write Throughput:** >1000 test results/second
- **WebSocket Broadcast Fanout:** <10ms to all connected clients

**Resource Usage:**
- **Memory:** ~200MB per concurrent model
- **CPU:** 1-2 cores per model during generation
- **Network:** ~50KB/s per model for streaming output

---

## Database Schema

```sql
-- Benchmark runs
CREATE TABLE benchmark_runs (
    id UUID PRIMARY KEY,
    name TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    completed_at TIMESTAMP,
    models TEXT[] NOT NULL,
    test_ids TEXT[] NOT NULL,
    status TEXT NOT NULL
);

-- Test results
CREATE TABLE test_results (
    id UUID PRIMARY KEY,
    benchmark_run_id UUID REFERENCES benchmark_runs(id),
    model_id TEXT NOT NULL,
    test_id TEXT NOT NULL,
    prompt TEXT NOT NULL,
    response TEXT NOT NULL,
    response_time_ms INTEGER,
    total_tokens INTEGER,
    tokens_per_sec REAL,
    correctness_score REAL,
    efficiency_score REAL,
    clarity_score REAL,
    overall_score REAL,
    manual_rating INTEGER,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Model aggregate statistics
CREATE TABLE model_stats (
    model_id TEXT PRIMARY KEY,
    total_runs INTEGER DEFAULT 0,
    total_wins INTEGER DEFAULT 0,
    avg_response_time_ms REAL,
    avg_tokens_per_sec REAL,
    avg_correctness REAL,
    avg_efficiency REAL,
    avg_clarity REAL,
    overall_score REAL,
    last_updated TIMESTAMP DEFAULT NOW()
);

-- Custom benchmark tests
CREATE TABLE custom_tests (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    prompt TEXT NOT NULL,
    expected_criteria JSONB,
    rubric JSONB,
    created_by TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);
```

---

## Integration with AgentX

Use arena results to optimize agent configuration:

```rust
// After benchmarking, select best model for code generation
let leaderboard = arena.get_leaderboard("code_gen").await?;
let best_model = leaderboard.models[0].model_id;

// Configure AgentX to use winning model
let ollama = Ollama::new("http://localhost:11434");
ollama.set_model(&best_model);

// Run agent workflow with optimized model
let company = SoftwareCompany::new(env, ollama);
company.start_project(idea).await?;
```

---

## Roadmap

**Phase 2.3 (Current):**
- ✅ Architecture design complete
- ✅ Backend API specification
- ✅ Frontend component design
- ⏳ Backend implementation
- ⏳ Frontend implementation
- ⏳ Database setup
- ⏳ WebSocket integration

**Future Enhancements:**
- 🔮 **Advanced Metrics:** Code complexity analysis, readability scores
- 🔮 **Multi-Language Support:** JavaScript, Go, Rust code generation tests
- 🔮 **Human Evaluation:** Crowdsourced model ratings
- 🔮 **Cost Tracking:** Token usage costs for cloud-based models
- 🔮 **A/B Testing:** Statistical significance testing
- 🔮 **Model Ensemble:** Combine outputs from multiple models
- 🔮 **Fine-tuning Tracking:** Compare base vs fine-tuned models

---

## Contributing

To add new benchmark tests:

1. Edit `crates/agentx-server/src/api/arena.rs`
2. Add test to `load_benchmark_tests()` function
3. Define expected criteria and rubric
4. Test locally
5. Submit PR

Example:

```rust
BenchmarkTest {
    id: "my_new_test".to_string(),
    name: "My New Test".to_string(),
    category: "code_gen".to_string(),
    prompt: "Your test prompt here...".to_string(),
    expected_criteria: ExpectedCriteria {
        keywords: vec!["keyword1".to_string(), "keyword2".to_string()],
        max_tokens: Some(100),
        max_time_ms: Some(5000),
        code_executes: Some(true),
    },
    rubric: EvaluationRubric {
        correctness_weight: 0.5,
        efficiency_weight: 0.3,
        clarity_weight: 0.2,
    },
},
```

---

## FAQ

**Q: How many models can I compare at once?**  
A: 2-8 models. More than 8 may cause performance degradation.

**Q: Can I use non-Ollama models (OpenAI, Anthropic)?**  
A: Not yet. Currently supports Ollama only. Cloud model support planned for Phase 3.

**Q: How long does a full benchmark take?**  
A: ~5 minutes for 10 tests across 4 models with parallel execution.

**Q: Can I export results?**  
A: Yes, via `/api/v1/arena/benchmark/:id` endpoint (JSON) or CSV export (planned).

**Q: Are test results stored permanently?**  
A: Yes, in PostgreSQL database with full historical tracking.

**Q: Can I create private custom tests?**  
A: Custom tests are workspace-specific. Multi-user with privacy planned for v0.2.

---

## License

MIT License - See main AgentX LICENSE file

---

**Built with ❤️ for the AgentX project**  
*Compare. Benchmark. Optimize.*
