# File Extension Options — Extension Analysis

Left-Right language requires a file extension for source files. This document analyzes `.lr` and alternative extensions, considering availability, associations, and naming conventions.

## .lr Extension Analysis

### Current Usage

The `.lr` extension is the default choice for Left-Right based on the language name.

### Availability Research

**Current Usage:**
- **No major language** currently uses `.lr` as primary extension
- **Not registered** with major language registries
- **Available** for Left-Right adoption

**Potential Conflicts:**
- **LogRhythm** (music software) — Uses `.lr` for rhythm files (minimal usage)
- **LaTeX** bibliography — Occasionally `.lr` for bibliography (rare)
- **Various tools** — Some tools use `.lr` for log/report files (not programming languages)

**Conclusion:** `.lr` is effectively available for Left-Right as a programming language extension.

## Alternative Extensions

Based on naming documents and brainstorming, these alternatives have been considered:

### Option 1: .lrsc

**Description:** PenroScript shortened (LR + SCript)

**Pros:**
- **Clear association** — Direct reference to language name
- **Distinctive** — Low conflict risk
- **Explicit** — Clearly indicates Left-Right Script

**Cons:**
- **Longer** — 5 characters vs typical 3
- **Non-standard** — Unusual 4-letter language extension
- **Typo-prone** — Easy to mistype as `.lsrc` or `.lrsc`

### Option 2: .lfr

**Description:** Left-Right shortened (LF + Right)

**Pros:**
- **Compact** — 3 characters, standard length
- **Memorable** — Easy to remember
- **Distinct** — No major conflicts

**Cons:**
- **Ambiguous** — Could mean "Left-From-Right" or other acronym
- **No "script"** — Doesn't indicate programming language
- **Confusable** — Similar to `.lfc` (other extension)

### Option 3: .ltr

**Description:** Left-Right abbreviated (LTR)

**Pros:**
- **Short** — 3 characters
- **Memorable** — Matches evaluation order name
- **Common pattern** — `.js`, `.py`, `.rb` style

**Cons:**
- **Ambiguous** — "Left To Right" has other meanings
- **Existing usage** — Some tools use `.ltr` for log/text files
- **Confusion** — Could be mistaken for plain text

### Option 4: .lft

**Description:** Left-Right abbreviated (LFT for LeFT)

**Pros:**
- **Short** — 3 characters
- **Simple** — Easy to type
- **Distinctive** — Not widely used

**Cons:**
- **Ambiguous** — "LFT" commonly means "Left"
- **Non-obvious** — Doesn't clearly reference language
- **Confusable** — Similar to other short extensions

### Option 5: .lrt

**Description:** Left-Right abbreviated (LRT)

**Pros:**
- **Short** — 3 characters
- **Clear** — Matches Left-Right abbreviation
- **Standard pattern** — 3-character extensions

**Cons:**
- **Existing usage** — Some tools use `.lrt` (less common)
- **Non-distinct** — Could be confused with other acronyms

### Option 6: .pon

**Description:** PenroScript/Left-Right abbreviated (PON)

**Pros:**
- **Short** — 3 characters
- **Historical** — References original name
- **Memorable** — Easy to remember

**Cons:**
- **Ambiguous** — "PON" has multiple meanings
- **Historical baggage** — Old language name
- **Confusion** — Could be mistaken for "Python" (.py)

### Option 7: .pnr

**Description:** PenroScript Right abbreviated (PNR)

**Pros:**
- **Short** — 3 characters
- **Distinctive** — Not widely used
- **Pattern** — Follows abbreviation style

**Cons:**
- **Historical baggage** — References old name
- **Less clear** — Doesn't indicate current language name
- **Confusable** — Similar to other extensions

### Option 8: .hmap

**Description:** Map (primary data structure)

**Pros:**
- **Descriptive** — Describes core data structure
- **Semantic** — Indicates map-based language
- **Distinctive** — Not widely used for programming

**Cons:**
- **Generic** — Doesn't reference language name
- **Length** — 5 characters (longer)
- **Not standard** — Unusual for programming languages

### Option 9: .gly

**Description:** Glyph abbreviated (GLY for Glyph)

**Pros:**
- **Short** — 3 characters
- **Artistic** — References operator symbology
- **Distinctive** — Not widely used

**Cons:**
- **Obscure** — "Glyph" not commonly associated
- **Confusing** — "GLY" has other meanings
- **Not descriptive** — Doesn't indicate language

### Option 10: .glyph

**Description:** Glyph (operator symbology focus)

**Pros:**
- **Descriptive** — Focuses on operator design
- **Unique** — Clearly distinct
- **Artistic** — Creative naming

**Cons:**
- **Long** — 6 characters (longer than typical)
- **Generic** — Could apply to any symbolic language
- **Non-standard** — Unusual for programming languages

## Comparison Table

| Extension | Length | Availability | Association | Clarity | Recommendation |
|-----------|---------|-------------|------------|----------------|
| `.lr` | 2 | Available | High (language name) | **Recommended** |
| `.lrsc` | 4 | Available | High (language name + script) | Alternative |
| `.lfr` | 3 | Available | Medium (ambiguous) | Not recommended |
| `.ltr` | 3 | Some usage | Medium (LTR acronym) | Alternative |
| `.lft` | 3 | Available | Low (ambiguous) | Not recommended |
| `.lrt` | 3 | Some usage | Medium (LRT acronym) | Alternative |
| `.pon` | 3 | Available | Medium (historical) | Not recommended |
| `.pnr` | 3 | Available | Medium (historical) | Not recommended |
| `.hmap` | 4 | Available | Low (descriptive) | Not recommended |
| `.gly` | 3 | Available | Low (artistic) | Not recommended |
| `.glyph` | 5 | Available | Low (artistic) | Not recommended |

## File Naming Conventions

### Source Files

**Recommended Convention:**
```bash
# Main files
main.lr
index.lr

# Module files
utils.lr
data-processor.lr

# Test files
utils.test.lr
data-processor.test.lr

# Configuration files
config.lr
settings.lr
```

### Project Structure

**Recommended Layout:**
```
project/
├── src/
│   ├── main.lr              # Entry point
│   ├── utils.lr             # Utilities
│   ├── data/
│   │   ├── processor.lr    # Data operations
│   │   └── validators.lr  # Validation
│   └── config.lr           # Configuration
├── tests/
│   ├── utils.test.lr
│   └── data.test.lr
├── dist/                      # Transpiled output
├── package.json                # NPM dependencies
├── Cargo.toml                 # Rust dependencies
└── README.md
```

### Migration from PenroScript

**Old Files:**
```bash
# PenroScript files
script.prsc
module.prsc
```

**Migration Path:**
```bash
# Rename files
mv script.prsc script.lr
mv module.prsc module.lr

# Update imports
# file[`./module.prsc`] → file[`./module.lr`]
```

## Recommendation

### Primary Recommendation: `.lr`

**Rationale:**

1. **Direct Association:**
   - `.lr` directly matches "Left-Right" language name
   - No ambiguity about what language the files represent

2. **Optimal Length:**
   - 2 characters is short and easy to type
   - Follows pattern of popular languages (`.js`, `.py`, `.rb`)

3. **Availability:**
   - No major programming language uses `.lr` currently
   - Minimal conflict risk
   - Available for widespread adoption

4. **Memorability:**
   - Easy to remember: "Left-Right" → `.lr`
   - Natural extension for the language name

5. **Industry Practice:**
   - Most languages use 2-3 character extensions
   - `.lr` fits this pattern perfectly

### Migration Strategy

**Phase 1: Initial Release**
```bash
# Support .lr primarily
lr --extension .lr script.lr
```

**Phase 2: Transition Period**
```bash
# Support both extensions temporarily
lr --extension .lr,.prsc script.lr
lr --extension .prsc script.prsc  # Deprecation warning
```

**Phase 3: Final Standardization**
```bash
# .lr only
lr script.lr  # .prsc support removed
```

## IDE and Tooling Support

### Syntax Highlighting

**File Associations:**
```json
// VS Code settings.json
{
  "files.associations": {
    "*.lr": "left-right",
    "*.lrsc": "left-right"
  }
}
```

### Language Configuration

**Language Server Protocol:**
```bash
# LSP integration
lr lsp --server
# File associations for IDE
```

### Build Tools

**CLI Integration:**
```bash
# Watch .lr files
lr watch src/ --pattern "*.lr"

# Build with extension
lr build src/ --extension .lr --output dist/
```

## Conclusion

The `.lr` extension is the recommended choice for Left-Right language files. It provides:
- Clear language association
- Optimal length (2 characters)
- High availability
- Easy memorability
- Industry-standard pattern

Alternative extensions (`.lrsc`, `.ltr`, `.lrt`) can serve as secondary options during transition or for specific use cases, but `.lr` should be the primary standard.

## Related Concepts

- **File Extensions** — Filename suffixes indicating file type
- **Language Association** — Mapping extensions to languages
- **Tooling Support** — IDE, syntax highlighting, build tools
- **Migration** — Converting from old to new standards
- **Naming Conventions** - File and directory organization
