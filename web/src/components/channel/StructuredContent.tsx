"use client";

import type { MessageContent, Block, InlineElement } from "@/lib/api/channel-message-types";
import { usePodStore } from "@/stores/pod";
import { getPodDisplayName } from "@/lib/pod-display-name";
import { cn } from "@/lib/utils";

interface StructuredContentProps {
  content: MessageContent;
  className?: string;
}

export function StructuredContent({ content, className }: StructuredContentProps) {
  if (!content.blocks?.length) return null;

  return (
    <div className={cn("prose prose-sm max-w-none", className)}>
      {content.blocks.map((block, i) => (
        <RenderBlock key={i} block={block} />
      ))}
    </div>
  );
}

function RenderBlock({ block }: { block: Block }) {
  switch (block.type) {
    case "paragraph":
      return (
        <p>
          {block.elements?.map((el, i) => (
            <RenderInline key={i} element={el} />
          ))}
        </p>
      );
    case "code_block":
      return (
        <pre className="p-3 bg-muted rounded-md text-sm overflow-x-auto">
          <code>{block.text}</code>
        </pre>
      );
    case "heading": {
      const Tag = `h${Math.min(block.level ?? 1, 3)}` as "h1" | "h2" | "h3";
      return (
        <Tag>
          {block.elements?.map((el, i) => (
            <RenderInline key={i} element={el} />
          ))}
        </Tag>
      );
    }
    case "quote":
      return (
        <blockquote>
          {block.elements?.map((el, i) => (
            <RenderInline key={i} element={el} />
          ))}
        </blockquote>
      );
    case "list": {
      const Tag = block.ordered ? "ol" : "ul";
      return (
        <Tag>
          {block.items?.map((item, i) => (
            <li key={i}>
              {item.map((el, j) => (
                <RenderInline key={j} element={el} />
              ))}
            </li>
          ))}
        </Tag>
      );
    }
    default:
      return null;
  }
}

function RenderInline({ element }: { element: InlineElement }) {
  switch (element.type) {
    case "text":
      return <TextSpan element={element} />;
    case "mention":
      return <MentionSpan element={element} />;
    case "link":
      return (
        <a href={element.url} target="_blank" rel="noopener noreferrer" className="text-primary underline">
          {element.text}
        </a>
      );
    case "linebreak":
      return <br />;
    default:
      return null;
  }
}

function TextSpan({ element }: { element: InlineElement }) {
  let content: React.ReactNode = element.text;
  if (element.code) content = <code className="px-1 py-0.5 bg-muted rounded text-sm">{content}</code>;
  if (element.bold) content = <strong>{content}</strong>;
  if (element.italic) content = <em>{content}</em>;
  if (element.strike) content = <del>{content}</del>;
  return <>{content}</>;
}

function MentionSpan({ element }: { element: InlineElement }) {
  const allPods = usePodStore((s) => s.pods);

  let displayName = element.display ?? element.entity_key ?? "unknown";

  if (element.entity_type === "pod" && element.entity_key) {
    const pod = allPods.find((p) => p.pod_key === element.entity_key);
    if (pod) displayName = getPodDisplayName(pod);
  }

  return (
    <span className="text-primary font-medium bg-primary/10 rounded px-0.5">
      @{displayName}
    </span>
  );
}

export default StructuredContent;
