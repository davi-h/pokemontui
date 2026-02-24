#DISCLAIMER - AI USED IN THIS PROJECT 

🧬 PokémonTUI — Plataforma de Simulação de Criaturas via Terminal

PokémonTUI é uma plataforma de simulação de criaturas executada inteiramente no terminal.

Ele combina:

- engine de jogo
- eventos dinâmicos
- spawn inteligente
- batalhas táticas
- arquitetura modular e extensível

Não é apenas um jogo CLI: é um ecossistema de simulação plugável que pode ser evoluído como jogo, laboratório de balanceamento ou backend.

## 🌐 Integração com dados reais

A infraestrutura integra duas fontes externas:

- **Sprites no terminal** via comando [`pokeget`](https://github.com/talwat/pokeget)
- **Dados oficiais** via [PokéAPI](https://pokeapi.co)

Para reduzir custo de rede:

- conexão é necessária apenas na **primeira execução** de cada Pokémon
- dados são armazenados em `assets/cache`
- depois disso, os comandos funcionam com cache local

## ✨ Recursos principais

### 🎲 Spawn inteligente

- raridade probabilística
- base para influência de clima/eventos
- RNG determinístico com seed configurável

### ⚔️ Batalha tática

- turn manager
- resolução determinística
- simulações reproduzíveis

### 📚 Pokédex visual

- dados carregados sob demanda
- cache local
- sprites renderizados direto no terminal

### 🧠 Eventos globais

- tempestades podem favorecer elétricos
- eclipses podem aumentar raridades
- eventos sazonais podem liberar encontros especiais

## 🔌 CLI

Comandos disponíveis no binário:

```bash
app spawn
app battle
app pokedex [nome]
app inspect [nome]
app events
app save
```

> Observação: a saída de ajuda usa o prefixo conceitual `poke ...`, mas no estado atual do projeto o executável compilado é `app`.

## 🧩 Arquitetura

A base segue separação em camadas:

- **Interface → Application → Engine → Domain**
- **Infrastructure** conectada por contratos

Veja também [`ARCHITECTURE.md`](./ARCHITECTURE.md).

## 🖥️ Compatibilidade

Projeto focado em:

- **Linux** (principal)
- **Windows**

Como é um projeto Rust multi-crate com dependências cross-platform (`crossterm`, `reqwest` etc.), a base é portável entre os dois sistemas, respeitando disponibilidade de ferramentas externas (como `pokeget`) no ambiente.
