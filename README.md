# iclass-tui

Interface de terminal (TUI) para o sistema [iClass](https://github.com), construída com Rust e ratatui.

---

## Funcionalidades

- Autenticação via API iClass com `accessToken` + `refreshToken`
- Sessão persistida em SQLite — login automático nas próximas execuções
- Navegação por teclado entre Dashboard, Usuários, Turmas, Tarefas e Relatórios
- Logout com limpeza da sessão local
- Senha mascarada no formulário de login
- **Usuários** — CRUD completo (somente admin): lista, adicionar, editar, remover
- **Turmas** — CRUD completo (somente admin): lista, adicionar (com seleção de professor e alunos), editar, remover
- **Tarefas** — CRUD completo (admin e teacher): lista por papel/turma, criar com score Fibonacci e turma, editar, remover; abertura de arquivo anexado (`o`)
- **Submissões** — student envia arquivo/texto (`s`); teacher/admin lista submissões por tarefa (`s`) e avalia com nota e feedback (`g`); abertura de arquivos no browser (`o`)
- **Relatórios** — teacher vê BarChart de pontos Fibonacci acumulados por aluno, uma turma por painel
- RBAC em todas as telas: atalhos e ações filtrados por papel (admin / teacher / student)

---

## Tecnologias

| Crate | Versão | Uso |
|---|---|---|
| [ratatui](https://ratatui.rs) | 0.29 | Widgets e layout do terminal |
| [crossterm](https://docs.rs/crossterm) | 0.29 | Input/output, raw mode |
| [tokio](https://tokio.rs) | 1 | Runtime assíncrono |
| [reqwest](https://docs.rs/reqwest) | 0.12 | Cliente HTTP (json + multipart) |
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
| `←` / `→` | Alternar foco entre sidebar e conteúdo |
| `↑` / `↓` | Navegar no menu lateral (foco sidebar) ou na lista (foco conteúdo) |
| `a` | Adicionar item (onde aplicável, respeitando RBAC) |
| `e` | Editar item selecionado (onde aplicável, respeitando RBAC) |
| `d` | Remover item selecionado (onde aplicável, respeitando RBAC) |
| `l` | Logout — apaga sessão e volta para o login |
| `q` | Fechar o aplicativo |

### Tarefas — submissões e arquivos

| Tecla | Quem | Ação |
|---|---|---|
| `s` | student | Abrir modal de envio de submissão (arquivo + texto) |
| `s` | teacher / admin | Listar submissões da tarefa selecionada |
| `o` | todos | Abrir arquivo da tarefa no browser do sistema |
| `g` | teacher / admin | Avaliar a submissão selecionada (nota + feedback) |
| `o` | teacher / admin | Abrir arquivo da submissão no browser (dentro da lista de submissões) |

### Formulários (modais Add/Edit)

| Tecla | Ação |
|---|---|
| `Tab` | Próximo campo |
| `Espaço` | Ciclar valor (perfil em usuários, score Fibonacci em tarefas, selecionar em pickers) |
| `↑` / `↓` | Navegar em pickers (professor, alunos, turma) |
| `Enter` | Salvar |
| `Esc` | Cancelar |

---

## Estrutura do projeto

```
src/
├── main.rs                    # Loop principal, init do banco, dispatch de eventos
├── config.rs                  # URL da API e caminho do banco
├── api/
│   ├── client.rs              # Cliente HTTP (Bearer token, JSON + multipart + post_form_with_file)
│   ├── auth.rs                # POST /auth/login
│   ├── users.rs               # CRUD /users (+ filtro ?role=)
│   ├── classes.rs             # CRUD /classes
│   ├── tasks.rs               # CRUD /tasks (multipart)
│   ├── submissions.rs         # POST /tasks/:id/submissions · GET · PATCH /submissions/:id
│   └── reports.rs             # GET /classes/:id/report
├── app/
│   ├── state.rs               # AppState, todos os forms/modals/pickers
│   ├── actions.rs             # Enum exaustivo de mutações
│   ├── reducer.rs             # Lógica de estado async
│   ├── routes.rs              # Enum Route
│   ├── focus.rs               # Enum Focus
│   └── resources.rs           # Resource<T> (Idle/Loading/Success/Error)
├── database/
│   ├── sqlite.rs              # Abre SqlitePool com create_if_missing
│   ├── migrations.rs          # CREATE TABLE (idempotente)
│   └── session_repository.rs  # save / load / delete session
├── models/
│   ├── auth.rs                # LoginRequest, LoginResponse, Session
│   ├── user.rs                # User, CreateUserRequest, UpdateUserRequest
│   ├── class.rs               # ClassRoom, CreateClassRequest, UpdateClassRequest
│   ├── task.rs                # Task, ClassRef
│   ├── submission.rs          # Submission, SubmissionStudent, GradeSubmissionRequest
│   └── report.rs              # ClassReport, ReportTask, ReportStudent
├── services/
│   ├── auth_service.rs        # Orquestra login → salva sessão
│   └── sync_service.rs        # Stub — sincronização futura
└── ui/
    ├── theme.rs               # Constantes de cor
    ├── layout.rs              # Split sidebar + conteúdo + footer
    ├── components/
    │   ├── header.rs          # Barra superior (app + email/role)
    │   ├── sidebar.rs         # Menu lateral navegável
    │   └── footer.rs          # Rodapé com atalhos contextuais
    └── screens/
        ├── login.rs           # Formulário de login centralizado
        ├── dashboard.rs       # Dashboard principal
        ├── users.rs           # Tela de usuários (CRUD, somente admin)
        ├── classes.rs         # Tela de turmas (CRUD admin; pickers de professor/alunos)
        ├── tasks.rs           # Tela de tarefas (CRUD admin/teacher; submissões; score Fibonacci)
        └── reports.rs         # Tela de relatórios (BarChart Fibonacci por aluno; somente teacher)
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
