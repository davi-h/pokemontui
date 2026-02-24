# Auditoria de Redundâncias Arquiteturais

Este documento mapeia redundâncias observadas por **nomes de pastas/arquivos** e por **conteúdo interno** dos módulos em `crates/`.

## 1) Modelos de domínio duplicados (`domain` x `logic-core`)

### Evidência
- `crates/domain/src/pokemon/entity.rs` define `Pokemon` com `name`, `level`, `stats`.
- `crates/logic-core/src/models/mod.rs` define outro `Pokemon` (com forma diferente), além de `Move`, `Item`, `PokedexEntry`, `GameState`.

### Impacto
- Dois “centros de verdade” para entidades de jogo.
- Conversões e acoplamento acidental entre camadas.

### Centralização recomendada
- **Manter o modelo canônico em `crates/domain/`**.
- Em `logic-core`, mover/renomear para DTOs/representações externas ou remover se obsoleto.

---

## 2) Contratos de API sobrepostos (`contracts` x `logic-core`)

### Evidência
- `crates/contracts/src/api.rs` expõe `PokemonApi`.
- `crates/logic-core/src/api/mod.rs` expõe `ApiClient`, `ApiError` e implementação concreta `ReqwestClient`.
- `crates/infrastructure/src/api/pokeapi_client.rs` também implementa API concreta (`PokeApiClient`).

### Impacto
- Responsabilidade de integração HTTP aparece em dois lugares (`logic-core` e `infrastructure`).

### Centralização recomendada
- **Interfaces em `crates/contracts/`**.
- **Implementações concretas em `crates/infrastructure/`**.
- `logic-core` deve ser removido dessa fronteira (ou extinto, se redundante).

---

## 3) Camada de storage duplicada (`logic-core` x `infrastructure`)

### Evidência
- `crates/logic-core/src/storage/mod.rs` define trait `Storage` + `InMemoryStorage`.
- `crates/infrastructure/src/storage/save_repository.rs` implementa persistência em arquivo (`SaveRepository`).

### Impacto
- Duas abordagens de persistência sem fronteira de contrato única.

### Centralização recomendada
- Criar contrato explícito em `crates/contracts/` para persistência.
- Manter implementações (`arquivo`, `memória`, etc.) em `crates/infrastructure/`.

---

## 4) Regra de dano duplicada (`domain` x `engine`)

### Evidência
- `crates/domain/src/battle/damage.rs` tem `calculate_damage`.
- `crates/engine/src/stats/calculator.rs` também tem `calculate_damage` (com variação da regra mínima de dano).

### Impacto
- Regra de negócio de batalha existe em dois lugares com comportamento divergente.

### Centralização recomendada
- Se cálculo for regra pura de negócio: centralizar em **`crates/domain/`** e reutilizar no `engine`.
- Se for detalhe de execução/simulação: manter em `engine` e remover o duplicado no `domain`.

---

## 5) RNG fragmentado em múltiplos módulos

### Evidência
- `crates/contracts/src/rng.rs` define trait `Rng`.
- `crates/infrastructure/src/rng/seeded_rng.rs` implementa `Rng`.
- `crates/engine/src/rng/mod.rs` cria `GameRng` usando `thread_rng`.
- `crates/engine/src/spawn/rng.rs` cria outro RNG (`SeededRng`, xorshift).

### Impacto
- Estratégias RNG paralelas e não padronizadas.

### Centralização recomendada
- Preservar **abstração em `contracts`**.
- Concentrar implementações concretas em `infrastructure`.
- `engine` deve depender da trait (injeção) ao invés de manter múltiplos RNG próprios.

---

## 6) Módulo de spawn duplicado e arquivo órfão

### Evidência
- `crates/engine/src/spawn/engine.rs` define `SpawnEngine`/`SpawnResult` com regras/modificadores/contexto.
- `crates/engine/src/spawn/distribution.rs` define **outro** `SpawnEngine`/`SpawnResult` baseado em `SpawnTable`.
- `distribution.rs` não é exportado em `crates/engine/src/spawn/mod.rs`.

### Impacto
- Implementação paralela potencialmente desatualizada.
- Ruído arquitetural: mesmo nome para engines com comportamentos diferentes.

### Centralização recomendada
- Escolher um design de spawn principal em `crates/engine/src/spawn/`.
- Remover/renomear o módulo alternativo para evitar ambiguidade.

---

## 7) Caso de uso de spawn duplicado (`app` x `application`)

### Evidência
- `crates/application/src/commands/spawn_pokemon.rs` define `SpawnPokemon<S: SpawnService>`.
- `crates/app/src/container/usecases.rs` define outro `SpawnPokemon<F: PokemonFactory>`.

### Impacto
- Casos de uso duplicados em camadas diferentes.

### Centralização recomendada
- **Casos de uso devem ficar em `crates/application/`**.
- `crates/app/` deve apenas compor/inicializar (bootstrap + DI + runtime).

---

## 8) Carregamento de sprite com responsabilidades sobrepostas

### Evidência
- `crates/contracts/src/sprites.rs` define trait `SpriteLoader`.
- `crates/infrastructure/src/sprites/sprite_loader.rs` contém implementação concreta.
- `crates/logic-core/src/api/mod.rs` também possui `fetch_sprite_ascii` executando `pokeget`.

### Impacto
- Mesma integração externa espalhada por dois crates.

### Centralização recomendada
- Manter interface em `contracts`.
- Consolidar execução de `pokeget` em `infrastructure`.
- Remover caminho paralelo em `logic-core`.

---

## 9) Sinais de camada “fantasma” (crates com pouco ou nenhum papel efetivo)

### Evidência
- `crates/interface/src/lib.rs`, `crates/shared/src/lib.rs`, `crates/testing/src/lib.rs` e `crates/plugins/src/lib.rs` estão vazios/minimais.
- `cargo check` reporta vários componentes do `app` não usados (runtime, config, shutdown, etc.).

### Impacto
- Estrutura de pastas sugere arquitetura mais avançada do que a efetivamente utilizada.

### Centralização recomendada
- Reduzir superfície: remover crates vazios, ou mover código útil para eles de forma explícita.
- Limitar `app` a orquestração e mover lógica para `application`/`engine`.

---

## Prioridade sugerida de refatoração

1. **Eliminar duplicidades de regra de negócio**: dano + spawn + caso de uso de spawn.
2. **Unificar fronteiras**: contratos em `contracts`, implementações em `infrastructure`.
3. **Revisar `logic-core`**: absorver no `domain/application/infrastructure` ou descontinuar.
4. **Limpar estrutura**: remover módulos órfãos e crates vazios sem função concreta.
