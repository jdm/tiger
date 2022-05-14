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
};

export type Frame = {
  path: string;
  name: string;
};

export type View = {
  contentTab: ContentTab;
};

export type ContentTab = "frames" | "animations";
