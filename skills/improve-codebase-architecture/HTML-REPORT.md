# Formato del informe HTML

La review arquitectónica se renderiza como un único archivo HTML autocontenido en el directorio temporal del SO. Tailwind y Mermaid vienen de CDNs. Mermaid maneja bien diagramas con forma de grafo; divs hechos a mano e SVG inline manejan los visuales más editoriales (mass diagrams, cross-sections). Mezcla los dos — no te apoyes solo en Mermaid o empezará a verse genérico.

Idioma del informe: **español** en títulos, problem/solution/wins y top recommendation. Los términos del glosario (`module`, `interface`, `seam`, etc.) se mantienen en inglés.

## Scaffold

```html
<!doctype html>
<html lang="es">
  <head>
    <meta charset="utf-8" />
    <title>Architecture review — {{nombre del repo}}</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <script type="module">
      import mermaid from "https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs";
      mermaid.initialize({ startOnLoad: true, theme: "neutral", securityLevel: "loose" });
    </script>
    <style>
      /* capa pequeña custom para lo que Tailwind no cubre limpio:
         líneas de seam discontinuas, puntas de flecha con feeling a mano, etc. */
      .seam { stroke-dasharray: 4 4; }
      .leak { stroke: #dc2626; }
      .deep { background: linear-gradient(135deg, #0f172a, #1e293b); }
    </style>
  </head>
  <body class="bg-stone-50 text-slate-900 font-sans">
    <main class="max-w-5xl mx-auto px-6 py-12 space-y-12">
      <header><!-- nombre repo, fecha, leyenda compacta --></header>
      <section id="candidates" class="space-y-10"><!-- cards --></section>
      <section id="top-recommendation"><!-- recomendación top --></section>
    </main>
  </body>
</html>
```

## Header

Nombre del repo, fecha y una leyenda compacta: caja sólida = module, línea discontinua = seam, flecha roja = leakage, caja oscura gruesa = deep module. Sin párrafo de introducción — directo a los candidatos.

## Card de candidato

Los diagramas llevan el peso. La prosa es escasa, clara y usa los términos del glosario (de la skill `codebase-design`) sin ceremonia.

Cada candidato es un `<article>`:

- **Title** — corto, nombra el deepening (p. ej. "Colapsar el pipeline de Order intake").
- **Badge row** — fuerza de recomendación (`Strong` = emerald, `Worth exploring` = amber, `Speculative` = slate), más un tag de categoría de dependencia (`in-process`, `local-substitutable`, `ports & adapters`, `mock`).
- **Files** — lista monoespaciada, `font-mono text-sm`.
- **Before / After diagram** — la pieza central. Dos columnas, lado a lado. Ver patrones abajo.
- **Problem** — una frase. Qué duele.
- **Solution** — una frase. Qué cambia.
- **Wins** — bullets, ≤6 palabras cada uno. p. ej. "Tests golpean una interface", "Pricing deja de filtrar", "Borrar 4 wrappers shallow".
- **ADR callout** (si aplica) — una línea en caja tintada ámbar.

Sin párrafos de explicación. Si el diagrama necesita un párrafo para entenderse, redibuja el diagrama.

## Patrones de diagrama

Elige el patrón que encaje con el candidato. Mézclalos. No hagas que todos los diagramas se vean iguales — la variedad es parte del punto.

### Grafo Mermaid (caballo de batalla para dependencias / call flow)

Usa un `flowchart` o `graph` de Mermaid cuando el punto sea "X llama a Y llama a Z, y mira el lío." Envuélvelo en una card con estilo Tailwind para que no parezca lanzado en paracaídas. Estiliza con classDef para colorear edges de leakage en rojo y el deep module en oscuro. Los sequence diagrams funcionan bien para "antes: 6 round-trips; después: 1."

```html
<div class="rounded-lg border border-slate-200 bg-white p-4">
  <pre class="mermaid">
    flowchart LR
      A[OrderHandler] --> B[OrderValidator]
      B --> C[OrderRepo]
      C -.leak.-> D[PricingClient]
      classDef leak stroke:#dc2626,stroke-width:2px;
      class C,D leak
  </pre>
</div>
```

### Cajas y flechas a mano (cuando el layout de Mermaid pelea)

Módulos como `div`s con bordes y labels. Flechas como elementos SVG inline `line` o `path` posicionados en absoluto sobre un contenedor relativo. Usa esto cuando quieras que el diagrama "after" se sienta como un deep module de borde grueso con internos atenuados — Mermaid no lo renderiza con el peso correcto.

### Cross-section (bueno para shallowness en capas)

Apila bandas horizontales (`h-12 border-l-4`) para mostrar las capas que atraviesa una llamada. Antes: 6 capas finas que no hacen nada. Después: 1 banda gruesa etiquetada con la responsabilidad consolidada.

### Mass diagram (bueno para "interface tan ancha como implementation")

Dos rectángulos por módulo — uno para el área de superficie de la interface, otro para la implementation. Antes: el rectángulo de interface es casi tan alto como el de implementation (shallow). Después: interface corta, implementation alta (deep).

### Call-graph collapse

Antes: un árbol de llamadas a funciones como cajas anidadas. Después: el mismo árbol colapsado en una caja, con las llamadas ahora internas mostradas atenuadas dentro.

## Guía de estilo

- Editorial, no corporate-dashboard. Whitespace generoso. Serif opcional en headings (`font-serif` funciona bien con stone/slate).
- Color con mesura: un acento (emerald o indigo) más rojo para leakage y ámbar para avisos.
- Mantén diagramas ~320px de alto para que before/after quepa cómodo lado a lado sin scroll.
- Usa `text-xs uppercase tracking-wider` para labels de módulo dentro de diagramas — deben leerse como esquema, no como UI.
- Los únicos scripts son el CDN de Tailwind y el import ESM de Mermaid. El informe es estático por lo demás — sin app code, sin interactividad más allá del render propio de Mermaid.

## Sección Top recommendation

Una card más grande. Nombre del candidato, una frase del porqué, enlace ancla a su card. Eso es todo.

## Tono

Español claro y conciso — pero los sustantivos y verbos arquitectónicos vienen directo de la skill `codebase-design`. La concisión no es excusa para derivar.

**Usa exactamente:** module, interface, implementation, depth, deep, shallow, seam, adapter, leverage, locality.

**Nunca sustituyas:** component, service, unit (por module) · API, signature (por interface) · boundary (por seam) · layer, wrapper (por module, cuando quieres decir module).

**Frases que encajan con el estilo:**

- "Order intake module es shallow — la interface casi coincide con la implementation."
- "Pricing filtra a través del seam."
- "Deepen: una interface, un lugar para testear."
- "Dos adapters justifican el seam: HTTP en prod, in-memory en tests."

**Wins bullets** nombran la ganancia en términos del glosario: *"locality: bugs se concentran en un module"*, *"leverage: una interface, N call sites"*, *"la interface se encoge; la implementation absorbe los wrappers"*. No escribas *"más fácil de mantener"* o *"código más limpio"* — esos términos no están en el glosario y no se ganan su sitio.

Sin hedges, sin aclaraciones de garganta, sin "vale la pena señalar que…". Si una frase podría ser un bullet, hazla bullet. Si un bullet se puede cortar, córtalo. Si un término no está en el glosario de `codebase-design`, alcanza uno que sí esté antes de inventar uno nuevo.
