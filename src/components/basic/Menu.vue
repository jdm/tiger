<template>
	<div ref="menuElement" class="relative">
		<MenuBackground :style="`transform: translate(${screenFit[0]}px, ${screenFit[1]}px)`" @mouseenter="onMouseEnter"
			@mouseleave="onMouseLeave">
			<div v-for="entry of content">
				<MenuItem v-if="'name' in entry" :entry="entry"
					:highlighted="(entry.key || entry.name) == (highlightEntry?.key || highlightEntry?.name)"
					@executed="onItemExecuted" @hovered="onItemHovered($event, entry)"
					@unhovered="onItemUnhovered($event, entry)" />
				<MenuSeparator v-else class="mx-7 my-2" />
			</div>
			<div class="absolute" :style="submenuPosition">
				<Menu v-if="submenuEntry?.submenus?.length" :content="submenuEntry?.submenus"
					@executed="emit('executed')" @hovered="onSubmenuHovered($event, submenuEntry!)" />
			</div>
		</MenuBackground>
	</div>
</template>

<script setup lang="ts">
import { computed, Ref, ref, watch } from "vue"
import MenuBackground from "@/components/basic/MenuBackground.vue"
import { MenuEntry, Separator } from "@/components/basic/MenuBar.vue";
import MenuItem from "@/components/basic/MenuItem.vue"
import MenuSeparator from "@/components/basic/MenuSeparator.vue"

const menuElement: Ref<HTMLElement | null> = ref(null);
const screenFit = ref([0, 0] as [number, number]);
const highlightEntry: Ref<MenuEntry | null> = ref(null);
const submenuEntry: Ref<MenuEntry | null> = ref(null);
const submenuElement: Ref<HTMLElement | null> = ref(null);
const didHoverSubmenu = ref(false);

const props = defineProps<{
	content: (MenuEntry | Separator)[],
}>();

const emit = defineEmits<{
	(e: "executed"): void,
	(e: 'hovered', element: HTMLElement): void,
	(e: "unhovered", element: HTMLElement): void,
}>();

function onItemExecuted() {
	emit("executed");
}

watch(() => props.content, () => {
	highlightEntry.value = null;
	submenuEntry.value = null;
	submenuElement.value = null;
	didHoverSubmenu.value = false;
});

watch(submenuEntry, () => {
	didHoverSubmenu.value = false;
});

watch(menuElement, () => {
	if (!menuElement.value) {
		return;
	}
	const boundingBox = menuElement.value.getBoundingClientRect();
	const padding = 8;
	screenFit.value = [
		Math.min(0, window.innerWidth - padding - boundingBox.right),
		Math.min(0, window.innerHeight - padding - boundingBox.bottom),
	];
});

function onItemHovered(element: HTMLElement, entry: MenuEntry) {
	highlightEntry.value = entry;
	window.setTimeout(() => {
		if (highlightEntry.value == entry) {
			if (!!entry.submenus?.length) {
				submenuEntry.value = entry;
				submenuElement.value = element;
			} else {
				submenuEntry.value = null;
				submenuElement.value = null;
			}
		}
	}, 200);
}

function onItemUnhovered(element: HTMLElement, entry: MenuEntry) {
	highlightEntry.value = null;
	window.setTimeout(() => {
		if (highlightEntry.value != entry && submenuEntry.value == entry && !didHoverSubmenu.value) {
			submenuEntry.value = null;
			submenuElement.value = null;
		}
	}, 500);
}

function onSubmenuHovered(element: HTMLElement, entry: MenuEntry) {
	highlightEntry.value = entry;
	didHoverSubmenu.value = true;
}

function onMouseEnter(event: MouseEvent) {
	emit("hovered", event.currentTarget as HTMLElement);
}

function onMouseLeave(event: MouseEvent) {
	emit("unhovered", event.currentTarget as HTMLElement);
}

const submenuPosition = computed(() => {
	const parent = submenuElement.value?.parentElement;
	const y = (parent?.offsetTop || 0) - 10;
	return {
		top: `${y}px`,
		right: "0px",
		transform: "translateX(calc(100% - 4px))",
	};
});
</script>