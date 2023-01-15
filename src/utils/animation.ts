import { ref, Ref, watch, WatchSource } from "vue";

type MultiWatchSources = (WatchSource<unknown> | object)[];

export function debounceAnimation<T extends MultiWatchSources>(
  sources: [...T],
  evaluate: () => boolean,
  delay?: number
): Ref<boolean> {
  const value = ref(true);
  watch(
    sources,
    () => {
      if (!evaluate()) {
        value.value = false;
      } else {
        window.setTimeout(() => {
          if (evaluate()) {
            value.value = true;
          }
        }, delay || 800);
      }
    },
    { immediate: true }
  );
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
