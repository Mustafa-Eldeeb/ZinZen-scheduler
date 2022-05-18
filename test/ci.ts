import { loadAPI, DateTime } from "../ts/api.ts";
import { assertEquals as _assert } from "https://deno.land/std@0.140.0/testing/asserts.ts";

// Load the API
const API = loadAPI("https://github.com/tijlleenders/ZinZen-scheduler/raw/main/ts/scheduler.wasm");

// Load JSON
const json = Deno.readTextFile("test/goals.json");

Deno.test("API.processTaskCount", () => {
	Promise.all([API, json]).then(([API, json]) => {
		const goals = JSON.parse(json);

		const start = [2019, 150, 0, 0, 0, 0] as DateTime;
		const finish = [2019, 170, 0, 0, 0, 0] as DateTime;

		const taskCounts = API.processTaskCount(goals, start, finish);
		// TODO: Insert assertions here

		const schedule = API.generateSchedule(goals, start, finish).sort((a, b) => a.flexibility - b.flexibility);
		// TODO: Insert assertions here
	}).catch(console.error);
});