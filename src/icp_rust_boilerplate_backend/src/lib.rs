#[macro_use]
extern crate serde;

mod course_management;
mod student_management;

use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

pub use course_management::Course;
pub use course_management::create_course;
pub use course_management::get_courses_with_available_slots;
pub use course_management::get_full_capacity_courses;
pub use course_management::get_courses_by_instructor;
pub use course_management::get_average_enrollment_rate;
pub use course_management::do_insert_course;
pub use student_management::enroll_student;


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



ic_cdk::export_candid!();
