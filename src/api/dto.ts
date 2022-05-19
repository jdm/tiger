import { Operation } from "fast-json-patch";

export type Patch = ReadonlyArray<Operation>;

export type AppState = {
  documents: Document[];
  currentDocumentPath: string | null;
};

export type Document = {
  path: string;
  name: string;
  sheet: Sheet;
  view: View;
};

export type Sheet = {
  frames: Frame[];
  animations: Record<string, Animation>;
};

export type Frame = {
  path: string;
  name: string;
  selected: boolean;
};

export type Animation = {
  name: string;
  selected: boolean;
};

export type View = {
  contentTab: ContentTab;
  workbenchOffset: [number, number];
  currentAnimationName: string | null;
};

export type ContentTab = "frames" | "animations";
