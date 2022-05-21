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
  contentTab: ContentTab;
  workbenchOffset: [number, number];
  currentAnimationName: string | null;
  timelineClockMillis: number;
  timelineIsPlaying: boolean;
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
  timeline: Keyframe[];
};

export type Keyframe = {
  source: string;
  name: string;
  duration_millis: number;
};

export type ContentTab = "frames" | "animations";
