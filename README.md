# Course Contract

This Rust-based Internet Computer (IC) canister provides functionality for managing courses. It allows you to create, update, retrieve, and delete course information. The contract is built using the DFINITY Canister SDK.

## Prerequisites

Ensure that you have the DFINITY Canister SDK installed. If not, you can find installation instructions [here](https://sdk.dfinity.org/docs/quickstart/quickstart-intro.html).

## Usage

### Core Functions 

1. `create_course`: Create a new course on the platform.
2. `edit_course`: Edit existing courses (title, description, instructor).
3. `delete_course`: Delete a course from the platform.
4. `get_course_by_id`: Retrieve a specific course by its unique ID.
5. `get_all_courses`: Retrieve all courses available on the platform.
6. `create_user`: Create a new user account on the platform.
7. `edit_user`: Edit user details (name, email).
8. `delete_user`: Delete a user account from the platform.
9. `enroll_in_course`: Enroll in a specific course.

### Query Functions

1. `get_user_by_id`: Retrieve user information by ID.
2. `get_all_users`: Retrieve all users on the platform.
3. `get_enrolled_courses`: Retrieve courses enrolled by a specific user.
4. `get_instructor_courses`: Retrieve courses taught by a specific instructor.

### Error Handling

- **Error:** Enum for handling various error scenarios, including not found, invalid payload, unauthorized access, and enrollment limits.

### Candid Interface

- Exported Candid interface for seamless interaction with the Internet Computer.

## More

To get started, explore the project directory structure and the default configuration file. For detailed Rust programming language documentation, refer to [Rust Documentation](https://doc.rust-lang.org/book/).

For Internet Computer-specific information:

- [Internet Computer SDK Developer Tools](https://sdk.dfinity.org/docs/quick-start/quick-start-intro.html)
- [IC-CDK Documentation](https://github.com/dfinity/cdk-rs)
- [Candid Introduction](https://sdk.dfinity.org/docs/candid-guide/candid-intro.html)



