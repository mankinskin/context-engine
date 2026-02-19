import Prism from 'prismjs';
import 'prismjs/components/prism-rust';
import type { SourceSnippet } from '../../types';

function escapeHtml(text: string): string {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

function highlightCode(code: string, language: string): string {
  try {
    const grammar = Prism.languages[language];
    if (grammar) {
      return Prism.highlight(code, grammar, language);
    }
  } catch {
    // Fall back to plain text
  }
  return escapeHtml(code);
}

interface CodeSnippetProps {
  snippet: SourceSnippet;
  file: string;
  isPanic?: boolean;
}

/**
 * Renders a code snippet with syntax highlighting and line numbers.
 * Highlights the specified highlight_line from the snippet.
 */
export function CodeSnippet({ snippet, file, isPanic = false }: CodeSnippetProps) {
  const language = file.endsWith('.rs') ? 'rust' : 'plaintext';
  const highlightedLines = snippet.content.split('\n').map(line => 
    highlightCode(line, language)
  );

  return (
    <div class={`code-snippet ${isPanic ? 'panic-snippet' : ''}`}>
      <pre class="snippet-code">
        {highlightedLines.map((line, i) => {
          const lineNum = snippet.start_line + i;
          const isHighlight = lineNum === snippet.highlight_line;
          return (
            <div key={i} class={`snippet-line ${isHighlight ? 'highlight' : ''}`}>
              <span class="line-number">{lineNum}</span>
              <code dangerouslySetInnerHTML={{ __html: line || ' ' }} />
            </div>
          );
        })}
      </pre>
    </div>
  );
}
