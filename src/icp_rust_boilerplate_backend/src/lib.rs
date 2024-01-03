#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct Course {
    id: u64,
    name: String,
    instructor: String,
    capacity: u64,
    enrolled_students: u64,
}

impl Storable for Course {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Course {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
    static COURSE_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );
    static COURSE_STORAGE: RefCell<StableBTreeMap<u64, Course, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );
}

#[derive(candid::CandidType, Serialize, Deserialize)]
struct CoursePayload {
    name: String,
    instructor: String,
    capacity: u64,
}

#[ic_cdk::update]
fn create_course(course_payload: CoursePayload) -> Result<Course, String> {
    let id = COURSE_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    if course_payload.name.is_empty()
        || course_payload.instructor.is_empty()
        || course_payload.capacity == 0
    {
        return Err("Invalid course payload, Fill in the fields".to_string());
    }

    let course = Course {
        id,
        name: course_payload.name,
        instructor: course_payload.instructor,
        capacity: course_payload.capacity,
        enrolled_students: 0,
    };
    do_insert_course(&course);
    Ok(course)
}

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

#[ic_cdk::query]
fn get_courses_with_available_slots() -> Vec<Course> {
    COURSE_STORAGE.with(|c| {
        c.borrow()
            .iter()
            .filter(|(_, course)| course.enrolled_students < course.capacity)
            .map(|(_, course)| course.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn get_full_capacity_courses() -> Vec<Course> {
    COURSE_STORAGE.with(|c| {
        c.borrow()
            .iter()
            .filter(|(_, course)| course.enrolled_students >= course.capacity)
            .map(|(_, course)| course.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn get_courses_by_instructor(instructor_name: String) -> Vec<Course> {
    COURSE_STORAGE.with(|c| {
        c.borrow()
            .iter()
            .filter(|(_, course)| course.instructor == instructor_name)
            .map(|(_, course)| course.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn get_average_enrollment_rate() -> f64 {
    let total_courses = COURSE_STORAGE.with(|c| c.borrow().len() as f64);
    let total_enrolled_students: f64 = COURSE_STORAGE.with(|c| {
        c.borrow()
            .iter()
            .map(|(_, course)| course.enrolled_students as f64)
            .sum()
    });

    if total_courses == 0.0 {
        0.0
    } else {
        total_enrolled_students / total_courses
    }
}

fn do_insert_course(course: &Course) {
    COURSE_STORAGE.with(|s| {
        s.borrow_mut().insert(course.id, course.clone());
    });
}

ic_cdk::export_candid!();
