# Pyohwa (표화) — Product Requirements Document

> **Rust + Elm 기반 정적 사이트 생성기**
>
> 버전: 1.0.0 | 최종 수정: 2026-02-08

---

## 목차

1. [제품 개요](#1-제품-개요)
2. [핵심 원칙](#2-핵심-원칙)
3. [기능 명세](#3-기능-명세)
4. [아키텍처 설계](#4-아키텍처-설계)
5. [프로젝트 구조](#5-프로젝트-구조)
6. [빌드 파이프라인](#6-빌드-파이프라인)
7. [설정 스키마](#7-설정-스키마)
8. [프론트매터 스키마](#8-프론트매터-스키마)
9. [코드 패턴 가이드](#9-코드-패턴-가이드)
10. [개발 Phase](#10-개발-phase)
11. [비기능 요구사항](#11-비기능-요구사항)
12. [검증 방법](#12-검증-방법)

---

## 1. 제품 개요

### 1.1 제품명

**Pyohwa (표화)** — 표현(表現)과 변화(化)의 합성어.

### 1.2 정의

VitePress를 참조한 Rust(빌드 엔진) + Elm(인터랙티브 UI) 기반의 정적 사이트 생성기(SSG).

### 1.3 핵심 가치

| 가치 | 설명 |
|------|------|
| **Zero Configuration** | `cargo install pyohwa` 후 `pyohwa dev`만 실행하면 동작. Elm 등 별도 도구 설치 불필요. md 파일만 있으면 됨. |
| **함수형 아키텍처** | Rust의 ADT + Result/Option, Elm의 TEA(The Elm Architecture)로 예측 가능한 코드 |
| **타입 안전성** | Rust + Elm 양쪽 모두 강력한 정적 타입 시스템 |
| **빠른 빌드** | 500페이지 풀 빌드 3초 이내, 증분 빌드 200ms 이내 |

### 1.4 타겟 사용자

- 기술 문서를 빠르게 배포하고 싶은 개발자
- Markdown 기반으로 팀 문서를 관리하는 조직
- VitePress를 사용하지만 빌드 속도와 타입 안전성을 원하는 팀

---

## 2. 핵심 원칙

### 2.1 Tidy First (Kent Beck)

구조 변경과 행동 변경을 분리하고, 점진적으로 개선한다.

- **Guard Clause**: 모든 fallible 함수 상단에서 조기 반환
- **Normalize Symmetries**: 동일 패턴의 함수는 동일 시그니처를 가짐
- **커밋 분리**: 구조 변경 커밋 → 행동 변경 커밋 순서
- **모듈 경계**: 모듈 간 통신은 공개 타입으로만 (내부 구현 접근 금지)

### 2.2 함수형 프로그래밍

**Rust 측**:
- 불변 기본 (`let` 기본, `mut`는 명시적 필요 시에만)
- `Result<T, E>` / `Option<T>`으로 에러 처리 (`unwrap` 금지)
- Iterator 체이닝으로 데이터 변환
- ADT(Algebraic Data Types)로 도메인 모델링
- `thiserror`로 정밀한 에러 타입 정의 (`String` 에러 금지)

**Elm 측**:
- TEA (Model → Update → View)
- `Cmd` / `Sub`으로 부수효과 격리
- Port로 JS interop (최소화)
- 모든 상태는 단일 `Model`에 집중

---

## 3. 기능 명세

### 3.1 P0 — MVP (Must Have)

| ID | 기능 | 설명 | 핵심 crate/라이브러리 |
|----|------|------|-----------------------|
| F-001 | Markdown 파싱 | CommonMark + GFM 확장 (테이블, 각주, strikethrough, 태스크 리스트) | `comrak` |
| F-002 | 프론트매터 파싱 | YAML 프론트매터 추출 및 파싱 | `gray_matter` |
| F-003 | 구문 강조 | 50+ 언어 지원, 커스텀 테마 | `syntect` |
| F-004 | 파일 기반 라우팅 | `content/*.md` → `dist/*.html` 매핑 | `pyohwa-core` |
| F-005 | HTML 생성 | 레이아웃 포함 완전한 HTML5 페이지 출력 | `pyohwa-core` |
| F-006 | CLI | `pyohwa init`, `pyohwa build` 명령어 | `clap` |
| F-007 | 기본 테마 | 사이드바 + 상단 네비게이션 포함 반응형 레이아웃 | Elm + Tailwind CSS v4 |
| F-008 | 설정 파일 | `pyohwa.toml` (옵셔널, 없어도 기본값으로 동작) | `toml`, `serde` |
| F-009 | 정적 에셋 | `static/` → `dist/` 복사 | `pyohwa-core` |

### 3.2 P1 — Should Have

| ID | 기능 | 설명 |
|----|------|------|
| F-101 | Dev Server | axum 기반 로컬 서버 + WebSocket live reload |
| F-102 | 사이드바 자동 생성 | 디렉토리 구조에서 자동으로 사이드바 트리 구성 |
| F-103 | 네비게이션 생성 | `pyohwa.toml` 기반 상단 네비게이션 바 |
| F-104 | 커스텀 테마 | 사용자 정의 Elm + Tailwind CSS 테마 지원 |
| F-105 | Table of Contents | 자동 TOC 생성 + Scroll Spy |
| F-106 | 증분 빌드 | content hash 기반 변경 감지, 변경된 파일만 재빌드 |
| F-107 | Elm 인터랙티브 위젯 | 접을 수 있는 섹션, 탭, 코드 그룹 등 |

### 3.3 P2 — Nice to Have

| ID | 기능 | 설명 |
|----|------|------|
| F-201 | 전문 검색 | 클라이언트 사이드 검색 (Ctrl+K) |
| F-202 | SEO | sitemap.xml, OG tags, meta tags 자동 생성 |
| F-203 | RSS/Atom 피드 | 블로그 포스트용 피드 생성 |
| F-204 | i18n 지원 | 다국어 문서 라우팅 및 언어 전환 |
| F-205 | 플러그인 시스템 | Markdown 변환 파이프라인 확장점 |
| F-206 | 이미지 최적화 | WebP 변환, lazy loading, srcset 생성 |

---

## 4. 아키텍처 설계

### 4.1 고수준 아키텍처

```
┌─────────────────────────────────────────────────────────────────┐
│                        pyohwa-cli                               │
│                     (사용자 진입점)                                │
│                  pyohwa init | build | dev                       │
└──────────┬──────────────────┬───────────────────┬───────────────┘
           │                  │                   │
           ▼                  ▼                   ▼
┌──────────────────┐ ┌────────────────┐ ┌─────────────────────┐
│   pyohwa-core    │ │ pyohwa-server  │ │   pyohwa-search     │
│                  │ │                │ │                     │
│ - Config         │ │ - axum HTTP    │ │ - 인덱스 생성        │
│ - Content        │ │ - WebSocket    │ │ - JSON 출력         │
│ - Markdown       │ │ - File Watcher │ │                     │
│ - Site Graph     │ │ - Live Reload  │ │        (P2)         │
│ - Render         │ │                │ │                     │
│ - Build Pipeline │ │     (P1)       │ │                     │
│ - Embedded Assets│ │                │ │                     │
└──────────────────┘ └────────────────┘ └─────────────────────┘
           │
           │ include_str!
           ▼
┌──────────────────┐
│  Embedded Assets │
│                  │
│ - elm.min.js     │
│ - theme.css      │
│   (Tailwind v4)  │
└──────────────────┘
```

### 4.2 프론트엔드 에셋 바이너리 임베딩 전략

Pyohwa의 핵심 설계 결정은 **Elm JS와 Tailwind CSS를 Rust 바이너리에 임베딩**하는 것이다.

**동작 방식**:

1. **Pyohwa 개발 시점**: `elm/src/`를 컴파일 → `elm.min.js`, Tailwind CSS v4를 빌드 → `theme.css`
2. **`build.rs`**: Cargo 빌드 시 Elm 컴파일 + Tailwind CSS 빌드 자동 실행 (Pyohwa 개발자용)
3. **`embedded.rs`**: `include_str!`로 Elm JS, Tailwind CSS 출력물을 바이너리에 포함
4. **사용자 측**: `cargo install pyohwa`만으로 Elm 런타임 + CSS 포함. 별도 설치 없음.

**Tailwind CSS v4 빌드 전략**:

Tailwind CSS v4는 CSS-first 설정을 사용한다. JS 설정 파일(`tailwind.config.js`) 없이 CSS 내 `@theme` 디렉티브로 디자인 토큰을 정의한다. Pyohwa는 **Tailwind CSS v4 standalone CLI**를 사용하여 개발 시점에 CSS를 빌드한다.

```
themes/default/theme.css (Tailwind 소스)
        │
        ▼  tailwindcss CLI (build.rs에서 실행)
        │  - elm/src/**/*.elm 에서 클래스 스캔
        │  - themes/default/theme.css 처리
        ▼
themes/default/dist/theme.css (빌드된 CSS)
        │
        ▼  include_str!
        │
  Rust 바이너리에 임베딩
```

**`build.rs` 로직 (의사코드)**:

```rust
// crates/pyohwa-core/build.rs
fn main() {
    // Elm 소스가 변경되었을 때만 재컴파일
    println!("cargo:rerun-if-changed=../../elm/src");
    println!("cargo:rerun-if-changed=../../themes");

    // Elm 컴파일러가 설치되어 있을 때만 실행 (CI/개발 환경)
    // 없으면 기존 dist 파일 사용
    if has_elm_compiler() {
        compile_elm("../../elm/src/Main.elm", "../../elm/dist/elm.min.js");
        minify("../../elm/dist/elm.min.js");
    }

    // Tailwind CSS v4 standalone CLI
    if has_tailwind_cli() {
        build_tailwind(
            "--input ../../themes/default/theme.css",
            "--output ../../themes/default/dist/theme.css",
            "--content '../../elm/src/**/*.elm'",
            "--minify",
        );
    }
}
```

**`embedded.rs`**:

```rust
// crates/pyohwa-core/src/embedded.rs

/// Elm 앱 (minified)
pub const ELM_JS: &str = include_str!("../../elm/dist/elm.min.js");

/// Tailwind CSS v4 빌드 결과 (minified)
pub const THEME_CSS: &str = include_str!("../../themes/default/dist/theme.css");
```

**장점**:
- 사용자는 Elm, Tailwind 모두 설치할 필요 없음 → Zero Configuration 유지
- 단일 바이너리 배포 → 의존성 관리 단순화
- Tailwind v4의 CSS-first 설정으로 JS 도구 체인 의존 제거
- 커스텀 테마 개발 시에만 Elm + Tailwind 설치 필요 (선택적, P1)

### 4.3 Tailwind CSS v4 테마 시스템

**CSS-first 설정**: Tailwind v4는 JS 설정 파일이 아닌 CSS 내 `@theme` 디렉티브로 디자인 토큰을 정의한다.

**`themes/default/theme.css` (Tailwind 소스)**:

```css
@import "tailwindcss";

/*
 * Pyohwa 디자인 토큰 정의
 * Tailwind v4의 @theme 디렉티브로 CSS 변수 생성
 */
@theme {
  /* 컬러 팔레트 */
  --color-primary-50: #eff6ff;
  --color-primary-100: #dbeafe;
  --color-primary-500: #3b82f6;
  --color-primary-600: #2563eb;
  --color-primary-700: #1d4ed8;

  /* 사이드바 */
  --width-sidebar: 272px;
  --width-toc: 224px;

  /* 콘텐츠 영역 */
  --width-content-max: 768px;

  /* 폰트 */
  --font-family-sans: "Inter", ui-sans-serif, system-ui, sans-serif;
  --font-family-mono: "JetBrains Mono", ui-monospace, monospace;
}

/* Markdown 콘텐츠 타이포그래피 */
@import "./typography.css";
```

**`themes/default/typography.css`**: Markdown에서 렌더링된 HTML 콘텐츠에 타이포그래피 스타일을 적용한다. Tailwind의 `@apply`를 활용하되, prose 콘텐츠에 특화된 스타일을 정의한다.

```css
/* Markdown 렌더링 결과에 적용되는 타이포그래피 */
.pyohwa-prose {
  @apply text-base leading-7 text-gray-700 dark:text-gray-300;

  h1 { @apply text-3xl font-bold tracking-tight text-gray-900 dark:text-white mt-8 mb-4; }
  h2 { @apply text-2xl font-semibold tracking-tight text-gray-900 dark:text-white mt-8 mb-3 border-b border-gray-200 dark:border-gray-800 pb-2; }
  h3 { @apply text-xl font-semibold text-gray-900 dark:text-white mt-6 mb-2; }

  p  { @apply my-4; }
  a  { @apply text-primary-600 dark:text-primary-400 hover:underline; }

  code { @apply text-sm bg-gray-100 dark:bg-gray-800 px-1.5 py-0.5 rounded font-mono; }
  pre  { @apply my-4 rounded-lg overflow-x-auto; }
  pre code { @apply bg-transparent p-0; }

  table { @apply w-full border-collapse my-4; }
  th { @apply border border-gray-300 dark:border-gray-700 px-4 py-2 bg-gray-50 dark:bg-gray-800 font-semibold text-left; }
  td { @apply border border-gray-300 dark:border-gray-700 px-4 py-2; }

  blockquote { @apply border-l-4 border-primary-500 pl-4 my-4 text-gray-600 dark:text-gray-400; }
  ul { @apply list-disc pl-6 my-4; }
  ol { @apply list-decimal pl-6 my-4; }
  li { @apply my-1; }
}
```

**Tailwind v4에서의 dark mode**: Tailwind v4는 `@media (prefers-color-scheme: dark)` 또는 클래스 기반 dark mode를 지원한다. Pyohwa는 `dark:` variant를 사용하여 시스템 설정 기반 다크 모드를 자동 적용한다.

**콘텐츠 스캔 범위**: `build.rs`에서 Tailwind CLI 실행 시 `--content` 옵션으로 Elm 소스 파일을 스캔 대상으로 지정한다. Elm 코드에 작성된 Tailwind 클래스만 최종 CSS에 포함되어 번들 크기를 최소화한다.

### 4.4 데이터 흐름: Rust → Elm

Rust가 생성한 HTML에 Elm 앱이 마운트되며, 페이지 데이터는 `window.__PYOHWA_DATA__`를 통해 전달된다.

```
Rust (Build Time)                          Elm (Runtime)
─────────────────                          ────────────

content/*.md
    │
    ▼
[파싱 + 변환]
    │
    ▼
HTML 템플릿 생성
    │
    ├─→ <script>
    │   window.__PYOHWA_DATA__ = {
    │     page: { title, toc, content },
    │     site: { nav, sidebar },
    │     theme: { ... }
    │   }
    │   </script>
    │
    ├─→ <script src="elm.min.js"/>    ───→ Elm.Main.init({
    │                                        flags: window.__PYOHWA_DATA__
    │                                      })
    └─→ <div id="app">
        <div id="content">
          [서버 렌더링된 HTML]       ───→    Elm이 인터랙션 담당
        </div>                              (사이드바, TOC, 검색 등)
        </div>
```

**`window.__PYOHWA_DATA__` 스키마**:

```typescript
interface PyohwaData {
  page: {
    title: string;
    description: string;
    content: string;        // 렌더링된 HTML 문자열
    toc: TocItem[];         // { id, text, level }[]
    frontmatter: Record<string, unknown>;
    editLink?: string;
    lastUpdated?: string;
  };
  site: {
    title: string;
    description: string;
    base: string;
    nav: NavItem[];         // { text, link, active }[]
    sidebar: SidebarGroup[]; // { text, items: { text, link, active }[] }[]
  };
  theme: {
    highlightTheme: string;
  };
}
```

### 4.5 순수/불순 경계

명확한 경계를 두어 테스트 가능성과 예측 가능성을 확보한다.

| 구분 | 함수/모듈 | 설명 |
|------|-----------|------|
| **순수** | `markdown::parse` | Markdown 텍스트 → HTML 문자열 |
| **순수** | `content::parse_frontmatter` | 원시 문자열 → `Frontmatter` 구조체 |
| **순수** | `site::build_graph` | `Vec<Page>` → `SiteGraph` |
| **순수** | `render::render_page` | `Page` + `SiteGraph` + 템플릿 → HTML 문자열 |
| **순수** | `site::resolve_routes` | `Vec<Page>` → `Vec<Route>` |
| **불순 (IO)** | `config::load` | 파일 시스템에서 `pyohwa.toml` 읽기 |
| **불순 (IO)** | `content::discover` | 디렉토리 탐색으로 md 파일 목록 수집 |
| **불순 (IO)** | `build::write_output` | 렌더링 결과를 `dist/`에 파일 쓰기 |
| **불순 (IO)** | `server::serve` | HTTP 서버 + WebSocket 실행 |

---

## 5. 프로젝트 구조

### 5.1 Pyohwa 소스 프로젝트 구조

```
pyohwa/                              # Pyohwa 도구 자체의 소스코드
├── Cargo.toml                       # [workspace] 정의
├── crates/
│   ├── pyohwa-cli/                  # CLI 진입점
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs              # clap 기반 명령어 라우팅
│   │
│   ├── pyohwa-core/                 # 핵심 빌드 로직
│   │   ├── Cargo.toml
│   │   ├── build.rs                 # Elm 컴파일 자동화 (개발자용)
│   │   └── src/
│   │       ├── lib.rs               # 공개 API re-export
│   │       ├── config.rs            # pyohwa.toml 로딩 + 기본값
│   │       ├── embedded.rs          # include_str! 매크로 (Elm JS, CSS)
│   │       ├── content/
│   │       │   ├── mod.rs
│   │       │   ├── loader.rs        # 파일 시스템 탐색 → Vec<RawContent>
│   │       │   ├── frontmatter.rs   # YAML 파싱 → Frontmatter
│   │       │   └── page.rs          # Page 타입 정의
│   │       ├── markdown/
│   │       │   ├── mod.rs
│   │       │   ├── parser.rs        # comrak 기반 Markdown → HTML
│   │       │   └── highlight.rs     # syntect 기반 구문 강조
│   │       ├── site/
│   │       │   ├── mod.rs
│   │       │   ├── graph.rs         # SiteGraph: 페이지 간 관계
│   │       │   ├── route.rs         # 파일 경로 → URL 매핑
│   │       │   └── taxonomy.rs      # 태그, 카테고리 (P2)
│   │       ├── render/
│   │       │   ├── mod.rs
│   │       │   ├── template.rs      # HTML 템플릿 렌더링
│   │       │   ├── layout.rs        # 레이아웃 (doc, home, page)
│   │       │   └── assets.rs        # 정적 에셋 복사
│   │       └── build/
│   │           ├── mod.rs
│   │           ├── pipeline.rs      # 전체 빌드 파이프라인 오케스트레이션
│   │           ├── incremental.rs   # 증분 빌드 (content hash, P1)
│   │           └── output.rs        # dist/ 출력 관리
│   │
│   ├── pyohwa-server/               # Dev server (P1)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── server.rs            # axum 정적 파일 + API
│   │       ├── watcher.rs           # notify 파일 워처
│   │       └── reload.rs            # WebSocket live reload
│   │
│   └── pyohwa-search/               # 검색 (P2)
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── indexer.rs           # 검색 인덱스 JSON 생성
│           └── tokenizer.rs        # 텍스트 토크나이징
│
├── elm/                             # Elm 프론트엔드 소스 (TEA)
│   ├── elm.json
│   ├── dist/
│   │   └── elm.min.js              # 컴파일된 Elm (Git 추적)
│   └── src/
│       ├── Main.elm                 # 진입점 (init, update, view, subscriptions)
│       ├── Model.elm                # 전체 앱 상태 타입
│       ├── Msg.elm                  # 메시지 타입
│       ├── Update.elm               # 상태 업데이트 로직
│       ├── View.elm                 # 최상위 뷰
│       ├── Flags.elm                # JS → Elm 초기 데이터 디코더
│       ├── Route.elm                # 클라이언트 사이드 라우팅
│       ├── Theme/
│       │   ├── Sidebar.elm          # 사이드바 컴포넌트
│       │   ├── Navbar.elm           # 상단 네비게이션 바
│       │   ├── Toc.elm              # Table of Contents + Scroll Spy
│       │   ├── Footer.elm           # 하단 영역
│       │   └── Layout.elm           # 전체 레이아웃 조합
│       ├── Search/                  # 검색 UI (P2)
│       │   ├── Search.elm
│       │   └── Modal.elm
│       └── Ports.elm                # JS interop 포트 정의
│
├── themes/
│   └── default/
│       ├── theme.css                # Tailwind v4 소스 (@import "tailwindcss" + @theme)
│       ├── typography.css           # Markdown 콘텐츠 타이포그래피 (@plugin 또는 커스텀)
│       └── dist/
│           └── theme.css            # 빌드된 CSS (Git 추적, include_str! 대상)
│
├── scaffold/                        # pyohwa init 템플릿
│   ├── pyohwa.toml
│   ├── content/
│   │   └── index.md
│   └── static/
│       └── .gitkeep
│
└── tests/                           # 통합 테스트
    ├── integration/
    │   ├── build_test.rs
    │   ├── config_test.rs
    │   └── cli_test.rs
    └── fixtures/
        ├── minimal/                 # 최소 프로젝트
        ├── full/                    # 전체 기능 프로젝트
        └── large/                   # 500페이지 성능 테스트
```

### 5.2 사용자 프로젝트 구조 (`pyohwa init` 결과)

```
my-docs/                             # 사용자의 문서 프로젝트
├── pyohwa.toml                      # 선택적 설정 (없어도 동작)
├── content/
│   ├── index.md                     # 홈 페이지
│   └── guide/
│       └── getting-started.md       # 가이드 문서
├── static/                          # 정적 에셋 (이미지, 폰트 등)
└── dist/                            # 빌드 출력 (자동 생성, .gitignore 권장)
    ├── index.html
    ├── guide/
    │   └── getting-started.html
    └── assets/
        ├── elm.min.js               # 임베딩에서 추출
        └── theme.css                # 임베딩에서 추출
```

---

## 6. 빌드 파이프라인

### 6.1 파이프라인 개요 (8단계)

사용자는 Elm 컴파일이 필요 없다. 모든 프론트엔드 에셋은 Rust 바이너리에 이미 포함되어 있다.

```
Stage 1        Stage 2         Stage 3           Stage 4
Config 로드 → 콘텐츠 탐색 → 프론트매터 파싱 → Markdown→HTML
(IO)           (IO)            (순수)             (순수)

    Stage 5          Stage 6           Stage 7              Stage 8
→ 구문 강조 → 사이트 그래프 구성 → HTML 템플릿 렌더 → 출력
  (순수)          (순수)        (순수, Elm JS 주입)    (IO)
```

### 6.2 각 Stage 상세

#### Stage 1: Config 로드

- **입력**: `pyohwa.toml` 파일 경로 (없으면 기본값)
- **출력**: `Config` 구조체
- **crate**: `pyohwa-core::config`
- **라이브러리**: `toml`, `serde`
- **동작**: 파일 존재 시 파싱, 부재 시 기본 `Config` 반환

```rust
pub struct Config {
    pub site: SiteConfig,
    pub build: BuildConfig,
    pub theme: ThemeConfig,
    pub nav: Vec<NavItem>,
    pub sidebar: SidebarConfig,
}

impl Default for Config { /* 모든 필드에 합리적 기본값 */ }
```

#### Stage 2: 콘텐츠 탐색

- **입력**: `Config.build.content_dir` 경로
- **출력**: `Vec<RawContent>` (파일 경로 + 원시 문자열)
- **crate**: `pyohwa-core::content::loader`
- **라이브러리**: `walkdir`
- **동작**: `content_dir` 재귀 탐색, `.md` 파일만 수집, `draft: true` 필터링

```rust
pub struct RawContent {
    pub path: PathBuf,       // content/guide/getting-started.md
    pub raw: String,         // 파일 전체 내용
}
```

#### Stage 3: 프론트매터 파싱

- **입력**: `Vec<RawContent>`
- **출력**: `Vec<ParsedContent>` (프론트매터 + 본문 분리)
- **crate**: `pyohwa-core::content::frontmatter`
- **라이브러리**: `gray_matter`
- **순수 함수**: 입력 문자열만으로 결과 결정

```rust
pub struct Frontmatter {
    pub title: String,
    pub description: Option<String>,
    pub layout: Layout,          // Doc | Home | Page | Custom(String)
    pub order: Option<i32>,
    pub tags: Vec<String>,
    pub date: Option<NaiveDate>,
    pub draft: bool,
    pub prev: Option<String>,
    pub next: Option<String>,
}

pub struct ParsedContent {
    pub path: PathBuf,
    pub frontmatter: Frontmatter,
    pub body: String,            // 프론트매터 제외한 Markdown 본문
}
```

#### Stage 4: Markdown → HTML

- **입력**: `Vec<ParsedContent>`
- **출력**: `Vec<RenderedContent>` (HTML + TOC)
- **crate**: `pyohwa-core::markdown::parser`
- **라이브러리**: `comrak`
- **순수 함수**: CommonMark + GFM 확장 적용

```rust
pub struct TocItem {
    pub id: String,          // heading의 slug화된 ID
    pub text: String,        // heading 텍스트
    pub level: u8,           // 1-6
}

pub struct RenderedContent {
    pub path: PathBuf,
    pub frontmatter: Frontmatter,
    pub html: String,            // 렌더링된 HTML
    pub toc: Vec<TocItem>,       // 추출된 TOC
}
```

#### Stage 5: 구문 강조

- **입력**: `Vec<RenderedContent>` (HTML 내 `<code>` 블록)
- **출력**: `Vec<RenderedContent>` (구문 강조 적용된 HTML)
- **crate**: `pyohwa-core::markdown::highlight`
- **라이브러리**: `syntect`
- **순수 함수**: HTML 내 `<pre><code class="language-*">` 블록을 찾아 하이라이팅 적용

#### Stage 6: 사이트 그래프 구성

- **입력**: `Vec<RenderedContent>` + `Config`
- **출력**: `SiteGraph`
- **crate**: `pyohwa-core::site::graph`
- **순수 함수**: 페이지 관계, 사이드바 트리, 네비게이션, prev/next 링크 계산

```rust
pub struct SiteGraph {
    pub pages: Vec<Page>,
    pub sidebar: Vec<SidebarGroup>,
    pub nav: Vec<NavItem>,
}

pub struct Page {
    pub route: Route,            // /guide/getting-started
    pub frontmatter: Frontmatter,
    pub html: String,
    pub toc: Vec<TocItem>,
    pub prev: Option<Route>,
    pub next: Option<Route>,
}

pub struct SidebarGroup {
    pub text: String,
    pub items: Vec<SidebarItem>,
}

pub struct SidebarItem {
    pub text: String,
    pub link: String,
}
```

#### Stage 7: HTML 템플릿 렌더

- **입력**: `Page` + `SiteGraph` + 임베딩 에셋
- **출력**: 완전한 HTML5 문자열
- **crate**: `pyohwa-core::render::template`
- **순수 함수**: 페이지 데이터 + Elm JS + CSS를 조합하여 최종 HTML 생성

생성되는 HTML 구조:

```html
<!DOCTYPE html>
<html lang="{config.site.language}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{page.title} | {config.site.title}</title>
    <meta name="description" content="{page.description}">
    <link rel="stylesheet" href="/assets/theme.css"> <!-- Tailwind v4 빌드 결과 -->
</head>
<body class="bg-white text-gray-900 dark:bg-gray-950 dark:text-gray-100">
    <div id="app">
        <div id="content" class="prose dark:prose-invert max-w-none">{page.html}</div>
    </div>

    <script>
    window.__PYOHWA_DATA__ = {
        page: {
            title: "{page.title}",
            description: "{page.description}",
            content: "{page.html (escaped)}",
            toc: [{toc_items}],
            frontmatter: {frontmatter_json}
        },
        site: {
            title: "{config.site.title}",
            description: "{config.site.description}",
            base: "{config.site.base_url}",
            nav: [{nav_items}],
            sidebar: [{sidebar_groups}]
        },
        theme: {
            highlightTheme: "{config.theme.highlight_theme}"
        }
    };
    </script>
    <script src="/assets/elm.min.js"></script>
    <script>
    var app = Elm.Main.init({
        node: document.getElementById('app'),
        flags: window.__PYOHWA_DATA__
    });
    </script>
</body>
</html>
```

#### Stage 8: 출력

- **입력**: `Vec<(Route, String)>` (라우트 + HTML) + 임베딩 에셋
- **출력**: `dist/` 디렉토리에 파일 쓰기
- **crate**: `pyohwa-core::build::output`
- **동작**:
  1. `dist/` 디렉토리 초기화 (이전 빌드 정리)
  2. HTML 파일 쓰기 (라우트 구조 유지)
  3. `embedded::ELM_JS` → `dist/assets/elm.min.js`
  4. `embedded::THEME_CSS` → `dist/assets/theme.css` (Tailwind v4 빌드 결과)
  5. `static/` → `dist/` 복사

---

## 7. 설정 스키마

### 7.1 `pyohwa.toml` 전체 스키마

모든 필드는 옵셔널이다. 파일 자체가 없어도 기본값으로 동작한다.

```toml
# === 사이트 기본 정보 ===
[site]
title = "My Documentation"          # 기본값: "Documentation"
description = "Project description"  # 기본값: ""
base_url = "/"                       # 기본값: "/"
language = "en"                      # 기본값: "en"

# === 빌드 설정 ===
[build]
content_dir = "content"              # 기본값: "content"
output_dir = "dist"                  # 기본값: "dist"
static_dir = "static"               # 기본값: "static"

# === 테마 설정 ===
[theme]
name = "default"                     # 기본값: "default"
highlight_theme = "one-dark"         # 기본값: "one-dark"
custom_css = "custom.css"            # 기본값: None (추가 CSS 파일 경로, Tailwind 클래스 사용 가능)

# === 상단 네비게이션 ===
[[nav]]
text = "Guide"
link = "/guide/getting-started"

[[nav]]
text = "API"
link = "/api/"

# === 사이드바 ===
[sidebar]
auto = true                          # true: 디렉토리에서 자동 생성
                                     # false: 아래 manual items 사용
                                     # 기본값: true

# 수동 사이드바 (auto = false일 때)
[[sidebar.groups]]
text = "Introduction"

  [[sidebar.groups.items]]
  text = "Getting Started"
  link = "/guide/getting-started"

  [[sidebar.groups.items]]
  text = "Configuration"
  link = "/guide/configuration"

# === 검색 (P2) ===
[search]
enabled = true                       # 기본값: true

# === SEO (P2) ===
[seo]
sitemap = true                       # 기본값: true
rss = false                          # 기본값: false
og_image = "og-image.png"            # 기본값: None
```

### 7.2 기본값 규칙

| 상황 | 동작 |
|------|------|
| `pyohwa.toml` 없음 | 모든 기본값 적용 |
| `[site]` 섹션 없음 | `title="Documentation"`, `base_url="/"` 등 |
| `[nav]` 없음 | 네비게이션 바 미표시 |
| `[sidebar]` 없음 | `auto = true` (디렉토리에서 자동 생성) |
| `content/` 없음 | 에러: "No content directory found" |

### 7.3 Rust 타입 매핑

```rust
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub site: SiteConfig,
    pub build: BuildConfig,
    pub theme: ThemeConfig,
    pub nav: Vec<NavItem>,
    pub sidebar: SidebarConfig,
    pub search: SearchConfig,
    pub seo: SeoConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct SiteConfig {
    pub title: String,           // "Documentation"
    pub description: String,     // ""
    pub base_url: String,        // "/"
    pub language: String,        // "en"
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct BuildConfig {
    pub content_dir: PathBuf,    // "content"
    pub output_dir: PathBuf,     // "dist"
    pub static_dir: PathBuf,     // "static"
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ThemeConfig {
    pub name: String,            // "default"
    pub highlight_theme: String, // "one-dark"
    pub custom_css: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
pub struct NavItem {
    pub text: String,
    pub link: String,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct SidebarConfig {
    pub auto: bool,              // true
    pub groups: Vec<SidebarGroup>,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct SearchConfig {
    pub enabled: bool,           // true
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct SeoConfig {
    pub sitemap: bool,           // true
    pub rss: bool,               // false
    pub og_image: Option<String>,
}
```

---

## 8. 프론트매터 스키마

### 8.1 전체 스키마

```yaml
---
title: "Getting Started"          # 필수 — 페이지 제목
description: "Quick start guide"  # 선택 — SEO meta description
layout: doc                       # 선택 — doc(기본) | home | page | custom
order: 1                          # 선택 — 사이드바 정렬 순서 (낮을수록 위)
tags: [guide, tutorial]           # 선택 — 태그 목록
date: 2026-01-15                  # 선택 — 작성일 (YYYY-MM-DD, 블로그용)
draft: false                      # 선택 — true면 빌드에서 제외
prev: /intro                      # 선택 — 이전 페이지 링크 오버라이드
next: /guide/configuration        # 선택 — 다음 페이지 링크 오버라이드
---
```

### 8.2 레이아웃 타입

| 레이아웃 | 설명 | 구성 요소 |
|----------|------|-----------|
| `doc` | 기본 문서 레이아웃 | 사이드바 + 콘텐츠 + TOC |
| `home` | 홈 페이지 | Hero 섹션 + 기능 소개 그리드 |
| `page` | 단독 페이지 | 콘텐츠만 (사이드바/TOC 없음) |
| `custom` | 커스텀 | 사용자 정의 (P1) |

### 8.3 Rust 타입 매핑

```rust
#[derive(Debug, Clone)]
pub struct Frontmatter {
    pub title: String,
    pub description: Option<String>,
    pub layout: Layout,
    pub order: Option<i32>,
    pub tags: Vec<String>,
    pub date: Option<NaiveDate>,
    pub draft: bool,
    pub prev: Option<String>,
    pub next: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub enum Layout {
    #[default]
    Doc,
    Home,
    Page,
    Custom(String),
}
```

### 8.4 기본값 규칙

| 필드 | 기본값 | 비고 |
|------|--------|------|
| `title` | **필수** | 없으면 빌드 에러 |
| `description` | `None` | |
| `layout` | `doc` | |
| `order` | `None` | 미지정 시 파일명 알파벳순 |
| `tags` | `[]` | |
| `date` | `None` | |
| `draft` | `false` | |
| `prev` / `next` | `None` | 미지정 시 사이트 그래프에서 자동 계산 |

---

## 9. 코드 패턴 가이드

### 9.1 Rust 패턴

#### Guard Clause (Tidy First)

```rust
pub fn parse_frontmatter(raw: &str) -> Result<Frontmatter, ContentError> {
    // Guard: 빈 입력 조기 반환
    if raw.trim().is_empty() {
        return Err(ContentError::EmptyContent);
    }

    // Guard: 프론트매터 구분자 확인
    let matter = Matter::<YAML>::new();
    let parsed = matter.parse(raw);
    let data = parsed.data
        .ok_or(ContentError::MissingFrontmatter)?;

    // 정상 경로
    let frontmatter = deserialize_frontmatter(data)?;
    Ok(frontmatter)
}
```

#### ADT로 도메인 모델링

```rust
/// 빌드 파이프라인의 각 단계를 나타내는 타입
/// 각 Stage는 이전 Stage의 출력을 입력으로 받음
pub enum BuildStage {
    Configured(Config),
    Discovered(Config, Vec<RawContent>),
    Parsed(Config, Vec<ParsedContent>),
    Rendered(Config, Vec<RenderedContent>),
    Highlighted(Config, Vec<RenderedContent>),
    Graphed(Config, SiteGraph),
    Templated(Vec<(Route, String)>),
    Complete,
}
```

#### thiserror 에러 타입

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContentError {
    #[error("empty content at {path}")]
    EmptyContent { path: PathBuf },

    #[error("missing frontmatter in {path}")]
    MissingFrontmatter { path: PathBuf },

    #[error("invalid frontmatter in {path}: {reason}")]
    InvalidFrontmatter { path: PathBuf, reason: String },

    #[error("missing required field 'title' in {path}")]
    MissingTitle { path: PathBuf },
}

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("content directory not found: {0}")]
    ContentDirNotFound(PathBuf),

    #[error("config error: {0}")]
    Config(#[from] ConfigError),

    #[error("content error: {0}")]
    Content(#[from] ContentError),

    #[error("render error: {0}")]
    Render(#[from] RenderError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
```

#### Iterator 체이닝

```rust
pub fn build_sidebar(pages: &[Page], config: &SidebarConfig) -> Vec<SidebarGroup> {
    if !config.auto {
        return config.groups.clone();
    }

    pages
        .iter()
        .filter(|p| p.frontmatter.layout == Layout::Doc)
        .sorted_by(|a, b| {
            a.frontmatter.order
                .unwrap_or(i32::MAX)
                .cmp(&b.frontmatter.order.unwrap_or(i32::MAX))
        })
        .group_by(|p| p.route.parent_dir())
        .into_iter()
        .map(|(dir, pages)| SidebarGroup {
            text: dir.display_name(),
            items: pages
                .map(|p| SidebarItem {
                    text: p.frontmatter.title.clone(),
                    link: p.route.path().to_string(),
                })
                .collect(),
        })
        .collect()
}
```

### 9.2 Elm 패턴

#### TEA (The Elm Architecture)

```elm
-- Model.elm
type alias Model =
    { page : PageData
    , site : SiteData
    , sidebar : SidebarState
    , toc : TocState
    , search : SearchState
    , viewport : Viewport
    }

type SidebarState
    = SidebarOpen
    | SidebarClosed

type TocState
    = TocIdle
    | TocScrolling String  -- active heading ID
```

```elm
-- Msg.elm
type Msg
    = ToggleSidebar
    | ScrollToHeading String
    | OnScroll Float
    | OnViewportChange Viewport
    | SearchOpen
    | SearchClose
    | SearchInput String
    | NoOp
```

```elm
-- Update.elm
update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ToggleSidebar ->
            ( { model | sidebar = toggleSidebar model.sidebar }
            , Cmd.none
            )

        ScrollToHeading id ->
            ( { model | toc = TocScrolling id }
            , scrollToElement id
            )

        OnScroll scrollY ->
            ( { model | toc = updateActiveHeading scrollY model.page.toc }
            , Cmd.none
            )

        _ ->
            ( model, Cmd.none )
```

```elm
-- View.elm
-- Elm 뷰에서 Tailwind v4 유틸리티 클래스를 직접 사용한다.
-- build.rs가 Elm 소스 파일을 스캔하여 사용된 클래스만 CSS에 포함한다.
view : Model -> Html Msg
view model =
    div [ class "min-h-screen flex flex-col" ]
        [ Theme.Navbar.view model.site.nav
        , div [ class "flex flex-1 mx-auto w-full max-w-screen-2xl" ]
            [ Theme.Sidebar.view model.sidebar model.site.sidebar
            , main_ [ class "flex-1 px-6 py-8 lg:px-8 overflow-y-auto" ]
                [ viewContent model.page.content ]
            , Theme.Toc.view model.toc model.page.toc
            ]
        , Theme.Footer.view
        ]
```

#### Flags 디코딩 (JS → Elm)

```elm
-- Flags.elm
type alias Flags =
    { page : PageData
    , site : SiteData
    , theme : ThemeData
    }

flagsDecoder : Decoder Flags
flagsDecoder =
    Decode.map3 Flags
        (Decode.field "page" pageDecoder)
        (Decode.field "site" siteDecoder)
        (Decode.field "theme" themeDecoder)

pageDecoder : Decoder PageData
pageDecoder =
    Decode.map5 PageData
        (Decode.field "title" Decode.string)
        (Decode.field "description" Decode.string)
        (Decode.field "content" Decode.string)
        (Decode.field "toc" (Decode.list tocItemDecoder))
        (Decode.field "frontmatter" (Decode.dict Decode.value))
```

#### Ports (JS Interop)

```elm
-- Ports.elm
port module Ports exposing (..)

-- Elm → JS
port scrollToElement : String -> Cmd msg

-- JS → Elm
port onScroll : (Float -> msg) -> Sub msg
port onResize : ({ width : Int, height : Int } -> msg) -> Sub msg
```

---

## 10. 개발 Phase

각 Phase는 독립적으로 구현 및 검증 가능하다. LLM Agent는 Phase 순서대로 구현한다.

### Phase 1: Minimal Core

> **목표**: `pyohwa init && pyohwa build`로 md → html 변환 동작

#### 태스크 목록

| # | 태스크 | crate | 파일 | 의존 |
|---|--------|-------|------|------|
| 1.1 | Cargo workspace 초기화 | root | `Cargo.toml` | - |
| 1.2 | 4개 crate 스텁 생성 | all | `crates/*/Cargo.toml`, `src/lib.rs` | 1.1 |
| 1.3 | 에러 타입 정의 | core | `src/lib.rs` (thiserror) | 1.2 |
| 1.4 | Config 타입 + 로더 | core | `src/config.rs` | 1.3 |
| 1.5 | 프론트매터 타입 + 파서 | core | `src/content/frontmatter.rs`, `page.rs` | 1.3 |
| 1.6 | 콘텐츠 로더 | core | `src/content/loader.rs` | 1.5 |
| 1.7 | Markdown 파서 | core | `src/markdown/parser.rs` | 1.3 |
| 1.8 | 구문 강조 | core | `src/markdown/highlight.rs` | 1.7 |
| 1.9 | Route 타입 + 변환 | core | `src/site/route.rs` | 1.3 |
| 1.10 | 사이트 그래프 (기본) | core | `src/site/graph.rs` | 1.5, 1.9 |
| 1.11 | HTML 템플릿 (최소) | core | `src/render/template.rs` | 1.3 |
| 1.12 | 레이아웃 (doc only) | core | `src/render/layout.rs` | 1.11 |
| 1.13 | 임베딩 에셋 스텁 | core | `src/embedded.rs` | 1.2 |
| 1.14 | 정적 에셋 복사 | core | `src/render/assets.rs` | 1.3 |
| 1.15 | 출력 관리 | core | `src/build/output.rs` | 1.3 |
| 1.16 | 빌드 파이프라인 | core | `src/build/pipeline.rs` | 1.4-1.15 |
| 1.17 | CLI (init + build) | cli | `src/main.rs` | 1.16 |
| 1.18 | scaffold 템플릿 | root | `scaffold/*` | - |
| 1.19 | Tailwind v4 테마 초기화 | root | `themes/default/theme.css`, `typography.css` | - |
| 1.20 | 단위 테스트 | core | `tests/` | 1.4-1.16 |
| 1.21 | 통합 테스트 (E2E) | root | `tests/integration/build_test.rs` | 1.17 |

#### 검증 기준

```bash
# 새 프로젝트 생성
pyohwa init my-docs
cd my-docs

# 빌드
pyohwa build

# 결과 확인
ls dist/
# → index.html, assets/theme.css

# HTML 내용 확인
# → 프론트매터 title이 <title>에 반영
# → Markdown이 HTML로 변환
# → 코드 블록에 구문 강조 적용
# → 기본 CSS 적용
```

---

### Phase 2: 테마 + 네비게이션 + Elm

> **목표**: 인터랙티브 테마가 적용된 문서 사이트

#### 태스크 목록

| # | 태스크 | 영역 | 파일 | 의존 |
|---|--------|------|------|------|
| 2.1 | Elm 프로젝트 초기화 | elm | `elm/elm.json` | Phase 1 |
| 2.2 | Flags 디코더 | elm | `elm/src/Flags.elm` | 2.1 |
| 2.3 | Model + Msg 정의 | elm | `elm/src/Model.elm`, `Msg.elm` | 2.2 |
| 2.4 | Update 로직 | elm | `elm/src/Update.elm` | 2.3 |
| 2.5 | Ports 정의 | elm | `elm/src/Ports.elm` | 2.1 |
| 2.6 | Sidebar 컴포넌트 | elm | `elm/src/Theme/Sidebar.elm` | 2.3 |
| 2.7 | Navbar 컴포넌트 | elm | `elm/src/Theme/Navbar.elm` | 2.3 |
| 2.8 | TOC 컴포넌트 | elm | `elm/src/Theme/Toc.elm` | 2.3 |
| 2.9 | Footer 컴포넌트 | elm | `elm/src/Theme/Footer.elm` | 2.3 |
| 2.10 | Layout 조합 | elm | `elm/src/Theme/Layout.elm` | 2.6-2.9 |
| 2.11 | Main.elm (진입점) | elm | `elm/src/Main.elm` | 2.2-2.10 |
| 2.12 | Tailwind v4 테마 CSS (전체) | theme | `themes/default/theme.css`, `typography.css` | - |
| 2.13 | build.rs (Elm 컴파일 + Tailwind 빌드) | core | `crates/pyohwa-core/build.rs` | 2.11, 2.12 |
| 2.14 | embedded.rs 업데이트 | core | `src/embedded.rs` | 2.13 |
| 2.15 | 사이트 그래프 확장 | core | `src/site/graph.rs` | Phase 1 |
| 2.16 | 사이드바 자동 생성 | core | `src/site/graph.rs` | 2.15 |
| 2.17 | 네비게이션 생성 | core | `src/site/graph.rs` | 2.15 |
| 2.18 | `__PYOHWA_DATA__` 주입 | core | `src/render/template.rs` | 2.14, 2.15 |
| 2.19 | Home 레이아웃 | core | `src/render/layout.rs` | Phase 1 |
| 2.20 | Elm 단위 테스트 | elm | `elm/tests/` | 2.11 |
| 2.21 | 통합 테스트 | root | `tests/integration/` | 2.18 |

#### 검증 기준

```bash
pyohwa build
# dist/ 에 elm.min.js 포함
# 브라우저에서 열면:
# → 사이드바 토글 동작
# → 상단 네비게이션 표시
# → TOC 표시 + 클릭 시 스크롤
# → 반응형 레이아웃
```

---

### Phase 3: Dev Server

> **목표**: `pyohwa dev`로 로컬 서버 + 자동 리로드

#### 태스크 목록

| # | 태스크 | crate | 파일 | 의존 |
|---|--------|-------|------|------|
| 3.1 | axum 정적 파일 서버 | server | `src/server.rs` | Phase 2 |
| 3.2 | WebSocket 엔드포인트 | server | `src/reload.rs` | 3.1 |
| 3.3 | 파일 워처 (notify) | server | `src/watcher.rs` | 3.1 |
| 3.4 | 변경 감지 → 재빌드 → WS 알림 | server | `src/lib.rs` | 3.1-3.3 |
| 3.5 | 증분 빌드 매니페스트 | core | `src/build/incremental.rs` | Phase 2 |
| 3.6 | content hash 비교 | core | `src/build/incremental.rs` | 3.5 |
| 3.7 | CLI `dev` 명령어 | cli | `src/main.rs` | 3.4 |
| 3.8 | Live reload 클라이언트 JS | core | `src/embedded.rs` | 3.2 |
| 3.9 | 통합 테스트 | root | `tests/integration/` | 3.7 |

#### 검증 기준

```bash
pyohwa dev
# → http://localhost:3000 에서 사이트 확인
# → content/*.md 수정 시 브라우저 자동 리로드
# → 증분 빌드로 200ms 이내 반응
# → Ctrl+C로 종료
```

---

### Phase 4: 검색 + SEO

> **목표**: 프로덕션 배포 가능한 완전한 SSG

#### 태스크 목록

| # | 태스크 | crate/영역 | 파일 | 의존 |
|---|--------|-----------|------|------|
| 4.1 | 검색 인덱스 생성 | search | `src/indexer.rs` | Phase 3 |
| 4.2 | 텍스트 토크나이저 | search | `src/tokenizer.rs` | 4.1 |
| 4.3 | 검색 JSON 출력 | search | `src/lib.rs` | 4.1, 4.2 |
| 4.4 | Elm 검색 모달 UI | elm | `elm/src/Search/Modal.elm` | Phase 2 |
| 4.5 | Elm 검색 로직 | elm | `elm/src/Search/Search.elm` | 4.4 |
| 4.6 | Ctrl+K 키보드 바인딩 | elm | `elm/src/Main.elm` | 4.5 |
| 4.7 | sitemap.xml 생성 | core | `src/build/output.rs` | Phase 3 |
| 4.8 | RSS/Atom 피드 생성 | core | `src/build/output.rs` | Phase 3 |
| 4.9 | OG tags + meta 생성 | core | `src/render/template.rs` | Phase 3 |
| 4.10 | 빌드 파이프라인 통합 | core | `src/build/pipeline.rs` | 4.3, 4.7-4.9 |
| 4.11 | 검색 인덱스 통합 테스트 | root | `tests/integration/` | 4.10 |
| 4.12 | 성능 테스트 (500p) | root | `tests/fixtures/large/` | 4.10 |

#### 검증 기준

```bash
pyohwa build
# → dist/search-index.json 생성
# → dist/sitemap.xml 생성
# → HTML에 OG tags 포함

# 브라우저에서:
# → Ctrl+K로 검색 모달 열기
# → 검색어 입력 → 결과 표시 → 클릭 시 이동

# 성능:
# → 500페이지 풀 빌드 < 3초
```

---

## 11. 비기능 요구사항

### 11.1 성능

| 지표 | 목표 | 측정 방법 |
|------|------|-----------|
| 500페이지 풀 빌드 | < 3초 | `tests/fixtures/large/` + `cargo bench` |
| 증분 빌드 (1파일 변경) | < 200ms | dev server에서 측정 |
| Elm JS 번들 크기 | < 30KB (minified + gzipped) | `wc -c` + `gzip -9` |
| Tailwind CSS 번들 크기 | < 15KB (minified + gzipped) | `wc -c` + `gzip -9` (사용된 클래스만 포함) |
| 메모리 사용량 | < 200MB (500페이지) | `/usr/bin/time -v` |

### 11.2 빌드 결정론성

동일한 입력(소스 파일 + 설정)은 항상 동일한 출력을 생성해야 한다.

- 출력 HTML에 타임스탬프나 랜덤값 포함 금지
- 파일 순회 순서는 정렬하여 결정적으로
- 해시 기반 파일명은 내용에서만 유도

### 11.3 에러 처리

- 모든 에러는 파일 경로 + 줄 번호 포함 (가능한 경우)
- 사용자 친화적 에러 메시지 (무엇이 잘못되었고, 어떻게 고칠 수 있는지)
- `unwrap()` / `expect()` 금지 (테스트 코드 제외)
- 빌드 에러는 즉시 중단하지 않고 모든 에러를 수집 후 일괄 보고 고려

### 11.4 호환성

- Rust: stable toolchain (MSRV 관리)
- OS: Linux, macOS, Windows
- Markdown: CommonMark 0.31+ spec, GFM 확장

---

## 12. 검증 방법

### 12.1 자동화 테스트

```bash
# Rust 단위 + 통합 테스트
cargo test

# 특정 crate 테스트
cargo test -p pyohwa-core
cargo test -p pyohwa-cli
cargo test -p pyohwa-server
cargo test -p pyohwa-search

# Elm 테스트
cd elm && elm-test
```

### 12.2 E2E 검증

```bash
# 새 프로젝트 생성 → 빌드
pyohwa init test-project
cd test-project
pyohwa build

# 빌드 결과 확인
test -f dist/index.html && echo "OK" || echo "FAIL"
test -f dist/assets/elm.min.js && echo "OK" || echo "FAIL"
test -f dist/assets/theme.css && echo "OK" || echo "FAIL"

# Dev server
pyohwa dev &
sleep 2
curl -s http://localhost:3000 | grep -q "<title>" && echo "OK" || echo "FAIL"
kill %1
```

### 12.3 성능 검증

```bash
# 500페이지 fixture 생성
mkdir -p tests/fixtures/large/content
for i in $(seq 1 500); do
  cat > "tests/fixtures/large/content/page-$i.md" << EOF
---
title: "Page $i"
---
# Page $i
Content for page $i with some **bold** and \`code\`.
EOF
done

# 빌드 시간 측정
time pyohwa build --content-dir tests/fixtures/large/content
# 목표: real < 3s
```

### 12.4 빌드 결정론성 검증

```bash
pyohwa build
cp -r dist dist-1

pyohwa build
diff -r dist dist-1 && echo "Deterministic: OK" || echo "Deterministic: FAIL"
```

---

## 부록 A: 주요 Rust 의존성

| crate | 버전 | 용도 |
|-------|------|------|
| `clap` | 4.x | CLI 파서 |
| `comrak` | 0.x | CommonMark + GFM 파서 |
| `syntect` | 5.x | 구문 강조 |
| `gray_matter` | 0.x | YAML 프론트매터 파싱 |
| `toml` | 0.x | TOML 설정 파싱 |
| `serde` + `serde_json` | 1.x | 직렬화/역직렬화 |
| `thiserror` | 2.x | 에러 타입 매크로 |
| `walkdir` | 2.x | 재귀 디렉토리 탐색 |
| `axum` | 0.x | HTTP 서버 (P1) |
| `notify` | 7.x | 파일 시스템 워처 (P1) |
| `tokio` | 1.x | 비동기 런타임 (P1) |
| `tokio-tungstenite` | 0.x | WebSocket (P1) |

## 부록 B: 프론트엔드 의존성

### Elm 패키지

| 패키지 | 용도 |
|--------|------|
| `elm/browser` | Browser.application |
| `elm/html` | HTML 렌더링 |
| `elm/json` | JSON 디코딩 (Flags) |
| `elm/url` | URL 파싱 |
| `elm/http` | HTTP 요청 (검색, P2) |

### Tailwind CSS v4

| 도구 | 용도 |
|------|------|
| `@tailwindcss/cli` (standalone) | CSS 빌드 (build.rs에서 실행) |

**Tailwind v4 standalone CLI**: Node.js 의존 없이 단독 실행 가능한 바이너리. `build.rs`에서 호출하여 Elm 소스 내 Tailwind 클래스를 스캔하고 CSS를 생성한다. Pyohwa 개발자/CI 환경에만 필요하며, 최종 사용자는 불필요.

설치: `curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-{os}-{arch}` 또는 `pnpm add -D @tailwindcss/cli`

## 부록 C: 용어집

| 용어 | 정의 |
|------|------|
| **TEA** | The Elm Architecture (Model → Update → View) |
| **ADT** | Algebraic Data Type — Rust의 enum, Elm의 custom type |
| **Guard Clause** | 함수 상단에서 예외 상황을 먼저 처리하고 반환하는 패턴 |
| **SSG** | Static Site Generator — 빌드 타임에 HTML을 생성하는 도구 |
| **GFM** | GitHub Flavored Markdown — CommonMark 확장 |
| **Flags** | Elm 앱 초기화 시 JS에서 전달하는 데이터 |
| **Port** | Elm ↔ JS 간 통신 인터페이스 |
| **Tailwind v4** | 유틸리티 퍼스트 CSS 프레임워크. v4부터 CSS-first 설정 (`@theme`), 새 엔진 (Oxide) |
| **`@theme`** | Tailwind v4에서 디자인 토큰(컬러, 간격, 폰트 등)을 CSS 내에서 정의하는 디렉티브 |
