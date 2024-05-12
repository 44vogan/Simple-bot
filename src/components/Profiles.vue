<script setup lang="ts">
	import { ref } from "vue";
	import { invoke } from "@tauri-apps/api/tauri";
	import { store } from "./store.js";
	//varibles
	const x = ref(0);
	const y = ref(0);
	const theSwitch = ref("off");
	//functiongs
	// async function greet() {
	// 	// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
	// 	greetMsg.value = await invoke("greet", { name: name.value });
	// }
	function turnOffSwitch() {
		theSwitch.value = "off";
	}
	async function get_cursor() {
		if (theSwitch.value != "get_cursor") {
			theSwitch.value = "get_cursor";
			store.add_log("!! Start get position ...");
			let intervalGetCursor = setInterval(async () => {
				let cursorPoint: number[] = await invoke("get_cursor_position");
				store.add_log(`mouse x:${cursorPoint[0]},y:${cursorPoint[1]}`);
				if (theSwitch.value != "get_cursor") {
					clearInterval(intervalGetCursor);
					store.add_log("Stop get position !!");
				}
			}, 1000);
		} else {
			turnOffSwitch();
		}
	}
	function get_pixel_color() {
		if (theSwitch.value != "get_pixel_color") {
			theSwitch.value = "get_pixel_color";
			store.add_log(
				`!!Start get pixel rgb color @ x:${x.value},y:${y.value}...`
			);
			let intervalGetPointColor = setInterval(async () => {
				let pointColor: number[] = await invoke("get_pixel_color", {
					x: x.value,
					y: y.value,
				});
				store.add_log(
					`rgb::${pointColor[0]} ,${pointColor[1]} ,${pointColor[2]}`
				);
				if (theSwitch.value != "get_pixel_color") {
					clearInterval(intervalGetPointColor);
					store.add_log("Stop get pixel rgb color!!");
				}
			}, 1000);
		} else {
			turnOffSwitch();
		}
	}
	function get_cursor_position_and_color() {
		if (theSwitch.value != "get_cursor_position_and_color") {
			theSwitch.value = "get_cursor_position_and_color";
			store.add_log(`mouse -> rgb color `);
			let intervalGetPositionAndColor = setInterval(async () => {
				try {
					let positionAndColor: number[][] = await invoke(
						"get_cursor_position_and_color"
					);
					console.debug(positionAndColor[0]);
					store.add_log(`${positionAndColor[0]} -> rgb:${positionAndColor[1]}`);
					if (theSwitch.value != "get_cursor_position_and_color") {
						clearInterval(intervalGetPositionAndColor);
						store.add_log("Stop!!");
					}
				} catch (err) {
					store.add_log(`error:${err}`);
				}
			}, 1000);
		} else {
			turnOffSwitch();
		}
	}
</script>

<template>
	<div id="Profiles">
		<button @click="get_cursor()" title="开始获取指针坐标/秒">
			<span v-if="theSwitch != 'get_cursor'">{{
				store.lang === "en" ? "get mouse position" : "指针位置"
			}}</span>
			<span v-if="theSwitch == 'get_cursor'">{{
				store.lang === "en" ? "Stop !" : "停止 !"
			}}</span>
		</button>
		<div class="getCursorColor">
			<div>
				<div>
					<span>x:</span>
					<input v-model="x" type="number" />
				</div>

				<div>
					<span>y:</span>
					<input v-model="y" type="number" />
				</div>
			</div>
			<button @click="get_pixel_color()" title="获取坐标颜色">
				<span v-if="theSwitch != 'get_pixel_color'"
					>{{
						store.lang === "en" ? "get rgb color of pixel" : "获取 rgb 颜色"
					}}
					({{ x }},{{ y }})</span
				>
				<span v-if="theSwitch == 'get_pixel_color'">{{
					store.lang === "en" ? "Stop !" : "停止 !"
				}}</span>
			</button>
		</div>
		<!-- 同时获取鼠标位置和颜色 -->
		<button
			@click="get_cursor_position_and_color()"
			title="同时获取鼠标位置和颜色/秒"
		>
			<span v-if="theSwitch != 'get_cursor_position_and_color'">
				{{
					store.lang === "en"
						? "get mouse position and color"
						: "获取指针位置和颜色"
				}}
			</span>
			<span v-if="theSwitch == 'get_cursor_position_and_color'">
				{{ store.lang === "en" ? "Stop !" : "停止 !" }}
			</span>
		</button>
	</div>
</template>

<style scoped>
	#Profiles {
		padding: 5px;
		min-width: 350px;
		text-align: left;
		display: flex;
		flex-direction: column;
		justify-content: flex-start;
	}
	button {
		width: 100%;
	}
	.getCursorColor {
		margin: 20px 0;
		border-radius: 8px;
		border: 1px solid transparent;
		font-size: 1em;
		font-weight: 500;
		font-family: inherit;
		color: #ffffff;
		background-color: #0f0f0f98;
		transition: border-color 0.25s;
		box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
	}
	.getCursorColor div {
		padding: 0 5px;
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;
	}
	input {
		margin: 10px;
	}
</style>
