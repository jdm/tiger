import { Operation } from "fast-json-patch";

export type Patch = ReadonlyArray<Operation>;

export type AppState = {
  documents: Document[];
  currentDocumentPath: string | null;
  isReleaseBuild: boolean;
  error: UserFacingError | null;
};

export type UserFacingError = {
  key: string;
  title: string;
  summary: string;
  details: string;
};

export type Document = {
  path: string;
  name: string;
  hasUnsavedChanges: boolean;
  undoEffect: string | null;
  redoEffect: string | null;
  wasCloseRequested: boolean;
  sheet: Sheet;
  framesListMode: ListMode;
  framesFilter: string;
  animationsFilter: string;
  workbenchOffset: [number, number];
  workbenchZoom: number;
  currentAnimationName: string | null;
  currentSequenceDirection: Direction | null;
  currentKeyframeIndex: number | null;
  timelineClockMillis: number;
  timelineIsPlaying: boolean;
  timelineZoomFactor: number;
  timelineZoomAmount: number;
  darkenSprites: boolean;
  hideSprite: boolean;
  hideHitboxes: boolean;
  hideOrigin: boolean;
  lockHitboxes: boolean;
  preserveAspectRatio: boolean;
  isDraggingKeyframeDuration: boolean;
  framesBeingDragged: string[];
  keyframesBeingDragged: [Direction, number][];
  hitboxesBeingNudged: string[];
  hitboxesBeingResized: string[];
  exportSettingsBeingEdited: ExportSettings | null;
};

export type Sheet = {
  frames: Frame[];
  animations: Record<string, Animation>;
};

export type Frame = {
  path: string;
  name: string;
  selected: boolean;
  filteredOut: boolean;
};

export type Animation = {
  name: string;
  selected: boolean;
  filteredOut: boolean;
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
  hitboxes: Hitbox[];
  key: string;
};

export type Hitbox = {
  name: string;
  selected: boolean;
  topLeft: [number, number];
  size: [number, number];
  key: string;
};

export enum ListMode {
  Linear = "Linear",
  Grid4xN = "Grid4xN",
}

export enum ResizeAxis {
  N = "N",
  S = "S",
  W = "W",
  E = "E",
  NW = "NW",
  NE = "NE",
  SE = "SE",
  SW = "SW",
}

export enum NudgeDirection {
  Up = "Up",
  Down = "Down",
  Left = "Left",
  Right = "Right",
}

export enum BrowseDirection {
  Up = "Up",
  Down = "Down",
  Left = "Left",
  Right = "Right",
}

export type ExportSettings = {
  templateFile: string;
  textureFile: string;
  metadataFile: string;
  metadataPathsRoot: string;
};

export type TextureInvalidationEvent = {
  path: string;
};
