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
  animations: Animation[];
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
};

export type ContentTab = "frames" | "animations";
