//Responsible for generating slots that satisfy specific after_time/before_time time bounds,
//between a certain time period.
//For a visual step-by-step breakdown of the scheduler algorithm see https://docs.google.com/presentation/d/1Tj0Bg6v_NVkS8mpa-aRtbDQXM-WFkb3MloWuouhTnAM/edit?usp=sharing

use crate::repetition::{self, Repetition};
use crate::slot::Slot;
use crate::time_slot_iterator::TimeSlotIterator;
use chrono::{Duration, Timelike};

pub fn slot_generator(
    after_time: Option<usize>,
    before_time: Option<usize>,
    time_period: &Slot,
    repetition: Option<Repetition>,
) -> Vec<Slot> {
    if after_time.is_none() && before_time.is_none() {
        return vec![Slot {
            start: time_period.start,
            end: time_period.end,
        }];
    }

    //special case for 'every-x-hours' repetition: return the time_period as is
    if let Some(Repetition::EveryXhours(_)) = repetition {
        return vec![Slot {
            start: time_period.start,
            end: time_period.end,
        }];
    }

    let after_time = after_time.unwrap_or(0);
    let before_time = before_time.unwrap_or(24);

    //slides 2 - 7 (assign slots to tasks)
    //iterate through each hour in the time_period.
    //when we find an hour equal to the after_time, assign a slot starting from there up to the before_time of the task.
    //e.g. if the time_period is a day and aftertime is 10 and before time is 14,
    //we'll get to 10 and add a slot starting from 10 up until 14.
    let hour_iterator = TimeSlotIterator {
        start: time_period.start,
        end: time_period.end,
        repetition: Some(Repetition::HOURLY),
    };

    let mut slots: Vec<Slot> = Vec::new();
    let hours: Vec<Slot> = hour_iterator.collect();
    let mut i = 0;
    while i < hours.len() {
        if hours[i].start.hour() != after_time as u32 {
            i += 1;
            continue;
        }
        let num_of_slots = size_of_slots_to_be_assigned(after_time, before_time);
        let mut slot = assign_slots(num_of_slots, &hours, &mut i);
        //handle cases where beforetime is on the next day e.g. sleep 22-6
        if before_time < after_time {
            slot.end += Duration::hours(before_time as i64);
        }
        slots.push(slot);
        i += 1;
    }

    slots
}

fn assign_slots(num_of_slots: usize, hours: &Vec<Slot>, i: &mut usize) -> Slot {
    let start = hours[*i];
    let mut end = hours[*i];
    for _ in 1..num_of_slots as usize {
        if *i < hours.len() - 1 {
            *i += 1;
            end = hours[*i];
        } else {
            break;
        }
    }

    Slot {
        start: start.start,
        end: end.end,
    }
}

fn size_of_slots_to_be_assigned(after_time: usize, before_time: usize) -> usize {
    if before_time > after_time {
        before_time - after_time
    } else {
        before_time + (24 - after_time)
    }
}
