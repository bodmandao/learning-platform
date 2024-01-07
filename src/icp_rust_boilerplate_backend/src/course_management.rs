use candid::{CandidType, Deserialize, Serialize};
use ic_cdk::api::time;
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable, VirtualMemory};
use std::{borrow::Cow, cell::RefCell};

pub type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type IdCell = Cell<u64, Memory>;

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct Course {
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

    if course_payload.name.is_empty() {
        return Err("Name field cannot be empty.".to_string());
    }
    if course_payload.instructor.is_empty() {
        return Err("Instructor field cannot be empty.".to_string());
    }
    if course_payload.capacity == 0 {
        return Err("Capacity must be greater than 0.".to_string());
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
