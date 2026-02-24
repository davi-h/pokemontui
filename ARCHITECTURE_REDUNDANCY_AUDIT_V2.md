# Auditoria de Redundâncias Arquiteturais (v2 — orientada a conteúdo/função)

Esta auditoria foi gerada considerando:

1. O objetivo declarado no `README.md` (simulação determinística em terminal, arquitetura modular/extensível, integração com PokéAPI + `pokeget`, cache local).
2. A auditoria anterior (`ARCHITECTURE_REDUNDANCY_AUDIT.md`).
3. A implementação real por **conteúdo e função dos módulos**, não apenas por nome de pastas/arquivos.

---

## 1) Critério de aderência ao objetivo do projeto

Para o escopo do PokémonTUI, a arquitetura deveria privilegiar:

- **Fonte única de regras de negócio** (spawn, batalha, dano, raridade).
- **Determinismo controlado** (RNG único por contrato, seedável, reaproveitado).
- **Fronteiras limpas** (contratos em `contracts`, implementação em `infrastructure/adapters`, orquestração em `app`, regras em `domain/engine/application`).
- **Baixo custo de manutenção** (evitar caminhos paralelos para mesma funcionalidade externa: PokéAPI e `pokeget`).

Os pontos abaixo classificam onde o código diverge disso.

---

## 2) Redundâncias funcionais encontradas

## A. Modelo de Pokémon duplicado com semânticas diferentes

### Evidência de conteúdo
- `domain::pokemon::entity::Pokemon` modela entidade de batalha/simulação (`level`, `stats`, `shiny`).
- `logic-core::models::Pokemon` modela DTO/API (`id`, `types`) e convive com `Move`, `Item`, `PokedexEntry`, `GameState`.

### Problema arquitetural
Há duas “verdades” para `Pokemon`, cada uma atendendo objetivos diferentes, mas sem uma fronteira explícita entre **entidade de domínio** e **modelo de integração/apresentação**.

### Impacto no objetivo do projeto
- Aumenta conversões implícitas entre camadas.
- Dificulta evolução de regras táticas (batalha/spawn) com previsibilidade.

### Recomendação
- Manter `domain::Pokemon` como canônico para simulação.
- Renomear modelos de `logic-core` para DTOs explícitos (`PokemonApiDto`, `PokedexViewModel`) ou absorvê-los em `contracts/infrastructure`.

---

## B. Fronteira de API externa triplicada (contracts + infrastructure + logic-core + adapters)

### Evidência de conteúdo
- `contracts/src/api/*` define contrato (`PokemonDataSource`, `PokemonApiData`, `ApiError`).
- `infrastructure/src/api/pokeapi_client.rs` implementa cliente HTTP real com mapeamento de stats.
- `logic-core/src/api/mod.rs` define outro contrato (`ApiClient`) e outra implementação (`ReqwestClient`) com `fetch_sprite_ascii` via `pokeget`.
- `adapters/src/pokeapi/client.rs` implementa terceiro cliente com responsabilidades semelhantes.

### Problema arquitetural
A mesma capacidade (consumir PokéAPI e, em partes, sprite) foi implementada por múltiplas rotas sem um padrão único de uso.

### Impacto no objetivo do projeto
- Aumenta risco de inconsistência de dados (cada cliente trata campos/erros de forma distinta).
- Contraria modularidade extensível: para evoluir API/caching, há vários pontos para manter.

### Recomendação
- Definir uma única fronteira pública em `contracts::api`.
- Escolher **um** local para implementação HTTP principal (`infrastructure` ou `adapters`) e transformar o outro em thin wrapper (ou remover).
- Remover contrato paralelo de `logic-core::api`.

---

## C. Execução de `pokeget` redundante em múltiplos módulos

### Evidência de conteúdo
- `logic-core/src/api/mod.rs` executa `pokeget` em `fetch_sprite_ascii`.
- `adapters/src/sprite/render.rs` executa `pokeget` diretamente para render.
- `infrastructure/src/sprites/sprite_loader.rs` também executa `pokeget`, com cache em arquivo.

### Problema arquitetural
A mesma integração externa (sprite) está fragmentada em 3 caminhos com comportamentos diferentes (stdout, render direto, persistência em arquivo).

### Impacto no objetivo do projeto
- Enfraquece o requisito de cache local consistente do README.
- Dificulta fallback e tratamento uniforme de erro em ambiente sem `pokeget`.

### Recomendação
- Consolidar toda chamada a `pokeget` num único adaptador/serviço.
- Expor capacidades por interface (`ensure/fetch/render`) no mesmo componente.
- Demais camadas devem apenas consumir contrato.

---

## D. RNG inconsistente com múltiplas implementações concorrentes

### Evidência de conteúdo
- `contracts::rng::Rng` define trait comum.
- `infrastructure::rng::SeededRng` implementa com `StdRng` seedável.
- `engine::rng::GameRng` usa `thread_rng` estático (não injetável).
- `engine::spawn::rng::SeededRng` reimplementa RNG próprio (xorshift).

### Problema arquitetural
Há coexistência de RNG injetável e RNG global/não determinístico, além de dois RNG seedáveis distintos.

### Impacto no objetivo do projeto
- Compromete “simulações reproduzíveis” citadas no README.
- Comportamento de spawn/batalha pode variar conforme módulo usado.

### Recomendação
- Padronizar no contrato `contracts::rng::Rng` + implementação única seedável.
- Remover `thread_rng` de caminhos críticos de simulação.
- Unificar em um único `SeededRng` para toda pipeline de batalha/spawn.

---

## E. Spawn com múltiplos motores/serviços sobrepostos

### Evidência de conteúdo
No `engine` coexistem várias abordagens:
- `spawn/engine.rs`: motor com regras (`SpawnRule`) + modificadores de peso.
- `spawn/distribution.rs`: outro `SpawnEngine` simplificado baseado em `SpawnTable`.
- `spawn/spawn_engine.rs`: terceiro motor genérico com `PokemonFactory`.
- `spawn/spawn_service.rs`: serviço de spawn com `RarityEngine`, ambiente e shiny chance.

### Problema arquitetural
Quatro caminhos para “gerar Pokémon”, com diferenças de assinatura, dependências e estratégia.

### Impacto no objetivo do projeto
- Dificulta manter uma lógica única de “spawn inteligente” (raridade/evento/ambiente).
- Aumenta risco de regressão ao ajustar balanceamento.

### Recomendação
- Eleger **um pipeline oficial** de spawn (ex.: regra + modificadores + ambiente + factory).
- Rebaixar os demais para testes/experimentos (com nome explícito) ou remover.
- Garantir que CLI/use-cases chamem só o pipeline oficial.

---

## F. Regra de dano duplicada e divergente

### Evidência de conteúdo
- `domain/src/battle/damage.rs`: dano base por `attack - defense/2`.
- `engine/src/stats/calculator.rs`: mesma fórmula com piso mínimo de `1`.

### Problema arquitetural
Mesma regra em dois lugares com resultados potencialmente diferentes em cenários de ataque baixo.

### Impacto no objetivo do projeto
- Simulação tática perde previsibilidade (resultado depende de qual função foi chamada).

### Recomendação
- Definir uma única regra canônica de dano e reaproveitá-la em todo fluxo.

---

## G. Caso de uso de spawn duplicado em `application` e `app`

### Evidência de conteúdo
- `application/src/commands/spawn_pokemon.rs`: use-case via `contracts::spawn::SpawnService`.
- `app/src/container/usecases.rs`: outro `SpawnPokemon` acoplado a `engine::PokemonFactory` e I/O direto (`println!`).

### Problema arquitetural
A camada `app` mistura orquestração com regra de caso de uso.

### Impacto no objetivo do projeto
- Dificulta testes e plugins no nível de aplicação.
- Contraria arquitetura em camadas citada no README.

### Recomendação
- Manter casos de uso apenas em `application`.
- `app` deve só fazer wiring/CLI/roteamento.

---

## H. Infraestrutura de PokéAPI redundante (`infrastructure` x `adapters`)

### Evidência de conteúdo
- `infrastructure/src/pokeapi/*`: cliente, DTOs, repositório, builder de species registry.
- `adapters/src/pokeapi/*`: cliente, parser, cache, modelos, providers.

### Problema arquitetural
Há dois “sub-sistemas” de integração com PokéAPI crescendo paralelamente.

### Impacto no objetivo do projeto
- Alto custo de manutenção para evolução de cache e parser.
- Difícil estabelecer qual é o caminho oficial para produção.

### Recomendação
- Escolher um módulo oficial para integração externa (infrastructure **ou** adapters).
- Mover componentes úteis do outro lado e descontinuar duplicados.

---

## I. Camadas vazias/incipientes com baixa função atual

### Evidência de conteúdo
- `shared`, `plugins`, `testing` praticamente vazios.
- `interface` com comentário de diretriz, sem implementação funcional.

### Problema arquitetural
A topologia sugere maturidade modular maior que a capacidade efetiva entregue hoje.

### Impacto no objetivo do projeto
- Pode gerar complexidade acidental e confusão de onboarding.

### Recomendação
- Ou preencher com responsabilidade concreta de curto prazo.
- Ou remover/arquivar temporariamente para reduzir superfície arquitetural.

---

## 3) Priorização orientada ao objetivo do README

### Prioridade 1 — Determinismo e regra única (impacto direto na simulação)
1. Unificar RNG.
2. Unificar regra de dano.
3. Eleger pipeline único de spawn.

### Prioridade 2 — Fronteiras de integração externa
4. Consolidar PokéAPI em um único caminho.
5. Consolidar `pokeget` em um único serviço com cache.

### Prioridade 3 — Limpeza de camadas e governança
6. Remover duplicidade de use-case em `app`.
7. Enxugar `logic-core` (ou redefinir papel).
8. Reavaliar crates vazios.

---

## 4) Resultado executivo

A arquitetura atual contém componentes sólidos, mas apresenta **redundância funcional relevante** em áreas centrais (spawn, RNG, API externa e modelos), o que conflita com os objetivos de simulação determinística e modularidade evolutiva descritos no README.

A ação mais eficaz é reduzir para **um caminho canônico por capacidade crítica** (spawn, dano, RNG, PokéAPI, sprites), mantendo contratos explícitos e composição apenas na camada `app`.

---

## 5) Reestruturação aplicada (mínima, sem apagar módulos)

Para reduzir redundância sem excluir código histórico, os caminhos paralelos foram movidos para `test/` nos respectivos crates, preservando rastreabilidade e contexto:

- `crates/logic-core/src/{api,models,storage,systems}` → `crates/logic-core/src/test/*`.
- `crates/adapters/src/{pokeapi,sprite,weather_api}` → `crates/adapters/src/test/*`.
- `crates/engine/src/api` → `crates/engine/src/test/api`.
- `crates/engine/src/rng/mod.rs` → `crates/engine/src/test/rng/mod.rs`.
- `crates/engine/src/spawn/{distribution.rs,engine.rs,spawn_engine.rs}` → `crates/engine/src/spawn/test/*`.
- `crates/app/src/container/usecases.rs` → `crates/app/src/container/test/usecases.rs`.
- `crates/infrastructure/src/pokeapi/species_registry_builder.rs` → `crates/infrastructure/src/pokeapi/test/species_registry_builder.rs`.

Além disso, os `mod`/exports e dependências foram simplificados para reforçar o caminho canônico em produção:

- `app` sem dependências diretas de `logic-core` e `adapters`.
- `engine` sem dependências diretas de `infrastructure` e `logic-core`.
- `infrastructure` sem dependência direta de `engine`.

Isso mantém o código legado disponível para referência/teste, mas retira essas rotas do fluxo principal de compilação/arquitetura.
