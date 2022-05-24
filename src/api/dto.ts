import { Operation } from "fast-json-patch";

export type Patch = ReadonlyArray<Operation>;

export type AppState = {
  documents: Document[];
  currentDocumentPath: string | null;
  isReleaseBuild: boolean;
};

export type Document = {
  path: string;
  name: string;
  sheet: Sheet;
  contentTab: ContentTab;
  workbenchOffset: [number, number];
  workbenchZoom: number;
  currentAnimationName: string | null;
  currentSequenceDirection: Direction | null;
  currentKeyframeIndex: number | null;
  timelineClockMillis: number;
  timelineIsPlaying: boolean;
  timelineZoom: number;
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
  sequences: Record<Direction, Sequence>;
};

export enum Direction {
  East = "East",
  NorthEast = "NorthEast",
  North = "North",
  NorthWest = "NorthWest",
  West = "West",
  SouthWest = "SouthWest",
  South = "South",
  SouthEast = "SouthEast",
}

export type Sequence = {
  keyframes: Keyframe[];
  durationMillis: number | null;
};

export type Keyframe = {
  frame: string;
  name: string;
  durationMillis: number;
  offset: [number, number];
};

export enum ContentTab {
  Frames = "Frames",
  Animations = "Animations",
}
