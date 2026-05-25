# iclass-tui

Interface de terminal (TUI) para o sistema [iClass](https://github.com), construída com Rust e ratatui.

---

## Funcionalidades

- Autenticação via API iClass com `accessToken` + `refreshToken`
- Sessão persistida em SQLite — login automático nas próximas execuções
- Navegação por teclado entre Dashboard, Usuários, Turmas e Tarefas
- Logout com limpeza da sessão local
- Senha mascarada no formulário de login

---

## Tecnologias

| Crate | Versão | Uso |
|---|---|---|
| [ratatui](https://ratatui.rs) | 0.29 | Widgets e layout do terminal |
| [crossterm](https://docs.rs/crossterm) | 0.29 | Input/output, raw mode |
| [tokio](https://tokio.rs) | 1 | Runtime assíncrono |
| [reqwest](https://docs.rs/reqwest) | 0.12 | Cliente HTTP |
| [sqlx](https://docs.rs/sqlx) | 0.8 | SQLite (cache local) |
| [serde](https://serde.rs) | 1 | Serialização JSON |
| [anyhow](https://docs.rs/anyhow) | 1 | Propagação de erros |

---

## Pré-requisitos

- [Rust](https://rustup.rs) 1.80 ou superior
- API iClass rodando (padrão: `http://localhost:3000`)

---

## Instalação e execução

```bash
# 1. Clonar o repositório
git clone <url-do-repositorio>
cd iclass-tui

# 2. Compilar e executar
cargo run
```

Na primeira execução, o banco `cache.db` é criado automaticamente no diretório atual.  
Se já existir uma sessão salva, o login é pulado automaticamente.

---

## Configuração

Edite `src/config.rs` para ajustar os valores padrão:

| Campo | Padrão | Descrição |
|---|---|---|
| `api_url` | `http://localhost:3000` | URL base da API iClass |
| `database_url` | `sqlite://cache.db` | Caminho do banco SQLite local |

---

## Atalhos de teclado

### Tela de login

| Tecla | Ação |
|---|---|
| `Tab` | Alternar entre Email e Senha |
| `Enter` | Entrar |
| `Backspace` | Apagar caractere |
| `Ctrl+C` | Fechar o aplicativo |

### Demais telas

| Tecla | Ação |
|---|---|
| `↑` / `↓` | Navegar no menu lateral |
| `l` | Logout — apaga sessão e volta para o login |
| `q` | Fechar o aplicativo |

---

## Estrutura do projeto

```
src/
├── main.rs                    # Loop principal, init do banco, dispatch de eventos
├── config.rs                  # URL da API e caminho do banco
├── api/
│   ├── client.rs              # Cliente HTTP com Bearer token
│   └── auth.rs                # POST /auth/login
├── app/
│   ├── state.rs               # AppState + LoginForm
│   ├── actions.rs             # Enum de todas as mutações
│   ├── reducer.rs             # Lógica de estado (async)
│   ├── routes.rs              # Enum Route
│   ├── focus.rs               # Enum Focus
│   └── resources.rs           # Resource<T> (Idle/Loading/Success/Error)
├── database/
│   ├── sqlite.rs              # Abre SqlitePool com create_if_missing
│   ├── migrations.rs          # CREATE TABLE (idempotente)
│   └── session_repository.rs  # save / load / delete session
├── models/
│   ├── auth.rs                # LoginRequest, LoginResponse, Session
│   ├── user.rs                # User
│   ├── class.rs               # ClassRoom
│   └── task.rs                # Task
├── services/
│   ├── auth_service.rs        # Orquestra login → salva sessão
│   └── sync_service.rs        # Stub — sincronização futura
└── ui/
    ├── theme.rs               # Constantes de cor
    ├── layout.rs              # Split sidebar + conteúdo + footer
    ├── components/
    │   ├── sidebar.rs         # Menu lateral navegável
    │   └── footer.rs          # Rodapé com atalhos
    └── screens/
        ├── login.rs           # Formulário de login centralizado
        ├── dashboard.rs       # Dashboard principal
        ├── users.rs           # Tela de usuários
        ├── classes.rs         # Tela de turmas
        └── tasks.rs           # Tela de tarefas
```

Consulte [AGENTS.md](AGENTS.md) para documentação completa de arquitetura.

---

## Desenvolvimento

```bash
# Verificar tipos e erros sem compilar o binário
cargo check

# Build de desenvolvimento
cargo build

# Build otimizado para produção
cargo build --release
```

---

## Licença

MIT
