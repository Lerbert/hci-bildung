import { JSONContent } from "@tiptap/vue-3";

import { Mark, MarkJSON } from "./SheetDisplayMark";

export interface NodeJSON {
  type?: string;
  content: NodeJSON[];
  marks: MarkJSON[];
  source?: string;
  mimetype?: string;
  language?: string;
  level?: number;
  solution?: boolean;
  answer?: boolean;
  text?: string;
  [key: string]: unknown;
}

export class Node {
  type?: string;
  content: Node[];
  marks: Mark[];

  constructor(content: Node[], marks: Mark[], type?: string) {
    this.type = type;
    this.content = content;
    this.marks = marks;
  }

  public static fromTiptap(tiptapNode: JSONContent): Node {
    switch (tiptapNode.type) {
      case "audio":
        return Audio.fromTiptap(tiptapNode);
      case "codeBlock":
        return Codeblock.fromTiptap(tiptapNode);
      case "heading":
        return Heading.fromTiptap(tiptapNode);
      case "multipleChoice":
        return MultipleChoice.fromTiptap(tiptapNode);
      case "multipleChoiceAnswer":
        return MultipleChoiceAnswer.fromTiptap(tiptapNode);
      case "text":
        return Text.fromTiptap(tiptapNode);
      default:
        return new Node(
          Node.contentFromTiptap(tiptapNode),
          Node.marksFromTiptap(tiptapNode),
          tiptapNode.type
        );
    }
  }

  protected static contentFromTiptap(tiptapNode: JSONContent): Node[] {
    return tiptapNode.content?.map((node) => Node.fromTiptap(node)) ?? [];
  }

  protected static marksFromTiptap(tiptapNode: JSONContent): Mark[] {
    return (
      tiptapNode.marks?.map((mark) => Mark.fromTiptap(mark, tiptapNode)) ?? []
    );
  }

  public static fromJSON(json: NodeJSON): Node {
    switch (json.type) {
      case "audio":
        return Audio.fromJSON(json);
      case "codeBlock":
        return Codeblock.fromJSON(json);
      case "heading":
        return Heading.fromJSON(json);
      case "multipleChoice":
        return MultipleChoice.fromJSON(json);
      case "multipleChoiceAnswer":
        return MultipleChoiceAnswer.fromJSON(json);
      case "text":
        return Text.fromJSON(json);
      default:
        return new Node(
          Node.contentFromJSON(json),
          Node.marksFromJSON(json),
          json.type
        );
    }
  }

  protected static contentFromJSON(json: NodeJSON): Node[] {
    return json.content.map((node) => Node.fromJSON(node));
  }

  protected static marksFromJSON(json: NodeJSON): Mark[] {
    return json.marks.map((mark) => Mark.fromJSON(mark));
  }

  public static emptyDocument(): Node {
    return new Node([new Node([], [], "paragraph")], [], "doc");
  }

  public toTiptap(): JSONContent {
    return {
      type: this.type,
      content:
        this.content.length === 0
          ? undefined
          : this.content.map((node) => node.toTiptap()),
      marks:
        this.marks.length === 0
          ? undefined
          : this.marks.map((mark) => mark.toTiptap()),
    };
  }
}

export class Audio extends Node {
  source: string;
  mimetype: string;

  constructor(
    content: Node[],
    marks: Mark[],
    source: string,
    mimetype: string
  ) {
    super(content, marks, "audio");
    this.source = source;
    this.mimetype = mimetype;
  }

  public static fromTiptap(tiptapNode: JSONContent): Audio {
    return new Audio(
      Node.contentFromTiptap(tiptapNode),
      Node.marksFromTiptap(tiptapNode),
      tiptapNode.attrs?.source ?? "",
      tiptapNode.attrs?.mimetype ?? ""
    );
  }

  public static fromJSON(json: NodeJSON): Audio {
    return new Audio(
      Node.contentFromJSON(json),
      Node.marksFromJSON(json),
      json.source ?? "",
      json.mimetype ?? ""
    );
  }

  public toTiptap(): JSONContent {
    return {
      attrs: {
        source: this.source,
        mimetype: this.mimetype,
      },
      ...super.toTiptap(),
    };
  }
}

export class Codeblock extends Node {
  language: string;

  constructor(content: Node[], marks: Mark[], language: string) {
    super(content, marks, "codeBlock");
    this.language = language;
  }

  public static fromTiptap(tiptapNode: JSONContent): Codeblock {
    return new Codeblock(
      Node.contentFromTiptap(tiptapNode),
      Node.marksFromTiptap(tiptapNode),
      tiptapNode.attrs?.language ?? "plain"
    );
  }

  public static fromJSON(json: NodeJSON): Codeblock {
    return new Codeblock(
      Node.contentFromJSON(json),
      Node.marksFromJSON(json),
      json.language ?? "plain"
    );
  }

  public toTiptap(): JSONContent {
    return {
      attrs: {
        language: this.language,
      },
      ...super.toTiptap(),
    };
  }
}

export class Heading extends Node {
  level: number;

  constructor(content: Node[], marks: Mark[], level: number) {
    super(content, marks, "heading");
    this.level = level;
  }

  public static fromTiptap(tiptapNode: JSONContent): Heading {
    return new Heading(
      Node.contentFromTiptap(tiptapNode),
      Node.marksFromTiptap(tiptapNode),
      tiptapNode.attrs?.level ?? 1
    );
  }

  public static fromJSON(json: NodeJSON): Heading {
    return new Heading(
      Node.contentFromJSON(json),
      Node.marksFromJSON(json),
      json.level ?? 1
    );
  }

  public toTiptap(): JSONContent {
    return {
      attrs: {
        level: this.level,
      },
      ...super.toTiptap(),
    };
  }
}

export class MultipleChoice extends Node {
  declare content: MultipleChoiceAnswer[];

  constructor(content: MultipleChoiceAnswer[], marks: Mark[]) {
    super(content, marks, "multipleChoice");
  }

  public static fromTiptap(tiptapNode: JSONContent): MultipleChoice {
    return new MultipleChoice(
      MultipleChoice.contentFromTiptap(tiptapNode),
      Node.marksFromTiptap(tiptapNode)
    );
  }

  protected static contentFromTiptap(
    tiptapNode: JSONContent
  ): MultipleChoiceAnswer[] {
    if (
      tiptapNode.content !== undefined &&
      !tiptapNode.content?.every((node) => node.type === "multipleChoiceAnswer")
    ) {
      throw new Error(
        "MultipleChoice must only contain MultipleChoiceAnswer nodes"
      );
    }
    return (
      tiptapNode.content?.map((node) =>
        MultipleChoiceAnswer.fromTiptap(node)
      ) ?? []
    );
  }

  public static fromJSON(json: NodeJSON): MultipleChoice {
    return new MultipleChoice(
      MultipleChoice.contentFromJSON(json),
      Node.marksFromJSON(json)
    );
  }

  protected static contentFromJSON(json: NodeJSON): MultipleChoiceAnswer[] {
    if (!json.content.every((node) => node.type === "multipleChoiceAnswer")) {
      throw new Error(
        "MultipleChoice must only contain MultipleChoiceAnswer nodes"
      );
    }
    return json.content.map((node) => MultipleChoiceAnswer.fromJSON(node));
  }
}

export class MultipleChoiceAnswer extends Node {
  solution: boolean;
  answer: boolean;

  constructor(
    content: Node[],
    marks: Mark[],
    solution: boolean,
    answer: boolean
  ) {
    super(content, marks, "multipleChoiceAnswer");
    this.solution = solution;
    this.answer = answer;
  }

  public static fromTiptap(tiptapNode: JSONContent): MultipleChoiceAnswer {
    return new MultipleChoiceAnswer(
      Node.contentFromTiptap(tiptapNode),
      Node.marksFromTiptap(tiptapNode),
      tiptapNode.attrs?.checked ?? false,
      false
    );
  }

  public static fromJSON(json: NodeJSON): MultipleChoiceAnswer {
    return new MultipleChoiceAnswer(
      Node.contentFromJSON(json),
      Node.marksFromJSON(json),
      json.solution ?? false,
      json.answer ?? false
    );
  }

  public toTiptap(): JSONContent {
    return {
      attrs: {
        checked: this.solution,
      },
      ...super.toTiptap(),
    };
  }
}

export class Text extends Node {
  text: string;

  constructor(content: Node[], marks: Mark[], text: string) {
    super(content, marks, "text");
    this.text = text;
  }

  public static fromTiptap(tiptapNode: JSONContent): Text {
    return new Text(
      Node.contentFromTiptap(tiptapNode),
      Node.marksFromTiptap(tiptapNode),
      tiptapNode.text ?? ""
    );
  }

  public static fromJSON(json: NodeJSON): Text {
    return new Text(
      Node.contentFromJSON(json),
      Node.marksFromJSON(json),
      json.text ?? ""
    );
  }

  public toTiptap(): JSONContent {
    return {
      text: this.text,
      ...super.toTiptap(),
    };
  }
}
