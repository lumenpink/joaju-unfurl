# Spec: Ticket #11 - Alinhar ao Perfil Lilya e ci.yml

## 1. Contexto e Motivação
Alinhamento do repositório `joaju-unfurl` ao perfil `lilya` (`https://lilya.2lp.in/profile.md`), adoção de `.forgejo/workflows/ci.yml`, atualização dos scripts de cascata e higienização de terminologia obsoleta.

## 2. Requisitos Técnicos
- Renomeação de `.forgejo/workflows/ci.yaml` para `.forgejo/workflows/ci.yml`.
- Atualização de `tests/audit_gitops_cascade.sh` e `.forgejo/workflows/ci.yml` para direcionar dispatches a `ci.yml`.
- Higienização de termos obsoletos em `site/index.html` e scripts de teste.
- Conformidade total com `cccp check --strict`.

## 3. Critérios de Aceitação
- Testes unitários e testes de cascata GitOps 100% aprovados.
- 100% de conformidade com `cccp check --strict`.


## 4. Conclusão da Fase 9
- Workflow renomeado para `.forgejo/workflows/ci.yml`.
- Auditoria `tests/audit_gitops_cascade.sh` 100% verde.
- Perfil GOST alinhado para Lilya e vocabulário higienizado.
