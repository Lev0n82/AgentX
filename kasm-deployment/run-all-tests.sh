#!/bin/bash
# Master Test Suite - Runs all AgentX Phase 1 tests

echo "=========================================="
echo "  AgentX Phase 1 Master Test Suite"
echo "  Testing All Implementation Components"
echo "=========================================="
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOTAL_PASSED=0
TOTAL_FAILED=0

# Test 1: Deployment Configuration
echo "🧪 Running Test Suite 1: Kasm Deployment Configuration..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if bash "$SCRIPT_DIR/test-deployment.sh"; then
    echo "✅ Deployment tests PASSED"
    ((TOTAL_PASSED++))
else
    echo "❌ Deployment tests FAILED"
    ((TOTAL_FAILED++))
fi

echo ""
echo ""

# Test 2: Server Integration
echo "🧪 Running Test Suite 2: AgentX Server Integration..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if bash "$SCRIPT_DIR/test-server.sh"; then
    echo "✅ Server tests PASSED"
    ((TOTAL_PASSED++))
else
    echo "❌ Server tests FAILED"
    ((TOTAL_FAILED++))
fi

echo ""
echo ""

# Final Summary
echo "=========================================="
echo "  FINAL TEST RESULTS"
echo "=========================================="
echo ""
echo "Test Suites:"
echo "  ✅ Passed: $TOTAL_PASSED"
echo "  ❌ Failed: $TOTAL_FAILED"
echo ""

if [ $TOTAL_FAILED -eq 0 ]; then
    echo "╔════════════════════════════════════════╗"
    echo "║  🎉 ALL TESTS PASSED SUCCESSFULLY! 🎉  ║"
    echo "╚════════════════════════════════════════╝"
    echo ""
    echo "✅ Phase 1.1: Kasm Workspaces Deployment - READY"
    echo "✅ Phase 1.2: AgentX Web Server - OPERATIONAL"
    echo ""
    echo "📋 Acceptance Criteria Status:"
    echo "  ✅ AC-1.1.1: Kasm deployment configuration complete"
    echo "  ✅ AC-1.1.2: Custom workspace images ready"
    echo "  ✅ AC-1.1.3: Environment configuration ready"
    echo "  ✅ AC-1.2.1: AgentX server compiles and runs"
    echo "  ✅ AC-1.2.2: Kasm API client implemented"
    echo "  ✅ AC-1.2.3: WebSocket endpoints functional"
    echo ""
    echo "🚀 Phase 1 Implementation: COMPLETE & TESTED"
    echo ""
    echo "📖 Next Steps:"
    echo "  1. Deploy Kasm: cd kasm-deployment && ./deploy.sh"
    echo "  2. Build images: cd ../kasm-workspaces && ./build-all.sh"
    echo "  3. Start server: cd ../crates/agentx-server && cargo run"
    echo "  4. Begin Phase 2: Frontend Development"
    echo ""
    exit 0
else
    echo "╔════════════════════════════════════════╗"
    echo "║     ⚠️  SOME TESTS FAILED  ⚠️           ║"
    echo "╚════════════════════════════════════════╝"
    echo ""
    echo "Please review failed tests above."
    echo ""
    exit 1
fi
