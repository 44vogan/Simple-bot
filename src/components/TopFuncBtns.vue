<template>
	<div class="top-buttons">
		<button
			class="start-stop-btn"
			@click="$emit('start_or_stop')"
			title="Alt + p (Option + p)"
		>
			{{ startButton }}
		</button>
		<!-- test -->
		<!-- <button @click="testFunction()">test</button> -->
		<!-- testFunction -->
		<button
			class=""
			@click="$emit('toggleDisabledRules')"
			title="隐藏/显示禁用的规则"
		>
			{{ store.lang === "en" ? "Hide / Show disabled" : "隐藏/显示禁用的规则" }}
		</button>
		<div>
			<span title="execution interval >= 100ms (1000ms = 1s)">{{
				store.lang === "en" ? "Interval(ms):" : "间隔(毫秒)"
			}}</span>
			<input
				type="number"
				min="10"
				v-model="intervalTime.t"
				title="执行间隔:最小值为 10ms(1000ms = 1 秒)"
				@change="$emit('save_bot_settings')"
			/>
		</div>
		<button class="" @click="$emit('change_lang')" title="切换语言">
			{{ store.lang === "en" ? "语言 -> 中" : "lang -> en" }}
		</button>
	</div>
</template>

<script setup lang="ts">
	import { store } from "./store.js";
	import { exit } from "@tauri-apps/api/process";
	const props = defineProps<{
		startButton: string;
		intervalTime: { t: number };
	}>();
</script>

<style scoped>
	.top-buttons {
		display: flex;
		justify-content: space-between;
		margin-bottom: 20px;
	}
	.start-stop-btn {
		background: #b71d1d;
	}
</style>
