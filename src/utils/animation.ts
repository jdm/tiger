import { ref, Ref, watch, WatchSource } from "vue";

type MultiWatchSources = (WatchSource<unknown> | object)[];

export function debounceAnimation<T extends MultiWatchSources>(
  sources: [...T],
  evaluate: () => boolean
): Ref<boolean> {
  const value = ref(true);
  let timerHandle: number;
  watch(sources, () => {
    if (!evaluate()) {
      value.value = false;
    } else {
      window.clearTimeout(timerHandle);
      timerHandle = window.setTimeout(() => {
        if (evaluate()) {
          value.value = true;
        }
      }, 800);
    }
  });
  return value;
}

export function isStable<T extends MultiWatchSources>(
  sources: [...T]
): Ref<boolean> {
  const value = ref(true);
  let timerHandle: number;
  watch(sources, () => {
    value.value = false;
    window.clearTimeout(timerHandle);
    timerHandle = window.setTimeout(() => {
      value.value = true;
    }, 300);
  });
  return value;
}
