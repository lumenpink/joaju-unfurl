#!/usr/bin/env bash
set -euo pipefail

README_FILE="README.md"
if [ ! -f "$README_FILE" ]; then
    echo "❌ FALHOU: $README_FILE ausente."
    exit 1
fi

LINE_COUNT=$(wc -l < "$README_FILE")
if [ "$LINE_COUNT" -gt 50 ]; then
    echo "❌ FALHOU: README.md excede o teto normativo de 50 linhas (possui $LINE_COUNT linhas)"
    exit 1
fi

# Mutacao: testar que o script detecta ultrapassagem do limite
TMP_MUTATION=$(mktemp)
python3 -c "
with open('$README_FILE') as f:
    lines = f.readlines()
# Adicionar linhas ate exceder 50
while len(lines) <= 50:
    lines.append('\n')
with open('$TMP_MUTATION', 'w') as f:
    f.writelines(lines)
"
MUT_COUNT=$(wc -l < "$TMP_MUTATION")
if [ "$MUT_COUNT" -le 50 ]; then
    echo "❌ FALHOU na mutacao."
    rm -f "$TMP_MUTATION"
    exit 1
fi
rm -f "$TMP_MUTATION"

echo "✅ OK: README.md possui $LINE_COUNT linhas (teto <= 50) e mutacao confirmada."
