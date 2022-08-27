pub mod todo_store {
    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io;
    use std::io::{BufReader, Read, Write};

    use crate::err::TodoError;
    use crate::todo::{TodoItem, TodoItemSerializable};

    #[derive(Debug, Serialize, Deserialize)]
    struct TodoStoreSerializable {
        store: Vec<TodoItemSerializable>,
        next_id: usize,
    }

    pub struct TodoStore {
        store: Vec<TodoItem>,
        next_id: usize,
        persistence_filepath: String
    }

    impl TodoStore {
        pub fn new_from_persistence(filepath: &str) -> Result<TodoStore, TodoError> {
            // Open and read persistence store file contents
            let persistence_store = File::options()
                .read(true)
                .create(true)
                .write(true)
                .open(filepath)
                .unwrap();

            let mut persistence_store_contents = String::new();
            BufReader::new(persistence_store)
                .read_to_string(&mut persistence_store_contents)
                .unwrap();

            // Deserialize persistence store
            let store_dto: TodoStoreSerializable =
                serde_json::from_str(&persistence_store_contents).map_err(|err| {
                    TodoError::new(
                        String::from("Error reading persistence file. Data is likely corrupted."),
                        Box::new(err),
                    )
                })?;

            // Create TodoItems from TodoItemSerializables
            let todo_items: Vec<TodoItem> = store_dto
                .store
                .into_iter()
                .map(|item| TodoItem::deserialize(item))
                .collect::<Result<Vec<TodoItem>, TodoError>>()?;

            // Calculate longest title length
            let mut longest_title_length = 0;
            todo_items.iter().for_each(|item| {
                if item.title.len() > longest_title_length {
                    longest_title_length = item.title.len();
                }
            });

            Ok(TodoStore {
                next_id: store_dto.next_id,
                store: todo_items,
                persistence_filepath: String::from(filepath)
            })
        }

        pub fn create_new_todo(&mut self) -> Result<(), TodoError> {
            println!("Enter a new Todo Item or return to [m]enu:");
            println!("Format: YYYY-MM-DD {{Title}}");
            print!("> ");
            io::stdout().flush().unwrap();

            let mut new_todo = String::new();
            io::stdin().read_line(&mut new_todo).map_err(|err| {
                TodoError::new(String::from("Failed to read line."), Box::new(err))
            })?;

            self.add_item(TodoItem::new(new_todo, self.next_id)?);
            Ok(())
        }

        pub fn mark_as_done(&mut self) -> Result<(), TodoError> {
            println!("Enter the ID of the completed item or return to [m]enu:");
            print!("> ");
            io::stdout().flush().unwrap();

            // Get user input
            let mut completed_todo_id = String::new();
            io::stdin()
                .read_line(&mut completed_todo_id)
                .map_err(|err| {
                    TodoError::new(String::from("Failed to read line."), Box::new(err))
                })?;

            // Validate and parse ID
            let completed_todo_id = completed_todo_id.trim().parse::<usize>().map_err(|err| {
                TodoError::new(String::from("Input must be an ID."), Box::new(err))
            })?;

            // Mark specified item as complete
            self.store
                .iter_mut()
                .find(|item| item.id == completed_todo_id)
                .ok_or(TodoError::new_from_msg(String::from(
                    "Please select a valid ID.",
                )))?
                .mark_as_done();

            self.persist_data();

            Ok(())
        }

        pub fn list_all_todos(&self) -> Vec<&TodoItem> {
            self.get_filtered_store(|_| true)
        }

        pub fn list_incomplete_todos(&self) -> Vec<&TodoItem> {
            self.get_filtered_store(|item: &&TodoItem| !item.complete)
        }

        pub fn list_history(&self) -> Vec<&TodoItem> {
            self.get_filtered_store(|item: &&TodoItem| item.complete)
        }

        fn add_item(&mut self, new_item: TodoItem) {

            self.store.push(new_item);
            self.sort_store();
            self.next_id += 1;
            self.persist_data();
        }

        fn sort_store(&mut self) {
            self.store.sort_by(|a, b| a.due_date.cmp(&b.due_date))
        }

        // TODO: Think about ways to optimize this.. Can we append data? How do we edit existing data?
        //   Maybe I can create a living file of appended "actions". On quit, the store is persisted and
        //   the action list is deleted. If on startup, that file exists, recreate the state
        fn persist_data(&self) {
            let store: Vec<TodoItemSerializable> =
                self.store.iter().map(TodoItemSerializable::from).collect();

            let store_dto = TodoStoreSerializable {
                next_id: self.next_id,
                store,
            };

            let store_dto_json = serde_json::to_string_pretty(&store_dto).unwrap();
            let mut persistence = File::create(&self.persistence_filepath).unwrap();
            persistence.write_all(store_dto_json.as_bytes()).unwrap();
        }

        fn get_filtered_store<F>(&self, filter: F) -> Vec<&TodoItem>
        where
            F: FnMut(&&TodoItem) -> bool, // TODO: Is a double reference necessary?
        {
            self.store.iter().filter(filter).by_ref().collect()
        }
    }
}

pub mod todo_printer {
    use crate::TodoItem;

    #[derive(Debug, Clone)]
    enum Justification {
        Left,
        Right,
    }

    type DataSupplier = dyn Fn(&TodoItem) -> String;

    struct TableColumn {
        pub header: &'static str,
        pub justification: Justification,
        pub data_supplier: Box<DataSupplier>,
        pub width: usize,
    }

    impl TableColumn {
        fn new(
            header: &'static str,
            justification: Justification,
            data_supplier: Box<DataSupplier>,
        ) -> TableColumn {
            TableColumn {
                header,
                justification,
                data_supplier,
                width: header.chars().count(),
            }
        }
    }

    fn get_columns() -> Vec<TableColumn> {
        vec![
            TableColumn::new(
                "#",
                Justification::Right,
                Box::new(|item| item.id.to_string()),
            ),
            TableColumn::new(
                "√",
                Justification::Left,
                Box::new(|item| match item.complete {
                    true => String::from("X"),
                    false => String::from(" "),
                }),
            ),
            TableColumn::new(
                "Date due",
                Justification::Left,
                Box::new(|item| item.due_date.to_string()),
            ),
            TableColumn::new(
                "Title",
                Justification::Left,
                Box::new(|item| item.title.to_string()),
            ),
        ]
    }

    pub fn print_store(data_title: &str, collection: &[&TodoItem]) {
        // Get max width of each column
        let mut table_columns = get_columns();

        // Set column widths
        collection.iter().for_each(|item| {
            for i in 0..table_columns.len() {
                let item_data_at_column = (table_columns[i].data_supplier)(item);
                if item_data_at_column.len() > table_columns[i].width {
                    table_columns[i].width = item_data_at_column.len();
                }
            }
        });

        // Print all data
        print_title(data_title, &table_columns);
        print_header_rows(&table_columns);
        collection
            .iter()
            .for_each(|item| print_table_row(&table_columns, item));
    }

    fn print_title(data_title: &str, columns: &[TableColumn]) {
        // Calculate the total width of the table
        let table_column_width_sum: usize = columns.iter().map(|column| column.width).sum();
        let total_table_width = table_column_width_sum +
            (columns.len() * 2) + // Padding chars
            (columns.len() - 1); // Dividers

        // Calculate padding char count for left and right padding
        let total_padding_char_count = total_table_width - (data_title.len() + 2);
        let left_padding_length = total_padding_char_count / 2;
        // If total_padding_char_count is odd, add an extra char to the right padding
        let right_padding_length = left_padding_length + (total_padding_char_count % 2);

        // Create left and right padding strings
        let left_padding = String::from("=").repeat(left_padding_length);
        let right_padding = String::from("=").repeat(right_padding_length);

        // Print title
        println!("{} {} {}", left_padding, data_title, right_padding);
    }

    fn print_header_rows(columns: &[TableColumn]) {
        // Initialize header and divider cells
        let mut header_cells: Vec<String> = Vec::new();
        let mut divider_cells: Vec<String> = Vec::new();

        // For every TableColumn, create a left-justified string representing each header cell and
        // a divider with the length of the column
        columns.iter().for_each(|column| {
            let header_right_padding_char_count = column.width - column.header.chars().count();
            header_cells.push(format!(
                " {}{} ",
                column.header,
                String::from(" ").repeat(header_right_padding_char_count)
            ));
            divider_cells.push(format!("-{}-", String::from("-").repeat(column.width)));
        });

        // Print out all cells separated by a pipe
        println!("{}", header_cells.join("|"));
        println!("{}", divider_cells.join("|"));
    }

    fn print_table_row(columns: &[TableColumn], item: &TodoItem) {
        // Create a string representing the TodoItem data for each column
        let table_row = columns
            .iter()
            .map(|column| {
                let data: String = (column.data_supplier)(item);
                let padding_char_count = column.width - data.chars().count();

                match column.justification {
                    Justification::Left => {
                        format!(" {}{} ", data, String::from(" ").repeat(padding_char_count))
                    },
                    Justification::Right => {
                        format!(" {}{} ", String::from(" ").repeat(padding_char_count), data)
                    },
                }
            })
            .collect::<Vec<String>>()
            .join("|");

        println!("{}", table_row);
    }
}
