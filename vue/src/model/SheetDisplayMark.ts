import { JSONContent } from "@tiptap/vue-3";

type JSONContentMark = {
  type: string;
  attrs?: Record<string, unknown>;
  [key: string]: unknown;
};

export interface MarkJSON {
  type: string;
  solution?: string;
  answer?: string;
  source?: string;
  [key: string]: unknown;
}

export class Mark {
  type: string;

  constructor(type: string) {
    this.type = type;
  }

  public static fromTiptap(
    tiptapMark: JSONContentMark,
    parentNode: JSONContent
  ): Mark {
    switch (tiptapMark.type) {
      case "gap":
        return Gap.fromTiptap(parentNode);
      case "latex":
        return Latex.fromTiptap(parentNode);
      default:
        return new Mark(tiptapMark.type);
    }
  }

  public static fromJSON(json: MarkJSON) {
    switch (json.type) {
      case "gap":
        return Gap.fromJSON(json);
      case "latex":
        return Latex.fromJSON(json);
      default:
        return new Mark(json.type);
    }
  }

  public toTiptap(): JSONContentMark {
    return {
      type: this.type,
    };
  }
}

export class Gap extends Mark {
  solution: string;
  answer: string;

  constructor(solution: string, answer: string) {
    super("gap");
    this.solution = solution;
    this.answer = answer;
  }

  public static fromTiptap(parentNode: JSONContent): Gap {
    return new Gap(parentNode.text ?? "", "");
  }

  public static fromJSON(json: MarkJSON): Gap {
    return new Gap(json.solution ?? "", json.answer ?? "");
  }
}

export class Latex extends Mark {
  source: string;

  constructor(source: string) {
    super("latex");
    this.source = source;
  }

  public static fromTiptap(parentNode: JSONContent): Latex {
    return new Latex(parentNode.text ?? "");
  }

  public static fromJSON(json: MarkJSON): Latex {
    return new Latex(json.source ?? "");
  }
}
