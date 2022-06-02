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
  hasUnsavedChanges: boolean;
  wasCloseRequested: boolean;
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
  isDraggingKeyframeDuration: boolean;
  framesBeingDragged: string[];
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
  directionPreset: DirectionPreset | null;
  isLooping: boolean;
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

export enum DirectionPreset {
  FourDirections = "FourDirections",
  EightDirections = "EightDirections",
  LeftRight = "LeftRight",
  UpDown = "UpDown",
  Isometric = "Isometric",
  FixedAngle = "FixedAngle",
}

export type Sequence = {
  keyframes: Keyframe[];
  durationMillis: number | null;
};

export type Keyframe = {
  frame: string;
  name: string;
  selected: boolean;
  startTimeMillis: number;
  durationMillis: number;
  offset: [number, number];
};

export enum ContentTab {
  Frames = "Frames",
  Animations = "Animations",
}
