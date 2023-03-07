<template>
	<FocusTrap @escape="closeAboutDialog">
		<ModalDialog title="About Tiger" :icon="InformationCircleIcon">
			<template #icon>
				<div class="flex-shrink-0">
					<img src="src/mascot.svg" class="w-[124px]" />
				</div>
			</template>
			<template #body>
				<div class="flex flex-col gap-4">
					<div>
						<div><span class="">Version: </span><span class="text-plastic-300">{{ version }}</span></div>
						<div><span class="">Commit: </span><span class="text-plastic-300">{{ commit }}</span></div>
					</div>
					<div class="flex flex-col gap-1">
						<div class="flex items-center gap-2">
							<HeartIcon class="w-5 text-rose-500" />
							<div>Made with love by Antoine Gersant</div>
						</div>
						<div class="flex items-center gap-2">
							<CodeBracketSquareIcon class="w-5" />
							<a href="https://github.com/agersant/tiger" class="text-orange-500 underline cursor-pointer"
								target="_blank">https://github.com/agersant/tiger</a>
						</div>
					</div>
				</div>
			</template>
			<template #actions>
				<Button label="OK" @click="closeAboutDialog" tabbable positive class="w-20" />
			</template>
		</ModalDialog>
	</FocusTrap>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { CodeBracketSquareIcon, HeartIcon } from "@heroicons/vue/20/solid"
import { InformationCircleIcon } from "@heroicons/vue/24/outline"
import { closeAboutDialog } from "@/backend/api";
import { useStateStore } from "@/stores/state";
import Button from "@/components/basic/Button.vue"
import FocusTrap from "@/components/basic/FocusTrap.vue"
import ModalDialog from "@/components/basic/ModalDialog.vue"
import { app } from "@tauri-apps/api";

const state = useStateStore();

const version = ref("");
const commit = computed(() => state.commitHash || "Unknown" );

onMounted(async () => {
	version.value = await app.getVersion();
});
</script>