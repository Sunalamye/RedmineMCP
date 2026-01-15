# Textile Markup Syntax

Redmine uses Textile markup language for `notes` and Wiki content.

## Basic Formatting

| Effect | Syntax |
|--------|--------|
| **Bold** | `*bold*` |
| _Italic_ | `_italic_` |
| Underline | `+underline+` |
| Strikethrough | `-strikethrough-` |
| Code | `@code@` |

## Headings

```textile
h1. Heading 1
h2. Heading 2
h3. Heading 3
```

## Lists

```textile
* Unordered item
* Another item

# Ordered item
# Another item
```

## Others

| Effect | Syntax |
|--------|--------|
| Blockquote | `bq. Quote content` |
| Link | `"Display text":http://url` |
| Image | `!image.png!` |
| Code block | `<pre>code</pre>` |

## Example

```textile
h2. Fix Summary

*Issue fixed*, changes made:

# Modified file A
# Modified file B

@git commit abc123@
```
