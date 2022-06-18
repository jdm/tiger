import { ref, Ref, watch, WatchSource } from "vue";

type MultiWatchSources = (WatchSource<unknown> | object)[];

export function debounceAnimation<T extends MultiWatchSources>(
  sources: [...T],
  evaluate: () => boolean
): Ref<boolean> {
  const value = ref(true);
  watch(sources, () => {
    if (!evaluate()) {
      value.value = false;
    } else {
      setTimeout(() => {
        if (evaluate()) {
          value.value = true;
        }
      }, 300);
    }
  });
  return value;
}
