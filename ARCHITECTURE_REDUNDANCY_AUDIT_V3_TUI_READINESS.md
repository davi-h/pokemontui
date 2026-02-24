# Auditoria Arquitetural v3 (SEVERA) — Prontidão para início do TUI + Pokédex com `pokeget`

> Escopo desta auditoria: avaliar se o estado atual do código está apto para iniciar a etapa de TUI com Pokédex visual (sprites via `pokeget`), conforme objetivos do `README.md` e recomendações das auditorias anteriores.

---

## 0) Veredito executivo (severo)

**Status atual: NÃO PRONTO para implementação segura do TUI de Pokédex sem retrabalho imediato.**

Principais razões:
1. O comando `pokedex` existe como módulo, mas **não está integrado** ao roteador/registro de comandos.
2. A camada `app` ainda executa fluxo de batalha hardcoded no ciclo de vida, competindo com a ideia de runtime orientado a comandos/TUI.
3. Contratos de Pokédex/sprites e implementações concretas estão **desalinhados por assinatura e responsabilidade**.
4. Há inconsistências de modelo/API que quebram o caminho de dados para construir uma Pokédex real com cache e sprite.

---

## 1) Objetivo de referência para esta fase

Para iniciar TUI com Pokédex visual, o projeto precisa garantir minimamente:

- **Roteamento funcional de comando `pokedex`** (ou tela inicial TUI equivalente).
- **Pipeline único de dados da Pokédex**: fonte de dados → cache → render (sem caminhos paralelos).
- **Contratos coerentes** para sprites e listagem de espécies.
- **Integração operacional com `pokeget`** com fallback/erro explícito.
- **Separação de camadas**: app orquestra, application usa contratos, infrastructure implementa I/O externo.

---

## 2) Achados críticos (P0 — bloqueiam o início do TUI)

## P0.1 — `pokedex` não está no registro de comandos

### Evidência
- `app/src/commands/pokedex.rs` existe com `PokedexCommand`.
- `app/src/container/command_registry.rs` registra apenas `SpawnCommand` e `BattleCommand`.

### Impacto
Sem registro, `app pokedex` falha no roteador (comando desconhecido), inviabilizando o primeiro ciclo de validação da Pokédex via CLI/TUI.

### Ação obrigatória
- Registrar `PokedexCommand` no `CommandRegistry`.
- Garantir que implemente o mesmo contrato de comando usado no roteador.

---

## P0.2 — `PokedexCommand` não implementa o contrato `Command`

### Evidência
- `SpawnCommand` e `BattleCommand` implementam trait `Command`.
- `PokedexCommand` possui apenas método `run(&mut self)` e não implementa `execute(&self, ctx)`.

### Impacto
Mesmo se for registrado, o tipo não encaixa na interface do roteador atual.

### Ação obrigatória
- Adaptar `PokedexCommand` para trait `Command` (ou evoluir o roteador para suportar command handlers homogêneos).

---

## P0.3 — Contrato de sprite incompatível com implementação

### Evidência
- `contracts/src/sprites.rs`: trait `SpriteLoader { fn fetch(&self, name: &str); }` (sem retorno).
- `infrastructure/src/sprites/sprite_loader.rs`: `fetch(&self, name) -> Result<(), String>`, além de `ensure`, `exists`, `file`.

### Impacto
Não existe fronteira confiável para tratamento de erro (ex.: `pokeget` ausente, falha de download), essencial para UX de TUI.

### Ação obrigatória
- Revisar contrato para retornar `Result` e contemplar cache/path.
- Fazer `app/application` depender apenas do contrato, não da struct concreta.

---

## P0.4 — Estrutura de ciclo de vida conflita com runtime de comandos/TUI

### Evidência
- `lifecycle/app.rs` executa batalha hardcoded em `build()` e `run()`, incluindo criação de Pokémon e loop manual.
- `runtime/cli_runtime.rs` já possui modelo de roteamento por argumentos.

### Impacto
Dois entryflows competem: lifecycle de batalha scriptada vs runtime roteado. Isso bloqueia a evolução limpa para TUI de Pokédex.

### Ação obrigatória
- Eleger um único bootstrap runtime para fase atual (recomendado: roteador/comandos como etapa 1 antes de UI full-screen).

---

## 3) Achados altos (P1 — alto risco de retrabalho na fase TUI)

## P1.1 — `PokedexCommand` acoplado a implementações concretas

### Evidência
`PokedexCommand` instancia diretamente `PokeApiClient` e `SpriteLoader` de infrastructure.

### Impacto
Dificulta testes, mock de falhas, e troca de fonte/caching sem modificar camada de app.

### Ação recomendada
Injetar dependências por contratos (`PokemonDataSource`, contrato de sprite revisado, repositório de pokédex).

---

## P1.2 — Caminho de dados da Pokédex é artificial (hardcoded)

### Evidência
`PokedexCommand` usa lista fixa (`bulbasaur/charmander/squirtle`) e `PokedexService::build_entries` fabrica entries locais.

### Impacto
Não valida o objetivo real do README (dados sob demanda + cache local + integração externa).

### Ação recomendada
Substituir por pipeline real: listagem/consulta por espécie + cache + sprite disponível/baixado.

---

## P1.3 — Inconsistência de modelo de API no `engine::pokedex::pokedex_index`

### Evidência
`pokedex_index.rs` usa `data.id`, porém `PokemonApiData` em `contracts` não possui `id`.

### Impacto
Fluxo não é confiável para compilação/evolução da Pokédex indexada.

### Ação recomendada
Padronizar DTO de API (incluir `id` ou ajustar index para fonte coerente).

---

## P1.4 — Contrato de Pokédex minimalista demais para TUI

### Evidência
`contracts::pokedex::PokedexRepository` expõe apenas `list() -> Vec<String>`.

### Impacto
Não cobre paginação, busca, metadados de entrada, estado de sprite/cache — todos necessários para TUI real.

### Ação recomendada
Evoluir contrato para caso de uso de UI (entrada paginada, status de sprite, filtros, erros).

---

## P1.5 — Arquitetura de `app` ainda orientada a demo

### Evidência
`battle` command e lifecycle usam `unwrap/expect` extensivamente e I/O direto via `println!`.

### Impacto
Para TUI, isso degrada resiliência e observabilidade de erro.

### Ação recomendada
Introduzir erros tipados e camadas de apresentação para mensagens de falha.

---

## 4) Achados médios (P2 — qualidade/consistência para próxima sprint)

1. Assinaturas e naming de módulos ainda heterogêneos entre camadas (`api`, `pokeapi`, `service`, `repository`) sem contrato unificado de uso.
2. Fluxos de cache de sprites e cache de dados não estão explicitamente orquestrados no `application`.
3. Falta definição de “modo offline” para quando `pokeget` ou rede não estão disponíveis.

---

## 5) Blueprint mínimo (mudança mínima para iniciar TUI sem retrabalho)

## Etapa A — “CLI funcional de Pokédex” (pré-TUI full-screen)
1. Tornar `pokedex` um comando real (trait `Command` + registro no router).
2. Criar `application::query` de pokédex que retorna entries prontas para render textual.
3. Injetar contratos no command (sem new direto de infra).

## Etapa B — “Pipeline único de sprite/cache”
4. Revisar `contracts::sprites` para `Result` + `ensure` + `path`.
5. Adaptar `infrastructure::sprites::SpriteLoader` para implementar esse contrato.
6. Definir fallback: sprite ausente não derruba comando, apenas marca estado.

## Etapa C — “TUI bootstrap”
7. No `app`, substituir prints do comando por adapter de apresentação de lista/página.
8. Integrar teclado básico (next/prev/quit) com estado de paginação.
9. Só então evoluir para telas compostas (detalhe de Pokémon + sprite + metadados).

---

## 6) Critérios de aceite para liberar início do TUI

- `app pokedex` resolve comando sem erro de roteamento.
- Para um nome válido, o fluxo tenta `ensure sprite` e informa resultado sem panic.
- Com `pokeget` ausente, aplicação não cai; retorna erro amigável.
- Estruturas de `app` não instanciam infra diretamente (injeção por contrato).
- Não existe fluxo paralelo de boot que bypassa runtime de comandos para casos de uso da Pokédex.

---

## 7) Matriz de severidade consolidada

- **P0 (bloqueio):** roteamento/contrato do comando pokédex, contrato de sprite, conflito de lifecycle.
- **P1 (alto risco):** acoplamento direto em app, dados hardcoded, inconsistência de DTO/API, contrato de pokédex insuficiente.
- **P2 (médio):** padronização de nomenclatura e operacionalização de cache/offline.

---

## 8) Conclusão severa

O repositório avançou na limpeza de redundâncias estruturais, mas **a prontidão para iniciar TUI de Pokédex ainda está abaixo do mínimo operacional**.

A recomendação objetiva é executar primeiro um “hardening de integração” (P0 + parte de P1) para transformar `pokedex` em fluxo canônico e testável. Sem isso, a implementação de TUI tenderá a acumular retrabalho e exceções ad hoc já na primeira sprint.
