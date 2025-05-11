# 📚 Project READMEs

## 1. ⏱️ Date Difference Calculator: Simple Date Comparison Tool in Rust

This Rust application, `Date Difference Calculator`, provides a simple command-line utility to calculate the exact number of days between two dates. It uses the `chrono` crate for date parsing and calculation.

### 🔑 Key Features
- ✅ Takes input dates in YYYY-MM-DD format
- ✅ Automatically handles date order (earlier/later doesn't matter)
- ✅ Displays the exact number of days between the dates

### 📋 Example Output
```
Enter Two dates in YYYYMMDD format
Enter the First Date: 2023-05-10
Enter the Second Date: 2023-12-25
There is difference of 229 days
```

### 🚀 Usage
1. Run the program
2. Enter two dates when prompted (YYYY-MM-DD format)
3. View the difference in days between the dates

### 🔄 Dependencies
- 📦 chrono: For date parsing and calculation

---

## 2. ✅ Todo CLI: Task Management System in Rust

> Here is a sample code [url](https://github.com/BiMalxMe/Rust/blob/main/src/Practices/TodoCLI.rs)

This Rust application, `Todo CLI`, is a command-line task manager that helps you track your to-do items with deadlines. It offers a simple but effective interface for managing daily tasks.

### 🔑 Key Features
- ✅ Add new tasks with title, description, and deadline
- ✅ List all tasks with their details
- ✅ Mark tasks as complete
- ✅ Simple menu-driven interface

### 📋 Example Output

![Screen Shot 2025-05-10 at 05 36 35](https://github.com/user-attachments/assets/9aea7c08-8003-4c59-a91d-55527fcc99d1)


### 🚀 Usage
The application provides a menu with the following options:
1. Add Task - Create a new task
2. List Tasks - View all tasks
3. Mark Task as Complete - Update task status 
4. Exit - Close the application

### 🔄 Dependencies
- 📦 chrono: For date handling
- 📦 Standard Rust libraries for I/O

---

## 3. 📝 Logs Generator: Simple Asynchronous Log File Writer in Rust
> Here is a sample code [url](https://github.com/BiMalxMe/Rust/blob/main/src/Practices/LogsGenerator.rs)


[![Screen Shot 2025-05-09 at 12 39 54](https://github.com/user-attachments/assets/75432324-807e-444d-ab79-e71643b63a9b)](https://github.com/user-attachments/assets/75432324-807e-444d-ab79-e71643b63a9b)
[![Screen Shot 2025-05-09 at 12 39 48](https://github.com/user-attachments/assets/fc495531-8430-47ec-96e5-8682e0460efb)](https://github.com/user-attachments/assets/fc495531-8430-47ec-96e5-8682e0460efb)

This Rust application, `LogsGenerator`, demonstrates a basic asynchronous file writing process using the `tokio` runtime. It concurrently writes simple messages to two separate files: `main.txt` and `log.log`. The `log.log` file includes timestamps for each entry, providing a basic logging mechanism.

### 🔑 Key Features
- ✅ Creates both content and log files
- ✅ Asynchronously writes to files for better performance
- ✅ Timestamps each log entry
- ✅ Demonstrates Rust's async/await capabilities

### 📋 Example Output

**main.txt**:
```
hello brother 1
hello brother 2
hello brother 3
...
```

**log.log**:
```
[12:39:48] [2025-05-09] -> Log.1. Appended Successfully
[12:39:48] [2025-05-09] -> Log.2. Appended Successfully
[12:39:48] [2025-05-09] -> Log.3. Appended Successfully
...
```

### 🚀 Implementation
- Uses Tokio runtime for async operations
- Appends to existing files or creates them if they don't exist
- Generates 99 log entries with timestamps

### 🔄 Dependencies
- 📦 tokio: For asynchronous I/O operations
- 📦 chrono: For timestamp generation

## 4. ⏱️ MathQuiz: Arithmetic Game with Timer in Rust


> Here is a sample code [url](https://github.com/BiMalxMe/Rust/blob/main/src/Practices/MathQuiz.rs)

This Rust application, `MathQuiz`, is a small command-line math quiz that tests your mental arithmetic using randomly generated questions and tracks how fast and accurately you answer.

### 🔑 Key Features
- ✅ Randomly generates 5 math problems using +, -, *, /
- ✅ Uses `eval` crate to compute expressions dynamically
- ✅ Times the total duration taken to solve
- ✅ Provides final score out of 5

### 📋 Example Output
![Screen Shot 2025-05-11 at 08 52 51](https://github.com/user-attachments/assets/84aee396-852b-4531-a5a6-0e8ae6bc7ffc)


### 🚀 Usage
1. Run the application  
2. Solve 5 math problems as quickly as possible  
3. Get score and time summary

### 🔄 Dependencies
- 📦 eval: For evaluating math expressions
- 📦 rand: For generating random numbers and operations
- 📦 std: For I/O and timing utilities
