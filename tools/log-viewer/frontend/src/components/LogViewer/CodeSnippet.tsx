import Prism from 'prismjs';
import 'prismjs/components/prism-rust';
import type { SourceSnippet } from '../../types';
import { openSourceFile } from '../../store';

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
 * Click any line to open the full file at that line.
 */
export function CodeSnippet({ snippet, file, isPanic = false }: CodeSnippetProps) {
  const language = file.endsWith('.rs') ? 'rust' : 'plaintext';
  const highlightedLines = snippet.content.split('\n').map(line => 
    highlightCode(line, language)
  );

  const handleLineClick = (lineNum: number, e: MouseEvent) => {
    e.stopPropagation();
    openSourceFile(file, lineNum);
  };

  return (
    <div class={`code-snippet ${isPanic ? 'panic-snippet' : ''}`}>
      <pre class="snippet-code">
        {highlightedLines.map((line, i) => {
          const lineNum = snippet.start_line + i;
          const isHighlight = lineNum === snippet.highlight_line;
          return (
            <div 
              key={i} 
              class={`snippet-line clickable ${isHighlight ? 'highlight' : ''}`}
              onClick={(e) => handleLineClick(lineNum, e)}
              title="Click to open full file at this line"
            >
              <span class="line-number">{lineNum}</span>
              <code dangerouslySetInnerHTML={{ __html: line || ' ' }} />
            </div>
          );
        })}
      </pre>
    </div>
  );
}
