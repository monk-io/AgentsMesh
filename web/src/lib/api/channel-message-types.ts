export interface InlineElement {
  type: "text" | "mention" | "link" | "linebreak";
  text?: string;
  bold?: boolean;
  italic?: boolean;
  strike?: boolean;
  code?: boolean;
  entity_type?: "pod" | "user" | "ticket" | "channel";
  entity_key?: string;
  display?: string;
  url?: string;
}

export interface Block {
  type: "paragraph" | "heading" | "code_block" | "quote" | "list";
  elements?: InlineElement[];
  level?: number;
  language?: string;
  text?: string;
  ordered?: boolean;
  items?: InlineElement[][];
}

export interface MessageContent {
  kind: string;
  blocks?: Block[];
  attachment_key?: string;
}

export interface MessageMentions {
  pods?: string[];
  users?: number[];
  channel?: boolean;
}
