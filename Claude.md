Prompt for Claude Code
You are working inside an existing Leptos (Rust, WASM) + Tauri 2.0 application that currently has a Markdown viewer and attempts at live editing, but the implementation is buggy, laggy, and conceptually flawed.
Your task is to refactor and re-architect the editor to behave like Typora, not like a split-view Markdown editor.
🔴 Core Requirement (Read Carefully)
There must be NO split view.
No “raw markdown on the left, preview on the right”.
No syncing text → preview.
No re-rendering entire documents on keystrokes.
The editor must be:
Single-surface
WYSIWYG
Structure-first
Markdown is a serialization format only, NOT the editing format
🧠 Correct Mental Model (Mandatory)
The editor must work like this:
Markdown is parsed into an AST
This AST is the source of truth
NOT raw text
The AST is rendered into an editable DOM
Use contenteditable
Every block (paragraph, heading, list item, code block) maps to a node
User edits the rendered content directly
They are editing meaning, not syntax
Markdown syntax (##, \*\*, - , ) is:
Recognized as input triggers
Converted into AST nodes
Then removed from the visible DOM
Markdown is only generated:
On save
On export
On demand
🧩 What You Must Do

1. Audit the Existing Codebase
   Identify:
   Any split-view assumptions
   Any “string → markdown → html” loops
   Any full re-renders on keystroke
   Explicitly explain:
   Why each of these causes lag or bugs
2. Introduce a Document Model
   Implement a document AST, for example:
   Document
   Block nodes:
   Paragraph
   Heading(level)
   List
   CodeBlock
   Inline nodes:
   Text
   Strong
   Emphasis
   Code
   This model must:
   Live in Rust
   Be reactive-friendly for Leptos
   Have stable node IDs
3. AST → Editable DOM Renderer
   Render each AST node into:
   Semantic HTML
   contenteditable=true
   Each DOM node must contain:
   data-node-id
   data-node-type
   DO NOT:
   Re-render the entire document on input
   Diff HTML strings
4. Cursor & Selection Handling
   This is critical.
   You must:
   Track cursor as (node_id, text_offset)
   Restore cursor position after DOM mutations
   Never rely on raw DOM offsets alone
   Explain how selection is preserved.
5. Markdown Trigger System
   Implement syntax triggers, for example:
   Typing # at start → convert paragraph → heading
   Typing **text** → convert into Strong node
   Typing → code block
   Rules:
   Triggers modify the AST
   DOM updates follow AST changes
   Literal Markdown syntax is removed from view
6. Input Handling Strategy
   Explain and implement:
   beforeinput
   input
   keydown
   Paste handling:
   Raw markdown paste
   Plain text paste
   Avoid:
   Parsing the entire document on every keypress
7. Performance Rules (Non-Negotiable)
   No full document re-renders
   No markdown → html conversion during typing
   Minimal DOM mutations
   No debounce hacks
   Explain why this fixes lag.
8. Markdown Serialization
   Provide:
   AST → Markdown exporter
   Deterministic output
   No reliance on DOM scraping
   🧪 Testing Expectations
   Include:
   Edge cases (delete formatting boundaries)
   Cursor stability tests
   Undo/redo behavior explanation
   🛑 Explicitly Forbidden
   You must NOT:
   Keep split view
   Use preview syncing
   Treat markdown text as editable source
   Hide syntax with regex or CSS tricks
   📦 Deliverables
   When finished, provide:
   Explanation of the new architecture
   Key Rust structs
   Key Leptos components
   Event flow diagrams (textual)
   Why this now behaves like Typora
   🧠 Guiding Principle (Do Not Violate)
   The user is editing a document, not a string.
   Markdown is storage.
   AST is truth.
   DOM is a view.
