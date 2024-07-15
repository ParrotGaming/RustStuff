mod linked_list;
use linked_list::LinkedList;
fn main() {
    let mut test_list = LinkedList::new();
    test_list.append("t".to_string());
    test_list.append("e".to_string());
    test_list.append("s".to_string());
    test_list.append("t".to_string());
    test_list.append("!".to_string());
    test_list.print();
}
