<template>
	<div class="aspect-square checkerboard flex place-content-center relative rounded-sm overflow-hidden">
		<img ref="imageElement" :src="sprite.getURL(path)" @load="onImageLoaded" @error="onImageError"
			class="pixelated object-none" :class="isValid ? 'opacity-100' : 'opacity-0'" />
		<ExclamationTriangleIcon v-if="!isValid"
			class="w-6 text-amber-300 absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2" />
	</div>
</template>

<script setup lang="ts">
import { Ref, ref } from "vue";
import { ExclamationTriangleIcon } from "@heroicons/vue/24/solid"
import { useSpriteStore } from "@/stores/sprite";

const imageElement: Ref<HTMLImageElement | null> = ref(null);
const isValid = ref(false);

defineProps<{
	path: string,
}>();

const emit = defineEmits(["update:isValid"]);

const sprite = useSpriteStore();

function onImageLoaded() {
	isValid.value = (imageElement.value?.naturalWidth || 0) > 0;
	emit("update:isValid", isValid.value);
}

function onImageError() {
	isValid.value = false;
	emit("update:isValid", isValid.value);
}

</script>
