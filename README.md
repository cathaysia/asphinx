# Asphinx

A modern AsciiDoc static site generator designed for technical documentation and books.

## Features

- 🚀 **High Performance**: Built with Rust, supports concurrent processing
- 📚 **AsciiDoc Support**: Full AsciiDoc format support including mathematical formulas and diagrams
- 🎨 **Modern Theme**: Responsive theme based on React + Tailwind CSS
- 🔍 **Full-text Search**: Built-in search functionality for quick content discovery
- 📊 **Diagram Support**: Supports multiple diagram formats (PlantUML, Mermaid, Graphviz, etc.)
- ⚡ **Fast Build**: Smart caching and incremental builds
- 🌐 **SEO Friendly**: Generates optimized HTML structure

## Installation

### Build from Source

```bash
git clone https://github.com/your-username/asphinx.git
cd asphinx
cargo build --release
```

### System Requirements

- Rust 1.70+
- Node.js 18+ (for theme building)
- AsciiDoctor (for document processing)

## Quick Start

1. **Initialize project structure**:
   ```
   your-project/
   ├── content/
   │   ├── index.adoc
   │   ├── book1/
   │   │   ├── index.adoc
   │   │   ├── ch1.adoc
   │   │   └── ch2.adoc
   │   └── book2/
   │       ├── index.adoc
   │       ├── ch1.adoc
   │       └── ch2.adoc
   ├── theme/
   └── asphinx.toml
   ```

2. **Create your main index file** (`content/index.adoc`):
   ```asciidoc
   = My Documentation

   Welcome to my documentation site.

   - xref:book1/index.adoc[Book 1]
   - xref:book2/index.adoc[Book 2]
   ```

3. **Configure Asphinx** (`asphinx.toml`):
   ```toml
   [asciidoc]
   extensions = ["asciidoctor-mathematical", "asciidoctor-diagram"]

   [asciidoc.attributes]
   icons = "font"
   toc = 1
   experimental = ""
   source-highlighter = "pygments"
   ```

4. **Build the theme**:
   ```bash
   cd theme
   npm install
   npm run build
   ```

5. **Generate the site**:
   ```bash
   ./target/release/asphinx --theme theme
   ```

## Usage

### Command Line Options

```bash
asphinx [OPTIONS] --theme <THEME>

Options:
      --minify         Enable HTML minification
      --theme <THEME>  Path to the theme directory
  -h, --help           Print help
```

### Configuration

The `asphinx.toml` file contains the configuration for your site:

```toml
# Default configuration
no_default = false

[asciidoc]
extensions = ["asciidoctor-mathematical", "asciidoctor-diagram"]

[asciidoc.attributes]
icons = "font"
toc = 1
experimental = ""
source-highlighter = "pygments"
# Diagram formats
plantuml-format = "svg"
mermaid-format = "svg"
graphviz-format = "svg"
# ... more diagram formats
```

### Supported Diagram Types

Asphinx supports a wide variety of diagram formats:

- **PlantUML**: Sequence diagrams, class diagrams, activity diagrams
- **Mermaid**: Flowcharts, sequence diagrams, Gantt charts
- **Graphviz**: DOT language graphs
- **Ditaa**: ASCII art diagrams
- **BlockDiag**: Block diagrams
- **SeqDiag**: Sequence diagrams
- **ActDiag**: Activity diagrams
- **NwDiag**: Network diagrams
- And many more...

## Theme Development

The theme is built with modern web technologies:

- **React 18**: Component-based UI
- **TypeScript**: Type-safe development
- **Tailwind CSS**: Utility-first CSS framework
- **Vite**: Fast build tool
- **Radix UI**: Accessible component primitives

### Theme Structure

```
theme/
├── src/
│   ├── main.tsx          # Main React entry point
│   ├── search-bar.tsx    # Search functionality
│   ├── style.css         # Global styles
│   └── components/       # UI components
├── layouts/
│   └── page.html         # HTML template
└── assets/               # Static assets
```

### Customizing the Theme

1. Modify the React components in `theme/src/`
2. Update styles in `theme/src/style.css`
3. Rebuild the theme: `npm run build`

## Project Structure

```
asphinx/
├── src/
│   ├── main.rs           # Main application entry
│   ├── config.rs         # Configuration handling
│   ├── generator.rs      # HTML generation logic
│   ├── index.rs          # Search index management
│   └── utils/            # Utility modules
├── content/              # Example content
├── theme/                # Default theme
└── asphinx.toml          # Default configuration
```

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Powered by [AsciiDoctor](https://asciidoctor.org/)
- UI components from [Radix UI](https://www.radix-ui.com/)
- Styling with [Tailwind CSS](https://tailwindcss.com/)
