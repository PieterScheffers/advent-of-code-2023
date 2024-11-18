pub fn do_some_work(value: i32);

pub fn process_items(vec: Vec<i32>) {
    for item in vec {
        do_some_work(item);
    }
}
