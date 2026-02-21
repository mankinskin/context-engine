import { useRef, useCallback } from '@context-engine/viewer-api-frontend';

interface ResizeHandleProps {
  onResize: (delta: number) => void;
  onResizeStart?: () => void;
  direction: 'horizontal' | 'vertical';
}

export function ResizeHandle({ onResize, onResizeStart, direction }: ResizeHandleProps) {
  const isDragging = useRef(false);
  const lastPos = useRef(0);

  const handleMouseDown = useCallback((e: MouseEvent) => {
    e.preventDefault();
    isDragging.current = true;
    lastPos.current = direction === 'horizontal' ? e.clientX : e.clientY;
    document.body.style.cursor = direction === 'horizontal' ? 'col-resize' : 'row-resize';
    document.body.style.userSelect = 'none';
    
    // Notify parent that resize is starting (to capture current dimensions)
    onResizeStart?.();

    const handleMouseMove = (e: MouseEvent) => {
      if (!isDragging.current) return;
      const currentPos = direction === 'horizontal' ? e.clientX : e.clientY;
      const delta = currentPos - lastPos.current;
      lastPos.current = currentPos;
      onResize(delta);
    };

    const handleMouseUp = () => {
      isDragging.current = false;
      document.body.style.cursor = '';
      document.body.style.userSelect = '';
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };

    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
  }, [onResize, onResizeStart, direction]);

  return (
    <div 
      class={`resize-handle resize-handle-${direction}`}
      onMouseDown={handleMouseDown}
    />
  );
}
