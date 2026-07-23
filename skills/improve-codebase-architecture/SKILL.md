---
name: improve-codebase-architecture
description: >-
  Escanea un codebase en busca de oportunidades de deepening, las presenta como
  informe HTML visual y luego aplica grilling a la que elijas. Usar cuando el
  usuario pida mejorar arquitectura, deep modules, fricción arquitectónica,
  revisar seams, o mencione improve-codebase-architecture / architecture review HTML.
disable-model-invocation: true
---

# Improve Codebase Architecture

Saca a la luz fricción arquitectónica y propone **oportunidades de deepening** — refactors que convierten shallow modules en deep ones. El objetivo es testabilidad y navegabilidad por IA.

Este comando se *informa* del modelo de dominio del proyecto y se apoya en un vocabulario de diseño compartido:

- Ejecuta / lee la skill `codebase-design` para el vocabulario de arquitectura (**module**, **interface**, **depth**, **seam**, **adapter**, **leverage**, **locality**) y sus principios (the deletion test, "the interface is the test surface", "one adapter = hypothetical seam, two = real"). Usa estos términos exactamente en cada sugerencia — no derives a "component," "service," "API," o "boundary."
- El lenguaje de dominio en `CONTEXT.md` nombra buenos seams; los ADR en `docs/adr/` registran decisiones que este comando no debe re-litigar.

## Proceso

### 1. Explorar

**Acota el alcance antes de escanear — YAGNI.** Hacer deepening de un módulo vale la pena porque facilita cambios futuros en él, así que pondera más las partes del codebase que han cambiado recientemente. Decide *dónde* mirar antes de mirar:

- Si el usuario nombró una dirección — un módulo, un subsistema, un dolor — tómalo y salta la inferencia de abajo.
- Si no, recorre un buen tramo del historial de commits (`git log --oneline`) para encontrar los hot spots del codebase — archivos y áreas que vuelven a aparecer — y deja que esas rutas tiren primero de tu atención. Si los cambios están dispersos sin hot spot claro, amplía la red.

Lee primero el glosario de dominio del proyecto (`CONTEXT.md`) y cualquier ADR del área que toques.

Luego usa la herramienta Task con `subagent_type=explore` para recorrer el codebase. No sigas heurísticas rígidas — explora de forma orgánica y anota dónde sientes fricción:

- ¿Dónde entender un concepto obliga a saltar entre muchos módulos pequeños?
- ¿Dónde hay módulos **shallow** — interface casi tan compleja como la implementation?
- ¿Dónde se extrajeron pure functions solo por testabilidad, pero los bugs reales están en cómo se llaman (sin **locality**)?
- ¿Dónde módulos acoplados filtran a través de sus seams?
- ¿Qué partes del codebase no están testeadas, o son difíciles de testear a través de su interface actual?

Aplica el **deletion test** a todo lo que sospeches shallow: ¿borrarlo concentraría complejidad, o solo la movería? Un "sí, concentra" es la señal que quieres.

### 2. Presentar candidatos como informe HTML

Escribe un archivo HTML autocontenido en el directorio temporal del SO para que nada aterrice en el repo. Resuelve el temp dir desde `$TMPDIR`, con fallback a `/tmp` (o `%TEMP%` en Windows), y escribe a:

```text
$TMPDIR/architecture-review-<timestamp>.html
```

(ejemplo: `/tmp/architecture-review-20260720-065512.html`). Cada run obtiene un archivo fresco. Ábrelo para el usuario — `xdg-open <path>` en Linux, `open <path>` en macOS, `start <path>` en Windows — y dile la ruta absoluta.

El informe usa **Tailwind vía CDN** para layout y estilos, y **Mermaid vía CDN** para diagramas cuando un grafo/flujo/secuencia comunica bien la estructura. Mezcla Mermaid con visuales CSS/SVG hechos a mano — Mermaid cuando las relaciones son grafo (call graphs, dependencias, secuencias), y divs/SVG propios cuando quieras algo más editorial (mass diagrams, cross-sections, animaciones de colapso). Cada candidato recibe una visualización **before/after**. Sé visual.

Para cada candidato, renderiza una card con:

- **Files** — qué archivos/módulos intervienen
- **Problem** — por qué la arquitectura actual causa fricción
- **Solution** — descripción en español claro de qué cambiaría
- **Benefits** — explicados en términos de locality y leverage, y cómo mejorarían los tests
- **Before / After diagram** — lado a lado, dibujado a medida, ilustrando la shallowness y el deepening
- **Recommendation strength** — uno de `Strong`, `Worth exploring`, `Speculative`, como badge

Cierra el informe con una sección **Top recommendation**: qué candidato atacarías primero y por qué.

**Usa el vocabulario de CONTEXT.md para el dominio, y el de `codebase-design` para la arquitectura.** Si `CONTEXT.md` define "Order," habla de "the Order intake module" — no de "the FooBarHandler," ni de "the Order service."

**Conflictos con ADR**: si un candidato contradice un ADR existente, solo sácalo cuando la fricción sea real suficiente para justificar reabrir el ADR. Márcalo claramente en la card (p. ej. callout de aviso: _"contradice ADR-0007 — pero vale reabrir porque…"_). No listes todo refactor teórico que un ADR prohíbe.

Ver [HTML-REPORT.md](HTML-REPORT.md) para el scaffold HTML completo, patrones de diagrama y guía de estilo.

**NO propongas interfaces todavía.** Tras escribir el archivo, pregunta al usuario: "¿Cuál de estos te gustaría explorar?"

### 3. Bucle de grilling

Cuando el usuario elija un candidato, ejecuta la skill `grill-me` para recorrer el árbol de decisiones con él — restricciones, dependencias, la forma del módulo profundizado, qué queda detrás del seam, qué tests sobreviven.

Los side effects ocurren en línea conforme cristalizan las decisiones — mantén el modelo de dominio al día mientras avanzas:

- **¿Nombras un módulo profundizado con un concepto que no está en `CONTEXT.md`?** Añade el término a `CONTEXT.md`. Crea el archivo de forma lazy si no existe.
- **¿Afilas un término borroso durante la conversación?** Actualiza `CONTEXT.md` ahí mismo.
- **¿El usuario rechaza el candidato con un motivo de carga?** Ofrece un ADR, enmarcado así: _"¿Quieres que lo registre como ADR para que futuras reviews de arquitectura no lo vuelvan a sugerir?"_ Solo ofrece cuando el motivo realmente lo necesitaría un explorador futuro para no re-sugerir lo mismo — salta motivos efímeros ("ahora no merece la pena") y los autoevidentes.
- **¿Quieres explorar interfaces alternativas para el módulo profundizado?** Ejecuta la skill `codebase-design` y sigue su archivo `DESIGN-IT-TWICE.md` (patrón de subagentes en paralelo).

## Relación con otras skills

| Skill | Rol |
|-------|-----|
| `codebase-design` | Vocabulario y principios; design-it-twice |
| `grill-me` | Entrevista de decisiones tras elegir un candidato |
| `architecture-fitness` | Distinta: trade-offs / -ilities / fitness functions (Richards & Ford). No mezclar marcos sin pedirlo |

## Origen

Adaptado de [mattpocock/skills — improve-codebase-architecture](https://github.com/mattpocock/skills/tree/main/skills/engineering/improve-codebase-architecture). Correcciones locales: ruta temp, refs a skills de Cursor (`grill-me`, `codebase-design`), herramienta Task.
