use candid::{CandidType, Deserialize, Serialize};
use ic_cdk::api::time;
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable, VirtualMemory};
use std::{borrow::Cow, cell::RefCell};
use crate::course_management::Course;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type IdCell = Cell<u64, Memory>;

// Other functions and data structures related to student management go here...
#[ic_cdk::update]
fn enroll_student(course_id: u64) -> Result<Course, String> {
    let course = COURSE_STORAGE.with(|course| {
        course
            .borrow_mut()
            .get(&course_id)
            .ok_or_else(|| "Course not found".to_string())
    })?;

    if course.enrolled_students >= course.capacity {
        return Err("Course is already at full capacity".to_string());
    }

    let updated_course = Course {
        enrolled_students: course.enrolled_students + 1,
        ..course.clone()
    };
    do_insert_course(&updated_course);
    Ok(updated_course)
}