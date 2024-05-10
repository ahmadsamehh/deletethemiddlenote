fn main() {
    let node: Vec<i32> = vec![1, 3, 4, 7, 1, 2, 6];
    //you can edit your node as you want.
    println!("UnEdited Node : {:?}", node);
    println!("Edited Node : {:?}", delete_middle_note(node));
}

fn delete_middle_note(mut node_to_edit: Vec<i32>) -> Vec<i32> {
    let nodelength = node_to_edit.len();
    let position_to_delete = nodelength / 2;
    println!(
        "element to be deleted : {}",
        node_to_edit[position_to_delete]
    );
    node_to_edit.remove(position_to_delete);
    return node_to_edit;
}
