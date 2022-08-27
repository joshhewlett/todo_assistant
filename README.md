# Todo Assistant
Simple CLI todo app built with Rust

## Why?
I've had an interest in learning Rust for a while now. As I was learning the basics following simple tutorials, I determined
working on a project of my own would be beneficial. Building a CLI Todo app seemed like a good entry-level project. It would
teach me project organization, accepting user input, processing data, control flow, cargo tools, basic persistence, error-
handling and more. All in a project of less than 10 files!

## How to run
1. [Install Rust](https://www.rust-lang.org/tools/install) (if required)
2. Clone the repo
```
git clone https://github.com/joshhewlett/todo_assistant.git
```
3. Navigate to the project root directory
4. Execute program
```
cargo run
```

## Example Execution
```
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/todo_assistant`

Please select an action:
i - List [i]ncomplete items   n - Create [n]ew item
a - List [a]ll items          c - [C]omplete item
h - List [h]istory            q - [Q]uit...
> a
============= All items ==============
 # | √ | Date due   | Title           
---|---|------------|-----------------
 0 | X | 2023-01-01 | Todo item one   
 1 |   | 2023-02-02 | Todo item two   
 2 |   | 2023-03-03 | Todo item three 


Please select an action:
i - List [i]ncomplete items   n - Create [n]ew item
a - List [a]ll items          c - [C]omplete item
h - List [h]istory            q - [Q]uit...
> n
Enter a new Todo Item or return to [m]enu:
Format: YYYY-MM-DD {Title}
> 2022-08-27 Create README for repo


Please select an action:
i - List [i]ncomplete items   n - Create [n]ew item
a - List [a]ll items          c - [C]omplete item
h - List [h]istory            q - [Q]uit...
> c
============= Incomplete items ==============
 # | √ | Date due   | Title                  
---|---|------------|------------------------
 3 |   | 2022-08-27 | Create README for repo 
 1 |   | 2023-02-02 | Todo item two          
 2 |   | 2023-03-03 | Todo item three        
Enter the ID of the completed item or return to [m]enu:
> 3


Please select an action:
i - List [i]ncomplete items   n - Create [n]ew item
a - List [a]ll items          c - [C]omplete item
h - List [h]istory            q - [Q]uit...
> h
============== Completed items ==============
 # | √ | Date due   | Title                  
---|---|------------|------------------------
 3 | X | 2022-08-27 | Create README for repo 
 0 | X | 2023-01-01 | Todo item one          


Please select an action:
i - List [i]ncomplete items   n - Create [n]ew item
a - List [a]ll items          c - [C]omplete item
h - List [h]istory            q - [Q]uit...
> q
Goodbye.
```
