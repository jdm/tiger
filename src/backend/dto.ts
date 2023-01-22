import { Operation } from "fast-json-patch";

export type Patch = ReadonlyArray<Operation>;

export type State = {
  documents: Document[];
  currentDocumentPath: string | null;
  recentDocumentPaths: RecentDocument[];
  clipboardManifest: ClipboardManifest | null;
  isReleaseBuild: boolean;
  error: UserFacingError | null;
  onboardingStep: OnboardingStep;
  updateStep: UpdateStep;
};

export type RecentDocument = {
  path: string;
  name: string;
};

export type UserFacingError = {
  key: string;
  title: string;
  summary: string;
  details: string;
};

export enum OnboardingStep {
  NotStarted = "NotStarted",
  ImportFrame = "ImportFrame",
  CreateAnimation = "CreateAnimation",
  PlaceFrameOnTimeline = "PlaceFrameOnTimeline",
  Completed = "Completed",
}

export enum UpdateStep {
  Idle = "Idle",
  CheckingUpdate = "CheckingUpdate",
  UpdateAvailable = "UpdateAvailable",
  InstallingUpdate = "InstallingUpdate",
}

export enum ClipboardManifest {
  Animations = "Animations",
  Keyframes = "Keyframes",
  Hitboxes = "Hitboxes",
}

export type Document = {
  path: string;
  name: string;
  hasUnsavedChanges: boolean;
  undoEffect: string | null;
  redoEffect: string | null;
  wasCloseRequested: boolean;
  sheet: Sheet;
  framesListMode: ListMode;
  framesListOffset: number;
  framesFilter: string;
  animationsFilter: string;
  animationsListOffset: number;
  hitboxesListOffset: number;
  lastInteractedAnimation: string | null;
  lastInteractedFrame: string | null;
  lastInteractedHitbox: string | null;
  workbenchOffset: [number, number];
  workbenchZoom: number;
  currentAnimationName: string | null;
  currentSequenceDirection: Direction | null;
  currentKeyframeIndex: number | null;
  timelineClockMillis: number;
  timelineIsPlaying: boolean;
  timelineOffsetMillis: number;
  timelineZoomFactor: number;
  timelineZoomAmount: number;
  snapKeyframeDurations: boolean;
  snapKeyframesToOtherKeyframes: boolean;
  snapKeyframesToMultiplesOfDuration: boolean;
  keyframeSnappingBaseDurationMillis: number;
  darkenSprites: boolean;
  hideSprite: boolean;
  hideHitboxes: boolean;
  hideOrigin: boolean;
  lockHitboxes: boolean;
  preserveAspectRatio: boolean;
  isDraggingKeyframeDuration: boolean;
  animationBeingRenamed: string | null;
  hitboxBeingRenamed: string | null;
  framesBeingDragged: string[];
  framesBeingRelocated: Record<string, string> | null;
  keyframesBeingDragged: [Direction, number][];
  hitboxesBeingNudged: string[];
  hitboxesBeingResized: string[];
  exportSettingsBeingEdited: ExportSettings | null;
  exportSettingsValidation: ExportSettingsValidation | null;
};

export type Sheet = {
  frames: Frame[];
  animations: Animation[];
};

export type Frame = {
  path: string;
  name: string;
  selected: boolean;
  filteredOut: boolean;
  missingOnDisk: boolean;
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
  atlasImageFile: string;
  metadataFile: string;
  metadataPathsRoot: string;
};

export type TemplateError = {
  templateError: string;
};

export type ExportSettingsError =
  | "ExpectedAbsolutePath"
  | "ExpectedDirectory"
  | "ExpectedFile"
  | "FileNotFound"
  | TemplateError;

export type ExportSettingsValidation = {
  validSettings: boolean;
  templateFileError: ExportSettingsError | null;
  atlasImageFileError: ExportSettingsError | null;
  metadataFileError: ExportSettingsError | null;
  metadataPathsRootError: ExportSettingsError | null;
};

export type ExportError = {
  documentName: string;
  error: string;
};

export type ExportSuccess = {
  documentName: string;
  atlasImageFilePath: string;
  atlasImageFileName: string;
  metadataFilePath: string;
  metadataFileName: string;
};

export type UpdateError = {
  details: string;
};

export type UpdateSuccess = {
  versionNumber: string;
};

export type TextureInvalidation = {
  path: string;
};

export type OpenDocumentError = {
  documentName: string;
  error: string;
};

export type SaveDocumentError = {
  documentName: string;
  error: string;
};
