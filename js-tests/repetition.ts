import {schedule} from "../js-api/scheduler.js"
import {assertEquals, assertThrows} from "https://deno.land/std@0.141.0/testing/asserts.ts";

Deno.test("can duplicate with daily repetition", () => {
    assertEquals(
        schedule({
            "startDate": "2022-01-01T00:00:00Z",
            "endDate": "2022-01-04T00:00:00Z",
            "goals": [
                {
                    "id": 1,
                    "title": "walk",
                    "duration": 1,
                    "repetition": "daily"
                },
            ]
        }),
        {
            "tasks": [
                {
                    "id": 0,
                    "goal_id": 1,
                    "duration_to_schedule": 0,
                    "duration_scheduled": 1,
                    "status": "SCHEDULED"
                },
                {
                    "id": 1,
                    "goal_id": 1,
                    "duration_to_schedule": 0,
                    "duration_scheduled": 1,
                    "status": "SCHEDULED"
                },
                {
                    "id": 2,
                    "goal_id": 1,
                    "duration_to_schedule": 0,
                    "duration_scheduled": 1,
                    "status": "SCHEDULED"
                }
            ],
            "slots": [
                {
                    "task_id": 0,
                    "start": 0,
                    "end": 1
                },
                {
                    "task_id": 1,
                    "start": 24,
                    "end": 25,
                },
                {
                    "task_id": 2,
                    "start": 48,
                    "end": 49,
                }
            ]
        }
    )
});

// TODO should check the error for this test, see issue #26,
// if it is fixed please change this test to check the error
Deno.test("invalid repetition", () => {
    assertThrows(
        () => schedule({
            "startDate": "2022-01-01T00:00:00Z",
            "endDate": "2022-01-02T00:00:00Z",
            "goals": [
                {
                    "id": 1,
                    "title": "shopping",
                    "duration": 1,
                    "start": "2022-01-01T10:00:00Z",
                    "deadline": "2022-01-01T13:00:00Z",
                    "repetition": "invalid-value-AAAAAA"
                },
            ]
        })
    )
});
