<!--//////////////////////////////////////////////////// template ////////////////////////////////////////////////////-->
<!--//////////////////////////////////////////////////// template ////////////////////////////////////////////////////-->
<template>
	<div id="Details">
		<div class="sticky-ctn">
			<!-- Disclaimer -->
			<Disclaimer
				:agreeDisclaimer="agreeDisclaimer"
				@agreeToDisclaimer="agreeToDisclaimer"
			/>
			<div class="function-div">
				<!-- functional-buttons -->
				<TopFuncBtns
					:startButton="startButton"
					:intervalTime="intervalTime"
					@start_or_stop="start_or_stop"
					@toggleDisabledRules="toggleDisabledRules"
					@save_bot_settings="save_bot_settings"
					@change_lang="change_lang"
				/>
				<!-- tabs -->
				<Tabs
					:tabs="tabs"
					:currentTab="currentTab"
					@addNewTab="addNewTab"
					@changeTab="changeTab"
					@deleteTab="deleteTab"
				/>
			</div>
		</div>
		<!-- //////////////////////////////////////////////////// saved rules //////////////////////////////////////////////////// -->
		<SavedRules
			:currentTabName="tabs[currentTab]"
			:savedRules="savedRules"
			:hideDisabled="hideDisabled"
			@fold_this_rule="fold_this_rule"
			@copy_to_new_rule="copy_to_new_rule"
			@toggleEnable="toggleEnable"
			@delete_this_rule="delete_this_rule"
			@addConditionToNewRule="addConditionToNewRule"
		/>

		<!-- ////////////////////////////////////////////////////add rule //////////////////////////////////////////////////////////-->
		<AddNewRule
			:newRule="newRule"
			:showNewRule="showNewRule"
			@add_rule="add_rule"
			@delete_condition="delete_condition"
			@add_condition="add_condition"
			@delete_action="delete_action"
			@add_action="add_action"
			@comfirm_new_rule="comfirm_new_rule"
		/>
	</div>
</template>

<script setup lang="ts">
	import { confirm } from "@tauri-apps/api/dialog";
	import { computed, watch, ref, reactive } from "vue";
	import { invoke } from "@tauri-apps/api/tauri";
	import { store } from "./store.js";
	import { onMounted } from "vue";
	import { registerAll, unregisterAll } from "@tauri-apps/api/globalShortcut";
	import { exit } from "@tauri-apps/api/process";
	import Disclaimer from "./Disclaimer.vue";
	import Embed from "./Embed.vue";
	import TopFuncBtns from "./TopFuncBtns.vue";
	import SavedRules from "./SavedRules.vue";
	import AddNewRule from "./AddNewRule.vue";
	import Tabs from "./Tabs.vue";
	import { Action, Condition, Rule } from "./types";
	//////////
	////////////////////////////////////////////////////// varibles ////////////////////////////////////////////////////
	////////////////////////////////////////////////////// varibles ////////////////////////////////////////////////////
	//press and hold keys 激活的长按按键[]
	const holdingRules = ref<number[]>([]);
	// 当前标签页
	const currentTab = ref(0);
	// 所有标签页
	const tabs = ref([""]);

	watch(
		tabs,
		(newVal) => {
			console.debug(`tabs new value ${tabs}`);
			localStorage.setItem("localTabs", JSON.stringify(tabs.value));
		},
		{ deep: true }
	);
	//是否在执行长按命令
	const doingActions = ref(false);
	//开始按钮文字
	const startButton = ref("Start");
	//动作间隔时间
	const intervalTime = ref({ t: 150 });
	//默认规则
	const aDefaultCondition: Condition = {
		matchPoint: [0, 0], //检测的像素点
		matchResult: "Match", //是否要匹配
		matchColor: [0, 0, 0], //rgb颜色
	};
	const aDefaultAction: Action = {
		actionType: "Move Mouse", //Move Mouse(relative to image location,absolute),Press(Key, Special Key),Send Text
		moveType: "relative",
		coor: [0, 0],
		keyType: "0-9,a-z",
		key: "0",
		specialKey: "Ctrl",
		pressDuration: 0,
		pressAndHold: false,
		text: "",
		delay: 0,
	};
	const aDefaultRule: Rule = {
		conditions: [aDefaultCondition], //多种条件[]
		actions: [aDefaultAction], //多种动作[]
		note: "note", //备注
		sleepAfterFinish: false, //动作全部完成后休眠
		sleepTime: 1000,
		fold: false, //默认不折叠显示
		active: true, //默认规则激活,检查conditions前设为false，条件不符合后设为true,actions 都做完后settimeout( 最长delay )设为true
		enabled: true,
		tab: "",
	};
	//初始化保存的规则
	const savedRules = ref([aDefaultRule]);
	//初始化新规则
	const newRule = ref(aDefaultRule);
	//初始不显示新规则
	const showNewRule = ref({ val: false });
	const hideDisabled = ref(false);
	const agreeDisclaimer = ref<boolean>(
		JSON.parse(localStorage.getItem("localAgreement") || "false")
	);
	//watcher
	// watch works directly on a ref
	watch(
		savedRules,
		(newConf) => {
			console.debug(`watch rules ${newConf}`);
			save_bot_settings();
		},
		{ deep: true }
	);
	watch(
		agreeDisclaimer,
		(newVal) => {
			console.debug(`agreeDisclaimer new value ${newVal}`);
			localStorage.setItem("localAgreement", JSON.stringify(newVal));
		},
		{ deep: true }
	);

	////////////////////////////////////////////////////// functions ////////////////////////////////////////////////////
	////////////////////////////////////////////////////// functions ////////////////////////////////////////////////////
	function cloneObj(obj: Object) {
		return JSON.parse(JSON.stringify(obj));
	}
	async function deleteTab() {
		let tabName = tabs.value[currentTab.value];
		console.debug(tabName);
		const confirmed = await confirm(
			`${
				store.lang === "en"
					? "Delete rules of current tab permanently?"
					: "永久删除当前标签页的所有规则？"
			}`,
			`${store.lang === "en" ? "Confirm delete" : "确认删除"}`
		);
		console.debug(`confirmed => ${confirmed}`);
		if (confirmed && currentTab.value === 0) {
			savedRules.value = [];
			tabs.value = [""];
		}
		if (confirmed && currentTab.value !== 0) {
			let newSavedRule: Rule[] = [];
			for (const rule of savedRules.value) {
				if (rule.tab !== tabName) {
					newSavedRule.push(rule);
				}
			}
			savedRules.value = cloneObj(newSavedRule);
			//delete current tabname in tabs
			tabs.value.splice(currentTab.value, 1);
			//change current tab --
			currentTab.value--;
		}
	}

	async function holdedKeyUp(r: Rule) {
		for (const action of r.actions) {
			// 取消长按普通按键
			if (
				action.actionType === "Press key" &&
				action.pressAndHold == true &&
				action.keyType === "0-9,a-z"
			) {
				try {
					await invoke("normal_key_up", { key: action.key });
					store.add_log(
						store.lang === "en"
							? `key ${action.key} up`
							: `松开 ${action.key} 键`
					);
				} catch (error) {
					store.add_log(`${error}`);
				}
			}
			//取消长按特殊按键
			if (
				action.actionType === "Press key" &&
				action.pressAndHold == true &&
				action.keyType !== "0-9,a-z"
			) {
				try {
					await invoke("special_key_up", { key: action.specialKey });
					store.add_log(
						store.lang === "en"
							? `key ${action.specialKey} up`
							: `松开 ${action.specialKey} 键`
					);
				} catch (error) {
					store.add_log(`${error}`);
				}
			}
		}
	}

	function ruleHasPressAndHoldAction(r: Rule) {
		for (const action of r.actions) {
			if (action.pressAndHold == true) {
				return true;
			}
		}
		return false;
	}
	function addConditionToNewRule(cdt: Condition) {
		console.debug(cdt);
		newRule.value.conditions.push(cloneObj(cdt));
		showNewRule.value.val = true;
		plainTextLog(
			`${store.lang === "en" ? "Added to new rule" : "已添加到新规则中"}`
		);
	}
	function changeTab(i: number) {
		currentTab.value = i;
	}
	//纯文本log
	function plainTextLog(log: string) {
		store.add_log(log);
	}
	//添加新tab，先检查有没有重复！
	function addNewTab(newTabName: string) {
		if (tabs.value.includes(newTabName)) {
			plainTextLog(store.lang === "en" ? "tab name exited" : "标签已存在");
		} else {
			tabs.value.push(cloneObj(newTabName));
		}
	}
	function agreeToDisclaimer() {
		agreeDisclaimer.value = true;
	}
	//获取时间
	function getTimeNow() {
		// create a new Date object
		let date = new Date();
		// get the hours, minutes and seconds from the date object
		let hours = date.getHours();
		let minutes = date.getMinutes();
		let seconds = date.getSeconds();
		// pad with leading zeros if needed
		let hoursStr = hours < 10 ? "0" + hours.toString() : hours;
		let minutesStr = minutes < 10 ? "0" + minutes.toString() : minutes;
		let secondsStr = seconds < 10 ? "0" + seconds.toString() : seconds;
		// return the time as a string
		return hoursStr + ":" + minutesStr + ":" + secondsStr;
	}
	async function delete_this_rule(i: number) {
		//删除一条rule
		// store.add_log(`Deleted one rule`);
		savedRules.value.splice(i, 1);
		//保存rules到本地文件
		save_bot_settings();
	}
	function copy_to_new_rule(i: number) {
		let deepCopyRule = JSON.parse(JSON.stringify(savedRules.value[i]));
		newRule.value = deepCopyRule;
		showNewRule.value.val = true;
	}
	function toggleEnable(i: number) {
		savedRules.value[i].enabled = !savedRules.value[i].enabled;
		savedRules.value[i].fold = !savedRules.value[i].enabled;
		save_bot_settings();
	}
	function fold_this_rule(i: number) {
		savedRules.value[i].fold = !savedRules.value[i].fold;
	}
	function toggleDisabledRules() {
		hideDisabled.value = !hideDisabled.value;
	}
	function add_rule() {
		//显示增加规则div
		showNewRule.value.val = !showNewRule.value.val;
		// store.add_log(`显示增加规则`);
	}
	// 新规则中添加条件
	function add_condition() {
		newRule.value.conditions.push({
			matchPoint: [0, 0],
			matchResult: "Match",
			matchColor: [0, 0, 0],
		});
	}
	function add_action() {
		newRule.value.actions.sort((a, b) => {
			return a.delay - b.delay; // ascending order
		});
		newRule.value.actions.push(JSON.parse(JSON.stringify(aDefaultAction)));
	}
	// 新规则中删除条件
	function delete_condition(idx: number) {
		newRule.value.conditions.splice(idx, 1);
	}
	function delete_action(idx: number) {
		newRule.value.actions.splice(idx, 1);
	}
	async function comfirm_new_rule() {
		//先验证数据是否可靠，特别是按键,不通过则log
		newRule.value.actions.forEach((action) => {
			action.delay = Math.abs(action.delay);
		});
		newRule.value.actions.sort((a, b) => {
			return a.delay - b.delay; // ascending order
		});
		const aRule = newRule.value;
		aRule.enabled = true; //添加后默认为enabled
		aRule.active = true; //添加后默认为avtive
		aRule.tab = tabs.value[currentTab.value]; //tab 设置为现在的tab
		savedRules.value.push(aRule);
		// store.add_log(`Add one rule`);
		//clone newRule，不然最后一条savedRule会跟着变
		newRule.value = cloneObj(newRule.value);
		//保存rules到本地文件
		save_bot_settings();
		showNewRule.value.val = false;
	}
	function color_diff(colorA: number[], colorB: number[]): number {
		let [ra, ga, ba] = colorA;
		let [rb, gb, bb] = colorB;
		console.debug(ra, ga, ba, rb, gb, bb);
		let diffR = Math.abs((ra - rb) / 255);
		let diffG = Math.abs((ga - gb) / 255);
		let diffB = Math.abs((ba - bb) / 255);
		console.debug("rough color diff ::", (diffB + diffG + diffR) / 3);
		return (diffB + diffG + diffR) / 3;
	}
	// 检查条件是否通过，通过 return true
	//通过rust返回像素颜色，再看匹配否
	async function checkConditions(aRule: Rule) {
		if (startButton.value === "Stop") {
			console.debug("checkConditions,rule", aRule);
			const conditions = aRule.conditions;
			console.debug("checkConditions,conditions", conditions);
			//检查conditions中的规则有没有匹配到
			for (const c of conditions) {
				console.debug("c:", c);
				let matchPoint = c.matchPoint;
				let matchColor = Array.from(c.matchColor);
				console.debug("matchColor:", matchColor);
				let pixelColor: number[] = await invoke("get_pixel_color", {
					x: matchPoint[0],
					y: matchPoint[1],
				});
				// store.add_log(`${pixelColor.toString() === matchColor.toString()}`);
				console.debug("pointColor:", pixelColor);
				const colorDiff = color_diff(pixelColor, matchColor);
				let checkCondition = colorDiff <= 0.01; //roughly same
				// let checkCondition = pixelColor.toString() === matchColor.toString();
				if (checkCondition === (c.matchResult === "Match")) {
					console.debug("1 color matched ");
					// store.add_log(`${getTimeNow()}->${matchPoint}::${pixelColor} ✔`);
				} else {
					console.debug("1 color not matched ");
					// store.add_log(`${getTimeNow()}->${matchPoint}::${pixelColor} ✗`);
					return false;
				}
			}
			return true;
		} else {
			return false;
		}
	}
	function get_longest_delay(rule: Rule) {
		const actions = rule.actions;
		let longestDelay: number = 0;
		for (const action of actions) {
			if (action.actionType === "Press key" || action.actionType === "按键") {
				let delay = action.delay + action.pressDuration;
				if (delay > longestDelay) {
					longestDelay = delay;
				}
			} else {
				if (action.delay > longestDelay) {
					longestDelay = action.delay;
				}
			}
		}
		return longestDelay;
	}
	//执行动作
	async function doAction(aRule: Rule) {
		//如果动作有 long press ，等执行完再做其他rule的动作，因为可能导致其他rule动作变形
		doingActions.value = true;
		if (store.lang === "en") {
			store.add_log(`do action of rule: ${aRule.note}`);
		} else {
			store.add_log(`执行规则 ${aRule.note} 的动作 `);
		}
		console.debug("rule", aRule);
		console.debug("actions", aRule.actions);
		const longestDelay = get_longest_delay(aRule);
		console.debug("longestDelay:", longestDelay);
		setTimeout(() => {
			doingActions.value = false;
		}, longestDelay);
		//foreach
		//for each 做动作
		let actions = aRule.actions;
		actions.forEach((action) => {
			const actionType = action.actionType;
			//根据antionType来 invoke rust 中对应的执行动作
			if (actionType === "Move Mouse" || actionType === "移动鼠标") {
				//移动鼠标位置
				setTimeout(async () => {
					store.add_log(getTimeNow());
					try {
						await invoke("move_mouse", {
							x: action.coor[0],
							y: action.coor[1],
						});
						store.add_log(
							store.lang === "en"
								? `move mouse to ${action.coor}`
								: `移动鼠标至 ${action.coor}`
						);
					} catch (error) {
						store.add_log(`${error}`);
					}
				}, action.delay);
			} else if (actionType === "Send Text" || actionType === "发送文字") {
				//发送文字
				store.add_log(getTimeNow());
				setTimeout(async () => {
					try {
						await invoke("input_text", { text: action.text });
						store.add_log(
							store.lang === "en"
								? `send text: ${action.text}`
								: `发送文字: ${action.text}`
						);
					} catch (error) {
						store.add_log(`${error}`);
					}
				}, action.delay);
			} else {
				// 按键,0-9,a-z||特殊按键
				store.add_log(getTimeNow());
				if (action.keyType === "0-9,a-z") {
					//0-9,a-z
					setTimeout(async () => {
						try {
							await invoke("normal_key_down", { key: action.key });
							store.add_log(
								store.lang === "en"
									? `key ${action.key} down`
									: `按下 ${action.key} 键`
							);
						} catch (error) {
							store.add_log(`${error}`);
						}
					}, action.delay);
					if (!action.pressAndHold) {
						//如果不是hold动作，过delay + pressDuration 时间后松开
						setTimeout(async () => {
							try {
								await invoke("normal_key_up", { key: action.key });
								store.add_log(
									store.lang === "en"
										? `key ${action.key} up`
										: `松开 ${action.key} 键`
								);
							} catch (error) {
								store.add_log(`${error}`);
							}
						}, action.delay + action.pressDuration);
					}
				} else {
					store.add_log(getTimeNow());
					//特殊按键 special key
					setTimeout(async () => {
						try {
							await invoke("special_key_down", { key: action.specialKey });
							store.add_log(
								store.lang === "en"
									? `key ${action.specialKey} down`
									: `按下 ${action.specialKey} 键`
							);
						} catch (error) {
							store.add_log(`${error}`);
						}
					}, action.delay);

					if (!action.pressAndHold) {
						//如果不是hold动作，过delay + pressDuration 时间后松开
						setTimeout(async () => {
							try {
								await invoke("special_key_up", { key: action.specialKey });
								store.add_log(
									store.lang === "en"
										? `key ${action.specialKey} up`
										: `松开 ${action.specialKey} 键`
								);
							} catch (error) {
								store.add_log(`${error}`);
							}
						}, action.delay + action.pressDuration);
					}
				}
			}
		});
	}
	//test
	// async function testFunction() {
	// }
	//开始bot 按钮，
	//每隔100毫秒执行所有rule
	async function start_or_stop() {
		holdingRules.value = [];
		// for (const rule of savedRules.value) {
		// 	holdedKeyUp(rule);
		// }
		if (!agreeDisclaimer.value) {
			console.debug(`agreeDisclaimer::${agreeDisclaimer}`);
			store.add_log(
				store.lang === "en"
					? "Agree to disclaimer before use"
					: "同意免责声明后使用"
			);
			return;
		}
		if (startButton.value === "Start" && savedRules.value.length > 0) {
			store.add_log(`alt + p (option + p on mac) to stop...`);
			startButton.value = "Stop";
			//检查用户设置的时间间隔
			if (intervalTime.value.t < 10) {
				intervalTime.value.t = 10;
			}
			//挑选出激活的rule && tab筛选 过滤
			let enabledRules: Rule[] = [];
			for (const rule of savedRules.value) {
				if (!rule.enabled) {
					continue;
				}
				// tab筛选 过滤
				if (currentTab.value === 0) {
					//所有tab时候
					enabledRules.push(rule);
				}
				if (tabs.value[currentTab.value] === rule.tab) {
					enabledRules.push(rule);
				}
			}
			enabledRules = cloneObj(enabledRules);
			let botting = setInterval(
				async () => {
					//如果已经按了Stop按钮，清除botting
					if (startButton.value != "Stop") {
						clearInterval(botting);
						store.add_log("Stop!!");
					}
					enabledRules.forEach(async (rule, index) => {
						// for (const rule of enabledRules) {
						console.debug("rule", rule);
						//如果rule是active状态，检查rule
						// if (!doingActions.value) {
						if (rule.active) {
							rule.active = false;
							const conditions = await checkConditions(rule);
							//符合条件，并且有hold动作的rule index 加入到 holdingRules中
							if (conditions && ruleHasPressAndHoldAction(rule)) {
								holdingRules.value.push(index);
							}
							//不符合条件，并且有hold动作的rule  从 holdingRules 中删除掉 index
							if (
								!conditions &&
								ruleHasPressAndHoldAction(rule) &&
								holdingRules.value.includes(index)
							) {
								const newHoldingRules = holdingRules.value.filter(
									(value) => value !== index
								);
								holdingRules.value = newHoldingRules;
								holdedKeyUp(rule);
							}
							//符合条件的执行动作
							if (conditions) {
								if (rule.sleepAfterFinish) {
									setTimeout(() => {
										rule.active = true;
									}, rule.sleepTime + get_longest_delay(rule));
								} else {
									rule.active = true;
								}
								// await doAction(rule);
								doAction(rule);
							} else {
								//不符合条件的
								rule.active = true;
							}
						}
					});
				},
				intervalTime.value.t,
				true
			);
		} else {
			startButton.value = "Start";
		}
	}
	//获取app数据本地储存的位置 await appLocalDataDir()
	// async function getBotSettingPath() {
	// 	const appLocalDataDirPath = await appLocalDataDir();
	// 	console.debug(appLocalDataDirPath + "app.conf");
	// 	return appLocalDataDirPath + "app.conf";
	// }
	//保存当前版本saved rules到本地文件
	function save_bot_settings() {
		try {
			//保存当前版本saved rules到本地文件
			let settings = {
				v050: savedRules.value,
				lang: store.lang,
				intervalTime: intervalTime.value.t,
			};
			//当前版本的设置 v,每次版本更新时候改版本号
			let savedRulesString = JSON.stringify(settings);
			localStorage.setItem("conf", savedRulesString);
			// await writeTextFile(await getBotSettingPath(), savedRulesString);
			// await writeTextFile("app.conf", savedRulesString, {
			// 	dir: BaseDirectory.AppData,
			// });
			store.add_log(`Saving settings...`);
		} catch (err) {
			store.add_log(`${err}`);
			store.add_log(`Save settings -> error`);
		}
	}
	//获取本地规则,当前版本
	function get_local_settings() {
		//载入本地配置，没有配置文件就新建文件
		try {
			//读取文件
			// let botSettingStr = await readTextFile(await getBotSettingPath());
			let conf = localStorage.getItem("conf");
			let botSettingStr: string = conf !== null ? conf : "";
			//json化
			const botSetting = JSON.parse(botSettingStr);
			console.debug(botSetting);
			console.debug(botSetting.v050);
			if (botSetting.v050 === undefined) {
				save_bot_settings();
			}
			if (botSetting.lang === undefined) {
				store.lang = "en";
				botSetting.lang = "en";
				save_bot_settings();
			}
			//加载到savedRules中
			store.add_log(`Loading local rules...`);
			savedRules.value = botSetting.v050;
			store.lang = botSetting.lang;
			intervalTime.value.t = botSetting.intervalTime;
			store.add_log(` Ok!`);
		} catch (err) {
			//debug错误信息
			console.debug(err);
			// store.add_log(`${err}`);
			//初始化app设置
			store.add_log(`Init rules ...`);
			save_bot_settings();
		}
		//读取本地tab
		try {
			let localTabs = localStorage.getItem("localTabs");
			if (localTabs === null) {
				plainTextLog(`localTabs ===  null`);
				setTimeout(() => {
					localStorage.setItem("localTabs", JSON.stringify(tabs.value));
				}, 200);
			} else {
				tabs.value = JSON.parse(localTabs);
			}
		} catch (error) {}
	}
	function change_lang() {
		store.lang = store.lang === "en" ? "中" : "en";
		save_bot_settings();
		// store.add_log(`${store.lang}`);
	}
	//Lifecycle
	//Lifecycle
	onMounted(async () => {
		// test
		// testFunction();
		get_local_settings();
		store.add_log(`alt + p (option + p on mac) to start / stop bot`);
		store.add_log(`${store.lang}`);
		await unregisterAll();
		await registerAll(["Alt+P", "Option+P"], (shortcut) => {
			// store.add_log(`Bot stopped by shortcut ${shortcut}`);
			// startButton.value = "Start";
			start_or_stop();
		});
	});
</script>

<!--//////////////////////////////////////////////////// style ////////////////////////////////////////////////////-->
<!--//////////////////////////////////////////////////// style ////////////////////////////////////////////////////-->
<style scoped>
	#Details {
		box-sizing: border-box;
		padding: 5px;
		min-width: 900px;
		text-align: left;
		display: flex;
		flex-direction: column;
		justify-content: flex-start;
	}
	.sticky-ctn {
		position: sticky;
		top: 0;
		background-color: #2f2f2f;
	}
	.function-div {
		backdrop-filter: blur(10px);
		background-color: rgba(20, 20, 20, 0.5);
		padding: 5px;
		border-radius: 5px;
		margin-bottom: 5px;
	}
</style>
